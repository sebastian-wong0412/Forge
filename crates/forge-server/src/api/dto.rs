use forge_application::{KeyResultSnapshot, TodayResult};
use forge_domain::{CheckIn, Cycle, DailyExecution, Objective, Project, Review, Task};
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

time::serde::format_description!(iso_date, Date, "[year]-[month]-[day]");

#[derive(Debug, Deserialize)]
pub struct CreateCycleRequest {
    pub name: String,
    #[serde(with = "iso_date")]
    pub start_on: Date,
    #[serde(with = "iso_date")]
    pub end_on: Date,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCycleRequest {
    pub name: String,
    #[serde(with = "iso_date")]
    pub start_on: Date,
    #[serde(with = "iso_date")]
    pub end_on: Date,
}

#[derive(Debug, Serialize)]
pub struct CycleResponse {
    pub id: String,
    pub name: String,
    #[serde(with = "iso_date")]
    pub start_on: Date,
    #[serde(with = "iso_date")]
    pub end_on: Date,
    pub status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&Cycle> for CycleResponse {
    fn from(value: &Cycle) -> Self {
        Self {
            id: value.id().to_string(),
            name: value.name().as_str().to_string(),
            start_on: value.start_on(),
            end_on: value.end_on(),
            status: value.status().as_str().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateObjectiveRequest {
    pub title: String,
    pub description: Option<String>,
    #[serde(default, with = "iso_date::option")]
    pub start_on: Option<Date>,
    #[serde(default, with = "iso_date::option")]
    pub end_on: Option<Date>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateObjectiveRequest {
    pub title: String,
    pub description: Option<String>,
    #[serde(default, with = "iso_date::option")]
    pub start_on: Option<Date>,
    #[serde(default, with = "iso_date::option")]
    pub end_on: Option<Date>,
}

#[derive(Debug, Serialize)]
pub struct ObjectiveResponse {
    pub id: String,
    pub cycle_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(with = "iso_date::option")]
    pub start_on: Option<Date>,
    #[serde(with = "iso_date::option")]
    pub end_on: Option<Date>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&Objective> for ObjectiveResponse {
    fn from(value: &Objective) -> Self {
        Self {
            id: value.id().to_string(),
            cycle_id: value.cycle_id().to_string(),
            title: value.title().as_str().to_string(),
            description: value.description().map(str::to_string),
            status: value.status().as_str().to_string(),
            start_on: value.start_on(),
            end_on: value.end_on(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateKeyResultRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_value: f64,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKeyResultRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_value: f64,
    pub target_value: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct KeyResultResponse {
    pub id: String,
    pub objective_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub start_value: f64,
    pub current_value: f64,
    pub target_value: Option<f64>,
    pub progress: Option<f64>,
    pub unit: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&KeyResultSnapshot> for KeyResultResponse {
    fn from(value: &KeyResultSnapshot) -> Self {
        let key_result = &value.key_result;
        Self {
            id: key_result.id().to_string(),
            objective_id: key_result.objective_id().to_string(),
            title: key_result.title().as_str().to_string(),
            description: key_result.description().map(str::to_string),
            status: key_result.status().as_str().to_string(),
            start_value: key_result.start_value(),
            current_value: value.current_value,
            target_value: key_result.target_value(),
            progress: value.progress,
            unit: key_result.unit().map(str::to_string),
            created_at: key_result.created_at(),
            updated_at: key_result.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckInRequest {
    pub value: f64,
    pub note: Option<String>,
    #[serde(with = "iso_date")]
    pub checked_on: Date,
}

#[derive(Debug, Serialize)]
pub struct CheckInResponse {
    pub id: String,
    pub key_result_id: String,
    pub value: f64,
    pub note: Option<String>,
    #[serde(with = "iso_date")]
    pub checked_on: Date,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&CheckIn> for CheckInResponse {
    fn from(value: &CheckIn) -> Self {
        Self {
            id: value.id().to_string(),
            key_result_id: value.key_result_id().to_string(),
            value: value.value(),
            note: value.note().map(str::to_string),
            checked_on: value.checked_on(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub objective_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&Project> for ProjectResponse {
    fn from(value: &Project) -> Self {
        Self {
            id: value.id().to_string(),
            objective_id: value.objective_id().to_string(),
            title: value.title().as_str().to_string(),
            description: value.description().map(str::to_string),
            status: value.status().as_str().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    #[serde(default, with = "iso_date::option")]
    pub scheduled_on: Option<Date>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleTaskRequest {
    #[serde(default, with = "iso_date::option")]
    pub scheduled_on: Option<Date>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(with = "iso_date::option")]
    pub scheduled_on: Option<Date>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub completed_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&Task> for TaskResponse {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id().to_string(),
            project_id: value.project_id().to_string(),
            title: value.title().as_str().to_string(),
            description: value.description().map(str::to_string),
            status: value.status().as_str().to_string(),
            scheduled_on: value.scheduled_on(),
            completed_at: value.completed_at(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateDailyExecutionRequest {
    #[serde(with = "iso_date")]
    pub execution_date: Date,
    pub notes: Option<String>,
    #[serde(default = "default_planned")]
    pub status: String,
}

fn default_planned() -> String {
    "planned".to_string()
}

#[derive(Debug, Deserialize)]
pub struct UpdateDailyExecutionRequest {
    pub notes: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    #[serde(with = "iso_date")]
    pub date: Date,
}

#[derive(Debug, Serialize)]
pub struct DailyExecutionResponse {
    pub id: String,
    pub task_id: String,
    #[serde(with = "iso_date")]
    pub execution_date: Date,
    pub notes: Option<String>,
    pub status: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&DailyExecution> for DailyExecutionResponse {
    fn from(value: &DailyExecution) -> Self {
        Self {
            id: value.id().to_string(),
            task_id: value.task_id().to_string(),
            execution_date: value.execution_date(),
            notes: value.notes().map(str::to_string),
            status: value.status().as_str().to_string(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub content: String,
    #[serde(default, with = "iso_date::option")]
    pub period_start: Option<Date>,
    #[serde(default, with = "iso_date::option")]
    pub period_end: Option<Date>,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: String,
    pub cycle_id: String,
    pub content: String,
    #[serde(with = "iso_date::option")]
    pub period_start: Option<Date>,
    #[serde(with = "iso_date::option")]
    pub period_end: Option<Date>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<&Review> for ReviewResponse {
    fn from(value: &Review) -> Self {
        Self {
            id: value.id().to_string(),
            cycle_id: value.cycle_id().to_string(),
            content: value.content().to_string(),
            period_start: value.period_start(),
            period_end: value.period_end(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TodayResponse {
    #[serde(with = "iso_date")]
    pub date: Date,
    pub scheduled: Vec<TaskResponse>,
    pub overdue: Vec<TaskResponse>,
    pub unscheduled_in_progress: Vec<TaskResponse>,
    pub completed: Vec<TaskResponse>,
}

impl From<&TodayResult> for TodayResponse {
    fn from(value: &TodayResult) -> Self {
        Self {
            date: value.date,
            scheduled: value.scheduled.iter().map(TaskResponse::from).collect(),
            overdue: value.overdue.iter().map(TaskResponse::from).collect(),
            unscheduled_in_progress: value
                .unscheduled_in_progress
                .iter()
                .map(TaskResponse::from)
                .collect(),
            completed: value.completed.iter().map(TaskResponse::from).collect(),
        }
    }
}
