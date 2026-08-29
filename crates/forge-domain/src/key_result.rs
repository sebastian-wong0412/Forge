use time::OffsetDateTime;

use crate::DomainError;
use crate::check_in::CheckIn;
use crate::ids::{KeyResultId, ObjectiveId};
use crate::status::KeyResultStatus;
use crate::title::Title;
use crate::util::empty_to_none;

#[derive(Debug, Clone, PartialEq)]
pub struct KeyResult {
    id: KeyResultId,
    objective_id: ObjectiveId,
    title: Title,
    description: Option<String>,
    status: KeyResultStatus,
    start_value: f64,
    target_value: Option<f64>,
    unit: Option<String>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl KeyResult {
    #[must_use]
    pub fn create(
        objective_id: ObjectiveId,
        title: Title,
        description: Option<String>,
        start_value: f64,
        target_value: Option<f64>,
        unit: Option<String>,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id: KeyResultId::new(),
            objective_id,
            title,
            description: empty_to_none(description),
            status: KeyResultStatus::Draft,
            start_value,
            target_value,
            unit: empty_to_none(unit),
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn reconstitute(
        id: KeyResultId,
        objective_id: ObjectiveId,
        title: Title,
        description: Option<String>,
        status: KeyResultStatus,
        start_value: f64,
        target_value: Option<f64>,
        unit: Option<String>,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            objective_id,
            title,
            description,
            status,
            start_value,
            target_value,
            unit,
            created_at,
            updated_at,
        }
    }

    pub fn update(
        &mut self,
        title: Title,
        description: Option<String>,
        start_value: f64,
        target_value: Option<f64>,
        unit: Option<String>,
        now: OffsetDateTime,
    ) -> Result<(), DomainError> {
        if matches!(
            self.status,
            KeyResultStatus::Completed | KeyResultStatus::Archived
        ) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: self.status.as_str(),
            });
        }
        self.title = title;
        self.description = empty_to_none(description);
        self.start_value = start_value;
        self.target_value = target_value;
        self.unit = empty_to_none(unit);
        self.updated_at = now;
        Ok(())
    }

    pub fn activate(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != KeyResultStatus::Draft {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: KeyResultStatus::Active.as_str(),
            });
        }
        self.status = KeyResultStatus::Active;
        self.updated_at = now;
        Ok(())
    }

    pub fn complete(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != KeyResultStatus::Active {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: KeyResultStatus::Completed.as_str(),
            });
        }
        self.status = KeyResultStatus::Completed;
        self.updated_at = now;
        Ok(())
    }

    pub fn archive(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status == KeyResultStatus::Archived {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: KeyResultStatus::Archived.as_str(),
            });
        }
        self.status = KeyResultStatus::Archived;
        self.updated_at = now;
        Ok(())
    }

    #[must_use]
    pub fn current_value(&self, latest: Option<&CheckIn>) -> f64 {
        latest.map_or(self.start_value, CheckIn::value)
    }

    #[must_use]
    pub fn progress(&self, current_value: f64) -> Option<f64> {
        progress(self.start_value, current_value, self.target_value)
    }

    #[must_use]
    pub fn id(&self) -> KeyResultId {
        self.id
    }

    #[must_use]
    pub fn objective_id(&self) -> ObjectiveId {
        self.objective_id
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
    pub fn status(&self) -> KeyResultStatus {
        self.status
    }

    #[must_use]
    pub fn start_value(&self) -> f64 {
        self.start_value
    }

    #[must_use]
    pub fn target_value(&self) -> Option<f64> {
        self.target_value
    }

    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
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

#[must_use]
pub fn progress(start_value: f64, current_value: f64, target_value: Option<f64>) -> Option<f64> {
    let target = target_value?;
    let denominator = target - start_value;
    if denominator == 0.0 {
        return None;
    }
    Some(((current_value - start_value) / denominator).clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;
    use crate::check_in::CheckIn;
    use time::macros::date;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    fn kr(start: f64, target: Option<f64>) -> KeyResult {
        KeyResult::create(
            ObjectiveId::new(),
            Title::parse("Metric").unwrap(),
            None,
            start,
            target,
            Some("kg".into()),
            NOW,
        )
    }

    #[test]
    fn current_defaults_to_start() {
        let kr = kr(10.0, Some(20.0));
        assert_eq!(kr.current_value(None), 10.0);
    }

    #[test]
    fn current_uses_latest_check_in() {
        let kr = kr(10.0, Some(20.0));
        let check = CheckIn::create(kr.id(), 14.0, None, date!(2026 - 01 - 10), NOW);
        assert_eq!(kr.current_value(Some(&check)), 14.0);
    }

    #[test]
    fn increasing_progress() {
        assert_eq!(progress(0.0, 50.0, Some(100.0)), Some(0.5));
    }

    #[test]
    fn decreasing_progress() {
        let value = progress(500.0, 300.0, Some(200.0)).unwrap();
        assert!((value - 0.6666666666666666).abs() < 1e-9);
    }

    #[test]
    fn missing_or_equal_target_is_none() {
        assert_eq!(progress(10.0, 12.0, None), None);
        assert_eq!(progress(10.0, 12.0, Some(10.0)), None);
    }

    #[test]
    fn clamps_progress() {
        assert_eq!(progress(0.0, 150.0, Some(100.0)), Some(1.0));
        assert_eq!(progress(0.0, -10.0, Some(100.0)), Some(0.0));
    }

    #[test]
    fn complete_is_explicit() {
        let mut kr = kr(0.0, Some(10.0));
        assert!(kr.complete(NOW).is_err());
        kr.activate(NOW).unwrap();
        kr.complete(NOW).unwrap();
        assert_eq!(kr.status(), KeyResultStatus::Completed);
        assert!(!kr.status().allows_check_in());
    }
}
