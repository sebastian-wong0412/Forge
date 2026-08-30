use time::{Date, OffsetDateTime};

use crate::DomainError;
use crate::ids::{CheckInId, KeyResultId};
use crate::progress::{MilestoneState, ProgressKind};
use crate::util::empty_to_none;

#[derive(Debug, Clone, PartialEq)]
pub struct CheckIn {
    id: CheckInId,
    key_result_id: KeyResultId,
    value: Option<f64>,
    state: Option<MilestoneState>,
    note: Option<String>,
    checked_on: Date,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl CheckIn {
    pub fn create(
        key_result_id: KeyResultId,
        kind: ProgressKind,
        value: Option<f64>,
        state: Option<MilestoneState>,
        note: Option<String>,
        checked_on: Date,
        now: OffsetDateTime,
    ) -> Result<Self, DomainError> {
        let note = empty_to_none(note);
        match kind {
            ProgressKind::Numeric | ProgressKind::Percentage => {
                if value.is_none() {
                    return Err(DomainError::InvalidCheckInPayload {
                        kind: kind.as_str(),
                        reason: "value is required",
                    });
                }
                if state.is_some() {
                    return Err(DomainError::InvalidCheckInPayload {
                        kind: kind.as_str(),
                        reason: "state is not used",
                    });
                }
            }
            ProgressKind::Milestone => {
                if state.is_none() {
                    return Err(DomainError::InvalidCheckInPayload {
                        kind: kind.as_str(),
                        reason: "state is required",
                    });
                }
                if value.is_some() {
                    return Err(DomainError::InvalidCheckInPayload {
                        kind: kind.as_str(),
                        reason: "value is not used",
                    });
                }
            }
            ProgressKind::Qualitative => {
                if note.is_none() {
                    return Err(DomainError::InvalidCheckInPayload {
                        kind: kind.as_str(),
                        reason: "note is required",
                    });
                }
                if value.is_some() || state.is_some() {
                    return Err(DomainError::InvalidCheckInPayload {
                        kind: kind.as_str(),
                        reason: "value and state are not used",
                    });
                }
            }
        }
        Ok(Self {
            id: CheckInId::new(),
            key_result_id,
            value,
            state,
            note,
            checked_on,
            created_at: now,
            updated_at: now,
        })
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: CheckInId,
        key_result_id: KeyResultId,
        value: Option<f64>,
        state: Option<MilestoneState>,
        note: Option<String>,
        checked_on: Date,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            key_result_id,
            value,
            state,
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
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    #[must_use]
    pub fn state(&self) -> Option<MilestoneState> {
        self.state
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
    use crate::progress::ProgressKind;

    #[test]
    fn latest_prefers_checked_on_then_created_at() {
        let kr = KeyResultId::new();
        let early = CheckIn::reconstitute(
            CheckInId::new(),
            kr,
            Some(10.0),
            None,
            None,
            date!(2026 - 01 - 10),
            datetime!(2026-01-20 12:00:00 UTC),
            datetime!(2026-01-20 12:00:00 UTC),
        );
        let same_day_first = CheckIn::reconstitute(
            CheckInId::new(),
            kr,
            Some(20.0),
            None,
            None,
            date!(2026 - 01 - 15),
            datetime!(2026-01-15 09:00:00 UTC),
            datetime!(2026-01-15 09:00:00 UTC),
        );
        let same_day_later = CheckIn::reconstitute(
            CheckInId::new(),
            kr,
            Some(30.0),
            None,
            None,
            date!(2026 - 01 - 15),
            datetime!(2026-01-15 18:00:00 UTC),
            datetime!(2026-01-15 18:00:00 UTC),
        );
        let items = [early, same_day_first, same_day_later];
        let latest = latest_check_in(&items).unwrap();
        assert_eq!(latest.value(), Some(30.0));
    }

    #[test]
    fn qualitative_requires_note() {
        let err = CheckIn::create(
            KeyResultId::new(),
            ProgressKind::Qualitative,
            None,
            None,
            None,
            date!(2026 - 01 - 10),
            datetime!(2026-01-15 12:00:00 UTC),
        )
        .unwrap_err();
        assert!(matches!(err, DomainError::InvalidCheckInPayload { .. }));
    }

    #[test]
    fn numeric_allows_value_and_note() {
        let check = CheckIn::create(
            KeyResultId::new(),
            ProgressKind::Numeric,
            Some(3.0),
            None,
            Some("week 1".into()),
            date!(2026 - 01 - 10),
            datetime!(2026-01-15 12:00:00 UTC),
        )
        .unwrap();
        assert_eq!(check.value(), Some(3.0));
        assert_eq!(check.note(), Some("week 1"));
    }

    #[test]
    fn mismatched_payload_is_rejected() {
        assert!(
            CheckIn::create(
                KeyResultId::new(),
                ProgressKind::Milestone,
                Some(1.0),
                Some(MilestoneState::InProgress),
                None,
                date!(2026 - 01 - 10),
                datetime!(2026-01-15 12:00:00 UTC),
            )
            .is_err()
        );
        assert!(
            CheckIn::create(
                KeyResultId::new(),
                ProgressKind::Numeric,
                None,
                None,
                Some("note".into()),
                date!(2026 - 01 - 10),
                datetime!(2026-01-15 12:00:00 UTC),
            )
            .is_err()
        );
    }
}
