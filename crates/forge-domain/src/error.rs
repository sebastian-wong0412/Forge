use thiserror::Error;

/// Recoverable violations of domain invariants.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("title must not be empty")]
    EmptyTitle,
    #[error("end date must be on or after start date")]
    InvalidDateRange,
    #[error("date range must fall within the parent cycle")]
    DateOutsideCycle,
    #[error("cannot transition from {from} to {to}")]
    InvalidStatusTransition {
        from: &'static str,
        to: &'static str,
    },
    #[error("review content must not be empty")]
    EmptyReviewContent,
    #[error("unknown status `{0}`")]
    UnknownStatus(String),
}
