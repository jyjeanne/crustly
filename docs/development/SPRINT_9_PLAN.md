# Sprint 9: Enhanced TUI Experience

## Goal
Transform the TUI from functional to delightful with markdown rendering, syntax highlighting, improved text editing, and polished animations.

## Current State

### What We Have ✅
- Basic TUI with Ratatui
- Simple text rendering
- Message history scrolling
- Session list overlay
- Basic input box
- Keyboard shortcuts

### What's Missing ❌
- No markdown rendering
- No syntax highlighting
- Limited text editing (single line)
- No visual polish
- No animations

---

## Sprint 9 Objectives

### 1. Markdown Rendering 📝
**Goal:** Render markdown in messages beautifully

**Features to Add:**
- **Headers** - H1-H6 with different styling
- **Bold** - `**text**` rendering
- **Italic** - `*text*` rendering
- **Code inline** - `` `code` `` rendering
- **Code blocks** - ` ```lang ... ``` ` rendering
- **Lists** - Bulleted and numbered
- **Links** - Render with visual indication
- **Blockquotes** - > quoted text
- **Horizontal rules** - `---`

**Implementation:**
- Use `pulldown-cmark` for markdown parsing
- Custom renderer for Ratatui widgets
- Preserve formatting in terminal

**Dependencies:**
```toml
pulldown-cmark = "0.9"
```

**Estimated Effort:** 4-6 hours

---

### 2. Syntax Highlighting 🎨
**Goal:** Beautiful code block highlighting

**Features to Add:**
- **Language detection** - Auto-detect from fence info
- **Color schemes** - Multiple themes (dark/light)
- **Popular languages** - Rust, Python, JavaScript, TypeScript, Go, etc.
- **Fallback** - Plain text for unknown languages

**Implementation:**
- Use `syntect` for syntax highlighting
- Load syntax definitions at startup
- Cache highlighted blocks
- Apply terminal color palette

**Dependencies:**
```toml
syntect = "5.0"
```

**Supported Languages:**
- Rust, Python, JavaScript, TypeScript
- Go, Java, C, C++, C#
- Ruby, PHP, Swift, Kotlin
- Shell, SQL, JSON, YAML, TOML
- HTML, CSS, Markdown

**Estimated Effort:** 3-4 hours

---

### 3. Better Text Editor 📝
**Goal:** Professional multi-line text editing

**Current Limitations:**
- Single-line input only
- No multi-line support
- Limited editing (no word navigation)
- No selection
- No undo/redo

**Features to Add:**
- **Multi-line editing** - Enter adds new line, Ctrl+Enter sends
- **Word navigation** - Ctrl+Left/Right for word jumping
- **Line navigation** - Home/End, Ctrl+Home/End
- **Selection** - Shift+arrows for text selection
- **Copy/Paste** - Standard clipboard operations
- **Undo/Redo** - Ctrl+Z/Ctrl+Y
- **Line numbers** - For multi-line input
- **Auto-indent** - Maintain indentation

**Implementation:**
- Use `tui-textarea` widget (better than basic input)
- Wrap with custom styling
- Add keyboard handler for shortcuts
- Vim mode support (optional)

**Dependencies:**
```toml
tui-textarea = "0.4"
```

**Estimated Effort:** 5-6 hours

---

### 4. Polish and Animations ✨
**Goal:** Delightful visual experience

**Features to Add:**

#### 4.1 Loading Animations
- **Typing indicator** - Animated "..." while Claude types
- **Processing spinner** - For long operations
- **Progress bars** - For file operations

#### 4.2 Smooth Transitions
- **Fade in/out** - For overlays
- **Slide animations** - For session list
- **Scroll smoothing** - Smooth chat scrolling

#### 4.3 Visual Polish
- **Better borders** - Rounded corners, shadows
- **Color scheme** - Cohesive color palette
- **Icons** - Unicode symbols for actions
- **Status indicators** - Connection status, typing, etc.

#### 4.4 Message Styling
- **User messages** - Right-aligned, distinct color
- **Assistant messages** - Left-aligned, different color
- **System messages** - Centered, muted color
- **Timestamps** - Subtle timestamps
- **Message separators** - Visual breaks

#### 4.5 Code Block Styling
- **Line numbers** - For code blocks
- **Copy button indicator** - Visual hint
- **Language label** - Show detected language
- **Background shading** - Distinct from text

**Implementation:**
- Custom rendering functions
- Animation state machine
- Frame timing for smooth animations
- Unicode box drawing characters

**Estimated Effort:** 4-5 hours

---

## Dependencies to Add

```toml
[dependencies]
# Existing dependencies remain...

# Markdown rendering
pulldown-cmark = "0.9"

# Syntax highlighting
syntect = "5.0"

# Better text editor
tui-textarea = "0.4"

# For animations (optional)
# tokio::time for frame timing already available
```

---

## Implementation Order

### Phase 1: Foundation (4-6 hours)
1. **Add dependencies** to Cargo.toml
2. **Markdown parsing** - Parse markdown to AST
3. **Basic markdown rendering** - Headers, bold, italic

### Phase 2: Code Highlighting (3-4 hours)
4. **Syntax highlighting setup** - Load syntaxes
5. **Code block rendering** - Highlighted code blocks
6. **Language detection** - Auto-detect from fence info

### Phase 3: Text Editor (5-6 hours)
7. **Integrate tui-textarea** - Replace basic input
8. **Multi-line support** - Enter for newline, Ctrl+Enter to send
9. **Enhanced editing** - Word navigation, selection, undo/redo

### Phase 4: Polish (4-5 hours)
10. **Loading animations** - Typing indicator, spinner
11. **Message styling** - Better layout and colors
12. **Visual polish** - Borders, icons, spacing
13. **Smooth transitions** - Fade/slide animations

**Total Estimated Effort:** 16-21 hours

---

## File Structure

```
src/tui/
├── mod.rs              # Main TUI module
├── app.rs              # App state
├── render.rs           # Main render function (TO UPDATE)
├── runner.rs           # Event loop
├── events.rs           # Event handling
├── markdown.rs         # NEW: Markdown rendering
├── highlight.rs        # NEW: Syntax highlighting
├── editor.rs           # NEW: Enhanced text editor
└── components/
    ├── logo.rs         # Logo component
    ├── chat.rs         # NEW: Chat message rendering
    ├── input.rs        # NEW: Enhanced input component
    └── spinner.rs      # NEW: Loading animations
```

---

## Technical Considerations

### 1. **Performance**
- Cache syntax highlighting results
- Lazy load syntax definitions
- Efficient markdown parsing
- Minimize redraws

### 2. **Terminal Compatibility**
- Test on Windows Terminal, iTerm2, Alacritty
- Fallback for limited color terminals
- Unicode support detection
- Handle various terminal sizes

### 3. **Memory Usage**
- Don't keep entire conversation in memory
- Paginate old messages
- Clear cached highlights periodically

### 4. **User Experience**
- Smooth animations (60fps target)
- Responsive input (< 16ms)
- Clear visual hierarchy
- Accessible keyboard shortcuts

---

## Testing Strategy

### Manual Testing
- Test with various markdown formatting
- Test code blocks in multiple languages
- Test multi-line editing
- Test on different terminal emulators
- Test with long conversations

### Automated Testing (Optional)
- Unit tests for markdown parser
- Unit tests for syntax highlighter
- Snapshot tests for rendering

---

## Success Criteria

### Must Have ✅
1. **Markdown rendering** - Headers, bold, italic, code blocks
2. **Syntax highlighting** - At least 10 languages
3. **Multi-line editing** - Enter for newline, Ctrl+Enter to send
4. **Code block styling** - Distinct background, line numbers
5. **Loading indicator** - Show when processing

### Should Have 📋
1. **Word navigation** - Ctrl+Left/Right
2. **Undo/Redo** - Ctrl+Z/Ctrl+Y
3. **Smooth scrolling** - No jumpy animations
4. **Better message layout** - Clear user/assistant distinction
5. **Visual polish** - Better colors and spacing

### Nice to Have 🎯
1. **Fade animations** - For overlays
2. **Copy indicator** - For code blocks
3. **Typing animation** - "..." while Claude types
4. **Progress bars** - For long operations
5. **Vim mode** - For power users

---

## Example: Before and After

### Before (Current)
```
┌─ Session: Test ────────────────────────┐
│ User: Hello                             │
│ Assistant: Hi there!                    │
│                                         │
│ > _                                     │
└─────────────────────────────────────────┘
```

### After (Sprint 9)
```
╭─ Session: Test ─────────────── 💬 ────╮
│                                         │
│  You (2:30 PM)                         │
│  Hello                                  │
│                                         │
│  🤖 Claude (2:30 PM)                   │
│  Hi there! I can help with:            │
│  • Code reviews                        │
│  • Bug fixes                           │
│  • Architecture advice                 │
│                                         │
│  Here's a Rust example:                │
│  ╭─ rust ─────────────────────╮       │
│  │ 1  fn main() {              │       │
│  │ 2      println!("Hello");   │       │
│  │ 3  }                        │       │
│  ╰─────────────────────────────╯       │
│                                         │
│  [⋯] Claude is typing...               │
│                                         │
├─────────────────────────────────────────┤
│  Enter your message...                  │
│  (Ctrl+Enter to send)                   │
│  > _                                    │
╰─────────────────────────────────────────╯
```

---

## Color Scheme

### Dark Theme (Default)
- **Background:** Dark gray (#1e1e1e)
- **Text:** Light gray (#d4d4d4)
- **User messages:** Cyan (#4ec9b0)
- **Assistant messages:** Green (#6a9955)
- **Code blocks:** Dark background (#252526)
- **Syntax colors:** VS Code Dark+ theme
- **Borders:** Muted gray (#3e3e3e)
- **Highlights:** Blue (#007acc)

### Light Theme (Future)
- Similar to VS Code Light+

---

## Risks and Mitigations

### Risk 1: Performance with Large Messages
**Mitigation:**
- Lazy rendering (only visible parts)
- Pagination for old messages
- Cache parsed/highlighted content

### Risk 2: Terminal Compatibility Issues
**Mitigation:**
- Fallback rendering for basic terminals
- Test on multiple emulators
- Graceful degradation

### Risk 3: Complex Text Editor State
**Mitigation:**
- Use proven library (tui-textarea)
- Keep state simple
- Add tests for edge cases

---

## Documentation Updates

After Sprint 9:
1. **Update README.md** - New UI features
2. **Create UI_GUIDE.md** - Screenshots and feature guide
3. **Update MANUAL_TESTING_GUIDE.md** - Test new features
4. **Create SPRINT_9_COMPLETE.md** - Completion report

---

## Definition of Done

- [ ] Markdown rendering implemented
- [ ] Syntax highlighting working for 10+ languages
- [ ] Multi-line text editor integrated
- [ ] Loading animations added
- [ ] Message styling improved
- [ ] Code tested manually on 3+ terminal emulators
- [ ] Documentation updated
- [ ] Code committed with clear messages
- [ ] SPRINT_9_COMPLETE.md created

---

## Next Steps After Sprint 9

**Sprint 10:** Advanced Features
- Streaming response animations
- File attachment preview
- Image rendering (if supported)
- Terminal bell notifications
- Custom themes/config

---

**Estimated Timeline:** 16-21 hours over 2-3 sessions
**Priority:** High - Greatly improves user experience
**Complexity:** Medium-High - Multiple moving parts
