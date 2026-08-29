use time::Date;

use crate::DomainError;

pub(crate) fn empty_to_none(value: Option<String>) -> Option<String> {
    value.and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub(crate) fn validate_date_range(
    start: Option<Date>,
    end: Option<Date>,
) -> Result<(), DomainError> {
    match (start, end) {
        (Some(start), Some(end)) if end < start => Err(DomainError::InvalidDateRange),
        _ => Ok(()),
    }
}

pub(crate) fn require_date_range(start: Date, end: Date) -> Result<(), DomainError> {
    if end < start {
        Err(DomainError::InvalidDateRange)
    } else {
        Ok(())
    }
}

pub fn dates_within_cycle(
    cycle_start: Date,
    cycle_end: Date,
    start_on: Option<Date>,
    end_on: Option<Date>,
) -> Result<(), DomainError> {
    validate_date_range(start_on, end_on)?;
    for date in [start_on, end_on].into_iter().flatten() {
        if date < cycle_start || date > cycle_end {
            return Err(DomainError::DateOutsideCycle);
        }
    }
    Ok(())
}
