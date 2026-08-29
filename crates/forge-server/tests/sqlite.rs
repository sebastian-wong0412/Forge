#![allow(clippy::unwrap_used, clippy::expect_used)]

use forge_application::repos::{
    CheckInRepository, CycleRepository, ObjectiveRepository, ReviewRepository, TaskRepository,
};
use forge_application::{
    CheckInService, CreateCheckIn, CreateCycle, CreateKeyResult, CreateObjective, CreateProject,
    CreateReview, CreateTask, CycleService, KeyResultService, ObjectiveService, ProjectService,
    ReviewService, TaskService,
};
use forge_server::sqlite::{
    SqliteCheckInRepository, SqliteCycleRepository, SqliteKeyResultRepository,
    SqliteObjectiveRepository, SqliteProjectRepository, SqliteReviewRepository,
    SqliteTaskRepository,
};
use time::macros::{date, datetime};

const NOW: time::OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

async fn migrated_pool() -> (sqlx::SqlitePool, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("forge.db");
    let pool = forge_server::db::connect(&path).await.unwrap();
    forge_server::db::migrate(&pool).await.unwrap();
    (pool, dir)
}

#[tokio::test]
async fn fresh_database_applies_phase_1a_schema() {
    let (pool, _dir) = migrated_pool().await;

    let tables: Vec<String> = sqlx::query_scalar(
        "SELECT name FROM sqlite_master
         WHERE type = 'table'
         ORDER BY name",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    for required in [
        "cycles",
        "objectives",
        "key_results",
        "check_ins",
        "projects",
        "tasks",
        "reviews",
        "visions",
        "daily_executions",
    ] {
        assert!(
            tables.iter().any(|name| name == required),
            "missing table {required}: {tables:?}"
        );
    }

    let versions: Vec<i64> =
        sqlx::query_scalar("SELECT version FROM _sqlx_migrations ORDER BY version")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert!(versions.contains(&1), "0001 was not applied: {versions:?}");
    assert!(versions.contains(&2), "0002 was not applied: {versions:?}");

    let completed_at_exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM pragma_table_info('tasks') WHERE name = 'completed_at'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(completed_at_exists, 1);

    let current_value_exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM pragma_table_info('key_results') WHERE name = 'current_value'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(current_value_exists, 0);
}

#[tokio::test]
async fn repositories_round_trip_phase_1a_entities() {
    let (pool, _dir) = migrated_pool().await;
    let cycles = SqliteCycleRepository::new(pool.clone());
    let objectives = SqliteObjectiveRepository::new(pool.clone());
    let key_results = SqliteKeyResultRepository::new(pool.clone());
    let check_ins = SqliteCheckInRepository::new(pool.clone());
    let projects = SqliteProjectRepository::new(pool.clone());
    let tasks = SqliteTaskRepository::new(pool.clone());
    let reviews = SqliteReviewRepository::new(pool);

    let cycle_svc = CycleService::new(cycles.clone());
    let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
    let kr_svc = KeyResultService::new(
        cycles.clone(),
        objectives.clone(),
        key_results.clone(),
        check_ins.clone(),
    );
    let check_svc = CheckInService::new(
        cycles.clone(),
        objectives.clone(),
        key_results.clone(),
        check_ins.clone(),
    );
    let project_svc = ProjectService::new(cycles.clone(), objectives.clone(), projects.clone());
    let task_svc = TaskService::new(
        cycles.clone(),
        objectives.clone(),
        projects.clone(),
        tasks.clone(),
    );
    let review_svc = ReviewService::new(cycles.clone(), reviews.clone());

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
    let loaded_cycle = cycles.get(cycle.id()).await.unwrap().unwrap();
    assert_eq!(loaded_cycle.name().as_str(), "Q1");

    let objective = objective_svc
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
    assert_eq!(objectives.list_by_cycle(cycle.id()).await.unwrap().len(), 1);

    let kr = kr_svc
        .create(
            objective.id(),
            CreateKeyResult {
                title: "Weight".into(),
                description: None,
                start_value: 500.0,
                target_value: Some(200.0),
                unit: Some("kg".into()),
            },
            NOW,
        )
        .await
        .unwrap();
    assert_eq!(kr.current_value, 500.0);

    check_svc
        .create(
            kr.key_result.id(),
            CreateCheckIn {
                value: 400.0,
                note: Some("week 1".into()),
                checked_on: date!(2026 - 01 - 10),
            },
            NOW,
        )
        .await
        .unwrap();
    check_svc
        .create(
            kr.key_result.id(),
            CreateCheckIn {
                value: 300.0,
                note: None,
                checked_on: date!(2026 - 02 - 01),
            },
            NOW,
        )
        .await
        .unwrap();
    let history = check_ins
        .list_by_key_result(kr.key_result.id())
        .await
        .unwrap();
    assert_eq!(history.len(), 2);
    let snapshot = kr_svc.get(kr.key_result.id()).await.unwrap();
    assert_eq!(snapshot.current_value, 300.0);
    assert!((snapshot.progress.unwrap() - 2.0 / 3.0).abs() < 1e-9);

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
    project_svc.activate(project.id(), NOW).await.unwrap();
    let task = task_svc
        .create(
            project.id(),
            CreateTask {
                title: "Do it".into(),
                description: None,
                scheduled_on: None,
            },
            NOW,
        )
        .await
        .unwrap();
    task_svc.start(task.id(), NOW).await.unwrap();
    let done = task_svc.complete(task.id(), NOW).await.unwrap();
    let stored_task = tasks.get(task.id()).await.unwrap().unwrap();
    assert_eq!(stored_task.completed_at(), done.completed_at());

    cycle_svc.activate(cycle.id(), NOW).await.unwrap();
    cycle_svc.close(cycle.id(), NOW).await.unwrap();
    review_svc
        .create(
            cycle.id(),
            CreateReview {
                content: "Closed-cycle notes".into(),
                period_start: Some(date!(2026 - 01 - 01)),
                period_end: Some(date!(2026 - 03 - 31)),
            },
            NOW,
        )
        .await
        .unwrap();
    assert_eq!(reviews.list_by_cycle(cycle.id()).await.unwrap().len(), 1);
}

#[tokio::test]
async fn visions_table_remains_as_unused_legacy_storage() {
    let (pool, _dir) = migrated_pool().await;
    sqlx::query(
        "INSERT INTO visions (id, title, status, created_at, updated_at)
         VALUES ('legacy', 'Old vision', 'active', '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
    )
    .execute(&pool)
    .await
    .unwrap();
    let title: String = sqlx::query_scalar("SELECT title FROM visions WHERE id = 'legacy'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(title, "Old vision");
}
