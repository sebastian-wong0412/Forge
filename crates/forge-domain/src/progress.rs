use std::str::FromStr;

use crate::DomainError;
use crate::util::empty_to_none;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressKind {
    Numeric,
    Percentage,
    Milestone,
    Qualitative,
}

impl ProgressKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::Percentage => "percentage",
            Self::Milestone => "milestone",
            Self::Qualitative => "qualitative",
        }
    }
}

impl FromStr for ProgressKind {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "numeric" => Ok(Self::Numeric),
            "percentage" => Ok(Self::Percentage),
            "milestone" => Ok(Self::Milestone),
            "qualitative" => Ok(Self::Qualitative),
            other => Err(DomainError::UnknownProgressKind(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneState {
    NotStarted,
    InProgress,
    Achieved,
}

impl MilestoneState {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Achieved => "achieved",
        }
    }

    #[must_use]
    pub fn progress(self) -> f64 {
        match self {
            Self::NotStarted => 0.0,
            Self::InProgress => 0.5,
            Self::Achieved => 1.0,
        }
    }
}

impl FromStr for MilestoneState {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "not_started" => Ok(Self::NotStarted),
            "in_progress" => Ok(Self::InProgress),
            "achieved" => Ok(Self::Achieved),
            other => Err(DomainError::UnknownMilestoneState(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgressDefinition {
    Numeric {
        start_value: f64,
        target_value: Option<f64>,
        unit: Option<String>,
    },
    Percentage {
        start_value: f64,
        target_value: f64,
    },
    Milestone,
    Qualitative,
}

impl ProgressDefinition {
    pub fn parse(
        kind: ProgressKind,
        start_value: Option<f64>,
        target_value: Option<f64>,
        unit: Option<String>,
    ) -> Result<Self, DomainError> {
        let unit = empty_to_none(unit);
        match kind {
            ProgressKind::Numeric => {
                let start_value = start_value.ok_or(DomainError::InvalidProgressDefinition {
                    kind: kind.as_str(),
                    reason: "start_value is required",
                })?;
                Ok(Self::Numeric {
                    start_value,
                    target_value,
                    unit,
                })
            }
            ProgressKind::Percentage => {
                let start_value = start_value.ok_or(DomainError::InvalidProgressDefinition {
                    kind: kind.as_str(),
                    reason: "start_value is required",
                })?;
                let target_value = target_value.ok_or(DomainError::InvalidProgressDefinition {
                    kind: kind.as_str(),
                    reason: "target_value is required",
                })?;
                if unit.is_some() {
                    return Err(DomainError::InvalidProgressDefinition {
                        kind: kind.as_str(),
                        reason: "unit is not used",
                    });
                }
                Ok(Self::Percentage {
                    start_value,
                    target_value,
                })
            }
            ProgressKind::Milestone => {
                if start_value.is_some() || target_value.is_some() || unit.is_some() {
                    return Err(DomainError::InvalidProgressDefinition {
                        kind: kind.as_str(),
                        reason: "numeric fields are not used",
                    });
                }
                Ok(Self::Milestone)
            }
            ProgressKind::Qualitative => {
                if start_value.is_some() || target_value.is_some() || unit.is_some() {
                    return Err(DomainError::InvalidProgressDefinition {
                        kind: kind.as_str(),
                        reason: "numeric fields are not used",
                    });
                }
                Ok(Self::Qualitative)
            }
        }
    }

    #[must_use]
    pub fn kind(&self) -> ProgressKind {
        match self {
            Self::Numeric { .. } => ProgressKind::Numeric,
            Self::Percentage { .. } => ProgressKind::Percentage,
            Self::Milestone => ProgressKind::Milestone,
            Self::Qualitative => ProgressKind::Qualitative,
        }
    }

    #[must_use]
    pub fn start_value(&self) -> Option<f64> {
        match self {
            Self::Numeric { start_value, .. } | Self::Percentage { start_value, .. } => {
                Some(*start_value)
            }
            Self::Milestone | Self::Qualitative => None,
        }
    }

    #[must_use]
    pub fn target_value(&self) -> Option<f64> {
        match self {
            Self::Numeric { target_value, .. } => *target_value,
            Self::Percentage { target_value, .. } => Some(*target_value),
            Self::Milestone | Self::Qualitative => None,
        }
    }

    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        match self {
            Self::Numeric { unit, .. } => unit.as_deref(),
            Self::Percentage { .. } | Self::Milestone | Self::Qualitative => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_kinds_and_states() {
        assert_eq!(
            "numeric".parse::<ProgressKind>().unwrap(),
            ProgressKind::Numeric
        );
        assert_eq!(
            "achieved".parse::<MilestoneState>().unwrap(),
            MilestoneState::Achieved
        );
        assert!(matches!(
            "nope".parse::<ProgressKind>(),
            Err(DomainError::UnknownProgressKind(_))
        ));
    }

    #[test]
    fn numeric_requires_start() {
        let err =
            ProgressDefinition::parse(ProgressKind::Numeric, None, Some(10.0), None).unwrap_err();
        assert!(matches!(err, DomainError::InvalidProgressDefinition { .. }));
    }

    #[test]
    fn percentage_requires_start_and_target() {
        assert!(
            ProgressDefinition::parse(ProgressKind::Percentage, Some(60.0), None, None).is_err()
        );
        let def = ProgressDefinition::parse(ProgressKind::Percentage, Some(60.0), Some(90.0), None)
            .unwrap();
        assert_eq!(def.kind(), ProgressKind::Percentage);
    }

    #[test]
    fn milestone_and_qualitative_reject_numeric_fields() {
        assert!(ProgressDefinition::parse(ProgressKind::Milestone, Some(0.0), None, None).is_err());
        assert!(
            ProgressDefinition::parse(ProgressKind::Qualitative, None, None, Some("%".into()))
                .is_err()
        );
        assert!(ProgressDefinition::parse(ProgressKind::Milestone, None, None, None).is_ok());
        assert!(ProgressDefinition::parse(ProgressKind::Qualitative, None, None, None).is_ok());
    }
}
