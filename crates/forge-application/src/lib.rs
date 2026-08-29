//! Application layer for Forge.
//!
//! Use cases and repository traits. Depends only on `forge-domain`.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod repos;
pub mod services;

mod error;

#[cfg(test)]
mod test_support;

pub use error::AppError;
pub use services::{
    CheckInService, CreateCheckIn, CreateCycle, CreateDailyExecution, CreateKeyResult,
    CreateObjective, CreateProject, CreateReview, CreateTask, CycleService, DailyExecutionService,
    KeyResultService, KeyResultSnapshot, ObjectiveService, ProjectService, ReviewService,
    TaskService, TodayResult, UpdateCycle, UpdateDailyExecution, UpdateKeyResult, UpdateObjective,
    UpdateProject, UpdateTask,
};
