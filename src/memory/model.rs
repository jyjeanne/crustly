//! Long-term memory types, normalisation, hashing and validation
//! (docs/improve-memory.md §6.1, §6.3 and §8 steps 1-2).
//!
//! The string forms of the enums are stored in the database, and must match
//! the `CHECK` lists in the long-term memory migration. A test below reads
//! the migration to keep them in step.

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Longest memory, in characters, after normalisation.
pub const MAX_CONTENT_CHARS: usize = 500;

/// Identifier of a project: hex SHA-256 of its canonical root path (§7).
pub type ProjectId = String;

/// What a memory is about. Stored as its `as_str` form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryCategory {
    Preference,
    Convention,
    Decision,
    Fact,
    Lesson,
    Episode,
}

impl MemoryCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preference => "preference",
            Self::Convention => "convention",
            Self::Decision => "decision",
            Self::Fact => "fact",
            Self::Lesson => "lesson",
            Self::Episode => "episode",
        }
    }
}

impl FromStr for MemoryCategory {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "preference" => Self::Preference,
            "convention" => Self::Convention,
            "decision" => Self::Decision,
            "fact" => Self::Fact,
            "lesson" => Self::Lesson,
            "episode" => Self::Episode,
            _ => return Err(ParseEnumError::new("category", s)),
        })
    }
}

/// Where a memory came from. Stored as its `as_str` form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemorySource {
    User,
    Tool,
    Extracted,
    Imported,
    Consolidated,
}

impl MemorySource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Tool => "tool",
            Self::Extracted => "extracted",
            Self::Imported => "imported",
            Self::Consolidated => "consolidated",
        }
    }
}

impl FromStr for MemorySource {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "user" => Self::User,
            "tool" => Self::Tool,
            "extracted" => Self::Extracted,
            "imported" => Self::Imported,
            "consolidated" => Self::Consolidated,
            _ => return Err(ParseEnumError::new("source", s)),
        })
    }
}

/// Lifecycle state of a memory. Stored as its `as_str` form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryStatus {
    Active,
    Pending,
    Superseded,
    Archived,
}

impl MemoryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }
}

impl FromStr for MemoryStatus {
    type Err = ParseEnumError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "active" => Self::Active,
            "pending" => Self::Pending,
            "superseded" => Self::Superseded,
            "archived" => Self::Archived,
            _ => return Err(ParseEnumError::new("status", s)),
        })
    }
}

/// Whether a memory belongs to the user everywhere or to one project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryScope {
    Global,
    Project(ProjectId),
}

/// Error for an unknown value read from the database or a user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnumError {
    kind: &'static str,
    value: String,
}

impl ParseEnumError {
    fn new(kind: &'static str, value: &str) -> Self {
        Self {
            kind,
            value: value.to_string(),
        }
    }
}

impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown memory {}: {:?}", self.kind, self.value)
    }
}

impl std::error::Error for ParseEnumError {}

/// One stored memory.
#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    pub id: Uuid,
    pub scope: MemoryScope,
    pub category: MemoryCategory,
    pub content: String,
    pub source: MemorySource,
    /// 1 to 5.
    pub importance: u8,
    pub status: MemoryStatus,
    pub superseded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_recalled_at: Option<DateTime<Utc>>,
    pub recall_count: u32,
    pub expires_at: Option<DateTime<Utc>>,
}

/// A memory about to be saved.
#[derive(Debug, Clone, PartialEq)]
pub struct NewMemory {
    pub content: String,
    pub scope: MemoryScope,
    pub category: MemoryCategory,
    pub source: MemorySource,
    pub importance: u8,
    /// Memory this one corrects or updates.
    pub replaces: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Result of saving a memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveOutcome {
    Created(Uuid),
    /// An identical active memory already exists; its id is returned.
    Duplicate(Uuid),
    Rejected(RejectReason),
}

/// Why a memory was not saved. Each message is written to be shown to the
/// model or the user as-is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    Empty,
    TooLong,
    MultiParagraph,
    CodeBlock,
    /// A secret pattern matched. Only the rule's name is kept, never the text.
    Secret {
        rule: &'static str,
    },
    ScopeMismatch,
    /// The memory was saved by the user; only the user may change it.
    SavedByUser,
    /// The same text is already remembered in this scope.
    Duplicate,
    NotFound,
    ReadOnly,
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "the memory is empty"),
            Self::TooLong => write!(
                f,
                "too long: save one standalone fact of at most {MAX_CONTENT_CHARS} characters"
            ),
            Self::MultiParagraph => write!(f, "save one paragraph, not several"),
            Self::CodeBlock => write!(f, "do not save code blocks; save a fact about the code"),
            Self::Secret { rule } => write!(
                f,
                "not saved: the text looks like a secret ({rule}). Never store credentials"
            ),
            Self::ScopeMismatch => write!(f, "the replaced memory is in a different scope"),
            Self::Duplicate => write!(f, "that is already remembered"),
            Self::SavedByUser => write!(
                f,
                "that memory was saved by the user; only the user can change it"
            ),
            Self::NotFound => write!(f, "no memory with that id"),
            Self::ReadOnly => write!(f, "memory cannot change in read-only (plan) mode"),
        }
    }
}

/// Collapse runs of whitespace and trim the ends (§8 step 1).
pub fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Hash for exact-duplicate detection (§8 step 1): lower-case, one trailing
/// full stop removed, then SHA-256 as lowercase hex. Stable across releases.
pub fn content_hash(text: &str) -> String {
    let normalized = normalize(text).to_lowercase();
    let key = normalized.strip_suffix('.').unwrap_or(&normalized);
    Sha256::digest(key.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Checks from §8 step 2. Input is checked after normalisation, except that
/// blank lines are looked for in the trimmed text.
pub fn validate(text: &str) -> Result<(), RejectReason> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(RejectReason::Empty);
    }
    if trimmed.contains("```") {
        return Err(RejectReason::CodeBlock);
    }
    if trimmed.lines().any(|line| line.trim().is_empty()) {
        return Err(RejectReason::MultiParagraph);
    }
    if normalize(trimmed).chars().count() > MAX_CONTENT_CHARS {
        return Err(RejectReason::TooLong);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_CATEGORIES: [MemoryCategory; 6] = [
        MemoryCategory::Preference,
        MemoryCategory::Convention,
        MemoryCategory::Decision,
        MemoryCategory::Fact,
        MemoryCategory::Lesson,
        MemoryCategory::Episode,
    ];

    #[test]
    fn category_strings_round_trip() {
        for category in ALL_CATEGORIES {
            assert_eq!(
                category.as_str().parse::<MemoryCategory>().unwrap(),
                category
            );
        }
        assert!("coding_standard".parse::<MemoryCategory>().is_err());
    }

    #[test]
    fn source_and_status_strings_round_trip() {
        for source in [
            MemorySource::User,
            MemorySource::Tool,
            MemorySource::Extracted,
            MemorySource::Imported,
            MemorySource::Consolidated,
        ] {
            assert_eq!(source.as_str().parse::<MemorySource>().unwrap(), source);
        }
        for status in [
            MemoryStatus::Active,
            MemoryStatus::Pending,
            MemoryStatus::Superseded,
            MemoryStatus::Archived,
        ] {
            assert_eq!(status.as_str().parse::<MemoryStatus>().unwrap(), status);
        }
        assert!("deleted".parse::<MemoryStatus>().is_err());
    }

    #[test]
    fn strings_match_the_migration_check_lists() {
        // The migration is the source of truth for what the database accepts.
        let sql = include_str!("../../migrations/20261123000001_long_term_memory.sql");
        for category in ALL_CATEGORIES {
            assert!(
                sql.contains(&format!("'{}'", category.as_str())),
                "category {} is missing from the migration",
                category.as_str()
            );
        }
        for word in ["user", "tool", "extracted", "imported", "consolidated"] {
            assert!(sql.contains(&format!("'{word}'")), "source {word} missing");
        }
        for word in ["active", "pending", "superseded", "archived"] {
            assert!(sql.contains(&format!("'{word}'")), "status {word} missing");
        }
    }

    #[test]
    fn content_hash_matches_a_fixed_digest() {
        // Computed independently with: printf '%s' 'the user prefers tokio' | sha256sum
        assert_eq!(
            content_hash("The   user prefers TOKIO."),
            "0ed65c6ebbb98c26800e1da8b92f2dcb67bafc56ffa9131d29679b2ce3678a8a"
        );
    }

    #[test]
    fn content_hash_ignores_case_spacing_and_one_final_period() {
        assert_eq!(content_hash("Use tokio."), content_hash("use   tokio"));
        assert_ne!(content_hash("Use tokio"), content_hash("Use rayon"));
    }

    #[test]
    fn normalize_collapses_whitespace() {
        assert_eq!(normalize("  a \t b\n c  "), "a b c");
    }

    #[test]
    fn validate_accepts_a_single_fact() {
        assert_eq!(validate("The user prefers tabs."), Ok(()));
    }

    #[test]
    fn validate_rejects_empty_text() {
        assert_eq!(validate("   \n "), Err(RejectReason::Empty));
    }

    #[test]
    fn validate_rejects_long_text() {
        let long = "a".repeat(MAX_CONTENT_CHARS + 1);
        assert_eq!(validate(&long), Err(RejectReason::TooLong));
        assert_eq!(validate(&"a".repeat(MAX_CONTENT_CHARS)), Ok(()));
    }

    #[test]
    fn validate_rejects_several_paragraphs() {
        assert_eq!(
            validate("First.\n\nSecond."),
            Err(RejectReason::MultiParagraph)
        );
        assert_eq!(
            validate("First.\n   \nSecond."),
            Err(RejectReason::MultiParagraph)
        );
    }

    #[test]
    fn validate_rejects_code_blocks() {
        assert_eq!(
            validate("Run ```cargo test```"),
            Err(RejectReason::CodeBlock)
        );
    }

    #[test]
    fn reject_messages_are_actionable() {
        assert!(RejectReason::TooLong
            .to_string()
            .contains("at most 500 characters"));
        assert!(RejectReason::Secret { rule: "api key" }
            .to_string()
            .contains("api key"));
    }
}
