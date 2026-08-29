use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use forge_domain::{
    CheckIn, CheckInId, Cycle, CycleId, DailyExecution, DailyExecutionId, KeyResult, KeyResultId,
    Objective, ObjectiveId, Project, ProjectId, Review, ReviewId, Task, TaskId,
};
use time::{Date, OffsetDateTime};

use crate::AppError;
use crate::repos::{
    CheckInRepository, CycleRepository, DailyExecutionRepository, KeyResultRepository,
    ObjectiveRepository, ProjectRepository, ReviewRepository, TaskRepository,
};

fn lock<'a, T>(mutex: &'a Mutex<T>) -> Result<std::sync::MutexGuard<'a, T>, AppError> {
    mutex
        .lock()
        .map_err(|_| AppError::persistence("in-memory lock poisoned"))
}

fn created_key(created_at: OffsetDateTime, id: impl ToString) -> (OffsetDateTime, String) {
    (created_at, id.to_string())
}

#[derive(Clone, Default)]
pub struct InMemoryCycleRepo {
    inner: Arc<Mutex<HashMap<CycleId, Cycle>>>,
}

impl CycleRepository for InMemoryCycleRepo {
    async fn create(&self, cycle: &Cycle) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&cycle.id()) {
            return Err(AppError::conflict("cycle already exists"));
        }
        items.insert(cycle.id(), cycle.clone());
        Ok(())
    }

    async fn get(&self, id: CycleId) -> Result<Option<Cycle>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list(&self) -> Result<Vec<Cycle>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?.values().cloned().collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }

    async fn update(&self, cycle: &Cycle) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if !items.contains_key(&cycle.id()) {
            return Err(AppError::not_found("cycle", cycle.id()));
        }
        items.insert(cycle.id(), cycle.clone());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryObjectiveRepo {
    inner: Arc<Mutex<HashMap<ObjectiveId, Objective>>>,
}

impl ObjectiveRepository for InMemoryObjectiveRepo {
    async fn create(&self, objective: &Objective) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&objective.id()) {
            return Err(AppError::conflict("objective already exists"));
        }
        items.insert(objective.id(), objective.clone());
        Ok(())
    }

    async fn get(&self, id: ObjectiveId) -> Result<Option<Objective>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list_by_cycle(&self, cycle_id: CycleId) -> Result<Vec<Objective>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.cycle_id() == cycle_id)
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }

    async fn update(&self, objective: &Objective) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if !items.contains_key(&objective.id()) {
            return Err(AppError::not_found("objective", objective.id()));
        }
        items.insert(objective.id(), objective.clone());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryKeyResultRepo {
    inner: Arc<Mutex<HashMap<KeyResultId, KeyResult>>>,
}

impl KeyResultRepository for InMemoryKeyResultRepo {
    async fn create(&self, key_result: &KeyResult) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&key_result.id()) {
            return Err(AppError::conflict("key result already exists"));
        }
        items.insert(key_result.id(), key_result.clone());
        Ok(())
    }

    async fn get(&self, id: KeyResultId) -> Result<Option<KeyResult>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list_by_objective(
        &self,
        objective_id: ObjectiveId,
    ) -> Result<Vec<KeyResult>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.objective_id() == objective_id)
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }

    async fn update(&self, key_result: &KeyResult) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if !items.contains_key(&key_result.id()) {
            return Err(AppError::not_found("key_result", key_result.id()));
        }
        items.insert(key_result.id(), key_result.clone());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryCheckInRepo {
    inner: Arc<Mutex<HashMap<CheckInId, CheckIn>>>,
}

impl CheckInRepository for InMemoryCheckInRepo {
    async fn create(&self, check_in: &CheckIn) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&check_in.id()) {
            return Err(AppError::conflict("check-in already exists"));
        }
        items.insert(check_in.id(), check_in.clone());
        Ok(())
    }

    async fn get(&self, id: CheckInId) -> Result<Option<CheckIn>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list_by_key_result(
        &self,
        key_result_id: KeyResultId,
    ) -> Result<Vec<CheckIn>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.key_result_id() == key_result_id)
            .cloned()
            .collect();
        items.sort_by(|left, right| {
            left.checked_on()
                .cmp(&right.checked_on())
                .then_with(|| left.created_at().cmp(&right.created_at()))
                .then_with(|| left.id().as_uuid().cmp(&right.id().as_uuid()))
        });
        Ok(items)
    }
}

#[derive(Clone, Default)]
pub struct InMemoryProjectRepo {
    inner: Arc<Mutex<HashMap<ProjectId, Project>>>,
}

impl ProjectRepository for InMemoryProjectRepo {
    async fn create(&self, project: &Project) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&project.id()) {
            return Err(AppError::conflict("project already exists"));
        }
        items.insert(project.id(), project.clone());
        Ok(())
    }

    async fn get(&self, id: ProjectId) -> Result<Option<Project>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list_by_objective(&self, objective_id: ObjectiveId) -> Result<Vec<Project>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.objective_id() == objective_id)
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }

    async fn update(&self, project: &Project) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if !items.contains_key(&project.id()) {
            return Err(AppError::not_found("project", project.id()));
        }
        items.insert(project.id(), project.clone());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryTaskRepo {
    inner: Arc<Mutex<HashMap<TaskId, Task>>>,
}

impl TaskRepository for InMemoryTaskRepo {
    async fn create(&self, task: &Task) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&task.id()) {
            return Err(AppError::conflict("task already exists"));
        }
        items.insert(task.id(), task.clone());
        Ok(())
    }

    async fn get(&self, id: TaskId) -> Result<Option<Task>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list_by_project(&self, project_id: ProjectId) -> Result<Vec<Task>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.project_id() == project_id)
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }

    async fn update(&self, task: &Task) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if !items.contains_key(&task.id()) {
            return Err(AppError::not_found("task", task.id()));
        }
        items.insert(task.id(), task.clone());
        Ok(())
    }

    async fn list_today_candidates(&self, date: Date) -> Result<Vec<Task>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| is_today_candidate(item, date))
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }
}

fn is_today_candidate(task: &forge_domain::Task, date: Date) -> bool {
    use forge_domain::TaskStatus;
    match task.status() {
        TaskStatus::Cancelled => false,
        TaskStatus::Done => task
            .completed_at()
            .is_some_and(|completed_at| completed_at.date() == date),
        TaskStatus::Todo => task
            .scheduled_on()
            .is_some_and(|scheduled_on| scheduled_on <= date),
        TaskStatus::InProgress => match task.scheduled_on() {
            Some(scheduled_on) => scheduled_on <= date,
            None => true,
        },
    }
}

#[derive(Clone, Default)]
pub struct InMemoryDailyExecutionRepo {
    inner: Arc<Mutex<HashMap<DailyExecutionId, DailyExecution>>>,
}

impl DailyExecutionRepository for InMemoryDailyExecutionRepo {
    async fn create(&self, execution: &DailyExecution) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&execution.id()) {
            return Err(AppError::conflict("daily execution already exists"));
        }
        items.insert(execution.id(), execution.clone());
        Ok(())
    }

    async fn get(&self, id: DailyExecutionId) -> Result<Option<DailyExecution>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list_by_task(&self, task_id: TaskId) -> Result<Vec<DailyExecution>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.task_id() == task_id)
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }

    async fn list_by_date(&self, execution_date: Date) -> Result<Vec<DailyExecution>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.execution_date() == execution_date)
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }

    async fn update(&self, execution: &DailyExecution) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if !items.contains_key(&execution.id()) {
            return Err(AppError::not_found("daily_execution", execution.id()));
        }
        items.insert(execution.id(), execution.clone());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryReviewRepo {
    inner: Arc<Mutex<HashMap<ReviewId, Review>>>,
}

impl ReviewRepository for InMemoryReviewRepo {
    async fn create(&self, review: &Review) -> Result<(), AppError> {
        let mut items = lock(&self.inner)?;
        if items.contains_key(&review.id()) {
            return Err(AppError::conflict("review already exists"));
        }
        items.insert(review.id(), review.clone());
        Ok(())
    }

    async fn get(&self, id: ReviewId) -> Result<Option<Review>, AppError> {
        Ok(lock(&self.inner)?.get(&id).cloned())
    }

    async fn list_by_cycle(&self, cycle_id: CycleId) -> Result<Vec<Review>, AppError> {
        let mut items: Vec<_> = lock(&self.inner)?
            .values()
            .filter(|item| item.cycle_id() == cycle_id)
            .cloned()
            .collect();
        items.sort_by_key(|item| created_key(item.created_at(), item.id()));
        Ok(items)
    }
}
