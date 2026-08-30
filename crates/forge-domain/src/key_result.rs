use time::OffsetDateTime;

use crate::DomainError;
use crate::check_in::CheckIn;
use crate::ids::{KeyResultId, ObjectiveId};
use crate::progress::{MilestoneState, ProgressDefinition, ProgressKind};
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
    definition: ProgressDefinition,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl KeyResult {
    pub fn create(
        objective_id: ObjectiveId,
        title: Title,
        description: Option<String>,
        definition: ProgressDefinition,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id: KeyResultId::new(),
            objective_id,
            title,
            description: empty_to_none(description),
            status: KeyResultStatus::Draft,
            definition,
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
        definition: ProgressDefinition,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            objective_id,
            title,
            description,
            status,
            definition,
            created_at,
            updated_at,
        }
    }

    pub fn update(
        &mut self,
        title: Title,
        description: Option<String>,
        definition: ProgressDefinition,
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
        if definition.kind() != self.definition.kind() {
            return Err(DomainError::ProgressKindImmutable);
        }
        self.title = title;
        self.description = empty_to_none(description);
        self.definition = definition;
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
    pub fn current_value(&self, latest: Option<&CheckIn>) -> Option<f64> {
        match &self.definition {
            ProgressDefinition::Numeric { start_value, .. }
            | ProgressDefinition::Percentage { start_value, .. } => {
                Some(latest.and_then(CheckIn::value).unwrap_or(*start_value))
            }
            ProgressDefinition::Milestone | ProgressDefinition::Qualitative => None,
        }
    }

    #[must_use]
    pub fn current_state(&self, latest: Option<&CheckIn>) -> Option<MilestoneState> {
        match self.definition {
            ProgressDefinition::Milestone => Some(
                latest
                    .and_then(CheckIn::state)
                    .unwrap_or(MilestoneState::NotStarted),
            ),
            _ => None,
        }
    }

    #[must_use]
    pub fn latest_note<'a>(&'a self, latest: Option<&'a CheckIn>) -> Option<&'a str> {
        latest.and_then(CheckIn::note)
    }

    #[must_use]
    pub fn progress(&self, latest: Option<&CheckIn>) -> Option<f64> {
        match &self.definition {
            ProgressDefinition::Numeric {
                start_value,
                target_value,
                ..
            } => progress(*start_value, self.current_value(latest)?, *target_value),
            ProgressDefinition::Percentage {
                start_value,
                target_value,
            } => progress(
                *start_value,
                self.current_value(latest)?,
                Some(*target_value),
            ),
            ProgressDefinition::Milestone => {
                self.current_state(latest).map(MilestoneState::progress)
            }
            ProgressDefinition::Qualitative => None,
        }
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
    pub fn progress_kind(&self) -> ProgressKind {
        self.definition.kind()
    }

    #[must_use]
    pub fn definition(&self) -> &ProgressDefinition {
        &self.definition
    }

    #[must_use]
    pub fn start_value(&self) -> Option<f64> {
        self.definition.start_value()
    }

    #[must_use]
    pub fn target_value(&self) -> Option<f64> {
        self.definition.target_value()
    }

    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.definition.unit()
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
    use time::macros::{date, datetime};

    use super::*;
    use crate::check_in::CheckIn;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    fn numeric(start: f64, target: Option<f64>) -> KeyResult {
        KeyResult::create(
            ObjectiveId::new(),
            Title::parse("Metric").unwrap(),
            None,
            ProgressDefinition::parse(
                ProgressKind::Numeric,
                Some(start),
                target,
                Some("kg".into()),
            )
            .unwrap(),
            NOW,
        )
    }

    #[test]
    fn current_defaults_to_start() {
        let kr = numeric(10.0, Some(20.0));
        assert_eq!(kr.current_value(None), Some(10.0));
    }

    #[test]
    fn current_uses_latest_check_in() {
        let kr = numeric(10.0, Some(20.0));
        let check = CheckIn::create(
            kr.id(),
            ProgressKind::Numeric,
            Some(14.0),
            None,
            None,
            date!(2026 - 01 - 10),
            NOW,
        )
        .unwrap();
        assert_eq!(kr.current_value(Some(&check)), Some(14.0));
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
        let mut kr = numeric(0.0, Some(10.0));
        assert!(kr.complete(NOW).is_err());
        kr.activate(NOW).unwrap();
        kr.complete(NOW).unwrap();
        assert_eq!(kr.status(), KeyResultStatus::Completed);
        assert!(!kr.status().allows_check_in());
    }

    #[test]
    fn percentage_uses_same_formula() {
        let kr = KeyResult::create(
            ObjectiveId::new(),
            Title::parse("Coverage").unwrap(),
            None,
            ProgressDefinition::parse(ProgressKind::Percentage, Some(60.0), Some(90.0), None)
                .unwrap(),
            NOW,
        );
        let check = CheckIn::create(
            kr.id(),
            ProgressKind::Percentage,
            Some(75.0),
            None,
            Some("week 1".into()),
            date!(2026 - 01 - 10),
            NOW,
        )
        .unwrap();
        assert_eq!(kr.progress(Some(&check)), Some(0.5));
    }

    #[test]
    fn milestone_progress_is_discrete() {
        let kr = KeyResult::create(
            ObjectiveId::new(),
            Title::parse("Launch").unwrap(),
            None,
            ProgressDefinition::parse(ProgressKind::Milestone, None, None, None).unwrap(),
            NOW,
        );
        assert_eq!(kr.current_state(None), Some(MilestoneState::NotStarted));
        assert_eq!(kr.progress(None), Some(0.0));
        let check = CheckIn::create(
            kr.id(),
            ProgressKind::Milestone,
            None,
            Some(MilestoneState::Achieved),
            None,
            date!(2026 - 01 - 10),
            NOW,
        )
        .unwrap();
        assert_eq!(kr.progress(Some(&check)), Some(1.0));
        assert_eq!(kr.current_value(Some(&check)), None);
    }

    #[test]
    fn qualitative_progress_is_none() {
        let kr = KeyResult::create(
            ObjectiveId::new(),
            Title::parse("Learn").unwrap(),
            None,
            ProgressDefinition::parse(ProgressKind::Qualitative, None, None, None).unwrap(),
            NOW,
        );
        let check = CheckIn::create(
            kr.id(),
            ProgressKind::Qualitative,
            None,
            None,
            Some("read two papers".into()),
            date!(2026 - 01 - 10),
            NOW,
        )
        .unwrap();
        assert_eq!(kr.progress(Some(&check)), None);
        assert_eq!(kr.latest_note(Some(&check)), Some("read two papers"));
        assert_eq!(kr.current_value(Some(&check)), None);
    }

    #[test]
    fn progress_kind_cannot_change() {
        let mut kr = numeric(0.0, Some(10.0));
        let err = kr
            .update(
                Title::parse("Metric").unwrap(),
                None,
                ProgressDefinition::parse(ProgressKind::Qualitative, None, None, None).unwrap(),
                NOW,
            )
            .unwrap_err();
        assert_eq!(err, DomainError::ProgressKindImmutable);
    }

    #[test]
    fn complete_is_independent_of_full_progress() {
        let mut kr = numeric(0.0, Some(10.0));
        kr.activate(NOW).unwrap();
        let check = CheckIn::create(
            kr.id(),
            ProgressKind::Numeric,
            Some(10.0),
            None,
            None,
            date!(2026 - 01 - 10),
            NOW,
        )
        .unwrap();
        assert_eq!(kr.progress(Some(&check)), Some(1.0));
        assert_eq!(kr.status(), KeyResultStatus::Active);
        kr.complete(NOW).unwrap();
        assert_eq!(kr.status(), KeyResultStatus::Completed);
    }
}
