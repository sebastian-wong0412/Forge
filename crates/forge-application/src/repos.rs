use std::future::Future;

use forge_domain::{
    CheckIn, CheckInId, Cycle, CycleId, DailyExecution, DailyExecutionId, KeyResult, KeyResultId,
    Objective, ObjectiveId, Project, ProjectId, Review, ReviewId, Task, TaskId,
};
use time::Date;

use crate::AppError;

pub trait CycleRepository: Send + Sync {
    fn create(&self, cycle: &Cycle) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(&self, id: CycleId) -> impl Future<Output = Result<Option<Cycle>, AppError>> + Send;
    fn list(&self) -> impl Future<Output = Result<Vec<Cycle>, AppError>> + Send;
    fn update(&self, cycle: &Cycle) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub trait ObjectiveRepository: Send + Sync {
    fn create(&self, objective: &Objective) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(
        &self,
        id: ObjectiveId,
    ) -> impl Future<Output = Result<Option<Objective>, AppError>> + Send;
    fn list_by_cycle(
        &self,
        cycle_id: CycleId,
    ) -> impl Future<Output = Result<Vec<Objective>, AppError>> + Send;
    fn update(&self, objective: &Objective) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub trait KeyResultRepository: Send + Sync {
    fn create(&self, key_result: &KeyResult) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(
        &self,
        id: KeyResultId,
    ) -> impl Future<Output = Result<Option<KeyResult>, AppError>> + Send;
    fn list_by_objective(
        &self,
        objective_id: ObjectiveId,
    ) -> impl Future<Output = Result<Vec<KeyResult>, AppError>> + Send;
    fn update(&self, key_result: &KeyResult) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub trait CheckInRepository: Send + Sync {
    fn create(&self, check_in: &CheckIn) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(&self, id: CheckInId) -> impl Future<Output = Result<Option<CheckIn>, AppError>> + Send;
    fn list_by_key_result(
        &self,
        key_result_id: KeyResultId,
    ) -> impl Future<Output = Result<Vec<CheckIn>, AppError>> + Send;
}

pub trait ProjectRepository: Send + Sync {
    fn create(&self, project: &Project) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(&self, id: ProjectId) -> impl Future<Output = Result<Option<Project>, AppError>> + Send;
    fn list_by_objective(
        &self,
        objective_id: ObjectiveId,
    ) -> impl Future<Output = Result<Vec<Project>, AppError>> + Send;
    fn update(&self, project: &Project) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub trait TaskRepository: Send + Sync {
    fn create(&self, task: &Task) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(&self, id: TaskId) -> impl Future<Output = Result<Option<Task>, AppError>> + Send;
    fn list_by_project(
        &self,
        project_id: ProjectId,
    ) -> impl Future<Output = Result<Vec<Task>, AppError>> + Send;
    fn update(&self, task: &Task) -> impl Future<Output = Result<(), AppError>> + Send;
    fn list_today_candidates(
        &self,
        date: Date,
    ) -> impl Future<Output = Result<Vec<Task>, AppError>> + Send;
}

pub trait DailyExecutionRepository: Send + Sync {
    fn create(
        &self,
        execution: &DailyExecution,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(
        &self,
        id: DailyExecutionId,
    ) -> impl Future<Output = Result<Option<DailyExecution>, AppError>> + Send;
    fn list_by_task(
        &self,
        task_id: TaskId,
    ) -> impl Future<Output = Result<Vec<DailyExecution>, AppError>> + Send;
    fn list_by_date(
        &self,
        execution_date: Date,
    ) -> impl Future<Output = Result<Vec<DailyExecution>, AppError>> + Send;
    fn update(
        &self,
        execution: &DailyExecution,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub trait ReviewRepository: Send + Sync {
    fn create(&self, review: &Review) -> impl Future<Output = Result<(), AppError>> + Send;
    fn get(&self, id: ReviewId) -> impl Future<Output = Result<Option<Review>, AppError>> + Send;
    fn list_by_cycle(
        &self,
        cycle_id: CycleId,
    ) -> impl Future<Output = Result<Vec<Review>, AppError>> + Send;
}
