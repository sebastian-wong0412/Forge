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
        let mut ancestry = crate::parent_progression::load_ancestry(
            &self.cycles,
            &self.objectives,
            &self.projects,
            id,
        )
        .await?;
        let cycle_change = crate::parent_progression::ensure_cycle(&mut ancestry.cycle, now)?;
        let objective_change =
            crate::parent_progression::ensure_objective(&mut ancestry.objective, now)?;
        ancestry.project.activate(now)?;

        crate::parent_progression::persist_activated(
            &self.cycles,
            &self.objectives,
            &self.projects,
            crate::parent_progression::ActivatedParents {
                cycle: &ancestry.cycle,
                cycle_change,
                objective: &ancestry.objective,
                objective_change,
                project: None,
            },
        )
        .await?;
        self.projects.update(&ancestry.project).await?;
        Ok(ancestry.project)
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
        assert!(project.status().allows_tasks());
        let active = project_svc.activate(project.id(), NOW).await.unwrap();
        assert!(active.status().allows_tasks());
        assert_eq!(
            cycle_svc.get(cycle.id()).await.unwrap().status(),
            forge_domain::CycleStatus::Active
        );
        assert_eq!(
            objective_svc.get(objective.id()).await.unwrap().status(),
            forge_domain::ObjectiveStatus::Active
        );
    }

    #[tokio::test]
    async fn activate_project_rejects_closed_cycle_without_activating_project() {
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
        cycle_svc.activate(cycle.id(), NOW).await.unwrap();
        cycle_svc.close(cycle.id(), NOW).await.unwrap();
        let err = project_svc.activate(project.id(), NOW).await.unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
        assert_eq!(
            project_svc.get(project.id()).await.unwrap().status(),
            forge_domain::ProjectStatus::Draft
        );
    }
}
