use std::str::FromStr;

use crate::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleStatus {
    Planning,
    Active,
    Closed,
    Archived,
}

impl CycleStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::Active => "active",
            Self::Closed => "closed",
            Self::Archived => "archived",
        }
    }

    #[must_use]
    pub fn allows_tree_mutation(self) -> bool {
        matches!(self, Self::Planning | Self::Active)
    }
}

impl FromStr for CycleStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planning" => Ok(Self::Planning),
            "active" => Ok(Self::Active),
            "closed" => Ok(Self::Closed),
            "archived" => Ok(Self::Archived),
            other => Err(DomainError::UnknownStatus(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveStatus {
    Draft,
    Active,
    Completed,
    Archived,
}

impl ObjectiveStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Archived => "archived",
        }
    }

    #[must_use]
    pub fn allows_children(self) -> bool {
        matches!(self, Self::Draft | Self::Active)
    }
}

impl FromStr for ObjectiveStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "archived" => Ok(Self::Archived),
            other => Err(DomainError::UnknownStatus(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyResultStatus {
    Draft,
    Active,
    Completed,
    Archived,
}

impl KeyResultStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Archived => "archived",
        }
    }

    #[must_use]
    pub fn allows_check_in(self) -> bool {
        matches!(self, Self::Draft | Self::Active)
    }
}

impl FromStr for KeyResultStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "archived" => Ok(Self::Archived),
            other => Err(DomainError::UnknownStatus(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectStatus {
    Draft,
    Active,
    Completed,
    Archived,
}

impl ProjectStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Archived => "archived",
        }
    }

    #[must_use]
    pub fn allows_tasks(self) -> bool {
        matches!(self, Self::Draft | Self::Active)
    }
}

impl FromStr for ProjectStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "archived" => Ok(Self::Archived),
            other => Err(DomainError::UnknownStatus(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Cancelled,
}

impl TaskStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::InProgress => "in_progress",
            Self::Done => "done",
            Self::Cancelled => "cancelled",
        }
    }

    #[must_use]
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Done | Self::Cancelled)
    }

    #[must_use]
    pub fn is_active(self) -> bool {
        matches!(self, Self::Todo | Self::InProgress)
    }
}

impl FromStr for TaskStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(Self::Todo),
            "in_progress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            "cancelled" => Ok(Self::Cancelled),
            other => Err(DomainError::UnknownStatus(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyExecutionStatus {
    Planned,
    Completed,
    Skipped,
}

impl DailyExecutionStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Completed => "completed",
            Self::Skipped => "skipped",
        }
    }
}

impl FromStr for DailyExecutionStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planned" => Ok(Self::Planned),
            "completed" => Ok(Self::Completed),
            "skipped" => Ok(Self::Skipped),
            other => Err(DomainError::UnknownStatus(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_wire_values() {
        assert_eq!(
            CycleStatus::Planning
                .as_str()
                .parse::<CycleStatus>()
                .unwrap(),
            CycleStatus::Planning
        );
        assert_eq!(TaskStatus::InProgress.as_str(), "in_progress");
    }

    #[test]
    fn unknown_status_is_an_error() {
        assert!(matches!(
            "nope".parse::<CycleStatus>(),
            Err(DomainError::UnknownStatus(value)) if value == "nope"
        ));
    }

    #[test]
    fn child_rules() {
        assert!(CycleStatus::Planning.allows_tree_mutation());
        assert!(CycleStatus::Active.allows_tree_mutation());
        assert!(!CycleStatus::Closed.allows_tree_mutation());
        assert!(ObjectiveStatus::Draft.allows_children());
        assert!(!ObjectiveStatus::Completed.allows_children());
        assert!(ProjectStatus::Draft.allows_tasks());
        assert!(ProjectStatus::Active.allows_tasks());
        assert!(!ProjectStatus::Completed.allows_tasks());
        assert!(!ProjectStatus::Archived.allows_tasks());
        assert!(TaskStatus::Done.is_terminal());
        assert!(!TaskStatus::Todo.is_terminal());
    }
}
