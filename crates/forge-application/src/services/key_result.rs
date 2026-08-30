use forge_domain::{
    KeyResult, KeyResultId, MilestoneState, ObjectiveId, ProgressDefinition, ProgressKind, Title,
    latest_check_in,
};
use time::OffsetDateTime;

use crate::AppError;
use crate::repos::{CheckInRepository, CycleRepository, KeyResultRepository, ObjectiveRepository};

pub struct CreateKeyResult {
    pub title: String,
    pub description: Option<String>,
    pub progress_kind: ProgressKind,
    pub start_value: Option<f64>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
}

pub struct UpdateKeyResult {
    pub title: String,
    pub description: Option<String>,
    pub start_value: Option<f64>,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyResultSnapshot {
    pub key_result: KeyResult,
    pub current_value: Option<f64>,
    pub current_state: Option<MilestoneState>,
    pub latest_note: Option<String>,
    pub progress: Option<f64>,
}

#[derive(Clone)]
pub struct KeyResultService<C, O, K, I> {
    cycles: C,
    objectives: O,
    key_results: K,
    check_ins: I,
}

impl<C, O, K, I> KeyResultService<C, O, K, I>
where
    C: CycleRepository,
    O: ObjectiveRepository,
    K: KeyResultRepository,
    I: CheckInRepository,
{
    pub fn new(cycles: C, objectives: O, key_results: K, check_ins: I) -> Self {
        Self {
            cycles,
            objectives,
            key_results,
            check_ins,
        }
    }

    pub async fn create(
        &self,
        objective_id: ObjectiveId,
        cmd: CreateKeyResult,
        now: OffsetDateTime,
    ) -> Result<KeyResultSnapshot, AppError> {
        self.ensure_can_mutate_objective(objective_id).await?;
        let title = Title::parse(cmd.title)?;
        let definition = ProgressDefinition::parse(
            cmd.progress_kind,
            cmd.start_value,
            cmd.target_value,
            cmd.unit,
        )?;
        let key_result = KeyResult::create(objective_id, title, cmd.description, definition, now);
        self.key_results.create(&key_result).await?;
        self.snapshot(key_result).await
    }

    pub async fn get(&self, id: KeyResultId) -> Result<KeyResultSnapshot, AppError> {
        let key_result = self
            .key_results
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("key_result", id))?;
        self.snapshot(key_result).await
    }

    pub async fn list_by_objective(
        &self,
        objective_id: ObjectiveId,
    ) -> Result<Vec<KeyResultSnapshot>, AppError> {
        self.objectives
            .get(objective_id)
            .await?
            .ok_or_else(|| AppError::not_found("objective", objective_id))?;
        let mut snapshots = Vec::new();
        for key_result in self.key_results.list_by_objective(objective_id).await? {
            snapshots.push(self.snapshot(key_result).await?);
        }
        Ok(snapshots)
    }

    pub async fn update(
        &self,
        id: KeyResultId,
        cmd: UpdateKeyResult,
        now: OffsetDateTime,
    ) -> Result<KeyResultSnapshot, AppError> {
        let mut key_result = self
            .key_results
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("key_result", id))?;
        let title = Title::parse(cmd.title)?;
        let definition = ProgressDefinition::parse(
            key_result.progress_kind(),
            cmd.start_value,
            cmd.target_value,
            cmd.unit,
        )?;
        key_result.update(title, cmd.description, definition, now)?;
        self.key_results.update(&key_result).await?;
        self.snapshot(key_result).await
    }

    pub async fn activate(
        &self,
        id: KeyResultId,
        now: OffsetDateTime,
    ) -> Result<KeyResultSnapshot, AppError> {
        let mut key_result = self
            .key_results
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("key_result", id))?;
        key_result.activate(now)?;
        self.key_results.update(&key_result).await?;
        self.snapshot(key_result).await
    }

    pub async fn complete(
        &self,
        id: KeyResultId,
        now: OffsetDateTime,
    ) -> Result<KeyResultSnapshot, AppError> {
        let mut key_result = self
            .key_results
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("key_result", id))?;
        key_result.complete(now)?;
        self.key_results.update(&key_result).await?;
        self.snapshot(key_result).await
    }

    pub async fn archive(
        &self,
        id: KeyResultId,
        now: OffsetDateTime,
    ) -> Result<KeyResultSnapshot, AppError> {
        let mut key_result = self
            .key_results
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("key_result", id))?;
        key_result.archive(now)?;
        self.key_results.update(&key_result).await?;
        self.snapshot(key_result).await
    }

    async fn snapshot(&self, key_result: KeyResult) -> Result<KeyResultSnapshot, AppError> {
        let check_ins = self.check_ins.list_by_key_result(key_result.id()).await?;
        let latest = latest_check_in(&check_ins);
        Ok(KeyResultSnapshot {
            current_value: key_result.current_value(latest),
            current_state: key_result.current_state(latest),
            latest_note: key_result.latest_note(latest).map(str::to_string),
            progress: key_result.progress(latest),
            key_result,
        })
    }

    async fn ensure_can_mutate_objective(&self, objective_id: ObjectiveId) -> Result<(), AppError> {
        let objective = self
            .objectives
            .get(objective_id)
            .await?
            .ok_or_else(|| AppError::not_found("objective", objective_id))?;
        if !objective.status().allows_children() {
            return Err(AppError::conflict(
                "cannot add a key result to a completed or archived objective",
            ));
        }
        let cycle = self
            .cycles
            .get(objective.cycle_id())
            .await?
            .ok_or_else(|| AppError::not_found("cycle", objective.cycle_id()))?;
        if !cycle.status().allows_tree_mutation() {
            return Err(AppError::conflict(
                "cannot add a key result to a closed or archived cycle",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::services::check_in::{CheckInService, CreateCheckIn};
    use crate::services::cycle::{CreateCycle, CycleService};
    use crate::services::objective::{CreateObjective, ObjectiveService};
    use crate::test_support::{
        InMemoryCheckInRepo, InMemoryCycleRepo, InMemoryKeyResultRepo, InMemoryObjectiveRepo,
    };

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[tokio::test]
    async fn derived_current_and_progress() {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let key_results = InMemoryKeyResultRepo::default();
        let check_ins = InMemoryCheckInRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
        let kr_svc = KeyResultService::new(
            cycles.clone(),
            objectives.clone(),
            key_results.clone(),
            check_ins.clone(),
        );
        let check_svc = CheckInService::new(cycles, objectives, key_results, check_ins);

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
                    title: "Health".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let created = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "Weight".into(),
                    description: None,
                    progress_kind: ProgressKind::Numeric,
                    start_value: Some(500.0),
                    target_value: Some(200.0),
                    unit: Some("kg".into()),
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(created.current_value, Some(500.0));
        assert_eq!(created.progress, Some(0.0));
        assert_eq!(created.key_result.progress_kind(), ProgressKind::Numeric);

        check_svc
            .create(
                created.key_result.id(),
                CreateCheckIn {
                    value: Some(300.0),
                    state: None,
                    note: None,
                    checked_on: date!(2026 - 02 - 01),
                },
                NOW,
            )
            .await
            .unwrap();
        let updated = kr_svc.get(created.key_result.id()).await.unwrap();
        assert_eq!(updated.current_value, Some(300.0));
        assert!((updated.progress.unwrap() - 2.0 / 3.0).abs() < 1e-9);

        objective_svc.activate(objective.id(), NOW).await.unwrap();
        objective_svc.complete(objective.id(), NOW).await.unwrap();
        let blocked = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "Late".into(),
                    description: None,
                    progress_kind: ProgressKind::Numeric,
                    start_value: Some(0.0),
                    target_value: Some(1.0),
                    unit: None,
                },
                NOW,
            )
            .await
            .unwrap_err();
        assert!(matches!(blocked, AppError::Conflict { .. }));
    }

    #[tokio::test]
    async fn creates_each_progress_kind() {
        let cycles = InMemoryCycleRepo::default();
        let objectives = InMemoryObjectiveRepo::default();
        let key_results = InMemoryKeyResultRepo::default();
        let check_ins = InMemoryCheckInRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
        let kr_svc = KeyResultService::new(cycles, objectives, key_results, check_ins);
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

        let percentage = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "Coverage".into(),
                    description: None,
                    progress_kind: ProgressKind::Percentage,
                    start_value: Some(60.0),
                    target_value: Some(90.0),
                    unit: None,
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(percentage.progress, Some(0.0));

        let milestone = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "Launch".into(),
                    description: None,
                    progress_kind: ProgressKind::Milestone,
                    start_value: None,
                    target_value: None,
                    unit: None,
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(milestone.current_state, Some(MilestoneState::NotStarted));
        assert_eq!(milestone.progress, Some(0.0));
        assert_eq!(milestone.current_value, None);

        let qualitative = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "Learn".into(),
                    description: None,
                    progress_kind: ProgressKind::Qualitative,
                    start_value: None,
                    target_value: None,
                    unit: None,
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(qualitative.progress, None);
        assert_eq!(qualitative.latest_note, None);
    }
}
