//! Integration test for auto-run plan mode (QS-1.3, FR-002).
//!
//! Run with: cargo test plan_autorun

use crustly::tui::plan::{AutoRunMode, PlanModeState, PlanTask, TaskStatus, TaskType};
use uuid::Uuid;

fn make_plan_task(order: usize, title: &str) -> PlanTask {
    PlanTask {
        id: Uuid::new_v4(),
        order,
        title: title.to_string(),
        description: String::new(),
        task_type: TaskType::Research,
        dependencies: vec![],
        complexity: 1,
        acceptance_criteria: vec![],
        status: TaskStatus::Pending,
        notes: None,
        completed_at: None,
        execution_history: vec![],
        retry_count: 0,
        max_retries: 3,
        artifacts: vec![],
        reflection: None,
    }
}

/// QS-1.3: With mode = "auto_plan", approval transitions to AutoExecuting, not Executing.
/// Zero per-task permission dialogs for read-only tasks (tool_needs_approval returns false).
#[test]
fn auto_plan_approval_goes_to_auto_executing() {
    let plan_id = Uuid::new_v4();
    let tasks = vec![
        make_plan_task(0, "read and analyze code"),
        make_plan_task(1, "search for usages"),
        make_plan_task(2, "read tests"),
    ];

    // With auto_plan = true
    let state = PlanModeState::approve(plan_id, tasks, true);

    match &state {
        PlanModeState::AutoExecuting {
            task_index,
            total,
            mode,
            ..
        } => {
            assert_eq!(*task_index, 0);
            assert_eq!(*total, 3);
            assert_eq!(*mode, AutoRunMode::AutoPlan);
        }
        other => panic!("expected AutoExecuting, got {:?}", other),
    }

    // Read-only tools must not require approval in AutoExecuting state
    for read_tool in &["read_file", "glob", "grep", "ls", "web_search"] {
        assert!(
            !state.tool_needs_approval(read_tool, 70),
            "{} must not need approval in AutoExecuting",
            read_tool
        );
    }
}

/// Interactive mode: approval goes to Executing (per-task dialogs active).
#[test]
fn interactive_approval_goes_to_executing() {
    let plan_id = Uuid::new_v4();
    let tasks = vec![make_plan_task(0, "do something")];

    let state = PlanModeState::approve(plan_id, tasks, false);
    assert!(matches!(state, PlanModeState::Executing { .. }));

    // All tools require approval in Executing state
    assert!(state.tool_needs_approval("read_file", 70));
    assert!(state.tool_needs_approval("bash", 70));
}

/// High-risk tools must pause AutoExecuting to Paused { RiskThresholdExceeded }.
#[test]
fn high_risk_tools_pause_auto_execution() {
    let plan_id = Uuid::new_v4();
    let state = PlanModeState::AutoExecuting {
        plan_id,
        task_index: 1,
        total: 5,
        mode: AutoRunMode::AutoPlan,
    };

    for high_risk_tool in &["bash", "write_file", "edit_file", "code_exec"] {
        assert!(
            PlanModeState::is_high_risk_tool(high_risk_tool),
            "{} must be classified high-risk",
            high_risk_tool
        );
        assert!(
            state.tool_needs_approval(high_risk_tool, 70),
            "{} must need approval in AutoExecuting",
            high_risk_tool
        );
    }
}

/// advance() increments task_index until total, then transitions to Done.
#[test]
fn advance_transitions_through_tasks_to_done() {
    let plan_id = Uuid::new_v4();
    let state = PlanModeState::AutoExecuting {
        plan_id,
        task_index: 0,
        total: 3,
        mode: AutoRunMode::AutoPlan,
    };

    // Advance through 3 tasks
    let s1 = state.advance();
    match &s1 {
        PlanModeState::AutoExecuting { task_index, .. } => assert_eq!(*task_index, 1),
        _ => panic!("expected AutoExecuting after first advance"),
    }

    let s2 = s1.advance();
    match &s2 {
        PlanModeState::AutoExecuting { task_index, .. } => assert_eq!(*task_index, 2),
        _ => panic!("expected AutoExecuting after second advance"),
    }

    let s3 = s2.advance();
    match s3 {
        PlanModeState::Done { .. } => {} // expected
        other => panic!("expected Done after last advance, got {:?}", other),
    }
}
