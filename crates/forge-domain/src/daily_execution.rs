use time::{Date, OffsetDateTime};

use crate::ids::{DailyExecutionId, TaskId};
use crate::status::DailyExecutionStatus;
use crate::util::empty_to_none;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyExecution {
    id: DailyExecutionId,
    task_id: TaskId,
    execution_date: Date,
    notes: Option<String>,
    status: DailyExecutionStatus,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl DailyExecution {
    #[must_use]
    pub fn create(
        task_id: TaskId,
        execution_date: Date,
        notes: Option<String>,
        status: DailyExecutionStatus,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id: DailyExecutionId::new(),
            task_id,
            execution_date,
            notes: empty_to_none(notes),
            status,
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    pub fn reconstitute(
        id: DailyExecutionId,
        task_id: TaskId,
        execution_date: Date,
        notes: Option<String>,
        status: DailyExecutionStatus,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            task_id,
            execution_date,
            notes,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn update(
        &mut self,
        notes: Option<String>,
        status: DailyExecutionStatus,
        now: OffsetDateTime,
    ) {
        self.notes = empty_to_none(notes);
        self.status = status;
        self.updated_at = now;
    }

    #[must_use]
    pub fn id(&self) -> DailyExecutionId {
        self.id
    }

    #[must_use]
    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    #[must_use]
    pub fn execution_date(&self) -> Date {
        self.execution_date
    }

    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    #[must_use]
    pub fn status(&self) -> DailyExecutionStatus {
        self.status
    }

    #[must_use]
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    #[must_use]
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }
}
