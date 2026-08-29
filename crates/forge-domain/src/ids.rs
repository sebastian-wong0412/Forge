use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

macro_rules! entity_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(Uuid);

        impl $name {
            /// Allocate a new UUID v7 identifier.
            ///
            /// `Default` is intentionally omitted: a default ID would still be unique
            /// and would hide that a fresh identity is being created.
            #[must_use]
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            #[must_use]
            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            #[must_use]
            pub fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(Uuid::parse_str(s)?))
            }
        }
    };
}

entity_id!(CycleId);
entity_id!(ObjectiveId);
entity_id!(KeyResultId);
entity_id!(CheckInId);
entity_id!(ProjectId);
entity_id!(TaskId);
entity_id!(DailyExecutionId);
entity_id!(ReviewId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_round_trip() {
        let id = CycleId::new();
        let parsed: CycleId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn rejects_invalid_uuid() {
        let err = "not-a-uuid".parse::<CycleId>().unwrap_err();
        assert!(!err.to_string().is_empty());
    }
}
