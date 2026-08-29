use crate::DomainError;

/// Non-empty, trimmed display name for a domain entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(String);

impl Title {
    pub fn parse(raw: impl AsRef<str>) -> Result<Self, DomainError> {
        let trimmed = raw.as_ref().trim();
        if trimmed.is_empty() {
            return Err(DomainError::EmptyTitle);
        }
        Ok(Self(trimmed.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_accepts_non_empty() {
        let title = Title::parse("  Focus  ").unwrap();
        assert_eq!(title.as_str(), "Focus");
    }

    #[test]
    fn rejects_empty_and_whitespace() {
        assert_eq!(Title::parse("").unwrap_err(), DomainError::EmptyTitle);
        assert_eq!(Title::parse("   ").unwrap_err(), DomainError::EmptyTitle);
    }
}
