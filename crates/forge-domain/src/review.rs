use time::{Date, OffsetDateTime};

use crate::DomainError;
use crate::ids::{CycleId, ReviewId};
use crate::util::{dates_within_cycle, empty_to_none};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    id: ReviewId,
    cycle_id: CycleId,
    content: String,
    period_start: Option<Date>,
    period_end: Option<Date>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Review {
    pub fn create(
        cycle_id: CycleId,
        content: impl AsRef<str>,
        period_start: Option<Date>,
        period_end: Option<Date>,
        cycle_start: Date,
        cycle_end: Date,
        now: OffsetDateTime,
    ) -> Result<Self, DomainError> {
        dates_within_cycle(cycle_start, cycle_end, period_start, period_end)?;
        Ok(Self {
            id: ReviewId::new(),
            cycle_id,
            content: parse_content(content)?,
            period_start,
            period_end,
            created_at: now,
            updated_at: now,
        })
    }

    #[must_use]
    pub fn reconstitute(
        id: ReviewId,
        cycle_id: CycleId,
        content: String,
        period_start: Option<Date>,
        period_end: Option<Date>,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            cycle_id,
            content,
            period_start,
            period_end,
            created_at,
            updated_at,
        }
    }

    #[must_use]
    pub fn id(&self) -> ReviewId {
        self.id
    }

    #[must_use]
    pub fn cycle_id(&self) -> CycleId {
        self.cycle_id
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[must_use]
    pub fn period_start(&self) -> Option<Date> {
        self.period_start
    }

    #[must_use]
    pub fn period_end(&self) -> Option<Date> {
        self.period_end
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

fn parse_content(content: impl AsRef<str>) -> Result<String, DomainError> {
    empty_to_none(Some(content.as_ref().to_string())).ok_or(DomainError::EmptyReviewContent)
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[test]
    fn belongs_to_cycle_and_checks_period() {
        let cycle = CycleId::new();
        let review = Review::create(
            cycle,
            "Mid-cycle notes",
            Some(date!(2026 - 01 - 01)),
            Some(date!(2026 - 01 - 31)),
            date!(2026 - 01 - 01),
            date!(2026 - 03 - 31),
            NOW,
        )
        .unwrap();
        assert_eq!(review.cycle_id(), cycle);

        let err = Review::create(
            cycle,
            "outside",
            Some(date!(2025 - 12 - 01)),
            Some(date!(2026 - 01 - 31)),
            date!(2026 - 01 - 01),
            date!(2026 - 03 - 31),
            NOW,
        )
        .unwrap_err();
        assert_eq!(err, DomainError::DateOutsideCycle);
    }

    #[test]
    fn rejects_empty_content() {
        let err = Review::create(
            CycleId::new(),
            "   ",
            None,
            None,
            date!(2026 - 01 - 01),
            date!(2026 - 03 - 31),
            NOW,
        )
        .unwrap_err();
        assert_eq!(err, DomainError::EmptyReviewContent);
    }
}
