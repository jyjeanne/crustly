//! Plan Mode Data Structures
//!
//! Core data structures for plan mode, which enables structured task decomposition
//! and controlled execution for complex development tasks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Plan document containing tasks and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDocument {
    /// Unique plan ID
    pub id: Uuid,

    /// Session this plan belongs to
    pub session_id: Uuid,

    /// Plan title/goal
    pub title: String,

    /// Detailed description
    pub description: String,

    /// List of tasks to complete
    pub tasks: Vec<PlanTask>,

    /// Context and assumptions
    pub context: String,

    /// Identified risks and unknowns
    pub risks: Vec<String>,

    /// Plan status
    pub status: PlanStatus,

    /// When the plan was created
    pub created_at: DateTime<Utc>,

    /// When the plan was last updated
    pub updated_at: DateTime<Utc>,

    /// When the plan was approved (if applicable)
    pub approved_at: Option<DateTime<Utc>>,
}

impl PlanDocument {
    /// Create a new plan document
    pub fn new(session_id: Uuid, title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            title,
            description,
            tasks: Vec::new(),
            context: String::new(),
            risks: Vec::new(),
            status: PlanStatus::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            approved_at: None,
        }
    }

    /// Add a task to the plan
    pub fn add_task(&mut self, task: PlanTask) {
        self.tasks.push(task);
        self.updated_at = Utc::now();
    }

    /// Get tasks in dependency order using topological sort
    /// Returns None if there are circular dependencies
    pub fn tasks_in_order(&self) -> Option<Vec<&PlanTask>> {
        use std::collections::{HashMap, VecDeque};

        // Build dependency graph
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut dependents: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        // Initialize in-degree for all tasks
        for task in &self.tasks {
            in_degree.insert(task.id, task.dependencies.len());

            // Build reverse dependency map
            for &dep_id in &task.dependencies {
                dependents.entry(dep_id).or_default().push(task.id);
            }
        }

        // Kahn's algorithm for topological sort
        let mut queue: VecDeque<Uuid> = VecDeque::new();

        // Start with tasks that have no dependencies
        for task in &self.tasks {
            if task.dependencies.is_empty() {
                queue.push_back(task.id);
            }
        }

        let mut sorted_ids = Vec::new();

        while let Some(task_id) = queue.pop_front() {
            sorted_ids.push(task_id);

            // Process tasks that depend on this one
            if let Some(deps) = dependents.get(&task_id) {
                for &dependent_id in deps {
                    if let Some(degree) = in_degree.get_mut(&dependent_id) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent_id);
                        }
                    }
                }
            }
        }

        // Check for cycles - if we didn't process all tasks, there's a cycle
        if sorted_ids.len() != self.tasks.len() {
            return None; // Circular dependency detected
        }

        // Convert sorted IDs back to task references
        let task_map: HashMap<Uuid, &PlanTask> =
            self.tasks.iter().map(|t| (t.id, t)).collect();

        Some(sorted_ids.iter().filter_map(|id| task_map.get(id).copied()).collect())
    }

    /// Get task by ID
    pub fn get_task(&self, task_id: &Uuid) -> Option<&PlanTask> {
        self.tasks.iter().find(|t| t.id == *task_id)
    }

    /// Get mutable task by ID
    pub fn get_task_mut(&mut self, task_id: &Uuid) -> Option<&mut PlanTask> {
        self.updated_at = Utc::now();
        self.tasks.iter_mut().find(|t| t.id == *task_id)
    }

    /// Count tasks by status
    pub fn count_by_status(&self, status: TaskStatus) -> usize {
        self.tasks.iter().filter(|t| t.status == status).count()
    }

    /// Get progress percentage (0-100)
    pub fn progress_percentage(&self) -> f32 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        let completed = self.count_by_status(TaskStatus::Completed);
        (completed as f32 / self.tasks.len() as f32) * 100.0
    }

    /// Check if all tasks are completed
    pub fn is_complete(&self) -> bool {
        !self.tasks.is_empty()
            && self.tasks.iter().all(|t| {
                matches!(t.status, TaskStatus::Completed | TaskStatus::Skipped)
            })
    }

    /// Approve the plan
    pub fn approve(&mut self) {
        self.status = PlanStatus::Approved;
        self.approved_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Reject the plan
    pub fn reject(&mut self) {
        self.status = PlanStatus::Rejected;
        self.updated_at = Utc::now();
    }

    /// Mark plan as in progress
    pub fn start_execution(&mut self) {
        self.status = PlanStatus::InProgress;
        self.updated_at = Utc::now();
    }

    /// Mark plan as completed
    pub fn complete(&mut self) {
        self.status = PlanStatus::Completed;
        self.updated_at = Utc::now();
    }

    /// Validate task dependencies
    /// Returns Ok(()) if all dependencies are valid, or Err with description of issues
    pub fn validate_dependencies(&self) -> Result<(), String> {
        let task_ids: std::collections::HashSet<Uuid> =
            self.tasks.iter().map(|t| t.id).collect();

        // Check for invalid task references
        for task in &self.tasks {
            for &dep_id in &task.dependencies {
                if !task_ids.contains(&dep_id) {
                    return Err(format!(
                        "❌ Invalid Dependency\n\n\
                         Task '{}' (#{}) depends on a task that doesn't exist.\n\n\
                         💡 Fix: Remove this dependency or ensure the referenced task is added first.",
                        task.title, task.order
                    ));
                }
            }
        }

        // Check for circular dependencies using topological sort
        let ordered = self.tasks_in_order();
        if ordered.is_none() {
            // Identify unprocessed tasks (those in the cycle)
            let unprocessed: Vec<&str> = self
                .tasks
                .iter()
                .filter(|task| !task.dependencies.is_empty())
                .map(|task| task.title.as_str())
                .collect();

            return Err(format!(
                "❌ Circular Dependency Detected\n\n\
                 Tasks with dependencies: {}\n\n\
                 💡 Fix: Review the dependency chain and remove circular references.\n\
                 Example: If Task A depends on B, B depends on C, and C depends on A,\n\
                 you need to break one of these dependency links.",
                unprocessed.join(", ")
            ));
        }

        Ok(())
    }
}

/// Status of a plan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlanStatus {
    /// Plan is being drafted
    Draft,
    /// Plan is ready for review
    PendingApproval,
    /// Plan was approved by user
    Approved,
    /// Plan was rejected, needs revision
    Rejected,
    /// Plan is being executed
    InProgress,
    /// All tasks completed
    Completed,
    /// Plan was cancelled
    Cancelled,
}

impl std::fmt::Display for PlanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanStatus::Draft => write!(f, "Draft"),
            PlanStatus::PendingApproval => write!(f, "Pending Approval"),
            PlanStatus::Approved => write!(f, "Approved"),
            PlanStatus::Rejected => write!(f, "Rejected"),
            PlanStatus::InProgress => write!(f, "In Progress"),
            PlanStatus::Completed => write!(f, "Completed"),
            PlanStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Individual task within a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanTask {
    /// Unique task ID
    pub id: Uuid,

    /// Task number/order
    pub order: usize,

    /// Task title/summary
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Task type (for categorization)
    pub task_type: TaskType,

    /// Dependencies (task IDs that must complete first)
    pub dependencies: Vec<Uuid>,

    /// Estimated complexity (1-5)
    pub complexity: u8,

    /// Task status
    pub status: TaskStatus,

    /// Execution notes/results
    pub notes: Option<String>,

    /// When task was completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl PlanTask {
    /// Create a new task
    pub fn new(order: usize, title: String, description: String, task_type: TaskType) -> Self {
        Self {
            id: Uuid::new_v4(),
            order,
            title,
            description,
            task_type,
            dependencies: Vec::new(),
            complexity: 3, // Default medium complexity
            status: TaskStatus::Pending,
            notes: None,
            completed_at: None,
        }
    }

    /// Mark task as in progress
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
    }

    /// Complete the task
    pub fn complete(&mut self, notes: Option<String>) {
        self.status = TaskStatus::Completed;
        self.notes = notes;
        self.completed_at = Some(Utc::now());
    }

    /// Mark task as failed
    pub fn fail(&mut self, reason: String) {
        self.status = TaskStatus::Failed;
        self.notes = Some(reason);
    }

    /// Mark task as blocked
    pub fn block(&mut self, reason: String) {
        self.status = TaskStatus::Blocked(reason);
    }

    /// Skip the task
    pub fn skip(&mut self, reason: Option<String>) {
        self.status = TaskStatus::Skipped;
        if let Some(r) = reason {
            self.notes = Some(r);
        }
    }

    /// Get complexity stars (1-5)
    pub fn complexity_stars(&self) -> String {
        let filled = self.complexity.min(5);
        let empty = 5 - filled;
        "★".repeat(filled as usize) + &"☆".repeat(empty as usize)
    }
}

/// Types of tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    /// Research/exploration
    Research,
    /// File modification
    Edit,
    /// New file creation
    Create,
    /// File deletion
    Delete,
    /// Test creation/modification
    Test,
    /// Refactoring
    Refactor,
    /// Documentation
    Documentation,
    /// Configuration change
    Configuration,
    /// Build/deployment
    Build,
    /// Other
    Other(String),
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Research => write!(f, "Research"),
            TaskType::Edit => write!(f, "Edit"),
            TaskType::Create => write!(f, "Create"),
            TaskType::Delete => write!(f, "Delete"),
            TaskType::Test => write!(f, "Test"),
            TaskType::Refactor => write!(f, "Refactor"),
            TaskType::Documentation => write!(f, "Documentation"),
            TaskType::Configuration => write!(f, "Configuration"),
            TaskType::Build => write!(f, "Build"),
            TaskType::Other(s) => write!(f, "{}", s),
        }
    }
}

/// Status of individual tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    /// Not started
    Pending,
    /// Currently being worked on
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task skipped
    Skipped,
    /// Task failed
    Failed,
    /// Task blocked by dependencies or issues
    Blocked(String),
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Skipped => write!(f, "Skipped"),
            TaskStatus::Failed => write!(f, "Failed"),
            TaskStatus::Blocked(reason) => write!(f, "Blocked: {}", reason),
        }
    }
}

impl TaskStatus {
    /// Get status icon for UI display
    pub fn icon(&self) -> &str {
        match self {
            TaskStatus::Pending => "⏸️",
            TaskStatus::InProgress => "▶️",
            TaskStatus::Completed => "✅",
            TaskStatus::Skipped => "⏭️",
            TaskStatus::Failed => "❌",
            TaskStatus::Blocked(_) => "🚫",
        }
    }
}

#[cfg(test)]
#[path = "plan_tests.rs"]
mod plan_tests;
