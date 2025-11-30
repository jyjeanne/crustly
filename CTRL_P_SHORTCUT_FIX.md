# Ctrl+P Shortcut Fix - Plan Viewing Issue

**Date:** 2025-11-23
**Issue:** Ctrl+P shortcut not displaying Draft plans
**Status:** ✅ Fixed

---

## Problem Description

### Symptom
When creating a new plan, pressing **Ctrl+P** shows:
```
❌ "No plan available. Create a plan first."
```

Even though a plan was just created with tasks.

### Root Cause

The `check_and_load_plan()` method used by Ctrl+P handler **only loaded plans with status `PendingApproval`**:

```rust
// Lines 791 and 854 in app.rs
if plan.status == crate::tui::plan::PlanStatus::PendingApproval {
    // Load plan
} else {
    // Skip plan
}
```

### Why This Failed

**Plan Workflow:**
1. LLM creates plan → Status: **Draft** ❌ (not loaded by Ctrl+P)
2. LLM adds tasks → Status: **Draft** ❌ (not loaded by Ctrl+P)
3. LLM finalizes → Status: **PendingApproval** ✅ (loaded by Ctrl+P)

**Problem:** Users wanted to view plans at ANY stage (Draft, PendingApproval, InProgress, etc.), but Ctrl+P only worked after finalization.

---

## Solution Implemented

### Created Separate Method for Manual Viewing

**Location:** `src/tui/app.rs:768-838`

**New Method:** `load_plan_for_viewing()`

```rust
/// Load plan for manual viewing (Ctrl+P)
/// Loads ANY plan (Draft, PendingApproval, etc.) for viewing
async fn load_plan_for_viewing(&mut self) -> Result<()> {
    // Get session ID
    let session_id = self.current_session.as_ref()?.id;

    // Try loading from database first
    match self.plan_service.get_most_recent_plan(session_id).await {
        Ok(Some(plan)) => {
            tracing::info!(
                "✅ Loaded plan from database: '{}' ({:?}, {} tasks)",
                plan.title,
                plan.status,
                plan.tasks.len()
            );
            self.current_plan = Some(plan);
            return Ok(());
        }
        // ... fallback to JSON ...
    }

    Ok(())
}
```

**Key Difference:**
- **Old:** Only loads `PendingApproval` plans
- **New:** Loads ANY plan (Draft, PendingApproval, InProgress, Completed, etc.)

### Updated Ctrl+P Handler

**Location:** `src/tui/app.rs:302-304`

**Before:**
```rust
AppMode::Chat => {
    // Load plan before switching to Plan mode
    self.check_and_load_plan().await?;  // ❌ Only PendingApproval
    // ...
}
```

**After:**
```rust
AppMode::Chat => {
    // Try to load any plan (not just PendingApproval)
    self.load_plan_for_viewing().await?;  // ✅ ANY status
    // ...
}
```

### Kept Original Method for Automatic Loading

**Method:** `check_and_load_plan()`
**Purpose:** Automatic notification after LLM finalizes plan
**Behavior:** Still only loads `PendingApproval` plans (shows notification)

This separation ensures:
- ✅ Manual viewing (Ctrl+P) works for ALL plans
- ✅ Automatic notifications only for finalized plans
- ✅ No duplicate notifications for Draft plans

---

## Behavior Changes

### Before Fix

| Plan Status | Ctrl+P | Auto Notification |
|-------------|--------|-------------------|
| Draft | ❌ "No plan" | ❌ None |
| PendingApproval | ✅ Loads | ✅ Shows |
| InProgress | ❌ "No plan" | ❌ None |
| Completed | ❌ "No plan" | ❌ None |

### After Fix

| Plan Status | Ctrl+P | Auto Notification |
|-------------|--------|-------------------|
| Draft | ✅ Loads | ❌ None (correct) |
| PendingApproval | ✅ Loads | ✅ Shows (correct) |
| InProgress | ✅ Loads | ❌ None (correct) |
| Completed | ✅ Loads | ❌ None (correct) |

---

## Use Cases Now Supported

### 1. View Plan While Building (Draft)
```
User: Create a plan for authentication
LLM: plan(operation='create', ...)
LLM: plan(operation='add_task', ...)
User: [Ctrl+P] → ✅ See plan with current tasks
```

### 2. View Plan Before Approval (PendingApproval)
```
LLM: plan(operation='finalize')
System: "✅ Plan ready! Press Ctrl+P to review"
User: [Ctrl+P] → ✅ See full plan
User: [Ctrl+A] → Approve
```

### 3. View Plan During Execution (InProgress)
```
User: [Ctrl+A] Approves plan
System: Executing tasks...
User: [Ctrl+P] → ✅ See progress
```

### 4. View Completed Plans
```
User: [Ctrl+P] → ✅ See completed plan with results
```

---

## Implementation Details

### Method Comparison

| Aspect | `load_plan_for_viewing()` | `check_and_load_plan()` |
|--------|---------------------------|-------------------------|
| **Purpose** | Manual viewing (Ctrl+P) | Auto notification |
| **Status Filter** | None (loads ANY) | PendingApproval only |
| **Notification** | None | Adds chat message |
| **When Called** | User presses Ctrl+P | After agent response |
| **Use Case** | User wants to see plan | LLM finalized plan |

### Logging Added

**Success Case:**
```
✅ Loaded plan from database: 'Auth Plan' (Draft, 3 tasks)
```

**Not Found:**
```
No plan file found
```

This helps debug issues with plan loading.

---

## Testing

### Verification Steps

✅ **Code Quality:**
```bash
cargo fmt       # Formatted successfully
cargo clippy    # No warnings (with -D warnings)
cargo check     # Compiled successfully
cargo test      # All tests passing
```

✅ **Manual Testing Scenarios:**

1. **Draft Plan Viewing**
   - Create plan with tasks (Draft)
   - Press Ctrl+P
   - Expected: ✅ Plan displays
   - Actual: ✅ Works!

2. **Finalized Plan Viewing**
   - Finalize plan (PendingApproval)
   - Wait for notification
   - Press Ctrl+P
   - Expected: ✅ Plan displays
   - Actual: ✅ Works!

3. **No Plan**
   - New session, no plan
   - Press Ctrl+P
   - Expected: ❌ "No plan available"
   - Actual: ✅ Correct error

4. **In-Progress Plan**
   - Approve and start execution
   - Press Ctrl+P during execution
   - Expected: ✅ Shows progress
   - Actual: ✅ Works!

---

## Migration Notes

### Backward Compatibility

✅ **Fully Backward Compatible**
- Automatic notifications still work (PendingApproval only)
- JSON file fallback still works
- Database migration still works
- No breaking changes

### No Data Migration Needed

All changes are in the UI layer - no database or file format changes.

---

## Code Changes Summary

**Files Modified:** 1
- `src/tui/app.rs` (+70 lines for new method, +1 line for handler change)

**New Method:** `load_plan_for_viewing()` - Loads ANY plan for manual viewing

**Modified Handler:** Ctrl+P now calls `load_plan_for_viewing()` instead of `check_and_load_plan()`

**Tests:** All passing (1/1 app tests)

---

## Benefits

### For Users

1. **View Plans Anytime**
   - See Draft plans while building
   - Check progress during execution
   - Review completed plans

2. **Better Workflow**
   - Don't wait for finalization to see plan
   - Verify tasks as LLM adds them
   - Monitor execution progress live

3. **No Confusion**
   - Ctrl+P always works (if plan exists)
   - Clear error when no plan
   - Consistent behavior

### For Developers

1. **Clear Separation**
   - Manual viewing: `load_plan_for_viewing()`
   - Auto notification: `check_and_load_plan()`
   - Single responsibility

2. **Better Logging**
   - Know when plans are loaded
   - See plan status and task count
   - Easier debugging

3. **Maintainable Code**
   - Two focused methods vs one complex one
   - Clear comments explain purpose
   - Easy to extend

---

## Future Enhancements

### Plan History (Ctrl+H)
Could add a history viewer showing all plans:
```
User: [Ctrl+H]
Shows:
  1. ✅ Auth Plan (Completed, 5/5 tasks)
  2. 📋 Dashboard (InProgress, 2/8 tasks)
  3. ❌ Login (Rejected)
```

### Plan Status Indicator
Show plan status in status bar:
```
Chat Mode | Plan: "Auth Plan" (Draft, 3 tasks) | Ctrl+P to view
```

### Quick Plan Summary
Show brief summary in chat when plan updates:
```
💡 Plan updated: Added task "Setup database" (4 tasks total)
```

---

## Troubleshooting

### Ctrl+P Still Shows "No plan"

**Check:**
1. Plan exists in database or `.crustly_plan_*.json` file
2. Plan belongs to current session
3. Logs show plan loading attempt

**Debug:**
```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Check for these logs:
# "Loading plan for viewing (session: ...)"
# "Loaded plan from database: ..." or "No plan file found"
```

### Plan Shows But Is Empty

**Cause:** Plan has no tasks yet
**Solution:** This is expected for newly created plans. Wait for LLM to add tasks or add manually.

### Plan Shows Wrong Status

**Cause:** Cached plan in memory
**Solution:** Switch to Chat mode (Esc) then back to Plan mode (Ctrl+P) to reload

---

## Conclusion

The Ctrl+P shortcut now works correctly for viewing plans at **any stage**:

✅ Draft plans (being built)
✅ PendingApproval plans (ready for review)
✅ InProgress plans (executing)
✅ Completed plans (finished)

The fix provides a better user experience while maintaining backward compatibility and code quality.

---

*Fixed: 2025-11-23*
*Verified: cargo fmt, clippy, check, test all passing*
*Status: Production Ready ✅*
