#![allow(clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tempfile::TempDir;
use tower::ServiceExt;

const TODAY: &str = "2026-08-30";
const YESTERDAY: &str = "2026-08-29";
const TOMORROW: &str = "2026-08-31";

async fn setup_with_pool() -> (axum::Router, TempDir, sqlx::SqlitePool) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("forge.db");
    let pool = forge_server::db::connect(&path).await.unwrap();
    forge_server::db::migrate(&pool).await.unwrap();
    (forge_server::api::router(pool.clone()), dir, pool)
}

async fn setup() -> (axum::Router, TempDir) {
    let (app, dir, _pool) = setup_with_pool().await;
    (app, dir)
}

async fn send(
    app: &axum::Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    let request_body = if let Some(body) = body {
        builder = builder.header("content-type", "application/json");
        Body::from(serde_json::to_vec(&body).unwrap())
    } else {
        Body::empty()
    };
    let request = builder.body(request_body).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, json)
}

async fn create_cycle(app: &axum::Router) -> Value {
    let (status, cycle) = send(
        app,
        "POST",
        "/api/v1/cycles",
        Some(json!({
            "name": "2026 Q1",
            "start_on": "2026-01-01",
            "end_on": "2026-03-31"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    cycle
}

async fn active_project_id(app: &axum::Router) -> String {
    let cycle = create_cycle(app).await;
    let cycle_id = cycle["id"].as_str().unwrap();
    let (status, objective) = send(
        app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/objectives"),
        Some(json!({ "title": "Ship" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let objective_id = objective["id"].as_str().unwrap();

    let (status, project) = send(
        app,
        "POST",
        &format!("/api/v1/objectives/{objective_id}/projects"),
        Some(json!({ "title": "Work" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let project_id = project["id"].as_str().unwrap();

    let (status, active) = send(
        app,
        "POST",
        &format!("/api/v1/projects/{project_id}/activate"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(active["status"], "active");
    project_id.to_string()
}

async fn create_task(
    app: &axum::Router,
    project_id: &str,
    title: &str,
    scheduled_on: Option<&str>,
) -> Value {
    let mut body = json!({ "title": title });
    if let Some(scheduled_on) = scheduled_on {
        body["scheduled_on"] = json!(scheduled_on);
    }
    let (status, task) = send(
        app,
        "POST",
        &format!("/api/v1/projects/{project_id}/tasks"),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{task}");
    task
}

fn task_ids(bucket: &Value) -> Vec<String> {
    bucket
        .as_array()
        .unwrap()
        .iter()
        .map(|task| task["id"].as_str().unwrap().to_string())
        .collect()
}

fn assert_task_fields(task: &Value) {
    assert!(task.get("id").is_some());
    assert!(task.get("project_id").is_some());
    assert!(task.get("title").is_some());
    assert!(task.get("status").is_some());
    assert!(task.get("scheduled_on").is_some());
    assert!(task.get("completed_at").is_some());
}

fn assert_client_validation(status: StatusCode) {
    assert!(
        status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY,
        "expected existing validation status, got {status}"
    );
}

async fn set_completed_at(pool: &sqlx::SqlitePool, task_id: &str, completed_at: &str) {
    sqlx::query("UPDATE tasks SET completed_at = ? WHERE id = ?")
        .bind(completed_at)
        .bind(task_id)
        .execute(pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn health_ok() {
    let (app, _dir) = setup().await;
    let (status, body) = send(&app, "GET", "/health", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn cycle_hierarchy_checkins_and_reviews() {
    let (app, _dir) = setup().await;

    let cycle = create_cycle(&app).await;
    let cycle_id = cycle["id"].as_str().unwrap();
    assert_eq!(cycle["status"], "planning");

    let (status, fetched) = send(&app, "GET", &format!("/api/v1/cycles/{cycle_id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["name"], "2026 Q1");

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/api/v1/cycles/{cycle_id}"),
        Some(json!({
            "name": "2026 Q1 Plan",
            "start_on": "2026-01-01",
            "end_on": "2026-03-31"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["name"], "2026 Q1 Plan");

    let (status, active) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/activate"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(active["status"], "active");

    let (status, second) = send(
        &app,
        "POST",
        "/api/v1/cycles",
        Some(json!({
            "name": "2026 Q2",
            "start_on": "2026-04-01",
            "end_on": "2026-06-30"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let (status, second_active) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{}/activate", second["id"].as_str().unwrap()),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(second_active["status"], "active");

    let (status, objective) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/objectives"),
        Some(json!({
            "title": "Save more",
            "start_on": "2026-01-01",
            "end_on": "2026-03-31"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let objective_id = objective["id"].as_str().unwrap();
    assert_eq!(objective["cycle_id"], cycle_id);
    assert_eq!(objective["status"], "draft");

    let (status, key_result) = send(
        &app,
        "POST",
        &format!("/api/v1/objectives/{objective_id}/key-results"),
        Some(json!({
            "title": "Emergency fund",
            "start_value": 1000.0,
            "target_value": 10000.0,
            "unit": "USD"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let key_result_id = key_result["id"].as_str().unwrap();
    assert_eq!(key_result["current_value"], 1000.0);
    assert_eq!(key_result["progress"], 0.0);

    let (status, check_in) = send(
        &app,
        "POST",
        &format!("/api/v1/key-results/{key_result_id}/check-ins"),
        Some(json!({
            "value": 4000.0,
            "note": "first deposit",
            "checked_on": "2026-02-01"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(check_in["checked_on"], "2026-02-01");

    let (status, history) = send(
        &app,
        "GET",
        &format!("/api/v1/key-results/{key_result_id}/check-ins"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(history.as_array().unwrap().len(), 1);

    let (status, later) = send(
        &app,
        "POST",
        &format!("/api/v1/key-results/{key_result_id}/check-ins"),
        Some(json!({
            "value": 5500.0,
            "checked_on": "2026-02-15"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_ne!(later["id"], check_in["id"]);

    let (status, derived) = send(
        &app,
        "GET",
        &format!("/api/v1/key-results/{key_result_id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(derived["current_value"], 5500.0);
    assert!((derived["progress"].as_f64().unwrap() - 0.5).abs() < 1e-9);

    let (status, project) = send(
        &app,
        "POST",
        &format!("/api/v1/objectives/{objective_id}/projects"),
        Some(json!({ "title": "Automate expenses" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let project_id = project["id"].as_str().unwrap();
    assert_eq!(project["objective_id"], objective_id);
    assert_eq!(project["status"], "draft");

    let (status, _) = send(
        &app,
        "POST",
        &format!("/api/v1/projects/{project_id}/tasks"),
        Some(json!({ "title": "Too soon" })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    let (status, active_project) = send(
        &app,
        "POST",
        &format!("/api/v1/projects/{project_id}/activate"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(active_project["status"], "active");

    let (status, task) = send(
        &app,
        "POST",
        &format!("/api/v1/projects/{project_id}/tasks"),
        Some(json!({ "title": "Set up spreadsheet" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let task_id = task["id"].as_str().unwrap();
    assert_eq!(task["status"], "todo");
    assert!(task["scheduled_on"].is_null());
    assert!(task["completed_at"].is_null());

    let (status, started) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{task_id}/start"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(started["status"], "in_progress");

    let (status, done) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{task_id}/complete"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(done["status"], "done");
    assert!(!done["completed_at"].is_null());

    let (status, _) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{task_id}/cancel"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let (status, execution) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{task_id}/daily-executions"),
        Some(json!({
            "execution_date": "2026-01-15",
            "notes": "first pass",
            "status": "completed"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(execution["status"], "completed");

    let (status, by_date) = send(
        &app,
        "GET",
        "/api/v1/daily-executions?date=2026-01-15",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(by_date.as_array().unwrap().len(), 1);

    let (status, closed) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/close"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(closed["status"], "closed");

    let (status, _) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/objectives"),
        Some(json!({ "title": "Too late" })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    let (status, review) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/reviews"),
        Some(json!({
            "content": "Foundation is in place.",
            "period_start": "2026-01-01",
            "period_end": "2026-03-31"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(review["cycle_id"], cycle_id);

    let (status, reviews) = send(
        &app,
        "GET",
        &format!("/api/v1/cycles/{cycle_id}/reviews"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(reviews.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn visions_and_kr_project_routes_are_removed() {
    let (app, _dir) = setup().await;
    let (status, _) = send(
        &app,
        "POST",
        "/api/v1/visions",
        Some(json!({ "title": "Independence" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, _) = send(
        &app,
        "POST",
        "/api/v1/key-results/01900000-0000-7000-8000-000000000000/projects",
        Some(json!({ "title": "Nope" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn not_found_and_validation_errors() {
    let (app, _dir) = setup().await;
    let missing_id = "01900000-0000-7000-8000-000000000000";

    let (status, body) = send(&app, "GET", &format!("/api/v1/cycles/{missing_id}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");

    let (status, body) = send(&app, "GET", "/api/v1/cycles/not-a-uuid", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "bad_request");

    let (status, body) = send(
        &app,
        "POST",
        "/api/v1/cycles",
        Some(json!({
            "name": "  ",
            "start_on": "2026-01-01",
            "end_on": "2026-03-31"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "domain");

    let (status, body) = send(
        &app,
        "POST",
        "/api/v1/cycles",
        Some(json!({
            "name": "Inverted",
            "start_on": "2026-03-31",
            "end_on": "2026-01-01"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "domain");
}

#[tokio::test]
async fn dates_must_stay_inside_cycle() {
    let (app, _dir) = setup().await;
    let cycle = create_cycle(&app).await;
    let cycle_id = cycle["id"].as_str().unwrap();

    let (status, body) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/objectives"),
        Some(json!({
            "title": "Outside",
            "start_on": "2025-12-01",
            "end_on": "2026-03-31"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "domain");
}

#[tokio::test]
async fn completed_objective_cannot_gain_children() {
    let (app, _dir) = setup().await;
    let cycle = create_cycle(&app).await;
    let cycle_id = cycle["id"].as_str().unwrap();
    let (status, objective) = send(
        &app,
        "POST",
        &format!("/api/v1/cycles/{cycle_id}/objectives"),
        Some(json!({ "title": "Ship" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let objective_id = objective["id"].as_str().unwrap();
    send(
        &app,
        "POST",
        &format!("/api/v1/objectives/{objective_id}/activate"),
        None,
    )
    .await;
    send(
        &app,
        "POST",
        &format!("/api/v1/objectives/{objective_id}/complete"),
        None,
    )
    .await;

    let (status, _) = send(
        &app,
        "POST",
        &format!("/api/v1/objectives/{objective_id}/key-results"),
        Some(json!({
            "title": "Late KR",
            "start_value": 0.0,
            "target_value": 1.0
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn foreign_keys_are_enforced() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("forge.db");
    let pool = forge_server::db::connect(&path).await.unwrap();
    forge_server::db::migrate(&pool).await.unwrap();

    let enabled: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(enabled, 1);

    let err = sqlx::query(
        "INSERT INTO objectives (id, cycle_id, title, status, created_at, updated_at)
         VALUES ('o1', 'missing', 'orphan', 'draft', '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
    )
    .execute(&pool)
    .await
    .unwrap_err();
    assert!(err.to_string().to_lowercase().contains("foreign key"));
}

#[tokio::test]
async fn create_task_accepts_optional_scheduled_on() {
    let (app, _dir) = setup().await;
    let project_id = active_project_id(&app).await;

    let omitted = create_task(&app, &project_id, "Unscheduled omitted", None).await;
    assert!(omitted["scheduled_on"].is_null());
    assert_task_fields(&omitted);

    let (status, explicit_null) = send(
        &app,
        "POST",
        &format!("/api/v1/projects/{project_id}/tasks"),
        Some(json!({
            "title": "Unscheduled task",
            "description": null,
            "scheduled_on": null
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(explicit_null["scheduled_on"].is_null());
    assert!(explicit_null["description"].is_null());

    let (status, scheduled) = send(
        &app,
        "POST",
        &format!("/api/v1/projects/{project_id}/tasks"),
        Some(json!({
            "title": "Finish research",
            "description": "Complete the first draft",
            "scheduled_on": TODAY
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(scheduled["scheduled_on"], TODAY);
    assert_eq!(scheduled["description"], "Complete the first draft");
    assert_task_fields(&scheduled);

    for invalid in ["not-a-date", "2026-13-40", "2026-08-30T00:00:00Z"] {
        let (status, _) = send(
            &app,
            "POST",
            &format!("/api/v1/projects/{project_id}/tasks"),
            Some(json!({
                "title": "Bad date",
                "scheduled_on": invalid
            })),
        )
        .await;
        assert_client_validation(status);
    }
}

#[tokio::test]
async fn schedule_updates_or_clears_scheduled_on() {
    let (app, _dir) = setup().await;
    let project_id = active_project_id(&app).await;
    let task = create_task(&app, &project_id, "Plan work", None).await;
    let task_id = task["id"].as_str().unwrap();

    let (status, scheduled) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{task_id}/schedule"),
        Some(json!({ "scheduled_on": TODAY })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(scheduled["scheduled_on"], TODAY);
    assert_eq!(scheduled["status"], "todo");
    assert_task_fields(&scheduled);

    let (status, rescheduled) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{task_id}/schedule"),
        Some(json!({ "scheduled_on": TOMORROW })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(rescheduled["scheduled_on"], TOMORROW);

    let (status, unscheduled) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{task_id}/schedule"),
        Some(json!({ "scheduled_on": null })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(unscheduled["scheduled_on"].is_null());

    let started = create_task(&app, &project_id, "Started work", None).await;
    let started_id = started["id"].as_str().unwrap();
    let (status, _) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{started_id}/start"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, in_progress) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{started_id}/schedule"),
        Some(json!({ "scheduled_on": TODAY })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(in_progress["status"], "in_progress");
    assert_eq!(in_progress["scheduled_on"], TODAY);

    let done = create_task(&app, &project_id, "Finished", None).await;
    let done_id = done["id"].as_str().unwrap();
    send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{done_id}/start"),
        None,
    )
    .await;
    send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{done_id}/complete"),
        None,
    )
    .await;
    let (status, body) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{done_id}/schedule"),
        Some(json!({ "scheduled_on": TODAY })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "domain");

    let cancelled = create_task(&app, &project_id, "Dropped", None).await;
    let cancelled_id = cancelled["id"].as_str().unwrap();
    send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{cancelled_id}/cancel"),
        None,
    )
    .await;
    let (status, body) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{cancelled_id}/schedule"),
        Some(json!({ "scheduled_on": TODAY })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "domain");

    let (status, body) = send(
        &app,
        "POST",
        "/api/v1/tasks/01900000-0000-7000-8000-000000000000/schedule",
        Some(json!({ "scheduled_on": TODAY })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");
}

#[tokio::test]
async fn today_requires_an_explicit_date() {
    let (app, _dir) = setup().await;

    let (status, _) = send(&app, "GET", "/api/v1/today", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, _) = send(&app, "GET", "/api/v1/today?date=not-a-date", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, _) = send(&app, "GET", "/api/v1/today?date=2026-08-30T00:00:00Z", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn today_buckets_follow_the_requested_date() {
    let (app, _dir, pool) = setup_with_pool().await;
    let project_id = active_project_id(&app).await;

    let scheduled = create_task(&app, &project_id, "Scheduled today", Some(TODAY)).await;
    let overdue = create_task(&app, &project_id, "Scheduled yesterday", Some(YESTERDAY)).await;
    let unscheduled_in_progress =
        create_task(&app, &project_id, "Started without a date", None).await;
    let completed_today = create_task(&app, &project_id, "Done today", None).await;
    let completed_other = create_task(&app, &project_id, "Done tomorrow", None).await;
    let future = create_task(&app, &project_id, "Future", Some(TOMORROW)).await;
    let inbox = create_task(&app, &project_id, "Inbox todo", None).await;
    let cancelled = create_task(&app, &project_id, "Cancelled", Some(TODAY)).await;

    send(
        &app,
        "POST",
        &format!(
            "/api/v1/tasks/{}/start",
            unscheduled_in_progress["id"].as_str().unwrap()
        ),
        None,
    )
    .await;
    send(
        &app,
        "POST",
        &format!(
            "/api/v1/tasks/{}/start",
            completed_today["id"].as_str().unwrap()
        ),
        None,
    )
    .await;
    send(
        &app,
        "POST",
        &format!(
            "/api/v1/tasks/{}/complete",
            completed_today["id"].as_str().unwrap()
        ),
        None,
    )
    .await;
    send(
        &app,
        "POST",
        &format!(
            "/api/v1/tasks/{}/start",
            completed_other["id"].as_str().unwrap()
        ),
        None,
    )
    .await;
    send(
        &app,
        "POST",
        &format!(
            "/api/v1/tasks/{}/complete",
            completed_other["id"].as_str().unwrap()
        ),
        None,
    )
    .await;
    send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{}/cancel", cancelled["id"].as_str().unwrap()),
        None,
    )
    .await;

    set_completed_at(
        &pool,
        completed_today["id"].as_str().unwrap(),
        "2026-08-30T15:00:00Z",
    )
    .await;
    set_completed_at(
        &pool,
        completed_other["id"].as_str().unwrap(),
        "2026-08-31T15:00:00Z",
    )
    .await;

    let (status, today) = send(&app, "GET", &format!("/api/v1/today?date={TODAY}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(today["date"], TODAY);
    assert!(today.get("completed_date_basis").is_none());

    let scheduled_ids = task_ids(&today["scheduled"]);
    let overdue_ids = task_ids(&today["overdue"]);
    let in_progress_ids = task_ids(&today["unscheduled_in_progress"]);
    let completed_ids = task_ids(&today["completed"]);

    assert_eq!(scheduled_ids, vec![scheduled["id"].as_str().unwrap()]);
    assert_eq!(overdue_ids, vec![overdue["id"].as_str().unwrap()]);
    assert_eq!(
        in_progress_ids,
        vec![unscheduled_in_progress["id"].as_str().unwrap()]
    );
    assert_eq!(completed_ids, vec![completed_today["id"].as_str().unwrap()]);

    for task in today["scheduled"]
        .as_array()
        .unwrap()
        .iter()
        .chain(today["overdue"].as_array().unwrap())
        .chain(today["unscheduled_in_progress"].as_array().unwrap())
        .chain(today["completed"].as_array().unwrap())
    {
        assert_task_fields(task);
    }

    let excluded = [
        future["id"].as_str().unwrap(),
        inbox["id"].as_str().unwrap(),
        cancelled["id"].as_str().unwrap(),
        completed_other["id"].as_str().unwrap(),
    ];
    let mut all = scheduled_ids.clone();
    all.extend(overdue_ids);
    all.extend(in_progress_ids);
    all.extend(completed_ids.clone());
    let unique = all
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(all.len(), unique.len());
    for id in excluded {
        assert!(!all.iter().any(|item| item == id), "{id} leaked into today");
    }

    let (status, tomorrow) =
        send(&app, "GET", &format!("/api/v1/today?date={TOMORROW}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(tomorrow["date"], TOMORROW);
    assert!(task_ids(&tomorrow["scheduled"]).contains(&future["id"].as_str().unwrap().to_string()));
    assert!(
        task_ids(&tomorrow["overdue"]).contains(&scheduled["id"].as_str().unwrap().to_string())
    );
    assert!(
        task_ids(&tomorrow["completed"])
            .contains(&completed_other["id"].as_str().unwrap().to_string())
    );
    assert!(
        !task_ids(&tomorrow["completed"])
            .contains(&completed_today["id"].as_str().unwrap().to_string())
    );
}

#[tokio::test]
async fn today_is_independent_of_daily_execution() {
    let (app, _dir) = setup().await;
    let project_id = active_project_id(&app).await;

    let inbox = create_task(&app, &project_id, "Legacy inbox", None).await;
    let inbox_id = inbox["id"].as_str().unwrap();
    let (status, execution) = send(
        &app,
        "POST",
        &format!("/api/v1/tasks/{inbox_id}/daily-executions"),
        Some(json!({
            "execution_date": TODAY,
            "notes": "legacy",
            "status": "planned"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(execution["execution_date"], TODAY);

    let scheduled = create_task(&app, &project_id, "No daily execution", Some(TODAY)).await;
    let scheduled_id = scheduled["id"].as_str().unwrap();

    let (status, today) = send(&app, "GET", &format!("/api/v1/today?date={TODAY}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(task_ids(&today["scheduled"]), vec![scheduled_id]);
    assert!(!task_ids(&today["scheduled"]).contains(&inbox_id.to_string()));
    assert!(today["overdue"].as_array().unwrap().is_empty());
    assert!(
        today["unscheduled_in_progress"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert!(today["completed"].as_array().unwrap().is_empty());

    let (status, by_date) = send(
        &app,
        "GET",
        &format!("/api/v1/daily-executions?date={TODAY}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(by_date.as_array().unwrap().len(), 1);
    assert_eq!(by_date[0]["task_id"], inbox_id);
}

#[tokio::test]
async fn today_completed_uses_fixed_utc_midnight_bounds() {
    let (app, _dir, pool) = setup_with_pool().await;
    let project_id = active_project_id(&app).await;

    let before_midnight = create_task(&app, &project_id, "Before midnight", None).await;
    let after_midnight = create_task(&app, &project_id, "After midnight", None).await;
    let before_id = before_midnight["id"].as_str().unwrap().to_string();
    let after_id = after_midnight["id"].as_str().unwrap().to_string();

    for id in [&before_id, &after_id] {
        send(&app, "POST", &format!("/api/v1/tasks/{id}/start"), None).await;
        send(&app, "POST", &format!("/api/v1/tasks/{id}/complete"), None).await;
    }

    set_completed_at(&pool, &before_id, "2026-08-30T23:59:59Z").await;
    set_completed_at(&pool, &after_id, "2026-08-31T00:00:00Z").await;

    let (status, today) = send(&app, "GET", &format!("/api/v1/today?date={TODAY}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(task_ids(&today["completed"]), vec![before_id.clone()]);
    assert!(!task_ids(&today["completed"]).contains(&after_id));

    let (status, tomorrow) =
        send(&app, "GET", &format!("/api/v1/today?date={TOMORROW}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(task_ids(&tomorrow["completed"]), vec![after_id]);
    assert!(!task_ids(&tomorrow["completed"]).contains(&before_id));
}
