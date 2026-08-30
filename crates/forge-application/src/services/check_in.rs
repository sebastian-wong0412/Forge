use forge_domain::{CheckIn, KeyResultId};
use time::{Date, OffsetDateTime};

use crate::AppError;
use crate::repos::{CheckInRepository, CycleRepository, KeyResultRepository, ObjectiveRepository};

use forge_domain::MilestoneState;

pub struct CreateCheckIn {
    pub value: Option<f64>,
    pub state: Option<MilestoneState>,
    pub note: Option<String>,
    pub checked_on: Date,
}

#[derive(Clone)]
pub struct CheckInService<C, O, K, I> {
    cycles: C,
    objectives: O,
    key_results: K,
    check_ins: I,
}

impl<C, O, K, I> CheckInService<C, O, K, I>
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
        key_result_id: KeyResultId,
        cmd: CreateCheckIn,
        now: OffsetDateTime,
    ) -> Result<CheckIn, AppError> {
        let key_result = self
            .key_results
            .get(key_result_id)
            .await?
            .ok_or_else(|| AppError::not_found("key_result", key_result_id))?;
        if !key_result.status().allows_check_in() {
            return Err(AppError::conflict(
                "cannot add a check-in to a completed or archived key result",
            ));
        }
        let objective = self
            .objectives
            .get(key_result.objective_id())
            .await?
            .ok_or_else(|| AppError::not_found("objective", key_result.objective_id()))?;
        if !objective.status().allows_children() {
            return Err(AppError::conflict(
                "cannot add a check-in when the objective is completed or archived",
            ));
        }
        let cycle = self
            .cycles
            .get(objective.cycle_id())
            .await?
            .ok_or_else(|| AppError::not_found("cycle", objective.cycle_id()))?;
        if !cycle.status().allows_tree_mutation() {
            return Err(AppError::conflict(
                "cannot add a check-in to a closed or archived cycle",
            ));
        }
        let check_in = CheckIn::create(
            key_result_id,
            key_result.progress_kind(),
            cmd.value,
            cmd.state,
            cmd.note,
            cmd.checked_on,
            now,
        )?;
        self.check_ins.create(&check_in).await?;
        Ok(check_in)
    }

    pub async fn list_by_key_result(
        &self,
        key_result_id: KeyResultId,
    ) -> Result<Vec<CheckIn>, AppError> {
        self.key_results
            .get(key_result_id)
            .await?
            .ok_or_else(|| AppError::not_found("key_result", key_result_id))?;
        self.check_ins.list_by_key_result(key_result_id).await
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::services::cycle::{CreateCycle, CycleService};
    use crate::services::key_result::{CreateKeyResult, KeyResultService};
    use crate::services::objective::{CreateObjective, ObjectiveService};
    use crate::test_support::{
        InMemoryCheckInRepo, InMemoryCycleRepo, InMemoryKeyResultRepo, InMemoryObjectiveRepo,
    };

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[tokio::test]
    async fn rejects_check_in_on_closed_cycle() {
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
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let kr = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "KR".into(),
                    description: None,
                    progress_kind: forge_domain::ProgressKind::Numeric,
                    start_value: Some(0.0),
                    target_value: Some(10.0),
                    unit: None,
                },
                NOW,
            )
            .await
            .unwrap();
        cycle_svc.activate(cycle.id(), NOW).await.unwrap();
        cycle_svc.close(cycle.id(), NOW).await.unwrap();
        let err = check_svc
            .create(
                kr.key_result.id(),
                CreateCheckIn {
                    value: Some(3.0),
                    state: None,
                    note: None,
                    checked_on: date!(2026 - 02 - 01),
                },
                NOW,
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Conflict { .. }));
    }

    #[tokio::test]
    async fn check_ins_are_append_only_and_latest_wins() {
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
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let kr = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "KR".into(),
                    description: None,
                    progress_kind: forge_domain::ProgressKind::Numeric,
                    start_value: Some(0.0),
                    target_value: Some(10.0),
                    unit: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let first = check_svc
            .create(
                kr.key_result.id(),
                CreateCheckIn {
                    value: Some(3.0),
                    state: None,
                    note: None,
                    checked_on: date!(2026 - 01 - 10),
                },
                NOW,
            )
            .await
            .unwrap();
        let second = check_svc
            .create(
                kr.key_result.id(),
                CreateCheckIn {
                    value: Some(7.0),
                    state: None,
                    note: None,
                    checked_on: date!(2026 - 01 - 20),
                },
                NOW,
            )
            .await
            .unwrap();
        let history = check_svc
            .list_by_key_result(kr.key_result.id())
            .await
            .unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].id(), first.id());
        assert_eq!(history[1].id(), second.id());
        assert_eq!(
            kr_svc.get(kr.key_result.id()).await.unwrap().current_value,
            Some(7.0)
        );
    }

    #[tokio::test]
    async fn rejects_check_in_payload_that_does_not_match_kind() {
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
                    title: "Ship".into(),
                    description: None,
                    start_on: None,
                    end_on: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let kr = kr_svc
            .create(
                objective.id(),
                CreateKeyResult {
                    title: "Launch".into(),
                    description: None,
                    progress_kind: forge_domain::ProgressKind::Milestone,
                    start_value: None,
                    target_value: None,
                    unit: None,
                },
                NOW,
            )
            .await
            .unwrap();
        let err = check_svc
            .create(
                kr.key_result.id(),
                CreateCheckIn {
                    value: Some(1.0),
                    state: None,
                    note: None,
                    checked_on: date!(2026 - 02 - 01),
                },
                NOW,
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Domain(_)));
    }
}
