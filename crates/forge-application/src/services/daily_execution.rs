use forge_domain::{DailyExecution, DailyExecutionId, DailyExecutionStatus, TaskId};
use time::{Date, OffsetDateTime};

use crate::AppError;
use crate::repos::{DailyExecutionRepository, TaskRepository};

pub struct CreateDailyExecution {
    pub execution_date: Date,
    pub notes: Option<String>,
    pub status: DailyExecutionStatus,
}

pub struct UpdateDailyExecution {
    pub notes: Option<String>,
    pub status: DailyExecutionStatus,
}

#[derive(Clone)]
pub struct DailyExecutionService<T, E> {
    tasks: T,
    executions: E,
}

impl<T, E> DailyExecutionService<T, E>
where
    T: TaskRepository,
    E: DailyExecutionRepository,
{
    pub fn new(tasks: T, executions: E) -> Self {
        Self { tasks, executions }
    }

    pub async fn create(
        &self,
        task_id: TaskId,
        cmd: CreateDailyExecution,
        now: OffsetDateTime,
    ) -> Result<DailyExecution, AppError> {
        let task = self
            .tasks
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::not_found("task", task_id))?;
        if task.status() == forge_domain::TaskStatus::Cancelled {
            return Err(AppError::conflict(
                "cannot add a daily execution to a cancelled task",
            ));
        }
        let execution =
            DailyExecution::create(task_id, cmd.execution_date, cmd.notes, cmd.status, now);
        self.executions.create(&execution).await?;
        Ok(execution)
    }

    pub async fn get(&self, id: DailyExecutionId) -> Result<DailyExecution, AppError> {
        self.executions
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("daily_execution", id))
    }

    pub async fn list_by_task(&self, task_id: TaskId) -> Result<Vec<DailyExecution>, AppError> {
        self.tasks
            .get(task_id)
            .await?
            .ok_or_else(|| AppError::not_found("task", task_id))?;
        self.executions.list_by_task(task_id).await
    }

    pub async fn list_by_date(
        &self,
        execution_date: Date,
    ) -> Result<Vec<DailyExecution>, AppError> {
        self.executions.list_by_date(execution_date).await
    }

    pub async fn update(
        &self,
        id: DailyExecutionId,
        cmd: UpdateDailyExecution,
        now: OffsetDateTime,
    ) -> Result<DailyExecution, AppError> {
        let mut execution = self.get(id).await?;
        execution.update(cmd.notes, cmd.status, now);
        self.executions.update(&execution).await?;
        Ok(execution)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::services::cycle::{CreateCycle, CycleService};
    use crate::services::objective::{CreateObjective, ObjectiveService};
    use crate::services::project::{CreateProject, ProjectService};
    use crate::services::task::{CreateTask, TaskService};
    use crate::test_support::{
        InMemoryCycleRepo, InMemoryDailyExecutionRepo, InMemoryObjectiveRepo, InMemoryProjectRepo,
        InMemoryTaskRepo,
    };

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[tokio::test]
    async fn create_and_list_by_date() {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let projects = InMemoryProjectRepo::default();
        let tasks = InMemoryTaskRepo::default();
        let executions = InMemoryDailyExecutionRepo::default();

        let cycle = CycleService::new(cycles.clone())
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
        let objective = ObjectiveService::new(cycles.clone(), objectives.clone())
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
        let project_svc = ProjectService::new(cycles.clone(), objectives.clone(), projects.clone());
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
        let task = TaskService::new(cycles, objectives, projects, tasks.clone())
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
        let svc = DailyExecutionService::new(tasks, executions);
        let created = svc
            .create(
                task.id(),
                CreateDailyExecution {
                    execution_date: date!(2026 - 01 - 15),
                    notes: Some("notes".into()),
                    status: DailyExecutionStatus::Planned,
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(
            svc.list_by_date(date!(2026 - 01 - 15)).await.unwrap().len(),
            1
        );
        assert_eq!(created.task_id(), task.id());
    }
}
