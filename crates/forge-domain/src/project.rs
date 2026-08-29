use time::OffsetDateTime;

use crate::DomainError;
use crate::ids::{ObjectiveId, ProjectId};
use crate::status::ProjectStatus;
use crate::title::Title;
use crate::util::empty_to_none;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    id: ProjectId,
    objective_id: ObjectiveId,
    title: Title,
    description: Option<String>,
    status: ProjectStatus,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Project {
    #[must_use]
    pub fn create(
        objective_id: ObjectiveId,
        title: Title,
        description: Option<String>,
        now: OffsetDateTime,
    ) -> Self {
        Self {
            id: ProjectId::new(),
            objective_id,
            title,
            description: empty_to_none(description),
            status: ProjectStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    pub fn reconstitute(
        id: ProjectId,
        objective_id: ObjectiveId,
        title: Title,
        description: Option<String>,
        status: ProjectStatus,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            objective_id,
            title,
            description,
            status,
            created_at,
            updated_at,
        }
    }

    pub fn update(
        &mut self,
        title: Title,
        description: Option<String>,
        now: OffsetDateTime,
    ) -> Result<(), DomainError> {
        if matches!(
            self.status,
            ProjectStatus::Completed | ProjectStatus::Archived
        ) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: self.status.as_str(),
            });
        }
        self.title = title;
        self.description = empty_to_none(description);
        self.updated_at = now;
        Ok(())
    }

    pub fn activate(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != ProjectStatus::Draft {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: ProjectStatus::Active.as_str(),
            });
        }
        self.status = ProjectStatus::Active;
        self.updated_at = now;
        Ok(())
    }

    pub fn complete(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status != ProjectStatus::Active {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: ProjectStatus::Completed.as_str(),
            });
        }
        self.status = ProjectStatus::Completed;
        self.updated_at = now;
        Ok(())
    }

    pub fn archive(&mut self, now: OffsetDateTime) -> Result<(), DomainError> {
        if self.status == ProjectStatus::Archived {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status.as_str(),
                to: ProjectStatus::Archived.as_str(),
            });
        }
        self.status = ProjectStatus::Archived;
        self.updated_at = now;
        Ok(())
    }

    #[must_use]
    pub fn id(&self) -> ProjectId {
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
    pub fn status(&self) -> ProjectStatus {
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
    use time::macros::datetime;

    use super::*;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[test]
    fn only_active_allows_tasks() {
        let mut project =
            Project::create(ObjectiveId::new(), Title::parse("Work").unwrap(), None, NOW);
        assert!(!project.status().allows_tasks());
        project.activate(NOW).unwrap();
        assert!(project.status().allows_tasks());
        project.complete(NOW).unwrap();
        assert!(!project.status().allows_tasks());
    }
}
