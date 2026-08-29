use forge_domain::{CycleId, Objective, ObjectiveId, Title};
use time::{Date, OffsetDateTime};

use crate::AppError;
use crate::repos::{CycleRepository, ObjectiveRepository};

pub struct CreateObjective {
    pub title: String,
    pub description: Option<String>,
    pub start_on: Option<Date>,
    pub end_on: Option<Date>,
}

pub struct UpdateObjective {
    pub title: String,
    pub description: Option<String>,
    pub start_on: Option<Date>,
    pub end_on: Option<Date>,
}

#[derive(Clone)]
pub struct ObjectiveService<C, O> {
    cycles: C,
    objectives: O,
}

impl<C, O> ObjectiveService<C, O>
where
    C: CycleRepository,
    O: ObjectiveRepository,
{
    pub fn new(cycles: C, objectives: O) -> Self {
        Self { cycles, objectives }
    }

    pub async fn create(
        &self,
        cycle_id: CycleId,
        cmd: CreateObjective,
        now: OffsetDateTime,
    ) -> Result<Objective, AppError> {
        let cycle = self
            .cycles
            .get(cycle_id)
            .await?
            .ok_or_else(|| AppError::not_found("cycle", cycle_id))?;
        if !cycle.status().allows_tree_mutation() {
            return Err(AppError::conflict(
                "cannot add an objective to a closed or archived cycle",
            ));
        }
        let title = Title::parse(cmd.title)?;
        let objective = Objective::create(
            cycle_id,
            title,
            cmd.description,
            cmd.start_on,
            cmd.end_on,
            cycle.start_on(),
            cycle.end_on(),
            now,
        )?;
        self.objectives.create(&objective).await?;
        Ok(objective)
    }

    pub async fn get(&self, id: ObjectiveId) -> Result<Objective, AppError> {
        self.objectives
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("objective", id))
    }

    pub async fn list_by_cycle(&self, cycle_id: CycleId) -> Result<Vec<Objective>, AppError> {
        self.cycles
            .get(cycle_id)
            .await?
            .ok_or_else(|| AppError::not_found("cycle", cycle_id))?;
        self.objectives.list_by_cycle(cycle_id).await
    }

    pub async fn update(
        &self,
        id: ObjectiveId,
        cmd: UpdateObjective,
        now: OffsetDateTime,
    ) -> Result<Objective, AppError> {
        let mut objective = self.get(id).await?;
        let cycle = self
            .cycles
            .get(objective.cycle_id())
            .await?
            .ok_or_else(|| AppError::not_found("cycle", objective.cycle_id()))?;
        let title = Title::parse(cmd.title)?;
        objective.update(
            title,
            cmd.description,
            cmd.start_on,
            cmd.end_on,
            cycle.start_on(),
            cycle.end_on(),
            now,
        )?;
        self.objectives.update(&objective).await?;
        Ok(objective)
    }

    pub async fn activate(
        &self,
        id: ObjectiveId,
        now: OffsetDateTime,
    ) -> Result<Objective, AppError> {
        let mut objective = self.get(id).await?;
        objective.activate(now)?;
        self.objectives.update(&objective).await?;
        Ok(objective)
    }

    pub async fn complete(
        &self,
        id: ObjectiveId,
        now: OffsetDateTime,
    ) -> Result<Objective, AppError> {
        let mut objective = self.get(id).await?;
        objective.complete(now)?;
        self.objectives.update(&objective).await?;
        Ok(objective)
    }

    pub async fn archive(
        &self,
        id: ObjectiveId,
        now: OffsetDateTime,
    ) -> Result<Objective, AppError> {
        let mut objective = self.get(id).await?;
        objective.archive(now)?;
        self.objectives.update(&objective).await?;
        Ok(objective)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::services::cycle::{CreateCycle, CycleService};
    use crate::test_support::{InMemoryCycleRepo, InMemoryObjectiveRepo};

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    async fn setup() -> (
        CycleService<InMemoryCycleRepo>,
        ObjectiveService<InMemoryCycleRepo, InMemoryObjectiveRepo>,
    ) {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        (
            CycleService::new(cycles.clone()),
            ObjectiveService::new(cycles, objectives),
        )
    }

    #[tokio::test]
    async fn cannot_create_after_cycle_close() {
        let (cycles, objectives) = setup().await;
        let cycle = cycles
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
        cycles.activate(cycle.id(), NOW).await.unwrap();
        cycles.close(cycle.id(), NOW).await.unwrap();
        let err = objectives
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Nope".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Conflict { .. }));
    }

    #[tokio::test]
    async fn rejects_dates_outside_cycle() {
        let (cycles, objectives) = setup().await;
        let cycle = cycles
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
        let err = objectives
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Outside".into(),
                    description: None,
                    start_on: Some(date!(2025 - 12 - 01)),
                    end_on: Some(date!(2026 - 03 - 31)),
                },
                NOW,
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
    }

    #[tokio::test]
    async fn completed_objective_cannot_create_children_flag() {
        let (cycles, objectives) = setup().await;
        let cycle = cycles
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
        let created = objectives
            .create(
                cycle.id(),
                CreateObjective {
                    title: "Ship".into(),
                    description: None,
                    start_on: Some(date!(2026 - 01 - 01)),
                    end_on: Some(date!(2026 - 03 - 31)),
                },
                NOW,
            )
            .await
            .unwrap();
        objectives.activate(created.id(), NOW).await.unwrap();
        let completed = objectives.complete(created.id(), NOW).await.unwrap();
        assert!(!completed.status().allows_children());
    }
}
