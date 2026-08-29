use time::{Date, OffsetDateTime};

use crate::ids::{CheckInId, KeyResultId};
use crate::util::empty_to_none;

#[derive(Debug, Clone, PartialEq)]
pub struct CheckIn {
    id: CheckInId,
    key_result_id: KeyResultId,
    value: f64,
    note: Option<String>,
    checked_on: Date,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl CheckIn {
    #[must_use]
    pub fn create(
        key_result_id: KeyResultId,
        value: f64,
        note: Option<String>,
        checked_on: Date,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id: CheckInId::new(),
            key_result_id,
            value,
            note: empty_to_none(note),
            checked_on,
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    pub fn reconstitute(
        id: CheckInId,
        key_result_id: KeyResultId,
        value: f64,
        note: Option<String>,
        checked_on: Date,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            key_result_id,
            value,
            note,
            checked_on,
            created_at,
            updated_at,
        }
    }

    #[must_use]
    pub fn id(&self) -> CheckInId {
        self.id
    }

    #[must_use]
    pub fn key_result_id(&self) -> KeyResultId {
        self.key_result_id
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }

    #[must_use]
    pub fn checked_on(&self) -> Date {
        self.checked_on
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

/// Latest check-in by `checked_on`, then `created_at`, then id.
#[must_use]
pub fn latest_check_in(check_ins: &[CheckIn]) -> Option<&CheckIn> {
    check_ins.iter().max_by(|left, right| {
        left.checked_on()
            .cmp(&right.checked_on())
            .then_with(|| left.created_at().cmp(&right.created_at()))
            .then_with(|| left.id().as_uuid().cmp(&right.id().as_uuid()))
    })
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;

    #[test]
    fn latest_prefers_checked_on_then_created_at() {
        let kr = KeyResultId::new();
        let early = CheckIn::reconstitute(
            CheckInId::new(),
            kr,
            10.0,
            None,
            date!(2026 - 01 - 10),
            datetime!(2026-01-20 12:00:00 UTC),
            datetime!(2026-01-20 12:00:00 UTC),
        );
        let same_day_first = CheckIn::reconstitute(
            CheckInId::new(),
            kr,
            20.0,
            None,
            date!(2026 - 01 - 15),
            datetime!(2026-01-15 09:00:00 UTC),
            datetime!(2026-01-15 09:00:00 UTC),
        );
        let same_day_later = CheckIn::reconstitute(
            CheckInId::new(),
            kr,
            30.0,
            None,
            date!(2026 - 01 - 15),
            datetime!(2026-01-15 18:00:00 UTC),
            datetime!(2026-01-15 18:00:00 UTC),
        );
        let items = [early, same_day_first, same_day_later];
        let latest = latest_check_in(&items).unwrap();
        assert_eq!(latest.value(), 30.0);
    }
}
