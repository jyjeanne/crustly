//! Plan Repository
//!
//! Database operations for plans and plan tasks.

use crate::db::models::{Plan, PlanTask};
use crate::tui::plan::{PlanDocument, PlanStatus, TaskStatus, TaskType};
use anyhow::{Context, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

/// Repository for plan operations
#[derive(Clone)]
pub struct PlanRepository {
    pool: SqlitePool,
}

impl PlanRepository {
    /// Create a new plan repository
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Find plan by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PlanDocument>> {
        let plan = sqlx::query_as::<_, Plan>("SELECT * FROM plans WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .context("Failed to find plan")?;

        let Some(plan) = plan else {
            return Ok(None);
        };

        // Fetch associated tasks
        let tasks = self.find_tasks_by_plan_id(id).await?;

        // Convert database models to domain models
        Ok(Some(self.plan_from_db(plan, tasks)?))
    }

    /// Find all plans for a session
    pub async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<PlanDocument>> {
        let plans = sqlx::query_as::<_, Plan>(
            "SELECT * FROM plans WHERE session_id = ? ORDER BY updated_at DESC",
        )
        .bind(session_id.to_string())
        .fetch_all(&self.pool)
        .await
        .context("Failed to find plans by session")?;

        let mut result = Vec::new();
        for plan in plans {
            let tasks = self.find_tasks_by_plan_id(plan.id).await?;
            result.push(self.plan_from_db(plan, tasks)?);
        }

        Ok(result)
    }

    /// Find tasks for a plan
    async fn find_tasks_by_plan_id(&self, plan_id: Uuid) -> Result<Vec<PlanTask>> {
        let tasks = sqlx::query_as::<_, PlanTask>(
            "SELECT * FROM plan_tasks WHERE plan_id = ? ORDER BY task_order ASC",
        )
        .bind(plan_id.to_string())
        .fetch_all(&self.pool)
        .await
        .context("Failed to find plan tasks")?;

        Ok(tasks)
    }

    /// Create a new plan with tasks
    pub async fn create(&self, plan: &PlanDocument) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Insert plan
        let (db_plan, db_tasks) = self.plan_to_db(plan)?;

        sqlx::query(
            r#"
            INSERT INTO plans (id, session_id, title, description, context, risks,
                             status, created_at, updated_at, approved_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(db_plan.id.to_string())
        .bind(db_plan.session_id.to_string())
        .bind(&db_plan.title)
        .bind(&db_plan.description)
        .bind(&db_plan.context)
        .bind(&db_plan.risks)
        .bind(&db_plan.status)
        .bind(db_plan.created_at.timestamp())
        .bind(db_plan.updated_at.timestamp())
        .bind(db_plan.approved_at.map(|dt| dt.timestamp()))
        .execute(&mut *tx)
        .await
        .context("Failed to create plan")?;

        // Insert tasks
        for task in db_tasks {
            sqlx::query(
                r#"
                INSERT INTO plan_tasks (id, plan_id, task_order, title, description,
                                       task_type, dependencies, complexity, status,
                                       notes, completed_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(task.id.to_string())
            .bind(task.plan_id.to_string())
            .bind(task.task_order)
            .bind(&task.title)
            .bind(&task.description)
            .bind(&task.task_type)
            .bind(&task.dependencies)
            .bind(task.complexity)
            .bind(&task.status)
            .bind(&task.notes)
            .bind(task.completed_at.map(|dt| dt.timestamp()))
            .execute(&mut *tx)
            .await
            .context("Failed to create plan task")?;
        }

        tx.commit().await?;

        tracing::debug!("Created plan: {}", plan.id);
        Ok(())
    }

    /// Update an existing plan
    pub async fn update(&self, plan: &PlanDocument) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let (db_plan, db_tasks) = self.plan_to_db(plan)?;

        // Update plan
        sqlx::query(
            r#"
            UPDATE plans
            SET title = ?, description = ?, context = ?, risks = ?,
                status = ?, updated_at = ?, approved_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&db_plan.title)
        .bind(&db_plan.description)
        .bind(&db_plan.context)
        .bind(&db_plan.risks)
        .bind(&db_plan.status)
        .bind(db_plan.updated_at.timestamp())
        .bind(db_plan.approved_at.map(|dt| dt.timestamp()))
        .bind(db_plan.id.to_string())
        .execute(&mut *tx)
        .await
        .context("Failed to update plan")?;

        // Delete existing tasks and re-insert
        sqlx::query("DELETE FROM plan_tasks WHERE plan_id = ?")
            .bind(db_plan.id.to_string())
            .execute(&mut *tx)
            .await
            .context("Failed to delete old plan tasks")?;

        // Insert updated tasks
        for task in db_tasks {
            sqlx::query(
                r#"
                INSERT INTO plan_tasks (id, plan_id, task_order, title, description,
                                       task_type, dependencies, complexity, status,
                                       notes, completed_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(task.id.to_string())
            .bind(task.plan_id.to_string())
            .bind(task.task_order)
            .bind(&task.title)
            .bind(&task.description)
            .bind(&task.task_type)
            .bind(&task.dependencies)
            .bind(task.complexity)
            .bind(&task.status)
            .bind(&task.notes)
            .bind(task.completed_at.map(|dt| dt.timestamp()))
            .execute(&mut *tx)
            .await
            .context("Failed to update plan task")?;
        }

        tx.commit().await?;

        tracing::debug!("Updated plan: {}", plan.id);
        Ok(())
    }

    /// Delete a plan and all its tasks
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        // Tasks will be deleted automatically via CASCADE
        sqlx::query("DELETE FROM plans WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .context("Failed to delete plan")?;

        tracing::debug!("Deleted plan: {}", id);
        Ok(())
    }

    /// Convert database models to domain model
    fn plan_from_db(&self, db_plan: Plan, db_tasks: Vec<PlanTask>) -> Result<PlanDocument> {
        let risks: Vec<String> = serde_json::from_str(&db_plan.risks)
            .context("Failed to parse risks JSON")?;

        let status = self.parse_plan_status(&db_plan.status)?;

        let mut tasks = Vec::new();
        for db_task in db_tasks {
            tasks.push(self.task_from_db(db_task)?);
        }

        Ok(PlanDocument {
            id: db_plan.id,
            session_id: db_plan.session_id,
            title: db_plan.title,
            description: db_plan.description,
            tasks,
            context: db_plan.context,
            risks,
            status,
            created_at: db_plan.created_at,
            updated_at: db_plan.updated_at,
            approved_at: db_plan.approved_at,
        })
    }

    /// Convert database task to domain task
    fn task_from_db(&self, db_task: PlanTask) -> Result<crate::tui::plan::PlanTask> {
        let dependencies: Vec<Uuid> = serde_json::from_str(&db_task.dependencies)
            .context("Failed to parse dependencies JSON")?;

        let task_type = self.parse_task_type(&db_task.task_type)?;
        let status = self.parse_task_status(&db_task.status)?;

        Ok(crate::tui::plan::PlanTask {
            id: db_task.id,
            order: db_task.task_order as usize,
            title: db_task.title,
            description: db_task.description,
            task_type,
            dependencies,
            complexity: db_task.complexity as u8,
            status,
            notes: db_task.notes,
            completed_at: db_task.completed_at,
        })
    }

    /// Convert domain model to database models
    fn plan_to_db(&self, plan: &PlanDocument) -> Result<(Plan, Vec<PlanTask>)> {
        let risks = serde_json::to_string(&plan.risks)
            .context("Failed to serialize risks")?;

        let db_plan = Plan {
            id: plan.id,
            session_id: plan.session_id,
            title: plan.title.clone(),
            description: plan.description.clone(),
            context: plan.context.clone(),
            risks,
            status: self.format_plan_status(&plan.status),
            created_at: plan.created_at,
            updated_at: plan.updated_at,
            approved_at: plan.approved_at,
        };

        let mut db_tasks = Vec::new();
        for task in &plan.tasks {
            db_tasks.push(self.task_to_db(task, plan.id)?);
        }

        Ok((db_plan, db_tasks))
    }

    /// Convert domain task to database task
    fn task_to_db(&self, task: &crate::tui::plan::PlanTask, plan_id: Uuid) -> Result<PlanTask> {
        let dependencies = serde_json::to_string(&task.dependencies)
            .context("Failed to serialize dependencies")?;

        Ok(PlanTask {
            id: task.id,
            plan_id,
            task_order: task.order as i32,
            title: task.title.clone(),
            description: task.description.clone(),
            task_type: self.format_task_type(&task.task_type),
            dependencies,
            complexity: task.complexity as i32,
            status: self.format_task_status(&task.status),
            notes: task.notes.clone(),
            completed_at: task.completed_at,
        })
    }

    /// Parse plan status from string
    fn parse_plan_status(&self, status: &str) -> Result<PlanStatus> {
        Ok(match status {
            "Draft" => PlanStatus::Draft,
            "PendingApproval" => PlanStatus::PendingApproval,
            "Approved" => PlanStatus::Approved,
            "Rejected" => PlanStatus::Rejected,
            "InProgress" => PlanStatus::InProgress,
            "Completed" => PlanStatus::Completed,
            "Cancelled" => PlanStatus::Cancelled,
            _ => anyhow::bail!("Invalid plan status: {}", status),
        })
    }

    /// Format plan status to string
    fn format_plan_status(&self, status: &PlanStatus) -> String {
        match status {
            PlanStatus::Draft => "Draft",
            PlanStatus::PendingApproval => "PendingApproval",
            PlanStatus::Approved => "Approved",
            PlanStatus::Rejected => "Rejected",
            PlanStatus::InProgress => "InProgress",
            PlanStatus::Completed => "Completed",
            PlanStatus::Cancelled => "Cancelled",
        }
        .to_string()
    }

    /// Parse task type from string
    fn parse_task_type(&self, task_type: &str) -> Result<TaskType> {
        Ok(match task_type {
            "Research" => TaskType::Research,
            "Edit" => TaskType::Edit,
            "Create" => TaskType::Create,
            "Delete" => TaskType::Delete,
            "Test" => TaskType::Test,
            "Refactor" => TaskType::Refactor,
            "Documentation" => TaskType::Documentation,
            "Configuration" => TaskType::Configuration,
            "Build" => TaskType::Build,
            other => TaskType::Other(other.to_string()),
        })
    }

    /// Format task type to string
    fn format_task_type(&self, task_type: &TaskType) -> String {
        match task_type {
            TaskType::Research => "Research",
            TaskType::Edit => "Edit",
            TaskType::Create => "Create",
            TaskType::Delete => "Delete",
            TaskType::Test => "Test",
            TaskType::Refactor => "Refactor",
            TaskType::Documentation => "Documentation",
            TaskType::Configuration => "Configuration",
            TaskType::Build => "Build",
            TaskType::Other(s) => s,
        }
        .to_string()
    }

    /// Parse task status from string
    fn parse_task_status(&self, status: &str) -> Result<TaskStatus> {
        if let Some(reason) = status.strip_prefix("Blocked:") {
            return Ok(TaskStatus::Blocked(reason.trim().to_string()));
        }

        Ok(match status {
            "Pending" => TaskStatus::Pending,
            "InProgress" => TaskStatus::InProgress,
            "Completed" => TaskStatus::Completed,
            "Skipped" => TaskStatus::Skipped,
            "Failed" => TaskStatus::Failed,
            _ => anyhow::bail!("Invalid task status: {}", status),
        })
    }

    /// Format task status to string
    fn format_task_status(&self, status: &TaskStatus) -> String {
        match status {
            TaskStatus::Pending => "Pending".to_string(),
            TaskStatus::InProgress => "InProgress".to_string(),
            TaskStatus::Completed => "Completed".to_string(),
            TaskStatus::Skipped => "Skipped".to_string(),
            TaskStatus::Failed => "Failed".to_string(),
            TaskStatus::Blocked(reason) => format!("Blocked:{}", reason),
        }
    }
}
