use axum::Router;
use axum::routing::{get, post};
use forge_application::{
    CheckInService, CycleService, DailyExecutionService, KeyResultService, ObjectiveService,
    ProjectService, ReviewService, TaskService,
};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::sqlite::{
    SqliteCheckInRepository, SqliteCycleRepository, SqliteDailyExecutionRepository,
    SqliteKeyResultRepository, SqliteObjectiveRepository, SqliteProjectRepository,
    SqliteReviewRepository, SqliteTaskRepository,
};

mod check_ins;
mod cycles;
mod daily_executions;
mod dto;
mod error;
mod health;
mod key_results;
mod objectives;
mod projects;
mod reviews;
mod tasks;
mod today;

pub use error::ApiError;

#[derive(Clone)]
pub struct AppState {
    cycles: CycleService<SqliteCycleRepository>,
    objectives: ObjectiveService<SqliteCycleRepository, SqliteObjectiveRepository>,
    key_results: KeyResultService<
        SqliteCycleRepository,
        SqliteObjectiveRepository,
        SqliteKeyResultRepository,
        SqliteCheckInRepository,
    >,
    check_ins: CheckInService<
        SqliteCycleRepository,
        SqliteObjectiveRepository,
        SqliteKeyResultRepository,
        SqliteCheckInRepository,
    >,
    projects:
        ProjectService<SqliteCycleRepository, SqliteObjectiveRepository, SqliteProjectRepository>,
    tasks: TaskService<
        SqliteCycleRepository,
        SqliteObjectiveRepository,
        SqliteProjectRepository,
        SqliteTaskRepository,
    >,
    daily_executions: DailyExecutionService<SqliteTaskRepository, SqliteDailyExecutionRepository>,
    reviews: ReviewService<SqliteCycleRepository, SqliteReviewRepository>,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        let cycle_repo = SqliteCycleRepository::new(pool.clone());
        let objective_repo = SqliteObjectiveRepository::new(pool.clone());
        let key_result_repo = SqliteKeyResultRepository::new(pool.clone());
        let check_in_repo = SqliteCheckInRepository::new(pool.clone());
        let project_repo = SqliteProjectRepository::new(pool.clone());
        let task_repo = SqliteTaskRepository::new(pool.clone());
        let execution_repo = SqliteDailyExecutionRepository::new(pool.clone());
        let review_repo = SqliteReviewRepository::new(pool);

        Self {
            cycles: CycleService::new(cycle_repo.clone()),
            objectives: ObjectiveService::new(cycle_repo.clone(), objective_repo.clone()),
            key_results: KeyResultService::new(
                cycle_repo.clone(),
                objective_repo.clone(),
                key_result_repo.clone(),
                check_in_repo.clone(),
            ),
            check_ins: CheckInService::new(
                cycle_repo.clone(),
                objective_repo.clone(),
                key_result_repo,
                check_in_repo,
            ),
            projects: ProjectService::new(
                cycle_repo.clone(),
                objective_repo.clone(),
                project_repo.clone(),
            ),
            tasks: TaskService::new(
                cycle_repo.clone(),
                objective_repo,
                project_repo,
                task_repo.clone(),
            ),
            daily_executions: DailyExecutionService::new(task_repo, execution_repo),
            reviews: ReviewService::new(cycle_repo, review_repo),
        }
    }
}

pub fn router(pool: SqlitePool) -> Router {
    let state = AppState::new(pool);
    Router::new()
        .route("/health", get(health::health))
        .nest("/api/v1", v1())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn v1() -> Router<AppState> {
    Router::new()
        .route("/cycles", get(cycles::list).post(cycles::create))
        .route("/cycles/{id}", get(cycles::get).patch(cycles::update))
        .route("/cycles/{id}/activate", post(cycles::activate))
        .route("/cycles/{id}/close", post(cycles::close))
        .route("/cycles/{id}/archive", post(cycles::archive))
        .route(
            "/cycles/{cycle_id}/objectives",
            get(objectives::list).post(objectives::create),
        )
        .route(
            "/objectives/{id}",
            get(objectives::get).patch(objectives::update),
        )
        .route("/objectives/{id}/activate", post(objectives::activate))
        .route("/objectives/{id}/complete", post(objectives::complete))
        .route("/objectives/{id}/archive", post(objectives::archive))
        .route(
            "/objectives/{objective_id}/key-results",
            get(key_results::list).post(key_results::create),
        )
        .route(
            "/key-results/{id}",
            get(key_results::get).patch(key_results::update),
        )
        .route("/key-results/{id}/activate", post(key_results::activate))
        .route("/key-results/{id}/complete", post(key_results::complete))
        .route("/key-results/{id}/archive", post(key_results::archive))
        .route(
            "/key-results/{id}/check-ins",
            get(check_ins::list).post(check_ins::create),
        )
        .route(
            "/objectives/{objective_id}/projects",
            get(projects::list).post(projects::create),
        )
        .route("/projects/{id}", get(projects::get).patch(projects::update))
        .route("/projects/{id}/activate", post(projects::activate))
        .route("/projects/{id}/complete", post(projects::complete))
        .route("/projects/{id}/archive", post(projects::archive))
        .route(
            "/projects/{project_id}/tasks",
            get(tasks::list).post(tasks::create),
        )
        .route("/tasks/{id}", get(tasks::get).patch(tasks::update))
        .route("/tasks/{id}/start", post(tasks::start))
        .route("/tasks/{id}/complete", post(tasks::complete))
        .route("/tasks/{id}/cancel", post(tasks::cancel))
        .route("/tasks/{id}/schedule", post(tasks::schedule))
        .route("/today", get(today::get))
        .route(
            "/tasks/{task_id}/daily-executions",
            get(daily_executions::list_by_task).post(daily_executions::create),
        )
        .route("/daily-executions", get(daily_executions::list_by_date))
        .route(
            "/daily-executions/{id}",
            get(daily_executions::get).patch(daily_executions::update),
        )
        .route(
            "/cycles/{cycle_id}/reviews",
            get(reviews::list).post(reviews::create),
        )
}

fn parse_id<T: std::str::FromStr>(raw: &str, label: &str) -> Result<T, ApiError> {
    raw.parse()
        .map_err(|_| ApiError::bad_request(format!("invalid {label} id")))
}
