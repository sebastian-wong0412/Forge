use forge_domain::{Cycle, EnsureActive, Objective, ObjectiveId, Project, ProjectId};
use time::OffsetDateTime;

use crate::AppError;
use crate::repos::{CycleRepository, ObjectiveRepository, ProjectRepository};

pub(crate) struct Ancestry {
    pub cycle: Cycle,
    pub objective: Objective,
    pub project: Project,
}

pub(crate) async fn load_ancestry<C, O, P>(
    cycles: &C,
    objectives: &O,
    projects: &P,
    project_id: ProjectId,
) -> Result<Ancestry, AppError>
where
    C: CycleRepository,
    O: ObjectiveRepository,
    P: ProjectRepository,
{
    let project = projects
        .get(project_id)
        .await?
        .ok_or_else(|| AppError::not_found("project", project_id))?;
    let objective = load_objective(objectives, project.objective_id()).await?;
    let cycle = load_cycle(cycles, &objective).await?;
    Ok(Ancestry {
        cycle,
        objective,
        project,
    })
}

pub(crate) async fn load_objective<O: ObjectiveRepository>(
    objectives: &O,
    objective_id: ObjectiveId,
) -> Result<Objective, AppError> {
    objectives
        .get(objective_id)
        .await?
        .ok_or_else(|| AppError::not_found("objective", objective_id))
}

pub(crate) async fn load_cycle<C: CycleRepository>(
    cycles: &C,
    objective: &Objective,
) -> Result<Cycle, AppError> {
    cycles
        .get(objective.cycle_id())
        .await?
        .ok_or_else(|| AppError::not_found("cycle", objective.cycle_id()))
}

pub(crate) struct ActivatedParents<'a> {
    pub cycle: &'a Cycle,
    pub cycle_change: EnsureActive,
    pub objective: &'a Objective,
    pub objective_change: EnsureActive,
    pub project: Option<(&'a Project, EnsureActive)>,
}

pub(crate) async fn persist_activated<C, O, P>(
    cycles: &C,
    objectives: &O,
    projects: &P,
    parents: ActivatedParents<'_>,
) -> Result<(), AppError>
where
    C: CycleRepository,
    O: ObjectiveRepository,
    P: ProjectRepository,
{
    if parents.cycle_change == EnsureActive::Activated {
        cycles.update(parents.cycle).await?;
    }
    if parents.objective_change == EnsureActive::Activated {
        objectives.update(parents.objective).await?;
    }
    if let Some((project, EnsureActive::Activated)) = parents.project {
        projects.update(project).await?;
    }
    Ok(())
}

pub(crate) fn ensure_cycle(
    cycle: &mut Cycle,
    now: OffsetDateTime,
) -> Result<EnsureActive, AppError> {
    Ok(cycle.ensure_active(now)?)
}

pub(crate) fn ensure_objective(
    objective: &mut Objective,
    now: OffsetDateTime,
) -> Result<EnsureActive, AppError> {
    Ok(objective.ensure_active(now)?)
}

pub(crate) fn ensure_project(
    project: &mut Project,
    now: OffsetDateTime,
) -> Result<EnsureActive, AppError> {
    Ok(project.ensure_active(now)?)
}
