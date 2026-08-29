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
    assert!(versions.contains(&3), "0003 was not applied: {versions:?}");

    let scheduled_on_exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM pragma_table_info('tasks') WHERE name = 'scheduled_on'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(scheduled_on_exists, 1);

    let scheduled_on_index: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master
         WHERE type = 'index' AND name = 'idx_tasks_scheduled_on'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(scheduled_on_index, 1);

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
    assert_eq!(stored_task.scheduled_on(), None);

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

const TODAY: time::Date = date!(2026 - 08 - 30);
const YESTERDAY: time::Date = date!(2026 - 08 - 29);
const TOMORROW: time::Date = date!(2026 - 08 - 31);

struct TaskFixture {
    pool: sqlx::SqlitePool,
    tasks: SqliteTaskRepository,
    task_svc: TaskService<
        SqliteCycleRepository,
        SqliteObjectiveRepository,
        SqliteProjectRepository,
        SqliteTaskRepository,
    >,
    project_id: forge_domain::ProjectId,
    _dir: tempfile::TempDir,
}

async fn task_fixture() -> TaskFixture {
    let (pool, dir) = migrated_pool().await;
    let cycles = SqliteCycleRepository::new(pool.clone());
    let objectives = SqliteObjectiveRepository::new(pool.clone());
    let projects = SqliteProjectRepository::new(pool.clone());
    let tasks = SqliteTaskRepository::new(pool.clone());
    let cycle_svc = CycleService::new(cycles.clone());
    let objective_svc = ObjectiveService::new(cycles.clone(), objectives.clone());
    let project_svc = ProjectService::new(cycles.clone(), objectives.clone(), projects.clone());
    let task_svc = TaskService::new(
        cycles.clone(),
        objectives.clone(),
        projects.clone(),
        tasks.clone(),
    );
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
                title: "Work".into(),
                description: None,
            },
            NOW,
        )
        .await
        .unwrap();
    project_svc.activate(project.id(), NOW).await.unwrap();
    TaskFixture {
        pool,
        tasks,
        task_svc,
        project_id: project.id(),
        _dir: dir,
    }
}

async fn create_task(
    fx: &TaskFixture,
    title: &str,
    scheduled_on: Option<time::Date>,
) -> forge_domain::Task {
    fx.task_svc
        .create(
            fx.project_id,
            CreateTask {
                title: title.into(),
                description: None,
                scheduled_on,
            },
            NOW,
        )
        .await
        .unwrap()
}

fn ids(tasks: &[forge_domain::Task]) -> Vec<String> {
    tasks.iter().map(|task| task.id().to_string()).collect()
}

#[tokio::test]
async fn scheduled_on_round_trips_null_and_date() {
    let fx = task_fixture().await;
    let unscheduled = create_task(&fx, "Inbox", None).await;
    let stored = fx.tasks.get(unscheduled.id()).await.unwrap().unwrap();
    assert_eq!(stored.scheduled_on(), None);
    let raw: Option<String> = sqlx::query_scalar("SELECT scheduled_on FROM tasks WHERE id = ?")
        .bind(unscheduled.id().to_string())
        .fetch_one(&fx.pool)
        .await
        .unwrap();
    assert_eq!(raw, None);

    let scheduled = create_task(&fx, "Planned", Some(TODAY)).await;
    let stored = fx.tasks.get(scheduled.id()).await.unwrap().unwrap();
    assert_eq!(stored.scheduled_on(), Some(TODAY));
    let listed = fx.tasks.list_by_project(fx.project_id).await.unwrap();
    assert_eq!(
        listed
            .iter()
            .find(|task| task.id() == scheduled.id())
            .unwrap()
            .scheduled_on(),
        Some(TODAY)
    );

    fx.task_svc
        .schedule(scheduled.id(), Some(TOMORROW), NOW)
        .await
        .unwrap();
    assert_eq!(
        fx.tasks
            .get(scheduled.id())
            .await
            .unwrap()
            .unwrap()
            .scheduled_on(),
        Some(TOMORROW)
    );

    fx.task_svc
        .schedule(scheduled.id(), None, NOW)
        .await
        .unwrap();
    assert_eq!(
        fx.tasks
            .get(scheduled.id())
            .await
            .unwrap()
            .unwrap()
            .scheduled_on(),
        None
    );
    let raw: Option<String> = sqlx::query_scalar("SELECT scheduled_on FROM tasks WHERE id = ?")
        .bind(scheduled.id().to_string())
        .fetch_one(&fx.pool)
        .await
        .unwrap();
    assert_eq!(raw, None);

    let listed = fx.tasks.list_by_project(fx.project_id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].scheduled_on(), None);
    assert_eq!(listed[1].scheduled_on(), None);
}

#[tokio::test]
async fn list_today_candidates_includes_expected_tasks() {
    let fx = task_fixture().await;
    let todo_today = create_task(&fx, "Todo today", Some(TODAY)).await;
    let progress_today = create_task(&fx, "Progress today", Some(TODAY)).await;
    fx.task_svc.start(progress_today.id(), NOW).await.unwrap();
    let todo_yesterday = create_task(&fx, "Todo yesterday", Some(YESTERDAY)).await;
    let progress_yesterday = create_task(&fx, "Progress yesterday", Some(YESTERDAY)).await;
    fx.task_svc
        .start(progress_yesterday.id(), NOW)
        .await
        .unwrap();
    let unscheduled_progress = create_task(&fx, "Going", None).await;
    fx.task_svc
        .start(unscheduled_progress.id(), NOW)
        .await
        .unwrap();
    let done_today = create_task(&fx, "Done today", None).await;
    fx.task_svc.start(done_today.id(), NOW).await.unwrap();
    fx.task_svc
        .complete(done_today.id(), datetime!(2026-08-30 15:00:00 UTC))
        .await
        .unwrap();

    let candidates = fx.tasks.list_today_candidates(TODAY).await.unwrap();
    let candidate_ids = ids(&candidates);
    for id in [
        todo_today.id(),
        progress_today.id(),
        todo_yesterday.id(),
        progress_yesterday.id(),
        unscheduled_progress.id(),
        done_today.id(),
    ] {
        assert!(
            candidate_ids.contains(&id.to_string()),
            "missing candidate {id}"
        );
    }
}

#[tokio::test]
async fn list_today_candidates_excludes_non_today_tasks() {
    let fx = task_fixture().await;
    let inbox = create_task(&fx, "Inbox", None).await;
    let todo_tomorrow = create_task(&fx, "Todo tomorrow", Some(TOMORROW)).await;
    let progress_tomorrow = create_task(&fx, "Progress tomorrow", Some(TOMORROW)).await;
    fx.task_svc
        .start(progress_tomorrow.id(), NOW)
        .await
        .unwrap();
    let done_tomorrow = create_task(&fx, "Done tomorrow", None).await;
    fx.task_svc.start(done_tomorrow.id(), NOW).await.unwrap();
    fx.task_svc
        .complete(done_tomorrow.id(), datetime!(2026-08-31 15:00:00 UTC))
        .await
        .unwrap();
    let cancelled = create_task(&fx, "Cancelled", Some(TODAY)).await;
    fx.task_svc.cancel(cancelled.id(), NOW).await.unwrap();

    let candidates = fx.tasks.list_today_candidates(TODAY).await.unwrap();
    let candidate_ids = ids(&candidates);
    for id in [
        inbox.id(),
        todo_tomorrow.id(),
        progress_tomorrow.id(),
        done_tomorrow.id(),
        cancelled.id(),
    ] {
        assert!(
            !candidate_ids.contains(&id.to_string()),
            "unexpected candidate {id}"
        );
    }
}

#[tokio::test]
async fn list_today_candidates_uses_utc_midnight_boundary() {
    let fx = task_fixture().await;
    let before = create_task(&fx, "Before midnight", None).await;
    fx.task_svc.start(before.id(), NOW).await.unwrap();
    fx.task_svc
        .complete(before.id(), datetime!(2026-08-30 23:59:59 UTC))
        .await
        .unwrap();
    let after = create_task(&fx, "After midnight", None).await;
    fx.task_svc.start(after.id(), NOW).await.unwrap();
    fx.task_svc
        .complete(after.id(), datetime!(2026-08-31 00:00:00 UTC))
        .await
        .unwrap();

    let on_thirtieth = ids(&fx.tasks.list_today_candidates(TODAY).await.unwrap());
    let on_thirty_first = ids(&fx.tasks.list_today_candidates(TOMORROW).await.unwrap());
    assert!(on_thirtieth.contains(&before.id().to_string()));
    assert!(!on_thirtieth.contains(&after.id().to_string()));
    assert!(!on_thirty_first.contains(&before.id().to_string()));
    assert!(on_thirty_first.contains(&after.id().to_string()));
}

#[tokio::test]
async fn today_candidates_ignore_daily_execution_rows() {
    use forge_application::repos::DailyExecutionRepository;
    use forge_application::{CreateDailyExecution, DailyExecutionService};
    use forge_domain::DailyExecutionStatus;
    use forge_server::sqlite::SqliteDailyExecutionRepository;

    let fx = task_fixture().await;
    let executions = SqliteDailyExecutionRepository::new(fx.pool.clone());
    let execution_svc = DailyExecutionService::new(fx.tasks.clone(), executions.clone());

    let inbox = create_task(&fx, "Inbox with log", None).await;
    execution_svc
        .create(
            inbox.id(),
            CreateDailyExecution {
                execution_date: TODAY,
                notes: Some("legacy".into()),
                status: DailyExecutionStatus::Planned,
            },
            NOW,
        )
        .await
        .unwrap();
    let scheduled = create_task(&fx, "Scheduled without log", Some(TODAY)).await;

    let candidates = fx.tasks.list_today_candidates(TODAY).await.unwrap();
    let candidate_ids = ids(&candidates);
    assert!(!candidate_ids.contains(&inbox.id().to_string()));
    assert!(candidate_ids.contains(&scheduled.id().to_string()));

    let by_task = executions.list_by_task(inbox.id()).await.unwrap();
    assert_eq!(by_task.len(), 1);
    let by_date = executions.list_by_date(TODAY).await.unwrap();
    assert_eq!(by_date.len(), 1);
}
