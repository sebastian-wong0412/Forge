use time::{Date, OffsetDateTime};

use crate::DomainError;
use crate::ids::CycleId;
use crate::status::CycleStatus;
use crate::title::Title;
use crate::util::require_date_range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    id: CycleId,
    name: Title,
    start_on: Date,
    end_on: Date,
    status: CycleStatus,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Cycle {
    pub fn create(
        name: Title,
        start_on: Date,
        end_on: Date,
        now: OffsetDateTime,
    ) -> Result<Self, DomainError> {
        require_date_range(start_on, end_on)?;
        Ok(Self {
            id: CycleId::new(),
            name,
            start_on,
            end_on,
            status: CycleStatus::Planning,
            created_at: now,
            updated_at: now,
        })
    }

    #[must_use]
    pub fn reconstitute(
        id: CycleId,
        name: Title,
        start_on: Date,
        end_on: Date,
        status: CycleStatus,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            name,
            start_on,
            end_on,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn update(
        &mut self,
        name: Title,
        start_on: Date,
        end_on: Date,
        now: OffsetDateTime,
    ) -> Result<(), DomainError> {
        if matches!(self.status, CycleStatus::Closed | CycleStatus::Archived) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: self.status.as_str(),
            });
        }
        require_date_range(start_on, end_on)?;
        self.name = name;
        self.start_on = start_on;
        self.end_on = end_on;
        self.updated_at = now;
        Ok(())
    }

    pub fn activate(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != CycleStatus::Planning {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: CycleStatus::Active.as_str(),
            });
        }
        self.status = CycleStatus::Active;
        self.updated_at = now;
        Ok(())
    }

    pub fn close(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != CycleStatus::Active {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: CycleStatus::Closed.as_str(),
            });
        }
        self.status = CycleStatus::Closed;
        self.updated_at = now;
        Ok(())
    }

    pub fn archive(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status == CycleStatus::Archived {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: CycleStatus::Archived.as_str(),
            });
        }
        self.status = CycleStatus::Archived;
        self.updated_at = now;
        Ok(())
    }

    #[must_use]
    pub fn id(&self) -> CycleId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &Title {
        &self.name
    }

    #[must_use]
    pub fn start_on(&self) -> Date {
        self.start_on
    }

    #[must_use]
    pub fn end_on(&self) -> Date {
        self.end_on
    }

    #[must_use]
    pub fn status(&self) -> CycleStatus {
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

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    fn cycle() -> Cycle {
        Cycle::create(
            Title::parse("2026 Q1").unwrap(),
            date!(2026 - 01 - 01),
            date!(2026 - 03 - 31),
            NOW,
        )
        .unwrap()
    }

    #[test]
    fn rejects_inverted_dates() {
        let err = Cycle::create(
            Title::parse("Q1").unwrap(),
            date!(2026 - 03 - 31),
            date!(2026 - 01 - 01),
            NOW,
        )
        .unwrap_err();
        assert_eq!(err, DomainError::InvalidDateRange);
    }

    #[test]
    fn create_starts_planning() {
        assert_eq!(cycle().status(), CycleStatus::Planning);
    }

    #[test]
    fn planning_to_active_to_closed() {
        let mut cycle = cycle();
        cycle.activate(NOW).unwrap();
        assert_eq!(cycle.status(), CycleStatus::Active);
        cycle.close(NOW).unwrap();
        assert_eq!(cycle.status(), CycleStatus::Closed);
    }

    #[test]
    fn invalid_lifecycle_transitions() {
        let mut cycle = cycle();
        assert!(cycle.close(NOW).is_err());
        cycle.activate(NOW).unwrap();
        assert!(cycle.activate(NOW).is_err());
        cycle.close(NOW).unwrap();
        assert!(cycle.activate(NOW).is_err());
        assert!(cycle.close(NOW).is_err());
    }

    #[test]
    fn archive_from_any_non_archived() {
        let mut cycle = cycle();
        cycle.archive(NOW).unwrap();
        assert_eq!(cycle.status(), CycleStatus::Archived);
        assert!(cycle.archive(NOW).is_err());
        assert!(cycle.activate(NOW).is_err());
    }
}
