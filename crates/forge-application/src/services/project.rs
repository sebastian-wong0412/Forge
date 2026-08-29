use forge_domain::{ObjectiveId, Project, ProjectId, Title};
use time::OffsetDateTime;

use crate::AppError;
use crate::repos::{CycleRepository, ObjectiveRepository, ProjectRepository};

pub struct CreateProject {
    pub title: String,
    pub description: Option<String>,
}

pub struct UpdateProject {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct ProjectService<C, O, P> {
    cycles: C,
    objectives: O,
    projects: P,
}

impl<C, O, P> ProjectService<C, O, P>
where
    C: CycleRepository,
    O: ObjectiveRepository,
    P: ProjectRepository,
{
    pub fn new(cycles: C, objectives: O, projects: P) -> Self {
        Self {
            cycles,
            objectives,
            projects,
        }
    }

    pub async fn create(
        &self,
        objective_id: ObjectiveId,
        cmd: CreateProject,
        now: OffsetDateTime,
    ) -> Result<Project, AppError> {
        let objective = self
            .objectives
            .get(objective_id)
            .await?
            .ok_or_else(|| AppError::not_found("objective", objective_id))?;
        if !objective.status().allows_children() {
            return Err(AppError::conflict(
                "cannot add a project to a completed or archived objective",
            ));
        }
        let cycle = self
            .cycles
            .get(objective.cycle_id())
            .await?
            .ok_or_else(|| AppError::not_found("cycle", objective.cycle_id()))?;
        if !cycle.status().allows_tree_mutation() {
            return Err(AppError::conflict(
                "cannot add a project to a closed or archived cycle",
            ));
        }
        let title = Title::parse(cmd.title)?;
        let project = Project::create(objective_id, title, cmd.description, now);
        self.projects.create(&project).await?;
        Ok(project)
    }

    pub async fn get(&self, id: ProjectId) -> Result<Project, AppError> {
        self.projects
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("project", id))
    }

    pub async fn list_by_objective(
        &self,
        objective_id: ObjectiveId,
    ) -> Result<Vec<Project>, AppError> {
        self.objectives
            .get(objective_id)
            .await?
            .ok_or_else(|| AppError::not_found("objective", objective_id))?;
        self.projects.list_by_objective(objective_id).await
    }

    pub async fn update(
        &self,
        id: ProjectId,
        cmd: UpdateProject,
        now: OffsetDateTime,
    ) -> Result<Project, AppError> {
        let mut project = self.get(id).await?;
        let title = Title::parse(cmd.title)?;
        project.update(title, cmd.description, now)?;
        self.projects.update(&project).await?;
        Ok(project)
    }

    pub async fn activate(&self, id: ProjectId, now: OffsetDateTime) -> Result<Project, AppError> {
        let mut project = self.get(id).await?;
        project.activate(now)?;
        self.projects.update(&project).await?;
        Ok(project)
    }

    pub async fn complete(&self, id: ProjectId, now: OffsetDateTime) -> Result<Project, AppError> {
        let mut project = self.get(id).await?;
        project.complete(now)?;
        self.projects.update(&project).await?;
        Ok(project)
    }

    pub async fn archive(&self, id: ProjectId, now: OffsetDateTime) -> Result<Project, AppError> {
        let mut project = self.get(id).await?;
        project.archive(now)?;
        self.projects.update(&project).await?;
        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::services::cycle::{CreateCycle, CycleService};
    use crate::services::objective::{CreateObjective, ObjectiveService};
    use crate::test_support::{InMemoryCycleRepo, InMemoryObjectiveRepo, InMemoryProjectRepo};

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[tokio::test]
    async fn belongs_to_objective_and_lifecycle() {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let projects = InMemoryProjectRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
        let project_svc = ProjectService::new(cycles, objectives, projects);

        let cycle = cycle_svc
            .create(
                CreateCycle {
                    name: "Q1".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        let objective = objective_svc
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let project = project_svc
            .create(
                objective.id(),
                CreateProject {
                    title: "Workstream".into(),
                    description: None,
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(project.objective_id(), objective.id());
        assert!(!project.status().allows_tasks());
        let active = project_svc.activate(project.id(), NOW).await.unwrap();
        assert!(active.status().allows_tasks());
    }
}
