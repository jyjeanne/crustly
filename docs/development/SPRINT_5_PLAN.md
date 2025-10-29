# Sprint 5: Terminal User Interface (TUI)

## Overview
Implement a rich, interactive terminal user interface using Ratatui for the AI assistant, providing a ChatGPT/Claude-like experience in the terminal.

## Goals
- Create an intuitive, responsive TUI for interacting with the AI
- Support real-time streaming of LLM responses
- Provide session management and navigation
- Implement keyboard shortcuts and commands
- Display message history with markdown rendering
- Show token usage, costs, and system status

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Terminal (Ratatui)                       │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────┐  │
│  │              App State Manager                        │  │
│  │  - Current session                                    │  │
│  │  - UI mode (chat/session list/help)                  │  │
│  │  - Input buffer                                       │  │
│  │  - Scroll state                                       │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              UI Components                            │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │  Header (session info, token count, cost)       │  │  │
│  │  ├─────────────────────────────────────────────────┤  │  │
│  │  │  Chat Panel (messages with markdown)            │  │  │
│  │  │  - User messages (blue)                         │  │  │
│  │  │  - Assistant messages (green)                   │  │  │
│  │  │  - Tool use indicators (yellow)                 │  │  │
│  │  │  - Scrollable history                           │  │  │
│  │  ├─────────────────────────────────────────────────┤  │  │
│  │  │  Input Box (multi-line editor)                  │  │  │
│  │  │  - Syntax highlighting                          │  │  │
│  │  │  - Auto-completion                              │  │  │
│  │  ├─────────────────────────────────────────────────┤  │  │
│  │  │  Status Bar (shortcuts, mode, status)           │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Event Handler                            │  │
│  │  - Keyboard input                                     │  │
│  │  - Agent responses (async)                            │  │
│  │  - UI updates                                         │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
           │                    │                    │
           ▼                    ▼                    ▼
    AgentService          SessionService      MessageService
```

## Components

### 1. App State (`src/tui/app.rs`)
```rust
pub struct App {
    // Core state
    pub current_session: Option<Session>,
    pub messages: Vec<DisplayMessage>,
    pub input: InputBuffer,

    // UI state
    pub mode: AppMode,
    pub scroll_offset: usize,
    pub should_quit: bool,

    // Services
    pub agent_service: Arc<AgentService>,
    pub session_service: Arc<SessionService>,

    // Streaming
    pub streaming_response: Option<String>,
}

pub enum AppMode {
    Chat,           // Main chat interface
    SessionList,    // Browse/select sessions
    Help,           // Show help/shortcuts
    Settings,       // Configuration
}
```

### 2. UI Components (`src/tui/ui/`)

#### Header Component (`header.rs`)
- Session name and ID
- Current model
- Token count (input/output/total)
- Cost so far
- System status indicator

#### Chat Component (`chat.rs`)
- Scrollable message list
- Message rendering with:
  - User messages (right-aligned, blue)
  - Assistant messages (left-aligned, green)
  - Tool use indicators (yellow badges)
  - Timestamps
  - Markdown formatting (bold, italic, code blocks)
- Auto-scroll to bottom on new messages
- Manual scroll with arrow keys/PgUp/PgDn

#### Input Component (`input.rs`)
- Multi-line text editor
- Line numbers
- Cursor position indicator
- Submit on Ctrl+Enter
- Cancel on Escape
- Syntax highlighting for code

#### Status Bar (`status.rs`)
- Current mode indicator
- Keyboard shortcuts
- Loading indicator during requests
- Error messages

### 3. Event System (`src/tui/events.rs`)

```rust
pub enum TuiEvent {
    // Input events
    Input(KeyEvent),
    Resize(u16, u16),

    // Agent events
    MessageSent,
    ResponseChunk(String),
    ResponseComplete(AgentResponse),
    Error(String),

    // Navigation
    SwitchMode(AppMode),
    SelectSession(Uuid),
}
```

### 4. Renderer (`src/tui/render.rs`)
- Main render loop
- Component composition
- Layout management
- Theme/styling

## Features

### Phase 1: Basic Chat Interface (Day 1)
- [x] App state structure
- [x] Basic event loop
- [x] Header with session info
- [x] Message display (static)
- [x] Input box
- [x] Status bar
- [x] Send messages to agent
- [x] Display responses

### Phase 2: Enhanced UX (Day 2)
- [ ] Scrolling message history
- [ ] Multi-line input with editor
- [ ] Markdown rendering in messages
- [ ] Syntax highlighting for code blocks
- [ ] Loading indicators
- [ ] Error handling UI

### Phase 3: Session Management (Day 3)
- [ ] Session list view
- [ ] Create/switch sessions
- [ ] Archive/delete sessions
- [ ] Search sessions
- [ ] Session metadata display

### Phase 4: Streaming & Tools (Day 4)
- [ ] Real-time streaming of responses
- [ ] Tool use visualization
- [ ] Tool approval dialog
- [ ] Progress indicators
- [ ] Cancellation support

### Phase 5: Polish (Day 5)
- [ ] Keyboard shortcuts overlay (?)
- [ ] Settings panel
- [ ] Theme customization
- [ ] Help system
- [ ] Comprehensive testing

## Keyboard Shortcuts

### Global
- `Ctrl+C` - Quit application
- `Ctrl+N` - New session
- `Ctrl+L` - List sessions
- `Ctrl+H` - Show help
- `Ctrl+S` - Settings
- `Ctrl+Z` - Undo
- `Ctrl+Y` - Redo

### Chat Mode
- `Ctrl+Enter` - Send message
- `Escape` - Clear input
- `↑/↓` - Navigate message history
- `PgUp/PgDn` - Scroll chat
- `Home/End` - Jump to top/bottom
- `Ctrl+K` - Clear chat

### Session List
- `↑/↓` - Navigate sessions
- `Enter` - Select session
- `Delete` - Delete session
- `A` - Archive session
- `/` - Search

### Input
- `Ctrl+A` - Select all
- `Ctrl+X` - Cut
- `Ctrl+C` - Copy
- `Ctrl+V` - Paste
- `Tab` - Auto-complete (future)

## UI Theme

### Colors
- **Background**: Dark gray (#1e1e1e)
- **Foreground**: Light gray (#d4d4d4)
- **User messages**: Blue (#569cd6)
- **Assistant messages**: Green (#4ec9b0)
- **Tool use**: Yellow (#dcdcaa)
- **Errors**: Red (#f48771)
- **Success**: Green (#4ec9b0)
- **Borders**: Gray (#3e3e3e)

### Layout
```
┌─ Session: Test Chat ─ Model: claude-3-5-sonnet ─ Tokens: 1.2K ─ Cost: $0.003 ─┐
│                                                                                 │
│  User                                                                  10:30 AM │
│  > Can you help me implement a binary search?                                  │
│                                                                                 │
│  Assistant                                                             10:30 AM │
│  Sure! Here's a clean implementation:                                          │
│                                                                                 │
│  ```rust                                                                        │
│  fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {          │
│      let mut left = 0;                                                         │
│      let mut right = arr.len();                                                │
│      // ... implementation                                                     │
│  }                                                                              │
│  ```                                                                            │
│                                                                                 │
│  🔧 Used tool: read_file (src/algorithms.rs)                                   │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│ > _                                                                             │
│                                                                                 │
│   Ctrl+Enter: Send │ Esc: Clear │ Ctrl+L: Sessions │ Ctrl+H: Help            │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Technical Considerations

### 1. Async Event Handling
- Use `tokio::sync::mpsc` for event channels
- Separate thread for agent responses
- Non-blocking UI updates

### 2. Performance
- Lazy rendering (only visible messages)
- Efficient text wrapping
- Debounced redraws
- Buffer pooling for large messages

### 3. State Management
- Immutable state updates
- Command pattern for undo/redo
- State persistence between sessions

### 4. Testing
- Unit tests for state transitions
- Integration tests for event handling
- Snapshot tests for UI rendering
- Manual testing with real agent

## Dependencies

Already in Cargo.toml:
- `ratatui` - TUI framework
- `crossterm` - Terminal manipulation
- `tokio` - Async runtime
- `anyhow` - Error handling

Additional needed:
- `tui-textarea` - Multi-line input widget
- `syntect` (optional) - Syntax highlighting

## Success Criteria

1. ✅ Users can send messages and see responses
2. ✅ Messages are properly formatted with markdown
3. ✅ Scrolling works smoothly with large histories
4. ✅ Sessions can be created, switched, and managed
5. ✅ Streaming responses appear in real-time
6. ✅ Tool use is clearly indicated
7. ✅ Keyboard shortcuts work as expected
8. ✅ UI is responsive and doesn't block
9. ✅ Errors are displayed clearly
10. ✅ Token usage and costs are accurate

## Future Enhancements (Post-Sprint 5)
- Mouse support
- Copy/paste with system clipboard
- Export conversations
- Image preview (for vision models)
- Multi-pane layout (chat + file viewer)
- Vim/Emacs keybindings
- Customizable themes
- Plugin system for UI extensions
