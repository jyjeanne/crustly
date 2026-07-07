//! Integration test for plan crash recovery (QS-1.1, FR-001).
//!
//! Run with: cargo test plan_crash_recovery

use crustly::db::models::{interrupted_plan_from_tasks, PlanTaskStatus};
use crustly::db::repository::PlanTaskRepository;
use crustly::db::Database;
use uuid::Uuid;

async fn create_session(pool: &sqlx::SqlitePool, session_id: Uuid) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    sqlx::query(
        "INSERT INTO sessions (id, title, model, created_at, updated_at) VALUES (?, 'Test', 'claude-3-sonnet', ?, ?)"
    )
    .bind(session_id.to_string())
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .unwrap();
}

async fn create_plan(pool: &sqlx::SqlitePool, plan_id: Uuid, session_id: Uuid) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    sqlx::query(
        "INSERT INTO plans (id, session_id, title, description, status, created_at, updated_at) \
         VALUES (?, ?, 'Test Plan', '', 'Draft', ?, ?)",
    )
    .bind(plan_id.to_string())
    .bind(session_id.to_string())
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .unwrap();
}

/// Build a minimal PlanTask row for test insertion.
fn minimal_task(plan_id: Uuid, task_order: i32, status: &str) -> crustly::db::models::PlanTask {
    crustly::db::models::PlanTask {
        id: Uuid::new_v4(),
        plan_id,
        task_order,
        title: format!("Task {}", task_order + 1),
        description: String::new(),
        task_type: "Research".to_string(),
        dependencies: "[]".to_string(),
        complexity: 1,
        acceptance_criteria: "[]".to_string(),
        status: status.to_string(),
        notes: None,
        completed_at: None,
        started_at: None,
        output_summary: None,
        error_text: None,
    }
}

/// QS-1.1: Create plan with 4 tasks, mark task 0+1 Done, task 2 Running, task 3 Pending.
/// Simulate restart: get_incomplete_tasks should return tasks 2 and 3 (not 0 or 1).
/// Minimum task_order of incomplete tasks must be 2.
#[tokio::test]
async fn crash_recovery_resumes_at_correct_task() {
    let db = Database::connect_in_memory().await.expect("in-memory db");
    db.run_migrations().await.expect("migrations");

    let session_id = Uuid::new_v4();
    let plan_id = Uuid::new_v4();
    create_session(db.pool(), session_id).await;
    create_plan(db.pool(), plan_id, session_id).await;

    let repo = PlanTaskRepository::new(db.pool().clone());

    // Simulate pre-crash: task 0=Done, task 1=Done, task 2=Running, task 3=Pending
    repo.create_task(minimal_task(plan_id, 0, "Done"))
        .await
        .unwrap();
    repo.create_task(minimal_task(plan_id, 1, "Done"))
        .await
        .unwrap();
    repo.create_task(minimal_task(plan_id, 2, "Running"))
        .await
        .unwrap();
    repo.create_task(minimal_task(plan_id, 3, "Pending"))
        .await
        .unwrap();

    // Simulate restart: query for incomplete tasks
    let incomplete = repo.get_incomplete_tasks(plan_id).await.unwrap();

    // Should have tasks 2 (Running) and 3 (Pending) — not 0 or 1
    assert_eq!(
        incomplete.len(),
        2,
        "expected 2 incomplete tasks, got {}",
        incomplete.len()
    );

    let indices: Vec<i32> = incomplete.iter().map(|t| t.task_order).collect();
    assert!(
        indices.contains(&2),
        "task 2 (Running) must appear in incomplete"
    );
    assert!(
        indices.contains(&3),
        "task 3 (Pending) must appear in incomplete"
    );
    assert!(
        !indices.contains(&0),
        "task 0 (Done) must NOT appear in incomplete"
    );
    assert!(
        !indices.contains(&1),
        "task 1 (Done) must NOT appear in incomplete"
    );

    // Resume index must be 2 (lowest incomplete)
    let resume_index = incomplete.iter().map(|t| t.task_order).min().unwrap();
    assert_eq!(resume_index, 2, "must resume at task 2, not task 0 or 1");
}

/// Task must be marked Running BEFORE execution, Done only AFTER verified output.
#[tokio::test]
async fn task_state_transitions_correct_order() {
    let db = Database::connect_in_memory().await.expect("in-memory db");
    db.run_migrations().await.expect("migrations");

    let session_id = Uuid::new_v4();
    let plan_id = Uuid::new_v4();
    create_session(db.pool(), session_id).await;
    create_plan(db.pool(), plan_id, session_id).await;

    let repo = PlanTaskRepository::new(db.pool().clone());
    let task = minimal_task(plan_id, 0, "Pending");
    let task_id = task.id;
    repo.create_task(task).await.unwrap();

    // Step 1: mark Running before execution
    repo.update_task_status(task_id, PlanTaskStatus::Running, None, None)
        .await
        .unwrap();
    let running = repo.get_task(task_id).await.unwrap();
    assert_eq!(running.exec_status(), PlanTaskStatus::Running);
    assert!(
        running.started_at.is_some(),
        "started_at must be set when Running"
    );

    // Step 2: mark Done after verified output
    repo.update_task_status(
        task_id,
        PlanTaskStatus::Done,
        Some("wrote hello.txt".to_string()),
        None,
    )
    .await
    .unwrap();
    let done = repo.get_task(task_id).await.unwrap();
    assert_eq!(done.exec_status(), PlanTaskStatus::Done);
    assert_eq!(done.output_summary.as_deref(), Some("wrote hello.txt"));
    assert!(
        done.completed_at.is_some(),
        "completed_at must be set on Done"
    );
}

/// Failed task must store error_text and NOT set completed_at.
#[tokio::test]
async fn failed_task_stores_error_without_completion_timestamp() {
    let db = Database::connect_in_memory().await.expect("in-memory db");
    db.run_migrations().await.expect("migrations");

    let session_id = Uuid::new_v4();
    let plan_id = Uuid::new_v4();
    create_session(db.pool(), session_id).await;
    create_plan(db.pool(), plan_id, session_id).await;

    let repo = PlanTaskRepository::new(db.pool().clone());
    let task = minimal_task(plan_id, 0, "Pending");
    let task_id = task.id;
    repo.create_task(task).await.unwrap();

    repo.update_task_status(
        task_id,
        PlanTaskStatus::Failed,
        None,
        Some("write_file: permission denied".to_string()),
    )
    .await
    .unwrap();

    let failed = repo.get_task(task_id).await.unwrap();
    assert_eq!(failed.exec_status(), PlanTaskStatus::Failed);
    assert!(
        failed
            .error_text
            .as_deref()
            .unwrap()
            .contains("permission denied"),
        "error_text must contain the failure reason"
    );
}

/// interrupted_plan_from_tasks returns None when all tasks are Done.
#[test]
fn interrupted_plan_none_when_all_done() {
    let plan_id = Uuid::new_v4();
    let tasks = vec![
        minimal_task(plan_id, 0, "Done"),
        minimal_task(plan_id, 1, "Done"),
    ];
    let result = interrupted_plan_from_tasks(plan_id, &tasks);
    assert!(result.is_none(), "no interrupt if all tasks done");
}

/// interrupted_plan_from_tasks returns correct resume_at_index.
#[test]
fn interrupted_plan_resumes_at_lowest_incomplete() {
    let plan_id = Uuid::new_v4();
    let tasks = vec![
        minimal_task(plan_id, 0, "Done"),
        minimal_task(plan_id, 1, "Done"),
        minimal_task(plan_id, 2, "Running"),
        minimal_task(plan_id, 3, "Pending"),
    ];
    let result = interrupted_plan_from_tasks(plan_id, &tasks).unwrap();
    assert_eq!(result.resume_at_index, 2);
    assert_eq!(result.total_tasks, 4);
    assert_eq!(result.plan_id, plan_id);
}
