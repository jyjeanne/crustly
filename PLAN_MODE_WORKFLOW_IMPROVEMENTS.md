# Plan Mode Workflow Improvements

## Problem Statement

Current plan mode has UX issues:
1. LLM hallucinates "Plan finalized!" without actually calling the tool
2. Auto-switches to Plan Mode which confuses users
3. No way to request plan revisions without rejecting entirely
4. Finalize operation sometimes doesn't update file status properly

## Proposed Improved Workflow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. User: "Create a plan for JWT authentication"            │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 2. LLM builds plan silently (status: Draft)                │
│    - Calls plan(operation="create")                         │
│    - Calls plan(operation="add_task") multiple times        │
│    - User sees tool calls but stays in Chat Mode            │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 3. LLM finalizes plan                                       │
│    - Calls plan(operation="finalize")                       │
│    - Status changes: Draft → PendingApproval                │
│    - File is saved                                          │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 4. Crustly shows notification (stays in Chat Mode)          │
│    ✅ Plan ready! Press Ctrl+P to review                    │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 5. User presses Ctrl+P                                      │
│    - Switches to Plan Mode                                  │
│    - Shows full plan with all tasks                         │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 6. User makes decision:                                     │
│    • Ctrl+A: Approve → Execute tasks                        │
│    • Ctrl+R: Reject → Discard plan                          │
│    • Ctrl+I: Request revision → Back to chat with feedback  │
│    • Esc: Cancel → Back to chat (plan stays available)      │
└─────────────────────────────────────────────────────────────┘
```

## Required Code Changes

### 1. Don't Auto-Switch to Plan Mode

**File:** `src/tui/app.rs`
**Location:** Line 741-747 in `check_and_load_plan()`

**Current:**
```rust
if plan.status == crate::tui::plan::PlanStatus::PendingApproval {
    tracing::info!("✅ Loading plan from database and switching to Plan Mode");
    self.current_plan = Some(plan);
    self.mode = AppMode::Plan;  // ← AUTO-SWITCH (REMOVE THIS)
    self.plan_scroll_offset = 0;
    self.selected_task_index = None;
}
```

**Should Be:**
```rust
if plan.status == crate::tui::plan::PlanStatus::PendingApproval {
    // Load plan but DON'T switch mode
    tracing::info!("✅ Plan ready for review!");

    // Only load if not already loaded (avoid duplicate messages)
    if self.current_plan.is_none() {
        self.current_plan = Some(plan.clone());

        // Add notification message to chat
        let notification = DisplayMessage {
            role: Role::System,
            content: vec![ContentBlock::Text {
                text: format!(
                    "✅ Plan '{}' is ready!\n\n\
                     {} tasks • Press Ctrl+P to review\n\n\
                     Actions:\n\
                     • Ctrl+A: Approve and execute\n\
                     • Ctrl+R: Reject\n\
                     • Ctrl+I: Request changes\n\
                     • Ctrl+P: View plan",
                    plan.title,
                    plan.tasks.len()
                ),
            }],
            timestamp: chrono::Utc::now(),
        };

        self.messages.push(notification);
    }
}
```

**Also update line 780-789** (JSON fallback) with the same logic.

---

### 2. Add Ctrl+I Shortcut for Plan Revision

**File:** `src/tui/app.rs`
**Location:** After line 447 in `handle_plan_key()`

**Add:**
```rust
// Ctrl+I - Request plan revision
if event.code == KeyCode::Char('i') && event.modifiers.contains(KeyModifiers::CONTROL) {
    tracing::info!("🔄 Ctrl+I pressed - Requesting plan revision");
    if let Some(plan) = &self.current_plan {
        let plan_summary = format!(
            "Current plan '{}' has {} tasks:\n{}",
            plan.title,
            plan.tasks.len(),
            plan.tasks
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{}. {} ({})", i + 1, t.title, t.task_type))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // Switch back to chat mode
        self.switch_mode(AppMode::Chat).await?;

        // Pre-fill input with revision request
        self.input_text = format!(
            "Please revise this plan. {}\n\nRequested changes: ",
            plan_summary
        );

        // Don't clear plan - keep it for reference
    }
    return Ok(());
}
```

---

### 3. Update Plan Mode UI Help Text

**File:** `src/tui/render.rs` (or wherever plan UI is rendered)
**Location:** Plan Mode help text

**Update help bar to include:**
```
Ctrl+A: Approve | Ctrl+R: Reject | Ctrl+I: Request Changes | Esc: Back | ↑↓: Scroll
```

---

### 4. Add Logging to Diagnose Finalize Issues

**File:** `src/llm/tools/plan_tool.rs`
**Location:** Line 395-406 in finalize operation

**Add detailed logging:**
```rust
PlanOperation::Finalize => {
    tracing::info!("🔧 Finalize operation starting...");

    let current_plan = plan.as_mut().ok_or_else(|| {
        tracing::error!("❌ Finalize failed: No active plan");
        ToolError::InvalidInput("No active plan to finalize.".to_string())
    })?;

    if current_plan.tasks.is_empty() {
        tracing::warn!("⚠️ Cannot finalize: Plan has no tasks");
        return Ok(ToolResult::error(
            "Cannot finalize plan with no tasks. Add tasks first.".to_string(),
        ));
    }

    tracing::debug!(
        "📋 Finalizing plan: title='{}', tasks={}",
        current_plan.title,
        current_plan.tasks.len()
    );

    // Validate dependencies before finalizing
    if let Err(e) = current_plan.validate_dependencies() {
        tracing::error!("❌ Dependency validation failed: {}", e);
        return Ok(ToolResult::error(format!(
            "Cannot finalize plan: {}\n\n\
             Please fix the dependency issues before finalizing.",
            e
        )));
    }

    // Change status
    let old_status = current_plan.status;
    current_plan.status = PlanStatus::PendingApproval;
    current_plan.updated_at = Utc::now();

    tracing::info!(
        "✅ Plan status changed: {:?} → {:?}",
        old_status,
        current_plan.status
    );

    format!(
        "✓ Plan finalized and ready for review!\n\n\
         📋 Plan: {}\n\
         📝 {} tasks ready for execution\n\n\
         Press Ctrl+P to review the plan.",
        current_plan.title,
        current_plan.tasks.len()
    )
}
```

**Also add logging after file save (line 446):**
```rust
// Atomic rename (ensures consistency even if interrupted)
tokio::fs::rename(&temp_file, &plan_file)
    .await
    .map_err(ToolError::Io)?;

tracing::info!(
    "💾 Plan saved to file: {} (status: {:?})",
    plan_file.display(),
    current_plan.status
);
```

---

### 5. Make Finalize More Robust

**File:** `src/llm/tools/plan_tool.rs`
**Location:** After line 446

**Add verification:**
```rust
// Verify file was written correctly
if plan_file.exists() {
    match tokio::fs::read_to_string(&plan_file).await {
        Ok(content) => {
            match serde_json::from_str::<PlanDocument>(&content) {
                Ok(saved_plan) => {
                    tracing::debug!(
                        "✅ Verified saved plan: status={:?}",
                        saved_plan.status
                    );

                    if saved_plan.status != current_plan.status {
                        tracing::error!(
                            "❌ Status mismatch! Expected {:?}, got {:?}",
                            current_plan.status,
                            saved_plan.status
                        );
                    }
                }
                Err(e) => {
                    tracing::error!("❌ Failed to parse saved plan: {}", e);
                }
            }
        }
        Err(e) => {
            tracing::error!("❌ Failed to read saved plan: {}", e);
        }
    }
} else {
    tracing::error!("❌ Plan file does not exist after save!");
}
```

---

## Testing Plan

### Test 1: Create Plan Without Auto-Switch
1. Start Crustly
2. Send: "call plan tool: operation=create, title='Test', description='Test plan'"
3. Send: "call plan tool: operation=add_task, title='Task 1', task_type='create', description='Do something'"
4. Send: "call plan tool: operation=finalize"
5. **Expected:** Should see notification in chat, NOT auto-switch to Plan Mode
6. Press Ctrl+P
7. **Expected:** Plan Mode opens

### Test 2: Request Plan Revision
1. Create and finalize a plan
2. Press Ctrl+P to view
3. Press Ctrl+I
4. **Expected:** Returns to Chat Mode with pre-filled revision request

### Test 3: Finalize Debugging
1. Create a plan with tasks
2. Call finalize
3. Check logs for:
   - "🔧 Finalize operation starting..."
   - "✅ Plan status changed: Draft → PendingApproval"
   - "💾 Plan saved to file..."
   - "✅ Verified saved plan: status=PendingApproval"
4. **Expected:** All log messages appear, no errors

---

## Benefits of New Workflow

1. **No Hallucination Confusion**
   - LLM can't trick user by saying "Plan ready!"
   - User only sees notification when plan is ACTUALLY finalized

2. **Better Control**
   - User decides when to view plan (Ctrl+P)
   - Can continue chatting while plan is ready
   - Can request revisions without rejecting

3. **Clear States**
   - Building (Draft, silent)
   - Ready (PendingApproval, notification shown)
   - Viewing (Plan Mode)
   - Executing (Approved, tasks running)

4. **Improved UX**
   - Less jarring (no auto-mode-switch)
   - More predictable
   - Better feedback through notifications

5. **Easier Debugging**
   - Detailed logs for finalize operation
   - File verification after save
   - Status tracking throughout process

---

## Migration Notes

- Existing plans with status "PendingApproval" will still work
- No database schema changes needed
- Backwards compatible with JSON files

---

## Future Enhancements

1. **Plan Templates**
   - Save successful plans as templates
   - Reuse common workflows

2. **Plan Diff**
   - When requesting revision, show diff of changes

3. **Plan History**
   - View all previous plans for a project
   - Ctrl+H to open plan history

4. **Plan Validation**
   - Check for common issues before finalize
   - Warn if tasks are too complex or have circular dependencies

5. **Parallel Task Execution**
   - Execute independent tasks in parallel
   - Show progress for multiple tasks

---

## Implementation Priority

1. **High Priority** (Fix now)
   - Stop auto-switching to Plan Mode
   - Add detailed logging to finalize

2. **Medium Priority** (Next release)
   - Add Ctrl+I for revisions
   - Update UI help text

3. **Low Priority** (Future)
   - Plan templates
   - Plan history
   - Parallel execution
