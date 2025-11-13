Perfect! I've implemented the sequential task execution feature for Plan Mode. Here's the complete workflow:



📋 Plan Mode Complete Workflow

1\. Plan Creation (AI-driven)



User makes complex request

AI uses plan tool to create structured plan

AI adds tasks with dependencies using add\_task

AI finalizes with finalize operation

TUI loads plan and switches to Plan Mode

2\. User Review \& Approval



User reviews tasks, dependencies, complexity ratings

User approves with Ctrl+A or rejects with Ctrl+R

3\. Automatic Task Execution ✨ NEW



After approval, system automatically executes tasks sequentially

For each task in dependency order:

✓ Mark task as InProgress

✓ Send task description to AI agent

✓ Agent completes the task

✓ Mark task as Completed with timestamp

✓ Save state to .crustly\_plan.json

✓ Move to next task

When all tasks complete:

✓ Mark plan as Completed

✓ Show success message to user

Key Implementation Details

State Management:



Added executing\_plan: bool to track active execution

Tasks follow state flow: Pending → InProgress → Completed

Plan state persisted after each task update

Execution Engine:



execute\_plan\_tasks() - Starts execution after approval

execute\_next\_plan\_task() - Processes next task in dependency order

complete\_response() - Advances to next task after agent completes current one

Features:



Respects task dependencies via topological sort

Prevents execution if circular dependencies detected

Real-time progress tracking with timestamps

Automatic state persistence

All changes committed and pushed to claude/fix-clippy-errors-011CV172JVcJoom5yotQAjz6 ✅



give me a test scenario to manually test new plan mod, use cases



Here are comprehensive test scenarios for manually testing Plan Mode:



Test Scenario 1: Basic Plan Creation \& Execution

Objective: Test end-to-end workflow with simple tasks



Steps:



Start Crustly in TUI mode

Create a new chat session

Enter this request:

"I need to add a new feature: create a README.md file with project description, 

then add a license file (MIT), and finally update the package.json with new metadata"

Expected Behavior:



✓ AI creates a plan with 3 tasks

✓ TUI automatically switches to Plan Mode (Ctrl+P)

✓ Plan displays with:

Title (e.g., "Add Project Documentation")

3 tasks with descriptions

Complexity ratings (★)

Status: "Pending Approval"

✓ User sees action bar: \[Ctrl+A] Approve | \[Ctrl+R] Reject | \[Esc] Cancel

Approval Test: 4. Press Ctrl+A to approve 5. Observe sequential execution:



Task #1 marked "InProgress" → AI creates README.md → Completed ✓

Task #2 marked "InProgress" → AI creates LICENSE → Completed ✓

Task #3 marked "InProgress" → AI updates package.json → Completed ✓

See completion message: "✅ Plan completed successfully!"

Verification:



Check .crustly\_plan.json exists in working directory

Verify all 3 files were created

Confirm plan status is "Completed" in JSON

Test Scenario 2: Plan with Dependencies

Objective: Test topological sort and dependency management



Steps:



Enter this request:

"Help me refactor the authentication system:

1\. First, read and analyze the current auth.rs file

2\. Create a new auth\_service.rs based on the analysis

3\. Update main.rs to use the new service

4\. Finally, write tests for the auth service"

Expected Behavior:



✓ AI creates plan with 4 tasks

✓ Task dependencies are set:

Task 1: No dependencies

Task 2: Depends on Task 1

Task 3: Depends on Task 2

Task 4: Depends on Task 2

✓ Plan shows dependency info in UI

Execution Test: 2. Approve with Ctrl+A 3. Observe dependency order is respected:



Task 1 executes first (reads auth.rs)

Task 2 executes after Task 1 (creates service)

Task 3 \& 4 execute after Task 2 (can run in sequence)

Verification:



Tasks execute in correct order

No task starts before its dependencies complete

Check .crustly\_plan.json shows proper dependency IDs

Test Scenario 3: Plan Rejection \& Revision

Objective: Test rejection flow



Steps:



Enter: "Add comprehensive error handling to all API endpoints"

Wait for plan to appear

Press Ctrl+R to reject

Expected Behavior:



✓ Plan status changes to "Rejected"

✓ Returns to Chat mode

✓ Plan saved to .crustly\_plan.json as rejected

✓ No tasks execute

Revision Test: 4. Enter: "The previous plan was too broad. Create a simpler plan focusing only on the user endpoints" 5. AI should create a new plan with narrower scope 6. Approve this one with Ctrl+A



Test Scenario 4: Complex Multi-Step Workflow

Objective: Test realistic development scenario



Steps:



Enter:

"I need to add database migrations:

1\. Analyze the current schema in migrations/

2\. Create a new migration file for adding user\_profiles table

3\. Write the up migration SQL

4\. Write the down migration SQL

5\. Test the migration with cargo sqlx migrate run

6\. Document the migration in MIGRATIONS.md"

Expected Behavior:



✓ Plan with 6 tasks

✓ Mixed task types: Research, Create, Documentation, Test

✓ Complexity varies (1-5 stars)

✓ Dependencies properly set

Progress Tracking: 2. Approve and watch execution 3. Monitor:



Real-time status updates (Pending → InProgress → Completed)

Timestamps on completed tasks

Progress indicator (Task 3/6, etc.)

Test Scenario 5: Circular Dependency Detection

Objective: Test validation catches cycles



Manual Setup:



Create a plan JSON file with circular dependencies:

{

&nbsp; "id": "test-plan",

&nbsp; "title": "Test Circular",

&nbsp; "tasks": \[

&nbsp;   {

&nbsp;     "id": "task-a",

&nbsp;     "dependencies": \["task-b"]

&nbsp;   },

&nbsp;   {

&nbsp;     "id": "task-b", 

&nbsp;     "dependencies": \["task-c"]

&nbsp;   },

&nbsp;   {

&nbsp;     "id": "task-c",

&nbsp;     "dependencies": \["task-a"]

&nbsp;   }

&nbsp; ]

}

Try to finalize this plan

Expected Behavior:



✓ Validation fails with error: "Circular dependency detected"

✓ Plan cannot be finalized

✓ Clear error message shown to user

Test Scenario 6: Plan Mode Navigation

Objective: Test UI controls and keyboard shortcuts



Steps:



Create any plan

While in Plan Mode, test:

↑/↓ arrows - Scroll through tasks

PageUp/PageDown - Fast scroll

Ctrl+P - Toggle between Chat and Plan modes

Esc - Cancel and return to Chat

Expected Behavior:



✓ Smooth scrolling through task list

✓ Selected task highlighted

✓ Mode switches without losing state

✓ Plan preserved when switching modes

Test Scenario 7: Interrupted Execution

Objective: Test state persistence during execution



Steps:



Create plan with 5 tasks

Approve with Ctrl+A

Let 2 tasks complete

Force quit Crustly (Ctrl+C)

Restart Crustly

Check .crustly\_plan.json

Expected Behavior:



✓ Plan saved with correct state:

2 tasks marked "Completed" with timestamps

1 task marked "InProgress" (was executing)

2 tasks still "Pending"

✓ Can manually resume or restart

Test Scenario 8: Empty/Invalid Plans

Objective: Test error handling



Test Cases:



A. No Tasks:



"Create a plan for... actually never mind, don't do anything"

✓ Error: "Cannot finalize plan with no tasks"

B. Invalid Dependencies:



Manually edit .crustly\_plan.json to reference non-existent task ID

Try to execute

✓ Error: "Invalid dependency: task ID not found"

C. Missing Task Data:



Create plan with empty task title/description

✓ Validation catches incomplete data

Test Scenario 9: Read-Only Mode Testing

Objective: Test tool restrictions during planning



Steps:



Switch to Plan Mode manually (Ctrl+P)

Try entering commands that would modify files:

"Create a new file test.txt"

Expected Behavior:



✓ Write operations blocked

✓ Error message: "Write operations not allowed in Plan mode"

✓ Read-only commands work (ls, read\_file, grep)

Test Scenario 10: Progress Visualization

Objective: Test visual feedback during execution



Steps:



Create plan with 10 simple tasks

Approve and watch execution

Observe:

Task counter updates (Task 3/10)

Status icons change (📋 → ⏳ → ✅)

Completion timestamps appear

Progress bar (if implemented)

Quick Smoke Test Checklist

For rapid testing, verify these core functions:





AI creates plan from complex request



Plan displays in TUI with correct formatting



Ctrl+A approves and starts execution



Tasks execute in dependency order



Status updates: Pending → InProgress → Completed



Completion message appears after last task



\[object Object] persists state correctly



Ctrl+R rejects without executing



Circular dependencies detected and blocked



All keyboard shortcuts work

Edge Cases to Test

Very long task list (20+ tasks) - UI scrolling

Complex descriptions - Unicode, code blocks, special chars

Network interruption - State recovery

Concurrent plans - Multiple sessions

Plan during tool approval - Mode conflicts

