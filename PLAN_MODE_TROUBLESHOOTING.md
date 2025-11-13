# Plan Mode Troubleshooting Guide

## Your Issue

You see the message:
```
Plan finalized! The plan is now displayed in Plan Mode.
Press Ctrl+A to approve and execute, Ctrl+R to reject, or Esc to cancel.
```

But the keyboard shortcuts don't work.

## Root Cause

**The UI didn't switch to Plan Mode**, even though the plan was created.

### Why This Happens

1. **Plan file created** ✓ - You confirmed `.crustly_plan_*.json` exists
2. **Auto-load failed** ❌ - The UI should detect this file and switch to `[PLAN]` mode
3. **Still in Chat mode** - You're in `[CHAT]` mode, where Ctrl+A/R/Esc don't work for plans

### Possible Reasons

1. **Working directory mismatch** - Plan saved in one directory, Crustly looking in another
2. **Database/file sync issue** - Plan in file but not loaded into UI
3. **Session ID mismatch** - Wrong session ID in filename

## Quick Workarounds

### Workaround #1: Manual Mode Switch

While in Crustly:
1. Press **`Ctrl+P`** to manually toggle to Plan Mode
2. If plan exists, it should display
3. Then Ctrl+A, Ctrl+R, Esc should work

### Workaround #2: Check Status Bar

Look at the bottom of the screen:
- If it says `[CHAT]` - you're in Chat mode (shortcuts won't work)
- If it says `[PLAN]` - you're in Plan mode (shortcuts work)

## Proper Testing

I've created a test script for you: `/home/user/crustly/test_plan_mode.sh`

### Run the test:

```bash
cd /home/user/crustly
./test_plan_mode.sh
```

This will:
1. Create a clean test environment
2. Start Crustly with debug logging
3. Guide you through creating a plan
4. Show you exactly what's happening

### What to Look For

After creating a plan, check:

1. **Status bar** - Did it switch from `[CHAT]` to `[PLAN]`?
2. **Screen content** - Do you see the plan with tasks listed?
3. **Log file** - Look for `✅ Loading plan from ...` messages

## Debugging

If it still doesn't work:

### 1. Check Logs

After running test script:
```bash
cd /tmp/crustly_plan_test_*
grep -i "plan\|mode" test.log | tail -20
```

Look for:
- `"Checking for pending plan"`
- `"Found plan in database"` or `"Looking for plan file at"`
- `"✅ Loading plan from..."`
- `"Parsed plan: id=..."`

### 2. Check Files Created

```bash
ls -la .crustly_plan_*.json
cat .crustly_plan_*.json | jq '.status, .tasks | length'
```

Expected status: `"PendingApproval"`

### 3. Verify Working Directory

In Crustly, look at the header line showing the working directory. The plan file should be in that directory.

## The Fix I Applied

I fixed the bug where:
- **Before**: Plan saved to `working_directory`, but loaded from `current_dir` (mismatch!)
- **After**: Both save and load use `working_directory` (consistent!)

**Commit**: `5ce0209` - "Fix plan mode activation and add diagnostic logging"

## If Issue Persists

If the test script still shows the problem, I need more info:

1. **Screenshot** of Crustly showing:
   - The "Plan finalized!" message
   - The status bar (showing [CHAT] or [PLAN])
   - The working directory in the header

2. **Log output** from test.log showing:
   ```bash
   grep "plan\|Plan\|PLAN" test.log
   ```

3. **File listing**:
   ```bash
   ls -la .crustly_plan_*
   cat .crustly_plan_*.json | head -30
   ```

## Expected Behavior

### Correct Flow:
1. User: "create a plan to..."
2. LLM: Calls plan tool (create, add_task, finalize)
3. Plan tool: Saves `.crustly_plan_{session_id}.json` with status="PendingApproval"
4. TUI: Detects file, loads it, switches to Plan Mode ✅
5. Screen: Shows `[PLAN]` mode with task list
6. User: Presses Ctrl+A (approve), Ctrl+R (reject), or Esc (cancel)
7. TUI: Responds to keyboard shortcut

### What's Broken (before fix):
1-3. Same as above
4. TUI: Looks in wrong directory, doesn't find file ❌
5. Screen: Stays in `[CHAT]` mode
6. User: Presses Ctrl+A, nothing happens (wrong mode)

### What Should Work Now (after fix):
- Both save and load use same `working_directory`
- Auto-detection should work
- Mode should auto-switch to `[PLAN]`

## Next Steps

1. Run `./test_plan_mode.sh`
2. Follow the on-screen instructions
3. Check if mode switches to `[PLAN]`
4. If yes - problem solved! ✅
5. If no - share logs/screenshots for further debugging

---

**Created**: 2025-11-11
**Related Commits**:
- `5ce0209` - Fix plan mode activation
- `d43b084` - Implement recommended plan mode fixes
- `1c070cd` - Add comprehensive plan mode execution review
