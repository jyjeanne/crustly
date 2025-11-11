# Plan Mode User Guide

**Version:** 1.0
**Last Updated:** 2025-11-11

---

## Overview

Plan Mode is a powerful feature in Crustly that helps you break down complex development tasks into structured, reviewable plans before execution. This ensures you have full control and visibility over what Crustly will do.

### Key Benefits

- **🔍 Full Visibility** - See all tasks before they execute
- **✅ User Control** - Approve, reject, or cancel plans
- **📝 Documentation** - Auto-generated PLAN.md with full details
- **🔗 Dependencies** - Tasks execute in proper order
- **⭐ Complexity Tracking** - 1-5 star ratings for each task

---

## When to Use Plan Mode

Plan Mode is ideal for:

- **Multi-step features** - Adding authentication, implementing APIs, etc.
- **Major refactoring** - Restructuring code across multiple files
- **Complex workflows** - Tasks with dependencies (Task B requires Task A)
- **New projects** - Setting up project structure and scaffolding
- **Learning** - Understanding implementation steps before they run

### Simple vs. Plan Mode

| Scenario | Mode | Reason |
|----------|------|--------|
| "Fix this typo" | Chat | Single, simple change |
| "Add a login page" | **Plan** | Multiple files, multiple steps |
| "Update this function" | Chat | Single file edit |
| "Build a REST API" | **Plan** | Complex, multi-step task |

---

## How to Create a Plan

### Step 1: Request a Plan

In Chat Mode, simply ask Crustly to create a plan:

```
"Create a plan to implement user authentication"
"Make a plan for adding a dark mode toggle"
"Plan out implementing a shopping cart feature"
```

**Keywords that trigger Plan Mode:**
- "create a plan"
- "make a plan"
- "plan to implement"
- "plan for adding"

### Step 2: Crustly Creates the Plan

Crustly will:

1. Call the `plan` tool to create a new plan
2. Add tasks with details:
   - Task title and description
   - Task type (Research, Edit, Create, Test, etc.)
   - Complexity (1-5 stars)
   - Dependencies (which tasks must complete first)
3. Finalize the plan for your review

**Example Output:**
```
✓ Created new plan: 'Implement User Authentication'
✓ Added task #1: 'Create User Model'
✓ Added task #2: 'Implement Login API' (depends on Task 1)
✓ Added task #3: 'Create Login UI' (depends on Task 2)
✓ Plan finalized and ready for review!
```

### Step 3: Review in Plan Mode

After finalization, Crustly will **automatically switch to Plan Mode** and display the plan for your review.

**What You'll See:**
- Plan title and description
- All tasks with their details
- Task types and complexity ratings
- Dependencies between tasks
- Action buttons (Ctrl+A, Ctrl+R, Esc)

---

## Plan Mode Interface

### Plan Display

```
┌─────────────────────────────────────────┐
│          📋 PLAN MODE                   │
├─────────────────────────────────────────┤
│                                         │
│ 📋 Implement User Authentication        │
│                                         │
│ Status: Pending Approval                │
│                                         │
│ 📝 Description:                         │
│ Build authentication system with JWT    │
│                                         │
│ 📋 Tasks (3):                           │
│                                         │
│ ⏸️ 1. Create User Model                 │
│    Type: Create | Complexity: ★★☆☆☆    │
│                                         │
│ ⏸️ 2. Implement Login API               │
│    Type: Edit | Complexity: ★★★★☆      │
│    Dependencies: Task 1                 │
│                                         │
│ ⏸️ 3. Create Login UI                   │
│    Type: Create | Complexity: ★★★☆☆    │
│    Dependencies: Task 2                 │
│                                         │
├─────────────────────────────────────────┤
│ [Ctrl+A] Approve  [Ctrl+R] Reject       │
│ [Esc] Cancel                            │
└─────────────────────────────────────────┘
```

### Status Bar

At the bottom of the screen, you'll see:
```
[PLAN] Ready │ Ctrl+H: Help │ ...
```

The `[PLAN]` indicator shows you're in Plan Mode.

---

## Reviewing and Approving Plans

### Keyboard Controls

| Key | Action | Description |
|-----|--------|-------------|
| **Ctrl+A** | Approve | Approve plan and start execution immediately |
| **Ctrl+R** | Reject | Reject plan and return to Chat Mode to revise |
| **Esc** | Cancel | Return to Chat Mode without approving/rejecting |
| **↑/↓** | Scroll | Navigate through long task lists |
| **Page Up/Down** | Fast Scroll | Jump through tasks quickly |
| **Ctrl+P** | Toggle | Switch between Plan Mode and Chat Mode |

### What to Check Before Approving

Before pressing **Ctrl+A**, review:

1. **Task Accuracy** - Do the tasks match what you want?
2. **File Paths** - Are the correct files being created/modified?
3. **Dependencies** - Are tasks in the right order?
4. **Complexity** - Do you have time for this work?
5. **Risks** - Any concerns about the approach?

### Example Review Process

**Good Plan ✅**
```
Task 1: Create database schema (no dependencies)
Task 2: Create API endpoints (depends on Task 1)
Task 3: Add UI components (depends on Task 2)
```
Clear dependency chain, proper order.

**Needs Revision ⚠️**
```
Task 1: Create UI components
Task 2: Set up API
Task 3: Design database schema
```
Backwards! Reject and ask: "Revise the plan to create the database first"

---

## What Happens When You Approve

When you press **Ctrl+A**:

1. **Plan Status Changes**
   - From: `PendingApproval`
   - To: `Approved` → `InProgress`

2. **Markdown Export**
   - Automatically creates `PLAN.md` in your working directory
   - Contains all task details and implementation steps
   - Keep this for reference during execution

3. **Mode Switch**
   - Returns to Chat Mode
   - Execution begins immediately

4. **Task Execution**
   - Tasks execute sequentially in dependency order
   - Each task runs to completion before the next starts
   - Progress tracked in database

### Example Execution Flow

```
[Plan Approved]
  ↓
Exporting to PLAN.md... ✓
  ↓
Saving to database... ✓
  ↓
Switching to Chat Mode...
  ↓
Executing Task 1... ▶️
  → Creating src/models/user.js... ✓
  → Task 1 completed ✅
  ↓
Executing Task 2... ▶️
  → Implementing /api/login... ✓
  → Task 2 completed ✅
  ↓
Executing Task 3... ▶️
  → Creating UI components... ✓
  → Task 3 completed ✅
  ↓
[All Tasks Complete!] 🎉
```

---

## What Happens When You Reject

When you press **Ctrl+R**:

1. **Plan Status Changes**
   - From: `PendingApproval`
   - To: `Rejected`

2. **Plan Saved**
   - Rejected plan saved to database for history
   - Plan removed from memory

3. **Return to Chat**
   - Back to Chat Mode
   - Ready for you to provide feedback

### How to Revise a Rejected Plan

After rejecting, tell Crustly what needs to change:

```
"The plan needs to include error handling"
"Add a task for writing unit tests"
"Task 2 should happen before Task 1"
"The complexity seems too high, can we simplify?"
```

Crustly will create a new plan incorporating your feedback.

---

## Canceling a Plan

When you press **Esc**:

- **No status change** - Plan stays as `PendingApproval`
- **Stays in memory** - You can return to it later
- **Returns to Chat** - Back to Chat Mode

### When to Cancel vs. Reject

| Situation | Action | Why |
|-----------|--------|-----|
| Need to check something first | **Esc (Cancel)** | Plan still available to review later |
| Plan is wrong/incomplete | **Ctrl+R (Reject)** | Start over with new plan |
| Accidentally entered Plan Mode | **Esc (Cancel)** | Quick exit |
| Want to make changes | **Ctrl+R (Reject)** | Clear plan and revise |

---

## Working with PLAN.md

### File Location

After approval, `PLAN.md` is created in your working directory:
```
/your/project/directory/PLAN.md
```

### File Contents

```markdown
# Implement User Authentication

Build authentication system with JWT tokens

## Context

Existing app needs user login. Backend API ready.

## Risks & Considerations

- JWT secret management
- Password hashing security
- Session timeout handling

## Tasks

### Task 1: Create User Model

**Type:** Create | **Complexity:** ★★☆☆☆

**Implementation Steps:**

1. Create src/models/user.js file
2. Define User schema with email, password fields
3. Add bcrypt for password hashing
4. Export model

---

### Task 2: Implement Login API

**Type:** Edit | **Complexity:** ★★★★☆

**Dependencies:** Task(s) 1

**Implementation Steps:**

1. Create POST /api/login endpoint
2. Validate email and password
3. Generate JWT token on success
4. Return token in response
5. Handle errors gracefully

---

*Plan created: 2025-11-11 10:30:00*
*Last updated: 2025-11-11 10:32:15*
```

### Using PLAN.md

- **Reference** - Check implementation details during execution
- **Documentation** - Project documentation for future reference
- **Review** - Share with team for code review
- **Track** - Keep history of what was implemented

### Important Notes

⚠️ **Overwrite Behavior**: If you approve multiple plans in the same session, `PLAN.md` will be overwritten. Rename important plans before approving new ones:

```bash
mv PLAN.md PLAN_user_auth.md
```

---

## Plan Status Lifecycle

Plans go through multiple states:

```
Draft → PendingApproval → Approved → InProgress → Completed
                    ↓
                Rejected
                    ↓
                Cancelled
```

### Status Meanings

| Status | Description | User Action Available |
|--------|-------------|----------------------|
| **Draft** | Plan being created | Wait for finalization |
| **PendingApproval** | Ready for review | Approve, Reject, or Cancel |
| **Approved** | User approved | Auto-transitions to InProgress |
| **InProgress** | Tasks executing | Monitor progress |
| **Completed** | All tasks done | Review results |
| **Rejected** | User rejected | Create new plan |
| **Cancelled** | User cancelled | Resume or create new plan |

---

## Task Types

Plans can include 10 different task types:

| Type | Icon | Use Case |
|------|------|----------|
| **Research** | 🔍 | Exploring codebase, investigating APIs |
| **Edit** | ✏️ | Modifying existing files |
| **Create** | ➕ | Creating new files/components |
| **Delete** | 🗑️ | Removing files/code |
| **Test** | 🧪 | Writing unit/integration tests |
| **Refactor** | 🔄 | Restructuring code without changing behavior |
| **Documentation** | 📚 | Writing docs, comments, README |
| **Configuration** | ⚙️ | Updating config files, environment variables |
| **Build** | 🔨 | Build scripts, compilation, deployment |
| **Other** | 📋 | Anything else |

---

## Task Status Icons

During execution, you'll see these icons:

| Icon | Status | Meaning |
|------|--------|---------|
| ⏸️ | Pending | Not started yet |
| ▶️ | InProgress | Currently executing |
| ✅ | Completed | Successfully finished |
| ⏭️ | Skipped | Skipped (dependency failed or user choice) |
| ❌ | Failed | Execution failed |
| 🚫 | Blocked | Blocked by failed dependency |

---

## Tips & Best Practices

### For Users

1. **Review Thoroughly** - Take time to read all tasks before approving
2. **Check Dependencies** - Ensure tasks are in logical order
3. **Save Important Plans** - Rename PLAN.md files you want to keep
4. **Start Small** - Try simple plans first to get familiar
5. **Provide Feedback** - Reject plans that aren't quite right and explain why

### For Complex Projects

1. **Break It Down** - For very large features, create multiple smaller plans
2. **Test Incrementally** - Approve and test one plan before moving to the next
3. **Review Generated Code** - After execution, review the code before committing
4. **Keep Context** - Provide good context when requesting plans

### Example: Good Plan Request

❌ **Too Vague:**
```
"Create a login system"
```

✅ **Better:**
```
"Create a plan to implement user authentication with:
- Email/password login
- JWT tokens for sessions
- Password hashing with bcrypt
- Login API endpoint
- React login form component
- Error handling for invalid credentials

Context: Existing app uses Express backend and React frontend.
Database already has users table."
```

---

## Troubleshooting

### Plan Doesn't Appear

**Problem:** Plan finalized but not showing in Plan Mode

**Solution:**
1. Check status bar shows `[PLAN]`
2. Press `Ctrl+P` to manually switch to Plan Mode
3. Check for errors in status bar

### Can't Approve Plan

**Problem:** Ctrl+A doesn't work

**Solution:**
1. Ensure you're in Plan Mode (`[PLAN]` in status bar)
2. Check plan status is `PendingApproval`
3. Try `Esc` then `Ctrl+P` to re-enter Plan Mode

### PLAN.md Missing

**Problem:** Approved plan but no PLAN.md file

**Solution:**
1. Check your working directory (shown in header)
2. Look for file permission issues
3. Check error messages in status bar

### Want to See Plan Again

**Problem:** Accidentally pressed Esc, want to review plan

**Solution:**
Press `Ctrl+P` to switch back to Plan Mode. Plan is still in memory until you reject it or approve/complete execution.

---

## Advanced Features

### Plan Persistence

Plans are stored in both:
- **Database** (primary) - SQLite in your home directory
- **JSON files** (backup) - `.crustly_plan_{session_id}.json`

This means:
- Plans survive app restarts
- You can resume pending plans in new sessions
- Full history of approved/rejected plans

### Session Isolation

Each chat session has its own plan context:
- Creating a new session starts fresh
- Switching sessions loads that session's plan
- No cross-session plan interference

### Dependency Validation

Crustly validates dependencies before finalization:
- **Circular dependencies** - Rejected (Task A depends on B, B depends on A)
- **Invalid references** - Rejected (Task depends on non-existent task)
- **Order validation** - Tasks can only depend on earlier tasks

---

## Frequently Asked Questions

### Q: Can I edit a plan after it's created?

**A:** Not directly. If you want changes, reject the plan (Ctrl+R) and ask Crustly to create a new one with your modifications.

### Q: Can I pause execution mid-plan?

**A:** No, once approved, tasks execute sequentially to completion. Design your plans with this in mind.

### Q: What if a task fails during execution?

**A:** Execution stops, the failed task is marked as Failed (❌), and dependent tasks are marked as Blocked (🚫). You'll see error messages in Chat Mode.

### Q: Can I have multiple plans at once?

**A:** No, only one plan can be pending approval per session. Complete or reject the current plan before creating a new one.

### Q: Will Crustly always use Plan Mode?

**A:** No, Plan Mode is only used when you explicitly request a plan or for complex multi-step tasks. Simple requests execute immediately in Chat Mode.

### Q: Can I use Plan Mode without the LLM?

**A:** The plan tool requires the LLM to structure tasks. However, you can manually create plans by calling the tool operations if you're building integrations.

---

## Keyboard Shortcuts Reference

### In Plan Mode

| Shortcut | Action |
|----------|--------|
| `Ctrl+A` | Approve and execute plan |
| `Ctrl+R` | Reject plan |
| `Esc` | Cancel/return to Chat |
| `↑` / `↓` | Scroll through tasks |
| `Page Up` / `Page Down` | Fast scroll |
| `Ctrl+P` | Toggle to Chat Mode |
| `Ctrl+H` | Show help |
| `Ctrl+C` | Quit Crustly |

### In Chat Mode (with pending plan)

| Shortcut | Action |
|----------|--------|
| `Ctrl+P` | Switch to Plan Mode to review |
| `Ctrl+Enter` | Submit message (doesn't affect plan) |

---

## Summary

**Plan Mode gives you:**
- ✅ Full control over complex tasks
- 📋 Clear breakdown of what will happen
- 🔍 Visibility before execution
- 📝 Auto-generated documentation
- 🎯 Dependency-aware task ordering

**Basic Workflow:**
1. Request: "Create a plan to..."
2. Review: Check tasks in Plan Mode
3. Decide: Ctrl+A (approve), Ctrl+R (reject), or Esc (cancel)
4. Execute: If approved, watch tasks complete
5. Document: Refer to PLAN.md for details

**Remember:**
- Plans execute immediately after approval
- PLAN.md is auto-generated on approval
- You can reject and revise as many times as needed
- Start with simple plans to learn the system

---

**Need Help?**
- Press `Ctrl+H` in Crustly for quick help
- Check the status bar for current mode and shortcuts
- Review example plans in the documentation
- Experiment with simple plans first

**Feedback?**
Report issues at: https://github.com/anthropics/claude-code/issues

---

*Last updated: 2025-11-11*
*Crustly Version: 0.2.0*
