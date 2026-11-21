//! Agent memory: episodic cross-session summaries.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A compressed summary of one past session, injected at session start.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub id: Uuid,
    pub session_id: Uuid,
    pub summary_text: String,
    pub token_count: i32,
    pub files_touched: Vec<String>,
    pub decisions: Vec<String>,
    pub created_at: DateTime<Utc>,
}
