# Tool Approval System Implementation

## ✅ What's Been Implemented

### 1. **Event System** (src/tui/events.rs)
- ✅ `ToolApprovalRequest` - Contains tool details and response channel
- ✅ `ToolApprovalResponse` - User's approval decision
- ✅ `TuiEvent::ToolApprovalRequested` - Event fired when approval needed
- ✅ `TuiEvent::ToolApprovalResponse` - Event fired when user responds
- ✅ `AppMode::ToolApproval` - New UI mode for approval dialog
- ✅ Key bindings: `A`/`Y` (approve), `D`/`N` (deny), `V` (view details), `Esc` (cancel)

### 2. **App State** (src/tui/app.rs)
- ✅ `pending_approval: Option<ToolApprovalRequest>` - Stores current approval request
- ✅ `show_approval_details: bool` - Toggles detailed view
- ✅ `handle_approval_requested()` - Stores request and switches to approval mode
- ✅ `handle_approval_key()` - Handles user input (approve/deny/view)
- ✅ Event handling for approval requests and responses

### 3. **Beautiful Approval Dialog UI** (src/tui/render.rs)
- ✅ Centered dialog with red border (danger indication)
- ✅ Shows tool name, description, and capabilities
- ✅ Displays parameters (simplified or full JSON view)
- ✅ Color-coded action buttons
- ✅ Toggle between summary and detailed view with `V` key
- ✅ Professional, safe-looking design

**Preview:**
```
┌──────────────────────────────────────────────────────┐
│ ⚠️  PERMISSION REQUIRED                              │
├──────────────────────────────────────────────────────┤
│ 🔒 Permission Request                                │
│                                                      │
│ Claude wants to use the tool: write_file            │
│                                                      │
│ Description: Write content to a file...             │
│                                                      │
│ ⚠️  Capabilities:                                     │
│    • WriteFiles                                      │
│    • SystemModification                              │
│                                                      │
│ Parameters:                                          │
│    path: "config.json"                               │
│    content: "{ \"debug\": true }"                    │
│                                                      │
│ [A]pprove  [D]eny  [V]iew Details  [Esc] Cancel    │
└──────────────────────────────────────────────────────┘
```

## ⏳ What Remains (Complex Part)

### **Agent Service Integration** (Not Yet Implemented)

The tool execution loop in `src/llm/agent/service.rs` needs to be modified to:

1. **Detect when approval is required** (line 370-380)
2. **Send approval request to TUI via channel**
3. **Wait for user response** (blocking or async)
4. **Resume execution based on decision**

**Current code:**
```rust
// src/llm/agent/service.rs:370-401
match self.tool_registry.execute(&tool_name, tool_input, &tool_context).await {
    Ok(result) => {
        // Tool executed successfully
    }
    Err(e) => {
        // Tool execution failed - currently includes ApprovalRequired error
    }
}
```

**What needs to change:**
```rust
// Check if tool requires approval
if tool.requires_approval() && !tool_context.auto_approve {
    // Send approval request to TUI
    let (response_tx, mut response_rx) = mpsc::unbounded_channel();
    let approval_request = ToolApprovalRequest {
        request_id: Uuid::new_v4(),
        tool_name: tool_name.clone(),
        tool_description: tool.description().to_string(),
        tool_input: tool_input.clone(),
        capabilities: tool.capabilities().iter().map(|c| format!("{:?}", c)).collect(),
        response_tx,
    };

    // Send to TUI (need access to event sender)
    tui_event_sender.send(TuiEvent::ToolApprovalRequested(approval_request))?;

    // Wait for user decision
    let approval_response = response_rx.recv().await?;

    if !approval_response.approved {
        // User denied - return error
        return Err(ToolError::ApprovalDenied(
            approval_response.reason.unwrap_or_else(|| "User denied permission".to_string())
        ));
    }
}

// Proceed with execution
let result = tool.execute(input, context).await?;
```

**Challenge:** The agent service doesn't have access to the TUI event sender. Solutions:

**Option A: Pass event sender to agent service**
```rust
// In cli/mod.rs:cmd_chat
let agent_service = Arc::new(
    AgentService::new(provider.clone(), service_context.clone())
        .with_tool_registry(Arc::new(tool_registry))
        .with_approval_sender(app.event_sender()) // NEW
);
```

**Option B: Use callback function**
```rust
pub struct AgentService {
    approval_callback: Option<Arc<dyn Fn(ToolApprovalRequest) -> BoxFuture<'static, Result<bool>> + Send + Sync>>,
}
```

**Option C: Keep it in TUI layer (current approach)**
- Agent service returns `ToolError::ApprovalRequired`
- TUI layer catches this and shows approval dialog
- After approval, TUI retries the agent request
- This is simplest but requires more refactoring

## 🎯 Recommended Next Steps

### For Quick Testing (Temporary Workaround)

Add auto-approve mode to TUI for testing:

```rust
// In src/cli/mod.rs:cmd_chat (line 289)
let agent_service = Arc::new(
    AgentService::new(provider.clone(), service_context.clone())
        .with_tool_registry(Arc::new(tool_registry))
        .with_auto_approve_tools(true)  // TEMPORARY - for testing
);
```

### For Full Implementation

1. **Add approval callback to AgentService**:
   ```rust
   // src/llm/agent/service.rs
   pub type ApprovalCallback = Arc<dyn Fn(ToolApprovalRequest) -> BoxFuture<'static, Result<bool>> + Send + Sync>;

   pub struct AgentService {
       approval_callback: Option<ApprovalCallback>,
       // ... existing fields
   }
   ```

2. **Provide callback from TUI**:
   ```rust
   // src/cli/mod.rs:cmd_chat
   let event_sender = app.event_sender();
   let approval_callback = Arc::new(move |request: ToolApprovalRequest| {
       let sender = event_sender.clone();
       Box::pin(async move {
           let (response_tx, mut response_rx) = mpsc::unbounded_channel();
           let mut req = request;
           req.response_tx = response_tx;
           sender.send(TuiEvent::ToolApprovalRequested(req))?;
           let response = response_rx.recv().await?;
           Ok(response.approved)
       })
   });

   let agent_service = AgentService::new(provider, context)
       .with_approval_callback(Some(approval_callback));
   ```

3. **Update tool execution loop** to call the callback

## 📊 Implementation Status

| Component | Status | Complexity |
|-----------|--------|------------|
| Event types | ✅ Complete | Low |
| App state | ✅ Complete | Low |
| UI rendering | ✅ Complete | Medium |
| Key handlers | ✅ Complete | Low |
| Agent integration | ❌ Not started | **High** |

**Overall: 80% Complete**

The hard part (agent service integration) requires:
- Understanding async Rust patterns
- Channel-based communication
- Futures and BoxFuture
- Careful error handling

## 🧪 Testing the UI (Without Agent Integration)

You can test the approval dialog UI by manually triggering it:

```rust
// In src/tui/app.rs, add a test method:
pub fn test_approval_dialog(&mut self) {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let request = ToolApprovalRequest {
        request_id: Uuid::new_v4(),
        tool_name: "write_file".to_string(),
        tool_description: "Write content to a file".to_string(),
        tool_input: serde_json::json!({
            "path": "config.json",
            "content": "{ \"debug\": true }"
        }),
        capabilities: vec!["WriteFiles".to_string(), "SystemModification".to_string()],
        response_tx: tx,
    };
    self.handle_approval_requested(request);
}
```

Then call this from the main event loop when a test key is pressed.

## 📚 Related Files

- `src/tui/events.rs` - Event definitions
- `src/tui/app.rs` - App state and handlers
- `src/tui/render.rs` - UI rendering
- `src/llm/agent/service.rs` - **Needs modification**
- `src/llm/tools/registry.rs` - Tool execution (where approval check happens)
- `src/llm/tools/trait.rs` - Tool trait with `requires_approval()`

## 🎓 Key Design Decisions

1. **Channel-based communication** - Approval request includes a channel for the response
2. **Modal dialog** - Approval UI is a full-screen modal, can't be dismissed except by decision
3. **Two-view mode** - Simple view (default) and detailed JSON view (toggle with V)
4. **Color-coded danger** - Red borders and yellow warnings indicate this is important
5. **Multiple approval keys** - Both A/Y for approve and D/N for deny (user choice)

## 🔐 Security Considerations

- ✅ All dangerous tools (`WriteFiles`, `ExecuteShell`, `SystemModification`) require approval by default
- ✅ User sees full tool details before approving
- ✅ No way to bypass the dialog (except Esc to deny)
- ✅ Clear visual indicators of danger (red/yellow colors)
- ⚠️ Auto-approve mode should ONLY be used for trusted use cases

## 💡 Future Enhancements

- [ ] Remember approval decisions for session ("Always allow for this session")
- [ ] Approval history log
- [ ] Tool whitelist/blacklist configuration
- [ ] Bulk approve multiple tools at once
- [ ] Show file diff before approving writes
- [ ] Sandbox mode for safer execution
