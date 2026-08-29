use std::str::FromStr;

use forge_application::AppError;
use forge_domain::Title;
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{Date, OffsetDateTime};

const DATE: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day]");

pub fn title(raw: &str) -> Result<Title, AppError> {
    Title::parse(raw).map_err(|err| AppError::persistence(err.to_string()))
}

pub fn parse<T: FromStr>(raw: &str) -> Result<T, AppError>
where
    T::Err: std::fmt::Display,
{
    raw.parse::<T>()
        .map_err(|err| AppError::persistence(err.to_string()))
}

pub fn rfc3339(raw: &str) -> Result<OffsetDateTime, AppError> {
    OffsetDateTime::parse(raw, &Rfc3339).map_err(|err| AppError::persistence(err.to_string()))
}

pub fn format_rfc3339(dt: OffsetDateTime) -> Result<String, AppError> {
    dt.format(&Rfc3339)
        .map_err(|err| AppError::persistence(err.to_string()))
}

pub fn date(raw: &str) -> Result<Date, AppError> {
    Date::parse(raw, DATE).map_err(|err| AppError::persistence(err.to_string()))
}

pub fn optional_date(raw: &Option<String>) -> Result<Option<Date>, AppError> {
    raw.as_deref().map(date).transpose()
}

pub fn optional_rfc3339(raw: &Option<String>) -> Result<Option<OffsetDateTime>, AppError> {
    raw.as_deref().map(rfc3339).transpose()
}

pub fn map_sqlx(err: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(db) = &err {
        if db.is_unique_violation() {
            return AppError::conflict(db.message().to_string());
        }
        if db.is_foreign_key_violation() {
            return AppError::conflict("referenced parent does not exist");
        }
    }
    AppError::persistence(err.to_string())
}
