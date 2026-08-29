use time::{Date, OffsetDateTime};

use crate::DomainError;
use crate::ids::{CycleId, ObjectiveId};
use crate::status::ObjectiveStatus;
use crate::title::Title;
use crate::util::{dates_within_cycle, empty_to_none};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Objective {
    id: ObjectiveId,
    cycle_id: CycleId,
    title: Title,
    description: Option<String>,
    status: ObjectiveStatus,
    start_on: Option<Date>,
    end_on: Option<Date>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Objective {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        cycle_id: CycleId,
        title: Title,
        description: Option<String>,
        start_on: Option<Date>,
        end_on: Option<Date>,
        cycle_start: Date,
        cycle_end: Date,
        now: OffsetDateTime,
    ) -> Result<Self, DomainError> {
        dates_within_cycle(cycle_start, cycle_end, start_on, end_on)?;
        Ok(Self {
            id: ObjectiveId::new(),
            cycle_id,
            title,
            description: empty_to_none(description),
            status: ObjectiveStatus::Draft,
            start_on,
            end_on,
            created_at: now,
            updated_at: now,
        })
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: ObjectiveId,
        cycle_id: CycleId,
        title: Title,
        description: Option<String>,
        status: ObjectiveStatus,
        start_on: Option<Date>,
        end_on: Option<Date>,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            cycle_id,
            title,
            description,
            status,
            start_on,
            end_on,
            created_at,
            updated_at,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        title: Title,
        description: Option<String>,
        start_on: Option<Date>,
        end_on: Option<Date>,
        cycle_start: Date,
        cycle_end: Date,
        now: OffsetDateTime,
    ) -> Result<(), DomainError> {
        if matches!(
            self.status,
            ObjectiveStatus::Completed | ObjectiveStatus::Archived
        ) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: self.status.as_str(),
            });
        }
        dates_within_cycle(cycle_start, cycle_end, start_on, end_on)?;
        self.title = title;
        self.description = empty_to_none(description);
        self.start_on = start_on;
        self.end_on = end_on;
        self.updated_at = now;
        Ok(())
    }

    pub fn activate(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != ObjectiveStatus::Draft {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: ObjectiveStatus::Active.as_str(),
            });
        }
        self.status = ObjectiveStatus::Active;
        self.updated_at = now;
        Ok(())
    }

    pub fn complete(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != ObjectiveStatus::Active {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: ObjectiveStatus::Completed.as_str(),
            });
        }
        self.status = ObjectiveStatus::Completed;
        self.updated_at = now;
        Ok(())
    }

    pub fn archive(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status == ObjectiveStatus::Archived {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: ObjectiveStatus::Archived.as_str(),
            });
        }
        self.status = ObjectiveStatus::Archived;
        self.updated_at = now;
        Ok(())
    }

    #[must_use]
    pub fn id(&self) -> ObjectiveId {
        self.id
    }

    #[must_use]
    pub fn cycle_id(&self) -> CycleId {
        self.cycle_id
    }

    #[must_use]
    pub fn title(&self) -> &Title {
        &self.title
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[must_use]
    pub fn status(&self) -> ObjectiveStatus {
        self.status
    }

    #[must_use]
    pub fn start_on(&self) -> Option<Date> {
        self.start_on
    }

    #[must_use]
    pub fn end_on(&self) -> Option<Date> {
        self.end_on
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

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    fn objective() -> Objective {
        Objective::create(
            CycleId::new(),
            Title::parse("Ship 1A").unwrap(),
            None,
            Some(date!(2026 - 01 - 01)),
            Some(date!(2026 - 03 - 31)),
            date!(2026 - 01 - 01),
            date!(2026 - 03 - 31),
            NOW,
        )
        .unwrap()
    }

    #[test]
    fn rejects_dates_outside_cycle() {
        let err = Objective::create(
            CycleId::new(),
            Title::parse("Q1").unwrap(),
            None,
            Some(date!(2025 - 12 - 01)),
            Some(date!(2026 - 03 - 31)),
            date!(2026 - 01 - 01),
            date!(2026 - 03 - 31),
            NOW,
        )
        .unwrap_err();
        assert_eq!(err, DomainError::DateOutsideCycle);
    }

    #[test]
    fn draft_to_active_to_completed() {
        let mut objective = objective();
        assert_eq!(objective.status(), ObjectiveStatus::Draft);
        objective.activate(NOW).unwrap();
        objective.complete(NOW).unwrap();
        assert_eq!(objective.status(), ObjectiveStatus::Completed);
        assert!(objective.activate(NOW).is_err());
        assert!(!objective.status().allows_children());
    }

    #[test]
    fn archive_blocks_children() {
        let mut objective = objective();
        objective.archive(NOW).unwrap();
        assert!(!objective.status().allows_children());
        assert!(objective.archive(NOW).is_err());
    }
}
