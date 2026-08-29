#![allow(clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tempfile::TempDir;
use tower::ServiceExt;

async fn setup() -> (axum::Router, TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("forge.db");
    let pool = forge_server::db::connect(&path).await.unwrap();
    forge_server::db::migrate(&pool).await.unwrap();
    (forge_server::api::router(pool), dir)
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
