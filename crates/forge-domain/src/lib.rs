//! Domain layer for Forge.
//!
//! This crate contains entities, identifiers, and invariants. It must not depend
//! on HTTP, SQLite, Axum, SQLx, or any other infrastructure.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod check_in;
mod cycle;
mod daily_execution;
mod error;
mod ids;
mod key_result;
mod objective;
mod project;
mod review;
mod status;
mod task;
mod title;
mod util;

pub use check_in::{CheckIn, latest_check_in};
pub use cycle::Cycle;
pub use daily_execution::DailyExecution;
pub use error::DomainError;
pub use ids::{
    CheckInId, CycleId, DailyExecutionId, KeyResultId, ObjectiveId, ProjectId, ReviewId, TaskId,
};
pub use key_result::{KeyResult, progress};
pub use objective::Objective;
pub use project::Project;
pub use review::Review;
pub use status::{
    CycleStatus, DailyExecutionStatus, KeyResultStatus, ObjectiveStatus, ProjectStatus, TaskStatus,
};
pub use task::{Task, TodayBucket, today_bucket};
pub use title::Title;
pub use util::dates_within_cycle;
