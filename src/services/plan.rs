//! Plan Service
//!
//! Business logic for plan management operations.

use crate::db::repository::PlanRepository;
use crate::services::ServiceContext;
use crate::tui::plan::PlanDocument;
use anyhow::Result;
use uuid::Uuid;

/// Service for plan operations
#[derive(Clone)]
pub struct PlanService {
    repo: PlanRepository,
}

impl PlanService {
    /// Create a new plan service
    pub fn new(context: ServiceContext) -> Self {
        Self {
            repo: PlanRepository::new(context.pool()),
        }
    }

    /// Find plan by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PlanDocument>> {
        self.repo.find_by_id(id).await
    }

    /// Find all plans for a session
    pub async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<PlanDocument>> {
        self.repo.find_by_session_id(session_id).await
    }

    /// Get the most recent plan for a session
    pub async fn get_most_recent_plan(&self, session_id: Uuid) -> Result<Option<PlanDocument>> {
        let plans = self.find_by_session_id(session_id).await?;
        // Plans are already sorted by updated_at DESC in the repository
        Ok(plans.into_iter().next())
    }

    /// Create a new plan
    pub async fn create(&self, plan: &PlanDocument) -> Result<()> {
        self.repo.create(plan).await
    }

    /// Update an existing plan
    pub async fn update(&self, plan: &PlanDocument) -> Result<()> {
        self.repo.update(plan).await
    }

    /// Delete a plan
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await
    }

    /// Save plan to JSON file as backup
    /// This is a fallback/export mechanism
    pub async fn export_to_json(&self, plan: &PlanDocument, file_path: &std::path::Path) -> Result<()> {
        let json = serde_json::to_string_pretty(plan)?;

        // Atomic write: write to temp file, then rename
        let temp_file = file_path.with_extension("tmp");
        tokio::fs::write(&temp_file, &json).await?;
        tokio::fs::rename(&temp_file, file_path).await?;

        Ok(())
    }

    /// Import plan from JSON file
    /// Used for migrating existing JSON plans to database
    pub async fn import_from_json(&self, file_path: &std::path::Path) -> Result<PlanDocument> {
        let content = tokio::fs::read_to_string(file_path).await?;
        let plan: PlanDocument = serde_json::from_str(&content)?;
        Ok(plan)
    }
}
