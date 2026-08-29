use forge_domain::{Cycle, CycleId, Title};
use time::{Date, OffsetDateTime};

use crate::AppError;
use crate::repos::CycleRepository;

pub struct CreateCycle {
    pub name: String,
    pub start_on: Date,
    pub end_on: Date,
}

pub struct UpdateCycle {
    pub name: String,
    pub start_on: Date,
    pub end_on: Date,
}

#[derive(Clone)]
pub struct CycleService<R> {
    repo: R,
}

impl<R: CycleRepository> CycleService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(&self, cmd: CreateCycle, now: OffsetDateTime) -> Result<Cycle, AppError> {
        let name = Title::parse(cmd.name)?;
        let cycle = Cycle::create(name, cmd.start_on, cmd.end_on, now)?;
        self.repo.create(&cycle).await?;
        Ok(cycle)
    }

    pub async fn get(&self, id: CycleId) -> Result<Cycle, AppError> {
        self.repo
            .get(id)
            .await?
            .ok_or_else(|| AppError::not_found("cycle", id))
    }

    pub async fn list(&self) -> Result<Vec<Cycle>, AppError> {
        self.repo.list().await
    }

    pub async fn update(
        &self,
        id: CycleId,
        cmd: UpdateCycle,
        now: OffsetDateTime,
    ) -> Result<Cycle, AppError> {
        let mut cycle = self.get(id).await?;
        let name = Title::parse(cmd.name)?;
        cycle.update(name, cmd.start_on, cmd.end_on, now)?;
        self.repo.update(&cycle).await?;
        Ok(cycle)
    }

    pub async fn activate(&self, id: CycleId, now: OffsetDateTime) -> Result<Cycle, AppError> {
        let mut cycle = self.get(id).await?;
        cycle.activate(now)?;
        self.repo.update(&cycle).await?;
        Ok(cycle)
    }

    pub async fn close(&self, id: CycleId, now: OffsetDateTime) -> Result<Cycle, AppError> {
        let mut cycle = self.get(id).await?;
        cycle.close(now)?;
        self.repo.update(&cycle).await?;
        Ok(cycle)
    }

    pub async fn archive(&self, id: CycleId, now: OffsetDateTime) -> Result<Cycle, AppError> {
        let mut cycle = self.get(id).await?;
        cycle.archive(now)?;
        self.repo.update(&cycle).await?;
        Ok(cycle)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::test_support::InMemoryCycleRepo;
    use forge_domain::CycleStatus;

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    fn service() -> CycleService<InMemoryCycleRepo> {
        CycleService::new(InMemoryCycleRepo::default())
    }

    #[tokio::test]
    async fn create_activate_close_and_reject_children_after_close() {
        let svc = service();
        let created = svc
            .create(
                CreateCycle {
                    name: "  2026 Q1  ".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(created.name().as_str(), "2026 Q1");
        assert_eq!(created.status(), CycleStatus::Planning);

        let active = svc.activate(created.id(), NOW).await.unwrap();
        assert_eq!(active.status(), CycleStatus::Active);
        let closed = svc.close(created.id(), NOW).await.unwrap();
        assert_eq!(closed.status(), CycleStatus::Closed);
        assert!(!closed.status().allows_tree_mutation());
    }

    #[tokio::test]
    async fn multiple_active_cycles_are_allowed() {
        let svc = service();
        let first = svc
            .create(
                CreateCycle {
                    name: "A".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        let second = svc
            .create(
                CreateCycle {
                    name: "B".into(),
                    start_on: date!(2026 - 04 - 01),
                    end_on: date!(2026 - 06 - 30),
                },
                NOW,
            )
            .await
            .unwrap();
        svc.activate(first.id(), NOW).await.unwrap();
        svc.activate(second.id(), NOW).await.unwrap();
        assert_eq!(svc.list().await.unwrap().len(), 2);
    }
}
