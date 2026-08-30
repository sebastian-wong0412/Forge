use forge_domain::{ProjectId, Task, TaskId, Title, TodayBucket, today_bucket};
use time::{Date, OffsetDateTime};

use crate::AppError;
use crate::repos::{CycleRepository, ObjectiveRepository, ProjectRepository, TaskRepository};

pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub scheduled_on: Option<Date>,
}

pub struct UpdateTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodayResult {
    pub date: Date,
    pub scheduled: Vec<Task>,
    pub overdue: Vec<Task>,
    pub unscheduled_in_progress: Vec<Task>,
    pub completed: Vec<Task>,
}

#[derive(Clone)]
pub struct TaskService<C, O, P, T> {
    cycles: C,
    objectives: O,
    projects: P,
    tasks: T,
}

impl<C, O, P, T> TaskService<C, O, P, T>
where
    C: CycleRepository,
    O: ObjectiveRepository,
    P: ProjectRepository,
    T: TaskRepository,
{
    pub fn new(cycles: C, objectives: O, projects: P, tasks: T) -> Self {
        Self {
            cycles,
            objectives,
            projects,
            tasks,
        }
    }

    pub async fn create(
        &self,
        project_id: ProjectId,
        cmd: CreateTask,
        now: OffsetDateTime,
    ) -> Result<Task, AppError> {
        let project = self
            .projects
            .get(project_id)
            .await?
            .ok_or_else(|| AppError::not_found("project", project_id))?;
        if !project.status().allows_tasks() {
            return Err(AppError::conflict(
                "cannot add a task unless the project is draft or active",
            ));
        }
        let objective = self
            .objectives
            .get(project.objective_id())
            .await?
            .ok_or_else(|| AppError::not_found("objective", project.objective_id()))?;
        if !objective.status().allows_children() {
            return Err(AppError::conflict(
                "cannot add a task when the objective is completed or archived",
            ));
        }
        let cycle = self
            .cycles
            .get(objective.cycle_id())
            .await?
            .ok_or_else(|| AppError::not_found("cycle", objective.cycle_id()))?;
        if !cycle.status().allows_tree_mutation() {
            return Err(AppError::conflict(
                "cannot add a task to a closed or archived cycle",
            ));
        }
        let title = Title::parse(cmd.title)?;
        let task = Task::create(project_id, title, cmd.description, cmd.scheduled_on, now);
        self.tasks.create(&task).await?;
        Ok(task)
    }

    pub async fn get(&self, id: TaskId) -> Result<Task, AppError> {
        self.tasks
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("task", id))
    }

    pub async fn list_by_project(&self, project_id: ProjectId) -> Result<Vec<Task>, AppError> {
        self.projects
            .get(project_id)
            .await?
            .ok_or_else(|| AppError::not_found("project", project_id))?;
        self.tasks.list_by_project(project_id).await
    }

    pub async fn update(
        &self,
        id: TaskId,
        cmd: UpdateTask,
        now: OffsetDateTime,
    ) -> Result<Task, AppError> {
        let mut task = self.get(id).await?;
        let title = Title::parse(cmd.title)?;
        task.update(title, cmd.description, now)?;
        self.tasks.update(&task).await?;
        Ok(task)
    }

    pub async fn start(&self, id: TaskId, now: OffsetDateTime) -> Result<Task, AppError> {
        let mut task = self.get(id).await?;
        let mut ancestry = crate::parent_progression::load_ancestry(
            &self.cycles,
            &self.objectives,
            &self.projects,
            task.project_id(),
        )
        .await?;

        let cycle_change = crate::parent_progression::ensure_cycle(&mut ancestry.cycle, now)?;
        let objective_change =
            crate::parent_progression::ensure_objective(&mut ancestry.objective, now)?;
        let project_change = crate::parent_progression::ensure_project(&mut ancestry.project, now)?;
        task.start(now)?;

        crate::parent_progression::persist_activated(
            &self.cycles,
            &self.objectives,
            &self.projects,
            crate::parent_progression::ActivatedParents {
                cycle: &ancestry.cycle,
                cycle_change,
                objective: &ancestry.objective,
                objective_change,
                project: Some((&ancestry.project, project_change)),
            },
        )
        .await?;
        self.tasks.update(&task).await?;
        Ok(task)
    }

    pub async fn complete(&self, id: TaskId, now: OffsetDateTime) -> Result<Task, AppError> {
        let mut task = self.get(id).await?;
        task.complete(now)?;
        self.tasks.update(&task).await?;
        Ok(task)
    }

    pub async fn cancel(&self, id: TaskId, now: OffsetDateTime) -> Result<Task, AppError> {
        let mut task = self.get(id).await?;
        task.cancel(now)?;
        self.tasks.update(&task).await?;
        Ok(task)
    }

    pub async fn schedule(
        &self,
        id: TaskId,
        scheduled_on: Option<Date>,
        now: OffsetDateTime,
    ) -> Result<Task, AppError> {
        let mut task = self.get(id).await?;
        task.schedule(scheduled_on, now)?;
        self.tasks.update(&task).await?;
        Ok(task)
    }

    pub async fn today(&self, date: Date) -> Result<TodayResult, AppError> {
        let candidates = self.tasks.list_today_candidates(date).await?;
        let mut result = TodayResult {
            date,
            scheduled: Vec::new(),
            overdue: Vec::new(),
            unscheduled_in_progress: Vec::new(),
            completed: Vec::new(),
        };
        for task in candidates {
            match today_bucket(&task, date) {
                Some(TodayBucket::Scheduled) => result.scheduled.push(task),
                Some(TodayBucket::Overdue) => result.overdue.push(task),
                Some(TodayBucket::UnscheduledInProgress) => {
                    result.unscheduled_in_progress.push(task)
                }
                Some(TodayBucket::Completed) => result.completed.push(task),
                None => {}
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::services::cycle::{CreateCycle, CycleService};
    use crate::services::objective::{CreateObjective, ObjectiveService};
    use crate::services::project::{CreateProject, ProjectService};
    use crate::test_support::{
        InMemoryCycleRepo, InMemoryObjectiveRepo, InMemoryProjectRepo, InMemoryTaskRepo,
    };
    use forge_domain::TaskStatus;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[tokio::test]
    async fn draft_or_active_project_can_create_task() {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let projects = InMemoryProjectRepo::default();
        let tasks = InMemoryTaskRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
        let project_svc = ProjectService::new(cycles.clone(), objectives.clone(), projects.clone());
        let task_svc =
            TaskService::new(cycles.clone(), objectives.clone(), projects.clone(), tasks);

        let cycle = cycle_svc
            .create(
                CreateCycle {
                    name: "Q1".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        let objective = objective_svc
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let project = project_svc
            .create(
                objective.id(),
                CreateProject {
                    title: "Work".into(),
                    description: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let on_draft = task_svc
            .create(
                project.id(),
                CreateTask {
                    title: "Plan it".into(),
                    description: None,
                    scheduled_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(on_draft.status(), TaskStatus::Todo);

        project_svc.complete(project.id(), NOW).await.unwrap_err();
        project_svc.activate(project.id(), NOW).await.unwrap();
        let task = task_svc
            .create(
                project.id(),
                CreateTask {
                    title: "Do it".into(),
                    description: None,
                    scheduled_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let started = task_svc.start(task.id(), NOW).await.unwrap();
        assert_eq!(started.status(), TaskStatus::InProgress);
        let done = task_svc.complete(task.id(), NOW).await.unwrap();
        assert_eq!(done.status(), TaskStatus::Done);
        assert!(task_svc.cancel(task.id(), NOW).await.is_err());

        project_svc.complete(project.id(), NOW).await.unwrap();
        let blocked = task_svc
            .create(
                project.id(),
                CreateTask {
                    title: "Too late".into(),
                    description: None,
                    scheduled_on: None,
                },
                NOW,
            )
            .await
            .unwrap_err();
        assert!(matches!(blocked, AppError::Conflict { .. }));
    }

    const TODAY: Date = date!(2026 - 08 - 30);
    const YESTERDAY: Date = date!(2026 - 08 - 29);
    const TOMORROW: Date = date!(2026 - 08 - 31);
    const COMPLETED_TODAY: OffsetDateTime = datetime!(2026-08-30 15:00:00 UTC);
    const COMPLETED_TOMORROW: OffsetDateTime = datetime!(2026-08-31 15:00:00 UTC);

    struct Fixture {
        tasks: TaskService<
            InMemoryCycleRepo,
            InMemoryObjectiveRepo,
            InMemoryProjectRepo,
            InMemoryTaskRepo,
        >,
        task_repo: InMemoryTaskRepo,
        project_id: forge_domain::ProjectId,
    }

    async fn fixture() -> Fixture {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let projects = InMemoryProjectRepo::default();
        let task_repo = InMemoryTaskRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
        let project_svc = ProjectService::new(cycles.clone(), objectives.clone(), projects.clone());
        let tasks = TaskService::new(cycles, objectives, projects, task_repo.clone());
        let cycle = cycle_svc
            .create(
                CreateCycle {
                    name: "Q1".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        let objective = objective_svc
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let project = project_svc
            .create(
                objective.id(),
                CreateProject {
                    title: "Work".into(),
                    description: None,
                },
                NOW,
            )
            .await
            .unwrap();
        project_svc.activate(project.id(), NOW).await.unwrap();
        Fixture {
            tasks,
            task_repo,
            project_id: project.id(),
        }
    }

    async fn create_named(fx: &Fixture, title: &str, scheduled_on: Option<Date>) -> Task {
        fx.tasks
            .create(
                fx.project_id,
                CreateTask {
                    title: title.into(),
                    description: None,
                    scheduled_on,
                },
                NOW,
            )
            .await
            .unwrap()
    }

    fn ids(tasks: &[Task]) -> Vec<TaskId> {
        tasks.iter().map(Task::id).collect()
    }

    #[tokio::test]
    async fn create_without_scheduled_on() {
        let fx = fixture().await;
        let task = create_named(&fx, "Unscheduled", None).await;
        assert_eq!(task.scheduled_on(), None);
        assert_eq!(fx.tasks.get(task.id()).await.unwrap().scheduled_on(), None);
    }

    #[tokio::test]
    async fn create_with_scheduled_on() {
        let fx = fixture().await;
        let task = create_named(&fx, "Planned", Some(TODAY)).await;
        assert_eq!(task.scheduled_on(), Some(TODAY));
        assert_eq!(
            fx.tasks.get(task.id()).await.unwrap().scheduled_on(),
            Some(TODAY)
        );
    }

    #[tokio::test]
    async fn schedule_todo() {
        let fx = fixture().await;
        let task = create_named(&fx, "Todo", None).await;
        let scheduled = fx
            .tasks
            .schedule(task.id(), Some(TODAY), NOW)
            .await
            .unwrap();
        assert_eq!(scheduled.scheduled_on(), Some(TODAY));
        assert_eq!(scheduled.status(), TaskStatus::Todo);
    }

    #[tokio::test]
    async fn reschedule_todo() {
        let fx = fixture().await;
        let task = create_named(&fx, "Todo", Some(TODAY)).await;
        let scheduled = fx
            .tasks
            .schedule(task.id(), Some(TOMORROW), NOW)
            .await
            .unwrap();
        assert_eq!(scheduled.scheduled_on(), Some(TOMORROW));
    }

    #[tokio::test]
    async fn unschedule_todo() {
        let fx = fixture().await;
        let task = create_named(&fx, "Todo", Some(TODAY)).await;
        let unscheduled = fx.tasks.schedule(task.id(), None, NOW).await.unwrap();
        assert_eq!(unscheduled.scheduled_on(), None);
    }

    #[tokio::test]
    async fn schedule_in_progress() {
        let fx = fixture().await;
        let task = create_named(&fx, "Started", None).await;
        fx.tasks.start(task.id(), NOW).await.unwrap();
        let scheduled = fx
            .tasks
            .schedule(task.id(), Some(TODAY), NOW)
            .await
            .unwrap();
        assert_eq!(scheduled.scheduled_on(), Some(TODAY));
        assert_eq!(scheduled.status(), TaskStatus::InProgress);
    }

    #[tokio::test]
    async fn terminal_task_cannot_schedule() {
        let fx = fixture().await;
        let task = create_named(&fx, "Done", None).await;
        fx.tasks.start(task.id(), NOW).await.unwrap();
        fx.tasks.complete(task.id(), NOW).await.unwrap();
        let err = fx
            .tasks
            .schedule(task.id(), Some(TODAY), NOW)
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
    }

    #[tokio::test]
    async fn terminal_task_cannot_unschedule() {
        let fx = fixture().await;
        let task = create_named(&fx, "Done", Some(TODAY)).await;
        fx.tasks.start(task.id(), NOW).await.unwrap();
        fx.tasks.complete(task.id(), NOW).await.unwrap();
        let err = fx.tasks.schedule(task.id(), None, NOW).await.unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
        assert_eq!(
            fx.tasks.get(task.id()).await.unwrap().scheduled_on(),
            Some(TODAY)
        );
    }

    #[tokio::test]
    async fn schedule_persists_through_repository() {
        let fx = fixture().await;
        let task = create_named(&fx, "Persisted", None).await;
        fx.tasks
            .schedule(task.id(), Some(TODAY), NOW)
            .await
            .unwrap();
        assert_eq!(
            fx.task_repo
                .get(task.id())
                .await
                .unwrap()
                .unwrap()
                .scheduled_on(),
            Some(TODAY)
        );
    }

    #[tokio::test]
    async fn today_scheduled() {
        let fx = fixture().await;
        let task = create_named(&fx, "Today", Some(TODAY)).await;
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert_eq!(today.date, TODAY);
        assert_eq!(ids(&today.scheduled), vec![task.id()]);
        assert!(today.overdue.is_empty());
    }

    #[tokio::test]
    async fn today_overdue() {
        let fx = fixture().await;
        let task = create_named(&fx, "Late", Some(YESTERDAY)).await;
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert_eq!(ids(&today.overdue), vec![task.id()]);
        assert!(today.scheduled.is_empty());
    }

    #[tokio::test]
    async fn today_unscheduled_in_progress() {
        let fx = fixture().await;
        let task = create_named(&fx, "Going", None).await;
        fx.tasks.start(task.id(), NOW).await.unwrap();
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert_eq!(ids(&today.unscheduled_in_progress), vec![task.id()]);
    }

    #[tokio::test]
    async fn today_completed_on_requested_utc_date() {
        let fx = fixture().await;
        let task = create_named(&fx, "Finished", None).await;
        fx.tasks.start(task.id(), NOW).await.unwrap();
        fx.tasks.complete(task.id(), COMPLETED_TODAY).await.unwrap();
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert_eq!(ids(&today.completed), vec![task.id()]);
    }

    #[tokio::test]
    async fn today_excludes_completed_on_another_date() {
        let fx = fixture().await;
        let task = create_named(&fx, "Later", None).await;
        fx.tasks.start(task.id(), NOW).await.unwrap();
        fx.tasks
            .complete(task.id(), COMPLETED_TOMORROW)
            .await
            .unwrap();
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert!(today.completed.is_empty());
        assert!(!ids(&today.completed).contains(&task.id()));
    }

    #[tokio::test]
    async fn today_excludes_future_scheduled() {
        let fx = fixture().await;
        create_named(&fx, "Tomorrow", Some(TOMORROW)).await;
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert!(today.scheduled.is_empty());
        assert!(today.overdue.is_empty());
    }

    #[tokio::test]
    async fn today_excludes_unscheduled_todo() {
        let fx = fixture().await;
        create_named(&fx, "Inbox", None).await;
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert!(today.scheduled.is_empty());
        assert!(today.unscheduled_in_progress.is_empty());
    }

    #[tokio::test]
    async fn today_excludes_cancelled() {
        let fx = fixture().await;
        let task = create_named(&fx, "Dropped", Some(TODAY)).await;
        fx.tasks.cancel(task.id(), NOW).await.unwrap();
        let today = fx.tasks.today(TODAY).await.unwrap();
        assert!(today.scheduled.is_empty());
        assert!(today.overdue.is_empty());
        assert!(today.completed.is_empty());
    }

    #[tokio::test]
    async fn today_task_appears_in_exactly_one_bucket() {
        let fx = fixture().await;
        create_named(&fx, "Scheduled", Some(TODAY)).await;
        create_named(&fx, "Overdue", Some(YESTERDAY)).await;
        let started = create_named(&fx, "Started", None).await;
        fx.tasks.start(started.id(), NOW).await.unwrap();
        let done = create_named(&fx, "Done", Some(TODAY)).await;
        fx.tasks.start(done.id(), NOW).await.unwrap();
        fx.tasks.complete(done.id(), COMPLETED_TODAY).await.unwrap();

        let today = fx.tasks.today(TODAY).await.unwrap();
        let mut all = ids(&today.scheduled);
        all.extend(ids(&today.overdue));
        all.extend(ids(&today.unscheduled_in_progress));
        all.extend(ids(&today.completed));
        let unique = all
            .iter()
            .map(ToString::to_string)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(all.len(), unique.len());
        assert_eq!(all.len(), 4);
    }

    #[tokio::test]
    async fn daily_execution_does_not_affect_today() {
        use crate::services::daily_execution::{CreateDailyExecution, DailyExecutionService};
        use crate::test_support::InMemoryDailyExecutionRepo;
        use forge_domain::DailyExecutionStatus;

        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let projects = InMemoryProjectRepo::default();
        let task_repo = InMemoryTaskRepo::default();
        let executions = InMemoryDailyExecutionRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
        let project_svc = ProjectService::new(cycles.clone(), objectives.clone(), projects.clone());
        let tasks = TaskService::new(cycles, objectives, projects, task_repo.clone());
        let cycle = cycle_svc
            .create(
                CreateCycle {
                    name: "Q1".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        let objective = objective_svc
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let project = project_svc
            .create(
                objective.id(),
                CreateProject {
                    title: "Work".into(),
                    description: None,
                },
                NOW,
            )
            .await
            .unwrap();
        project_svc.activate(project.id(), NOW).await.unwrap();
        let unscheduled = tasks
            .create(
                project.id(),
                CreateTask {
                    title: "Inbox".into(),
                    description: None,
                    scheduled_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        DailyExecutionService::new(task_repo, executions)
            .create(
                unscheduled.id(),
                CreateDailyExecution {
                    execution_date: TODAY,
                    notes: Some("legacy".into()),
                    status: DailyExecutionStatus::Planned,
                },
                NOW,
            )
            .await
            .unwrap();
        let today = tasks.today(TODAY).await.unwrap();
        assert!(today.scheduled.is_empty());
        assert!(today.overdue.is_empty());
        assert!(today.unscheduled_in_progress.is_empty());
        assert!(today.completed.is_empty());
    }

    #[tokio::test]
    async fn supplied_date_determines_today() {
        let fx = fixture().await;
        let task = create_named(&fx, "Move", Some(TODAY)).await;
        let on_today = fx.tasks.today(TODAY).await.unwrap();
        let on_tomorrow = fx.tasks.today(TOMORROW).await.unwrap();
        assert_eq!(ids(&on_today.scheduled), vec![task.id()]);
        assert!(on_today.overdue.is_empty());
        assert!(on_tomorrow.scheduled.is_empty());
        assert_eq!(ids(&on_tomorrow.overdue), vec![task.id()]);
    }

    struct DraftTree {
        cycle_svc: CycleService<InMemoryCycleRepo>,
        objective_svc: ObjectiveService<InMemoryCycleRepo, InMemoryObjectiveRepo>,
        project_svc: ProjectService<InMemoryCycleRepo, InMemoryObjectiveRepo, InMemoryProjectRepo>,
        task_svc: TaskService<
            InMemoryCycleRepo,
            InMemoryObjectiveRepo,
            InMemoryProjectRepo,
            InMemoryTaskRepo,
        >,
        cycle_id: forge_domain::CycleId,
        objective_id: forge_domain::ObjectiveId,
        project_id: forge_domain::ProjectId,
        task_id: TaskId,
    }

    async fn draft_tree() -> DraftTree {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let projects = InMemoryProjectRepo::default();
        let tasks = InMemoryTaskRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
        let project_svc = ProjectService::new(cycles.clone(), objectives.clone(), projects.clone());
        let task_svc =
            TaskService::new(cycles.clone(), objectives.clone(), projects.clone(), tasks);
        let cycle = cycle_svc
            .create(
                CreateCycle {
                    name: "Q1".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        let objective = objective_svc
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let project = project_svc
            .create(
                objective.id(),
                CreateProject {
                    title: "Work".into(),
                    description: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let task = task_svc
            .create(
                project.id(),
                CreateTask {
                    title: "Do it".into(),
                    description: None,
                    scheduled_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        DraftTree {
            cycle_svc,
            objective_svc,
            project_svc,
            task_svc,
            cycle_id: cycle.id(),
            objective_id: objective.id(),
            project_id: project.id(),
            task_id: task.id(),
        }
    }

    #[tokio::test]
    async fn start_task_activates_full_parent_chain() {
        let tree = draft_tree().await;
        let started = tree.task_svc.start(tree.task_id, NOW).await.unwrap();
        assert_eq!(started.status(), TaskStatus::InProgress);
        assert_eq!(
            tree.cycle_svc.get(tree.cycle_id).await.unwrap().status(),
            forge_domain::CycleStatus::Active
        );
        assert_eq!(
            tree.objective_svc
                .get(tree.objective_id)
                .await
                .unwrap()
                .status(),
            forge_domain::ObjectiveStatus::Active
        );
        assert_eq!(
            tree.project_svc
                .get(tree.project_id)
                .await
                .unwrap()
                .status(),
            forge_domain::ProjectStatus::Active
        );
    }

    #[tokio::test]
    async fn start_task_skips_already_active_parents() {
        let tree = draft_tree().await;
        tree.cycle_svc.activate(tree.cycle_id, NOW).await.unwrap();
        tree.objective_svc
            .activate(tree.objective_id, NOW)
            .await
            .unwrap();
        tree.project_svc
            .activate(tree.project_id, NOW)
            .await
            .unwrap();
        let cycle_updated = tree
            .cycle_svc
            .get(tree.cycle_id)
            .await
            .unwrap()
            .updated_at();
        let started = tree.task_svc.start(tree.task_id, NOW).await.unwrap();
        assert_eq!(started.status(), TaskStatus::InProgress);
        assert_eq!(
            tree.cycle_svc
                .get(tree.cycle_id)
                .await
                .unwrap()
                .updated_at(),
            cycle_updated
        );
    }

    #[tokio::test]
    async fn start_task_rejects_closed_cycle_without_mutating_task() {
        let tree = draft_tree().await;
        tree.cycle_svc.activate(tree.cycle_id, NOW).await.unwrap();
        tree.cycle_svc.close(tree.cycle_id, NOW).await.unwrap();
        let err = tree.task_svc.start(tree.task_id, NOW).await.unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
        assert_eq!(
            tree.task_svc.get(tree.task_id).await.unwrap().status(),
            TaskStatus::Todo
        );
        assert_eq!(
            tree.project_svc
                .get(tree.project_id)
                .await
                .unwrap()
                .status(),
            forge_domain::ProjectStatus::Draft
        );
        assert_eq!(
            tree.objective_svc
                .get(tree.objective_id)
                .await
                .unwrap()
                .status(),
            forge_domain::ObjectiveStatus::Draft
        );
    }

    #[tokio::test]
    async fn start_task_rejects_completed_objective_without_mutating_task() {
        let tree = draft_tree().await;
        tree.objective_svc
            .activate(tree.objective_id, NOW)
            .await
            .unwrap();
        tree.objective_svc
            .complete(tree.objective_id, NOW)
            .await
            .unwrap();
        let err = tree.task_svc.start(tree.task_id, NOW).await.unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
        assert_eq!(
            tree.task_svc.get(tree.task_id).await.unwrap().status(),
            TaskStatus::Todo
        );
        assert_eq!(
            tree.project_svc
                .get(tree.project_id)
                .await
                .unwrap()
                .status(),
            forge_domain::ProjectStatus::Draft
        );
    }

    #[tokio::test]
    async fn start_task_rejects_archived_project_without_mutating_task() {
        let tree = draft_tree().await;
        tree.project_svc
            .archive(tree.project_id, NOW)
            .await
            .unwrap();
        let err = tree.task_svc.start(tree.task_id, NOW).await.unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
        assert_eq!(
            tree.task_svc.get(tree.task_id).await.unwrap().status(),
            TaskStatus::Todo
        );
        assert_eq!(
            tree.cycle_svc.get(tree.cycle_id).await.unwrap().status(),
            forge_domain::CycleStatus::Planning
        );
    }

    #[tokio::test]
    async fn complete_and_schedule_do_not_activate_parents() {
        let tree = draft_tree().await;
        tree.task_svc
            .schedule(tree.task_id, Some(TODAY), NOW)
            .await
            .unwrap();
        assert_eq!(
            tree.cycle_svc.get(tree.cycle_id).await.unwrap().status(),
            forge_domain::CycleStatus::Planning
        );
        tree.task_svc.start(tree.task_id, NOW).await.unwrap();
        tree.task_svc.complete(tree.task_id, NOW).await.unwrap();
        assert_eq!(
            tree.cycle_svc.get(tree.cycle_id).await.unwrap().status(),
            forge_domain::CycleStatus::Active
        );
    }
}
