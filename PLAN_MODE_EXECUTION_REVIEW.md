# Plan Mode Execution Review

**Date:** 2025-11-11
**Reviewer:** Claude
**Scope:** Complete review of plan mode implementation and workflow

---

## Executive Summary

The plan mode implementation is **largely functional** with all core components in place. However, there are **critical workflow inconsistencies** between the system prompt instructions and the actual TUI execution flow that could cause confusion or errors.

### Status Overview

✅ **Working Components:**
- Tool interface with all 6 operations
- Database persistence layer
- TUI plan display and approval workflow
- Automatic plan detection after finalization
- User approval controls (Ctrl+A, Ctrl+R, Esc)

⚠️ **Issues Found:**
1. Workflow mismatch between system prompt and actual execution
2. Dual markdown export implementations causing confusion
3. File overwrite behavior inconsistency
4. LLM calling export_markdown unnecessarily

---

## Component Review

### 1. ✅ Tool Interface (src/llm/tools/plan_tool.rs)

**Status:** COMPLETE

All 6 operations implemented:
- `create` - Initialize new plan (lines 249-283)
- `add_task` - Add tasks with dependencies (lines 285-355)
- `update_plan` - Update plan metadata (lines 357-382)
- `finalize` - Mark plan ready for review (lines 384-416)
- `status` - Check current plan state (lines 418-436)
- `export_markdown` - Generate PLAN.md (lines 438-518)

**Security Features:**
- Path validation (lines 66-104)
- File size limits (MAX_PLAN_FILE_SIZE: 10MB)
- Input validation (MAX_TITLE_LENGTH: 200, MAX_DESCRIPTION_LENGTH: 5000)
- Atomic file writes (lines 526-537)
- Symlink protection (lines 76-83)

**Strengths:**
- Comprehensive input validation
- Good error messages
- Dependency cycle detection before finalization

---

### 2. ⚠️ System Prompt (src/cli/mod.rs:10-76)

**Status:** FUNCTIONAL BUT INCONSISTENT WITH ACTUAL WORKFLOW

**Current System Prompt Instructions:**
```
Mandatory steps for plan creation:
1. IMMEDIATELY call plan tool with operation='create'
2. Call plan tool with operation='add_task' for each task
3. Call plan tool with operation='finalize'
4. AFTER finalization, call plan tool with operation='export_markdown'
5. ASK the user: "Would you like me to execute this plan..."
6. ONLY proceed with execution if user explicitly confirms
```

**Actual Execution Flow:**
```
1. LLM calls plan(operation='create')       ✅ Correct
2. LLM calls plan(operation='add_task')     ✅ Correct
3. LLM calls plan(operation='finalize')     ✅ Correct
   → Sets status to PendingApproval
   → TUI detects plan and switches to Plan Mode
4. USER presses Ctrl+A to approve           ⚠️ Different!
   → TUI exports markdown automatically
   → TUI starts execution
5. No user confirmation prompt              ❌ Missing!
```

**Problem:** The system prompt tells the LLM to:
- Call `export_markdown` after `finalize` (step 4)
- Ask user for confirmation (step 5)

But the actual workflow:
- TUI automatically switches to Plan mode after `finalize`
- User approves via Ctrl+A (no text prompt)
- Markdown export happens automatically on approval
- Execution starts immediately (no separate confirmation)

---

### 3. ⚠️ Markdown Export - DUAL IMPLEMENTATION ISSUE

**Status:** CONFLICTING IMPLEMENTATIONS

There are **TWO separate markdown export implementations**:

#### Implementation #1: Tool-based Export
**Location:** src/llm/tools/plan_tool.rs:438-518

**Behavior:**
- LLM calls `plan(operation='export_markdown')`
- Validates output path is within working directory
- **PREVENTS OVERWRITING** - Returns error if file exists (lines 499-502)
```rust
if output_path.exists() {
    return Err(ToolError::InvalidInput(
        format!("File '{}' already exists...")
    ));
}
```
- Writes to session-scoped plan file (.crustly_plan_{uuid}.json)

#### Implementation #2: TUI-based Export
**Location:** src/tui/app.rs:759-817

**Behavior:**
- Triggered automatically when user presses Ctrl+A
- **OVERWRITES WITHOUT CHECKING** (line 808)
```rust
// Write markdown file (overwrite if exists)
tokio::fs::write(&output_path, markdown)
    .await
```
- Always exports to "PLAN.md" in working directory

**Consequence:**
If the LLM follows the system prompt and calls `export_markdown` after `finalize`:
1. First export creates PLAN.md ✅
2. User presses Ctrl+A to approve
3. TUI tries to export again → overwrites PLAN.md ⚠️

If there are multiple plan iterations:
1. LLM exports PLAN.md (first plan)
2. User rejects, LLM creates new plan
3. LLM tries to export again → **ERROR: File exists** ❌

---

### 4. ✅ Approval Workflow (src/tui/app.rs)

**Status:** WORKING AS DESIGNED

**Plan Detection:** (lines 692-754)
- After agent response completes, calls `check_and_load_plan()`
- Loads most recent plan for current session
- Only loads if status is `PendingApproval`
- Automatically switches UI to Plan mode
- Tries database first, falls back to JSON file

**User Controls:**
- `Ctrl+A` - Approve plan (lines 391-405)
  1. Calls `plan.approve()` (sets status to Approved)
  2. Calls `plan.start_execution()` (sets status to InProgress)
  3. Exports markdown to PLAN.md
  4. Saves plan to database
  5. Switches back to Chat mode
  6. **Immediately starts executing tasks** (calls `execute_plan_tasks()`)

- `Ctrl+R` - Reject plan (lines 408-418)
  1. Calls `plan.reject()` (sets status to Rejected)
  2. Saves plan to database
  3. Clears plan from memory
  4. Switches back to Chat mode

- `Esc` - Cancel (lines 385-387)
  1. Returns to Chat mode
  2. Leaves plan in PendingApproval state

**Plan Display:** (src/tui/render.rs:738-861)
- Shows plan title, status, description
- Lists all tasks with icons, type, complexity
- Shows action bar with keybindings
- Supports scrolling for long plans

---

## Critical Issues

### Issue #1: System Prompt Workflow Mismatch

**Severity:** HIGH

**Description:**
The system prompt instructs the LLM to call `export_markdown` after `finalize` and ask for user confirmation. However, the actual workflow has the TUI handle both automatically when the user presses Ctrl+A.

**Impact:**
- LLM wastes API calls making unnecessary `export_markdown` calls
- Confusion about when markdown is actually exported
- No text-based user confirmation as promised in system prompt

**Evidence:**
- System prompt line 56: "AFTER finalization, call plan tool with operation='export_markdown'"
- System prompt line 65: "THEN ASK: 'The plan has been created and exported to LOGIN_PLAN.md. Would you like me to execute this plan...'"
- Actual: app.rs:397 exports markdown in `handle_plan_key()` on Ctrl+A

**Recommended Fix:**
Update system prompt to match actual workflow:
```
Mandatory steps for plan creation:
1. IMMEDIATELY call plan tool with operation='create'
2. Call plan tool with operation='add_task' for each task
3. Call plan tool with operation='finalize'
4. INFORM user: "Plan finalized! Review it in Plan Mode. Press Ctrl+A to approve and execute, Ctrl+R to reject, or Esc to cancel."
```

### Issue #2: Dual Markdown Export

**Severity:** MEDIUM

**Description:**
Two separate markdown export implementations with different behaviors (one prevents overwrite, one allows it).

**Impact:**
- File exists errors if LLM tries to export multiple times
- Confusion about which implementation is authoritative
- Code duplication and maintenance burden

**Evidence:**
- plan_tool.rs:499-502 - Prevents overwrite
- app.rs:808 - Overwrites without checking

**Recommended Fix:**
**Option A:** Remove `export_markdown` operation from plan tool entirely
- Markdown export only happens on user approval (Ctrl+A)
- Update system prompt to remove step 4
- Simpler, more predictable

**Option B:** Make TUI use the tool's export function
- Refactor to have single source of truth
- More complex but maintains tool API

**Recommendation:** Option A - Remove tool operation

### Issue #3: Missing User Confirmation

**Severity:** MEDIUM

**Description:**
System prompt promises text-based confirmation ("Would you like me to execute this plan?") but execution starts immediately on Ctrl+A without further prompting.

**Impact:**
- User expectation mismatch
- No way to approve plan without executing it
- Less control over execution timing

**Evidence:**
- System prompt line 65: "Would you like me to execute this plan and create the project files?"
- app.rs:403 - Immediately calls `execute_plan_tasks()` after approval

**Recommended Fix:**
**Option A:** Add confirmation step after Ctrl+A
- After approval, return to Chat mode
- LLM asks "Plan approved! Ready to execute?"
- User confirms via message
- More control but extra steps

**Option B:** Update system prompt to match behavior
- Document that Ctrl+A means "approve AND execute"
- Add separate "review only" mode if needed
- Simpler, matches current behavior

**Recommendation:** Option B - Update documentation

---

## Detailed Component Analysis

### Database Layer
**Location:** src/db/repository/plan.rs, migrations/20251111000001_add_plans.sql

**Status:** ✅ WORKING

**Features:**
- Full CRUD operations
- Session isolation
- Cascade deletes
- 7 plan statuses: Draft, PendingApproval, Approved, Rejected, InProgress, Completed, Cancelled
- Task dependency serialization (JSON)
- Timestamps for created_at, updated_at, approved_at

**Test Coverage:**
- Integration tests in tests/plan_mode_integration_test.rs
- State transition tests
- Dependency validation
- Session isolation

### Data Structures
**Location:** src/tui/plan.rs

**Status:** ✅ EXCELLENT

**PlanDocument Features:**
- UUID-based IDs for plans and tasks
- Topological sort for dependency ordering (Kahn's algorithm)
- Circular dependency detection
- Progress calculation
- Task status tracking

**Task Management:**
- 10 task types (Research, Edit, Create, Delete, Test, Refactor, Documentation, Configuration, Build, Other)
- 6 task statuses (Pending, InProgress, Completed, Skipped, Failed, Blocked)
- Complexity ratings (1-5 stars)
- Dependency tracking via UUID references

**Strong Points:**
- Proper validation before finalization
- Clear separation of concerns
- Comprehensive test coverage (src/tui/plan_tests.rs)

### Security
**Location:** src/llm/tools/plan_tool.rs:66-133

**Status:** ✅ ROBUST

**Protections:**
- Path traversal prevention (lines 69-73)
- Symlink attack prevention (lines 76-83)
- Filename pattern validation (lines 86-95)
- UUID validation (lines 98-101)
- File size limits (line 229)
- String length limits (lines 115-133)
- Atomic writes to prevent corruption (lines 526-537)

---

## Recommendations

### Priority 1: Fix System Prompt (IMMEDIATE)

**Action:** Update src/cli/mod.rs lines 46-65

**Old:**
```rust
Mandatory steps for plan creation:
1. IMMEDIATELY call plan tool with operation='create'
2. Call plan tool with operation='add_task' for each task
3. Call plan tool with operation='finalize'
4. AFTER finalization, call plan tool with operation='export_markdown'
5. ASK the user: "Would you like me to execute this plan..."
6. ONLY proceed with execution if user explicitly confirms
```

**New:**
```rust
Mandatory steps for plan creation:
1. IMMEDIATELY call plan tool with operation='create'
2. Call plan tool with operation='add_task' for each task (call multiple times)
   - IMPORTANT: The 'description' field MUST contain detailed implementation steps
3. Call plan tool with operation='finalize' to present the plan for user approval
4. INFORM user: "✅ Plan finalized! The plan is now displayed in Plan Mode.

   Press Ctrl+A to approve and execute
   Press Ctrl+R to reject and revise
   Press Esc to cancel

   The plan will be automatically exported to PLAN.md when you approve it."
5. WAIT for user to approve via Ctrl+A before execution begins

NOTE: Do NOT call export_markdown - this happens automatically when the user approves.
```

### Priority 2: Remove Duplicate Export (MEDIUM)

**Action:** Deprecate or remove `export_markdown` operation from plan tool

**Steps:**
1. Remove `ExportMarkdown` variant from `PlanOperation` enum (plan_tool.rs:54-58)
2. Remove export logic from match statement (plan_tool.rs:438-518)
3. Update input_schema to remove from enum (plan_tool.rs:152)
4. Keep TUI-based export as single source of truth
5. Update any documentation

**Alternative:** Add deprecation warning if removal breaks compatibility

### Priority 3: Add Plan Status Messaging (LOW)

**Action:** Add visual feedback when plan is finalized

**Option A:** Show system message in chat
```rust
// After plan is finalized and loaded in TUI
self.messages.push(DisplayMessage {
    role: "system".to_string(),
    content: "📋 Plan created and ready for review! Switch to Plan Mode (or it will auto-display) to approve or reject.".to_string(),
    ...
});
```

**Option B:** Add banner in Chat mode when plan is pending
```rust
// In render_chat_messages, show banner at top:
if app.current_plan.is_some() {
    "⚠️  Plan waiting for approval - Review in Plan Mode (Ctrl+P)"
}
```

### Priority 4: Document Actual Workflow (LOW)

**Action:** Create user-facing documentation

**File:** docs/PLAN_MODE_USER_GUIDE.md

**Contents:**
- How to request a plan
- What happens when plan is finalized
- How to review and approve
- What Ctrl+A, Ctrl+R, Esc do
- Where PLAN.md is exported
- How to monitor execution

---

## Testing Recommendations

### Unit Tests Needed
1. System prompt parsing test
2. Markdown export duplicate detection test
3. Plan finalization → mode switch test

### Integration Tests Needed
1. End-to-end plan creation + approval flow
2. Plan rejection + recreation flow
3. Multiple plans in same session
4. Plan approval with markdown export verification

### Manual Testing Checklist
- [ ] Create plan, verify it appears in Plan Mode
- [ ] Approve plan, verify PLAN.md is created
- [ ] Reject plan, create new one, verify no file conflict
- [ ] Cancel plan with Esc, verify state
- [ ] Create plan, exit app, restart, verify plan persists
- [ ] Approve plan, verify execution starts
- [ ] Check database has correct status transitions

---

## Conclusion

The plan mode implementation is **functionally complete** with excellent security, data structures, and database persistence. The primary issues are **workflow documentation mismatches** rather than code bugs.

**Critical Actions:**
1. ✅ Update system prompt to match actual workflow (removes confusion)
2. ⚠️ Remove or deprecate duplicate export_markdown tool operation (reduces errors)
3. ℹ️ Document actual user workflow (improves UX)

**Overall Assessment:** 8/10
- Deduct 1 point for workflow mismatch
- Deduct 1 point for duplicate export implementation
- Excellent architecture, security, and core functionality

---

**Next Steps:**
1. Review and approve recommended system prompt changes
2. Decide on export_markdown deprecation strategy
3. Add integration tests for complete workflow
4. Create user documentation

**Estimated Fix Time:**
- System prompt update: 15 minutes
- Remove duplicate export: 30 minutes
- Testing: 1 hour
- Documentation: 1 hour
**Total:** ~2.5 hours
