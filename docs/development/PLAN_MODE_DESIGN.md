# Plan Mode Design Document

## Overview

Plan Mode is a workflow feature that enables structured task decomposition and controlled execution for complex development tasks. Inspired by Claude Code's planning capabilities, it allows the LLM to explore and analyze codebases in a read-only mode, generate a structured plan with subtasks, and then execute those tasks in a controlled manner after user approval.

## Goals

1. **Safer Execution**: Prevent uncontrolled modifications during exploration/planning phase
2. **Better Task Management**: Break complex tasks into smaller, manageable subtasks
3. **User Control**: Maintain architectural control through explicit plan approval
4. **Progress Tracking**: Visual tracking of plan progress and task status
5. **Flexibility**: Support different models for planning vs execution

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Crustly Application                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌────────────┐     ┌─────────────┐     ┌──────────────┐  │
│  │  AppMode   │────▶│  Plan View  │────▶│ Tool Filter  │  │
│  │  (enum)    │     │  (UI)       │     │ (read-only)  │  │
│  └────────────┘     └─────────────┘     └──────────────┘  │
│         │                   │                              │
│         ▼                   ▼                              │
│  ┌────────────┐     ┌─────────────┐                       │
│  │   Mode     │◀────│  Plan Doc   │                       │
│  │  Switcher  │     │  (struct)   │                       │
│  └────────────┘     └─────────────┘                       │
│                            │                               │
│                            ▼                               │
│                     ┌──────────────┐                       │
│                     │  Task List   │                       │
│                     │  (Vec<Task>) │                       │
│                     └──────────────┘                       │
└─────────────────────────────────────────────────────────────┘
```

### Data Structures

#### 1. AppMode Extension

```rust
/// Application modes
pub enum AppMode {
    /// Splash screen
    Splash,
    /// Main chat interface (full execution)
    Chat,
    /// Plan mode (read-only, planning phase)
    Plan,
    /// Session list/management
    Sessions,
    /// Help screen
    Help,
    /// Settings
    Settings,
    /// Tool approval dialog
    ToolApproval,
}
```

#### 2. Plan Document Structure

```rust
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

/// Status of a plan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Types of tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Status of individual tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
```

#### 3. Tool Execution Context Enhancement

```rust
/// Execution context for tools (ENHANCED)
#[derive(Debug, Clone)]
pub struct ToolExecutionContext {
    /// Session ID
    pub session_id: Uuid,

    /// Working directory
    pub working_directory: std::path::PathBuf,

    /// Environment variables
    pub env_vars: HashMap<String, String>,

    /// Whether auto-approve is enabled
    pub auto_approve: bool,

    /// Maximum execution timeout in seconds
    pub timeout_secs: u64,

    /// NEW: Current application mode
    pub app_mode: AppMode,

    /// NEW: Active plan ID (if in plan mode or executing a plan)
    pub active_plan_id: Option<Uuid>,
}
```

## Mode Switching Logic

### Mode Transitions

```
         Shift+Tab (cycle modes)
         ┌───────────────────────┐
         │                       │
         ▼                       │
    ┌────────┐              ┌────────┐
    │  Chat  │◀────────────▶│  Plan  │
    │  Mode  │   Approve/   │  Mode  │
    │        │   Reject     │        │
    └────────┘              └────────┘
         │                       │
         │                       │
         ▼                       ▼
    Execute               Read-Only
    All Tools             Tools Only
```

### Keyboard Shortcuts

| Shortcut | From Mode | To Mode | Action |
|----------|-----------|---------|---------|
| **Shift+Tab** | Chat | Plan | Enter planning mode |
| **Shift+Tab** | Plan | Chat | Return to chat (if no pending plan) |
| **Ctrl+P** | Any | Plan | Toggle Plan mode |
| **Ctrl+A** | Plan | Chat | Approve plan and switch to execution |
| **Ctrl+R** | Plan | Plan | Reject plan, request revisions |
| **Esc** | Plan | Chat | Cancel plan, return to chat |

### UI Mode Indicator

In the TUI header, display current mode:

```
┌─ 🦀 Crustly AI Assistant ──────────────────────────────────┐
│ 📝 Session: My Project  │  🤖 Model: qwen2.5-coder  │  📋 MODE: PLAN │
└────────────────────────────────────────────────────────────┘
```

## Tool Restrictions in Plan Mode

### Read-Only Tools (Allowed in Plan Mode)

✅ **Allowed:**
- `read_file` - Read file contents
- `ls` - List directory contents
- `glob` - Find files by pattern
- `grep` - Search for patterns
- `bash` (restricted) - Only safe read-only commands:
  - `git status`
  - `git log`
  - `git diff`
  - `git branch`
  - `ls`, `cat`, `head`, `tail`, `grep`, `find`
  - `pwd`, `whoami`, `hostname`
- `web_search` - Research (if network allowed)
- `http` (GET only) - Fetch documentation/APIs

❌ **Blocked:**
- `write_file` - File modification
- `edit` - File editing
- `bash` (write operations) - Commands like:
  - `git commit`, `git push`
  - `rm`, `mv`, `cp`
  - `cargo build`, `cargo test` (modify target/)
  - Any command with `>`, `>>`, pipes to files
- `code_exec` - Code execution
- `notebook` - Notebook modification

### Implementation Approach

```rust
impl Tool for SomeTool {
    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WriteFiles]
    }

    async fn execute(
        &self,
        input: Value,
        context: &ToolExecutionContext,
    ) -> Result<ToolResult> {
        // NEW: Check if in Plan mode and tool has write capabilities
        if context.app_mode == AppMode::Plan
            && self.capabilities().contains(&ToolCapability::WriteFiles) {
            return Ok(ToolResult::error(
                "Write operations not allowed in Plan mode. \
                 Please complete the plan and switch to execution mode.".to_string()
            ));
        }

        // ... rest of execution
    }
}
```

### Bash Tool Special Handling

```rust
// In bash tool, check if command is safe for Plan mode
fn is_read_only_command(command: &str) -> bool {
    let safe_commands = [
        "git status", "git log", "git diff", "git branch", "git show",
        "ls", "cat", "head", "tail", "grep", "find", "pwd",
        "whoami", "hostname", "date", "echo", "which", "type",
    ];

    let cmd_start = command.split_whitespace().next().unwrap_or("");

    // Check if it's a safe command
    if safe_commands.iter().any(|&c| command.starts_with(c)) {
        // Additional check: no output redirection
        return !command.contains('>') && !command.contains(">>")
            && !command.contains('|') // Be strict about pipes
    }

    false
}
```

## UI Changes

### New Plan View

When in Plan mode, render a specialized view showing:

1. **Plan Header**
   - Plan title
   - Status badge (Draft/Pending/Approved/etc.)
   - Creation time

2. **Plan Summary**
   - Description/goal
   - Context and assumptions
   - Identified risks

3. **Task List**
   - Numbered tasks with status icons
   - Dependencies visualization
   - Complexity indicators
   - Expandable task details

4. **Action Bar**
   - Approve plan (Ctrl+A)
   - Reject/Request changes (Ctrl+R)
   - Cancel (Esc)

### Example Plan View Mockup

```
┌─ 📋 PLAN MODE ──────────────────────────────────────────────┐
│                                                             │
│  🎯 Add User Authentication System                         │
│  Status: [Pending Approval]  Created: 2025-11-10 14:23     │
│                                                             │
│  📝 Description:                                            │
│  Implement a complete user authentication system with      │
│  JWT tokens, password hashing, and session management.     │
│                                                             │
│  🔍 Context & Assumptions:                                  │
│  - Using axum for web framework                            │
│  - PostgreSQL for user storage                             │
│  - bcrypt for password hashing                             │
│  - Redis for session storage (optional)                    │
│                                                             │
│  ⚠️  Risks & Unknowns:                                      │
│  - Need to decide on JWT library (jsonwebtoken vs...)      │
│  - Session timeout duration not specified                  │
│                                                             │
│  📋 Tasks (8 total):                                        │
│                                                             │
│   [⏸️ ] 1. Research authentication best practices          │
│           Type: Research  |  Complexity: ★★☆☆☆             │
│                                                             │
│   [⏸️ ] 2. Create User model and database schema           │
│           Type: Create    |  Complexity: ★★☆☆☆             │
│           Dependencies: Task #1                            │
│                                                             │
│   [⏸️ ] 3. Implement password hashing utilities            │
│           Type: Create    |  Complexity: ★★★☆☆             │
│           Dependencies: Task #2                            │
│                                                             │
│   [⏸️ ] 4. Create JWT token generation/validation          │
│           Type: Create    |  Complexity: ★★★★☆             │
│           Dependencies: Task #2                            │
│                                                             │
│   [⏸️ ] 5. Implement login/logout endpoints                │
│           Type: Create    |  Complexity: ★★★☆☆             │
│           Dependencies: Task #3, #4                        │
│                                                             │
│   [⏸️ ] 6. Add authentication middleware                   │
│           Type: Create    |  Complexity: ★★★★☆             │
│           Dependencies: Task #4                            │
│                                                             │
│   [⏸️ ] 7. Write unit tests for auth logic                 │
│           Type: Test      |  Complexity: ★★★☆☆             │
│           Dependencies: Task #3, #4, #5                    │
│                                                             │
│   [⏸️ ] 8. Write integration tests for endpoints           │
│           Type: Test      |  Complexity: ★★★★☆             │
│           Dependencies: Task #5, #6                        │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  [A]pprove  [R]eject  [E]xpand Tasks  [Esc] Cancel        │
└─────────────────────────────────────────────────────────────┘
```

### During Execution View

When executing an approved plan:

```
┌─ 🚀 EXECUTING PLAN ─────────────────────────────────────────┐
│                                                             │
│  🎯 Add User Authentication System                         │
│  Progress: 3/8 tasks completed  [████████░░░░░░░] 37%      │
│                                                             │
│  ✅ 1. Research authentication best practices              │
│  ✅ 2. Create User model and database schema               │
│  ✅ 3. Implement password hashing utilities                │
│  ⏳ 4. Create JWT token generation/validation [IN PROGRESS]│
│  ⏸️  5. Implement login/logout endpoints                    │
│  ⏸️  6. Add authentication middleware                       │
│  ⏸️  7. Write unit tests for auth logic                     │
│  ⏸️  8. Write integration tests for endpoints               │
│                                                             │
│  💬 Current Task Details:                                   │
│  Creating JWT utilities in src/auth/jwt.rs...              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## System Prompts

### Plan Mode System Prompt

```
You are Crustly in PLAN MODE. In this mode, you are tasked with analyzing
requirements and creating a detailed, structured plan before any code changes.

CURRENT CONSTRAINTS:
- You can ONLY perform read-only operations
- Allowed: read files, list directories, search code, check git status
- NOT allowed: write/edit files, execute builds, modify any state

YOUR TASK:
1. Thoroughly analyze the codebase using read-only tools (glob, grep, read_file, ls)
2. Ask clarifying questions if requirements are unclear
3. Create a comprehensive plan that includes:
   - Clear goal statement
   - Context and assumptions
   - List of all impacted files/modules
   - Breakdown into specific, unitary tasks
   - Task dependencies and ordering
   - Identified risks and unknowns
   - Estimated complexity for each task

PLAN FORMAT:
Generate a plan using this structure:
- Title: Clear, concise goal
- Description: Detailed explanation of what needs to be done
- Context: Relevant background, constraints, assumptions
- Risks: Things that could go wrong or are uncertain
- Tasks: Numbered list of specific tasks, each with:
  * Clear title
  * Description of what the task does
  * Task type (Research/Edit/Create/Test/etc.)
  * Dependencies (which tasks must complete first)
  * Complexity (1-5 stars)

Example task:
"Task 3: Implement password hashing utility
 Description: Create a new module auth/hash.rs with bcrypt-based password
 hashing and validation functions.
 Type: Create
 Dependencies: Task 1 (research), Task 2 (user model)
 Complexity: ★★★☆☆"

Remember: You are in PLANNING mode. Focus on analysis and design, not execution.
Ask questions, explore thoroughly, and create a clear roadmap.
```

### Execution Mode System Prompt (when plan active)

```
You are Crustly in EXECUTION MODE with an ACTIVE PLAN.

You have an approved plan with the following tasks:
[Task list will be injected here]

CURRENT TASK: Task #{current_task_number}
{task_details}

YOUR FOCUS:
- Execute ONLY the current task as described
- Follow the plan exactly as approved
- If you encounter issues that require deviating from the plan, STOP and inform the user
- Update task status and notes as you progress

CONSTRAINTS:
- Stay focused on the current task
- Don't skip ahead to future tasks
- Don't modify tasks that weren't in the plan
- If dependencies are missing, report the issue

After completing the current task, report completion and ask if you should proceed
to the next task.
```

## Implementation Phases

### Phase 1: Core Infrastructure (2-3 days)
**Goal:** Basic Plan mode with data structures

1. ✅ Design architecture (this document)
2. Add `Plan` variant to `AppMode` enum
3. Create plan data structures (`PlanDocument`, `PlanTask`, etc.)
4. Add mode switching logic (Shift+Tab, Ctrl+P)
5. Implement basic Plan view UI (just displays "Plan Mode Active")
6. Update `ToolExecutionContext` to include `app_mode`

### Phase 2: Tool Restrictions (1-2 days)
**Goal:** Enforce read-only mode in planning

1. Implement capability checking in tool execution
2. Add read-only command validation for bash tool
3. Block write operations in Plan mode
4. Add helpful error messages when tools are blocked
5. Test all tools to ensure correct restrictions

### Phase 3: Plan Generation & UI (2-3 days)
**Goal:** LLM can generate plans, user can view them

1. Create Plan mode system prompt
2. Implement plan parsing from LLM responses
3. Build full Plan view UI with task list
4. Add plan document storage (in-memory or database)
5. Implement task status visualization

### Phase 4: Plan Approval & Execution (2-3 days)
**Goal:** User can approve/reject plans and execute tasks

1. Implement approval/rejection workflows
2. Add execution mode system prompts
3. Build progress tracking UI
4. Implement task-by-task execution
5. Add dependency validation

### Phase 5: Polish & Testing (2-3 days)
**Goal:** Production-ready feature

1. Add comprehensive error handling
2. Write unit tests for plan logic
3. Write integration tests for mode switching
4. Create user documentation
5. Add examples and usage guides
6. Performance testing and optimization

## Optional Enhancements (Future)

### Model Switching
- Support different models for plan vs execution
- Example: Use better model (Claude Opus) for planning, faster model (Sonnet) for execution
- Configuration: `plan_model` and `execution_model` in config

### Plan Templates
- Pre-defined templates for common tasks
- "Add REST API endpoint", "Implement feature flag", "Add database migration", etc.

### Plan History
- Store completed plans in database
- View past plans and their outcomes
- Learn from previous planning patterns

### Collaborative Planning
- Export plans as markdown for review
- Import plans from external tools
- Share plans between team members

### AI-Assisted Task Breakdown
- Tool to automatically suggest task breakdown
- Analyze task complexity automatically
- Identify dependencies using code analysis

### Sub-Agents
- Specialized agents for different task types
- Research agent for exploration
- Code agent for implementation
- Test agent for writing tests

## Database Schema (Optional Persistence)

If storing plans in SQLite:

```sql
-- Plans table
CREATE TABLE plans (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    context TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    approved_at DATETIME,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

-- Plan tasks table
CREATE TABLE plan_tasks (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    order_num INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    task_type TEXT NOT NULL,
    complexity INTEGER NOT NULL,
    status TEXT NOT NULL,
    notes TEXT,
    completed_at DATETIME,
    FOREIGN KEY (plan_id) REFERENCES plans(id)
);

-- Task dependencies table
CREATE TABLE task_dependencies (
    task_id TEXT NOT NULL,
    depends_on_task_id TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id),
    FOREIGN KEY (task_id) REFERENCES plan_tasks(id),
    FOREIGN KEY (depends_on_task_id) REFERENCES plan_tasks(id)
);

-- Plan risks table
CREATE TABLE plan_risks (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    description TEXT NOT NULL,
    severity TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES plans(id)
);
```

## Success Criteria

Plan Mode is considered successful when:

1. ✅ User can switch between Chat and Plan mode seamlessly
2. ✅ In Plan mode, LLM cannot perform write operations
3. ✅ LLM can explore codebase and generate structured plans
4. ✅ Plans are displayed in clear, readable format
5. ✅ User can approve, reject, or request plan revisions
6. ✅ Approved plans execute task-by-task with progress tracking
7. ✅ Mode switching doesn't lose conversation context
8. ✅ Tool restrictions are consistently enforced
9. ✅ Error messages are helpful and actionable
10. ✅ Documentation explains Plan mode workflow clearly

## References

- Claude Code Plan Mode: https://docs.claude.com/en/docs/claude-code/plan-mode
- Task Management: Crustly's existing task tool (src/llm/tools/task.rs)
- Approval System: Existing tool approval dialog (src/tui/render.rs:render_approval)

## Timeline Estimate

- **Total**: ~10-16 days for full implementation
- **MVP**: ~5-7 days (Phases 1-3)
- **Production Ready**: ~10-14 days (Phases 1-5)

## Next Steps

1. Review this design document with team/user
2. Get approval on architecture and data structures
3. Begin Phase 1 implementation
4. Iterate based on feedback

---

**Document Version**: 1.0
**Last Updated**: 2025-11-10
**Author**: Claude (Crustly Development)
**Status**: Draft - Awaiting Review
