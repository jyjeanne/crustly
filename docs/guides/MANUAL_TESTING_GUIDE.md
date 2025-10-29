# Crustly - Manual Testing Guide

## Prerequisites

Before starting manual testing, ensure you have:

1. **Anthropic API Key** - Get one from https://console.anthropic.com/
2. **Rust & Cargo** - Version 1.70+ installed
3. **Project Built** - Run `cargo build`
4. **Clean Environment** - Fresh database for testing

## Setup Steps

### Step 1: Set Environment Variables

```bash
# Required: Your Anthropic API key
export ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"

# Optional: Enable debug logging
export RUST_LOG="crustly=debug,sqlx=warn"
```

**Windows (PowerShell):**
```powershell
$env:ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"
$env:RUST_LOG="crustly=debug"
```

**Verification:**
```bash
# Check if API key is set
echo $ANTHROPIC_API_KEY  # Should show your key

# Or
cargo run -- config
# Should show: API Key: [SET]
```

### Step 2: Initialize Configuration

```bash
# Initialize config file
cargo run -- init

# Expected output:
# 🦀 Crustly Configuration Initialization
#
# ✅ Configuration initialized at: ~/.config/crustly/config.toml
#
# 📝 Next steps:
#    1. Edit the config file to add your API keys
#    2. Set ANTHROPIC_API_KEY environment variable
#    3. Run 'crustly' or 'crustly chat' to start
```

### Step 3: Initialize Database

```bash
# Initialize database
cargo run -- db init

# Expected output:
# 🗄️  Initializing database...
# INFO crustly::db: Connected to database: /path/to/crustly.db
# INFO crustly::db: Database migrations completed
# ✅ Database initialized at: /path/to/crustly.db
```

### Step 4: Verify Setup

```bash
# Show configuration
cargo run -- config

# Expected output:
# 🦀 Crustly Configuration
#
# Database: /path/to/crustly.db
# Log level: info
#
# Providers:
#   - anthropic: claude-3-5-sonnet-20240620
#     API Key: [SET]
#
# 💡 Use --show-secrets to display API keys
```

```bash
# Check database stats
cargo run -- db stats

# Expected output:
# 📊 Database Statistics
#
# Sessions: 0
# Messages: 0
# Tracked files: 0
```

## Test Scenarios

### Test 1: Simple Interactive Chat (TUI)

**Goal:** Verify the TUI launches and can send/receive messages.

**Steps:**

1. Launch TUI:
```bash
cargo run
```

2. **Expected:** Terminal clears and shows TUI with:
   - Header showing session info
   - Empty chat area
   - Input box at bottom
   - Status bar with keyboard shortcuts

3. Type a simple message in the input box:
```
Hello! Can you introduce yourself?
```

4. Press `Ctrl+Enter` to send

5. **Expected:**
   - Your message appears in blue
   - "Processing..." indicator shows
   - After 1-3 seconds, Claude's response appears in green
   - Token count and cost update in header

6. Send follow-up message:
```
What programming languages do you support?
```

7. Press `Ctrl+Enter` again

8. **Expected:**
   - Conversation continues
   - Previous messages remain visible
   - Can scroll with Page Up/Down

9. Press `Ctrl+C` to quit

**Success Criteria:**
- ✅ TUI launches without errors
- ✅ Messages send and receive successfully
- ✅ UI updates in real-time
- ✅ Keyboard shortcuts work
- ✅ Can quit cleanly

---

### Test 2: Non-Interactive Mode (Run Command)

**Goal:** Test single-command execution with different output formats.

**Steps:**

1. **Text Output (Default):**
```bash
cargo run -- run "What is 2+2?"
```

**Expected Output:**
```
🤔 Processing...

The answer is 4.

📊 Tokens: 42
💰 Cost: $0.000126
```

2. **JSON Output:**
```bash
cargo run -- run --format json "List 3 programming languages"
```

**Expected Output:**
```json
{
  "content": "Here are 3 programming languages:\n1. Python...",
  "usage": {
    "input_tokens": 15,
    "output_tokens": 67
  },
  "cost": 0.000246,
  "model": "claude-3-5-sonnet-20240620"
}
```

3. **Markdown Output:**
```bash
cargo run -- run --format markdown "Explain async/await in 2 sentences"
```

**Expected Output:**
```markdown
# Response

Async/await is a programming pattern...

---
**Tokens:** 89
**Cost:** $0.000267
```

**Success Criteria:**
- ✅ All three output formats work
- ✅ Real API responses received
- ✅ Token and cost tracking accurate
- ✅ No errors or crashes

---

### Test 3: Session Management

**Goal:** Verify session creation, listing, and switching.

**Steps:**

1. Create first session and chat:
```bash
cargo run
# Type: "This is my first conversation"
# Ctrl+Enter to send
# Wait for response
# Ctrl+C to quit
```

2. Create second session:
```bash
cargo run
# Type: "This is a different conversation"
# Ctrl+Enter
# Wait for response
```

3. Press `Ctrl+L` to list sessions

4. **Expected:**
   - See list of 2 sessions
   - Each with creation timestamp
   - Current session highlighted
   - Can navigate with ↑/↓

5. Select the first session:
   - Press ↑ to highlight first session
   - Press Enter to switch

6. **Expected:**
   - Chat history from first session loads
   - Previous messages visible
   - Can continue conversation

7. Press `Ctrl+N` to create new session

8. **Expected:**
   - New empty session created
   - Chat area clears
   - Ready for new conversation

9. Check database:
```bash
cargo run -- db stats
```

**Expected:**
```
Sessions: 3
Messages: 4  (2 from first session, 2 from second)
Tracked files: 0
```

**Success Criteria:**
- ✅ Multiple sessions work independently
- ✅ Session switching preserves history
- ✅ New sessions create cleanly
- ✅ Database tracks all data

---

### Test 4: Cost and Token Tracking

**Goal:** Verify accurate tracking of API usage and costs.

**Steps:**

1. Start new session:
```bash
cargo run
```

2. Send a short message:
```
Hi
```

3. Note the tokens and cost in header

4. Send a longer message:
```
Can you write a detailed explanation of how Rust's ownership system works, including examples of borrowing, moving, and lifetimes? Please be comprehensive.
```

5. **Expected:**
   - Higher token count for longer message
   - Higher cost accumulated
   - Numbers update after each message

6. Quit and check database:
```bash
cargo run -- db stats
```

7. Start TUI again and check the session total in header

**Expected:**
- Session total tokens = sum of all message tokens
- Session total cost = sum of all message costs
- Numbers persist across restarts

**Success Criteria:**
- ✅ Token counting accurate
- ✅ Cost calculation correct
- ✅ Totals accumulate properly
- ✅ Data persists in database

---

### Test 5: Error Handling

**Goal:** Test various error conditions and recovery.

**Test 5.1: Invalid API Key**

```bash
# Set invalid key
export ANTHROPIC_API_KEY="invalid-key"

cargo run
```

**Expected:**
- Clear error message about invalid API key
- Application exits gracefully
- No crashes or panics

**Test 5.2: Network Failure**

```bash
# Disconnect from internet or block api.anthropic.com
cargo run -- run "test"
```

**Expected:**
- Timeout or connection error
- Error message suggests checking network
- No data corruption

**Test 5.3: Empty Database Path**

```bash
# Edit config to have empty database path
# Try to run
cargo run
```

**Expected:**
- Error about invalid config
- Helpful message about fixing config
- No crash

**Success Criteria:**
- ✅ All errors handled gracefully
- ✅ Error messages are helpful
- ✅ No crashes or data corruption
- ✅ Can recover from errors

---

### Test 6: Multi-Turn Conversation with Context

**Goal:** Verify Claude maintains context across messages.

**Steps:**

1. Start TUI:
```bash
cargo run
```

2. Message 1:
```
My favorite color is blue.
```

3. Message 2:
```
What's my favorite color?
```

4. **Expected Response:**
   - Claude correctly responds "Your favorite color is blue"
   - Shows it maintained context

5. Message 3:
```
If I mix my favorite color with yellow, what do I get?
```

6. **Expected Response:**
   - Claude knows favorite color is blue
   - Responds with "green"

7. Check message history is complete

**Success Criteria:**
- ✅ Context maintained across messages
- ✅ Claude references previous information
- ✅ Full conversation history visible
- ✅ No context loss

---

### Test 7: Keyboard Shortcuts

**Goal:** Verify all TUI keyboard shortcuts work.

**Shortcuts to Test:**

| Shortcut | Action | Test Method |
|----------|--------|-------------|
| `Ctrl+C` | Quit | Press and verify clean exit |
| `Ctrl+N` | New session | Creates new empty session |
| `Ctrl+L` | List sessions | Shows session list overlay |
| `Ctrl+H` | Help | Shows help screen |
| `Ctrl+Enter` | Send message | Sends current input |
| `Escape` | Clear input | Clears input box |
| `Page Up` | Scroll up | Chat scrolls up |
| `Page Down` | Scroll down | Chat scrolls down |
| `↑` | Navigate up (in session list) | Moves selection |
| `↓` | Navigate down (in session list) | Moves selection |
| `Enter` | Select (in session list) | Switches to session |

**Success Criteria:**
- ✅ All shortcuts respond correctly
- ✅ No conflicts or unexpected behavior
- ✅ Shortcuts work consistently

---

### Test 8: Long Conversation

**Goal:** Test stability with many messages.

**Steps:**

1. Start new session

2. Send 20-30 messages in conversation format:
   - Mix of short and long messages
   - Different topics
   - Some with code examples

3. Monitor:
   - Memory usage doesn't grow excessively
   - UI remains responsive
   - Scrolling works smoothly
   - All messages render correctly

4. Quit and restart

5. Verify:
   - All messages persisted
   - Can scroll through full history
   - UI loads quickly

**Success Criteria:**
- ✅ No performance degradation
- ✅ All messages persist correctly
- ✅ UI remains responsive
- ✅ No memory leaks

---

### Test 9: Unicode and Special Characters

**Goal:** Test handling of non-ASCII text.

**Test Messages:**

```
1. Emojis: Hello! 👋 How are you? 😊
2. Accents: Crème brûlée, naïve café
3. Math: ∑ ∫ √ π ∞ ≠ ≈
4. CJK: 你好 こんにちは 안녕하세요
5. RTL: مرحبا עברית
6. Code: `let x = "string";`
7. Special: @#$%^&*(){}[]<>
```

**Expected:**
- All characters display correctly
- No encoding errors
- Claude responds appropriately
- Database stores correctly

**Success Criteria:**
- ✅ Unicode handled properly
- ✅ Display is correct
- ✅ No data corruption
- ✅ Retrieval works

---

## Performance Benchmarks

### Typical Response Times

| Operation | Expected Time | Notes |
|-----------|---------------|-------|
| TUI Launch | < 1 second | Database connection + UI init |
| Simple Message | 1-3 seconds | API latency dependent |
| Complex Message | 3-10 seconds | Longer responses take more time |
| Session Switch | < 500ms | Database query + UI update |
| Database Stats | < 100ms | Simple SQL queries |

### Cost Expectations

| Message Type | Approx. Tokens | Approx. Cost (Sonnet) |
|--------------|----------------|----------------------|
| Short (1-10 words) | 20-50 | $0.00006-$0.00015 |
| Medium (50-100 words) | 100-200 | $0.0003-$0.0006 |
| Long (500+ words) | 500-1000+ | $0.0015-$0.003+ |

*Costs as of October 2024 for Claude 3.5 Sonnet*

---

## Troubleshooting Common Issues

### Issue: TUI doesn't launch

**Check:**
```bash
# Verify API key
echo $ANTHROPIC_API_KEY

# Check database
cargo run -- db stats

# Try with debug logging
RUST_LOG=debug cargo run
```

### Issue: "API key not set" error

**Solution:**
```bash
# Set the environment variable
export ANTHROPIC_API_KEY="your-key-here"

# Or add to config file
cargo run -- init
# Edit ~/.config/crustly/config.toml
# Add: api_key = "your-key-here" under [providers.anthropic]
```

### Issue: Database errors

**Solution:**
```bash
# Reinitialize database
cargo run -- db init

# Or delete and recreate
rm ~/.local/share/crustly/crustly.db
cargo run -- db init
```

### Issue: Slow responses

**Check:**
- Internet connection speed
- Anthropic API status: https://status.anthropic.com/
- Message length (longer = slower)
- Try with simpler messages

### Issue: UI glitches

**Try:**
- Resize terminal window
- Restart application
- Check terminal emulator compatibility
- Try different terminal (iTerm2, Windows Terminal, etc.)

---

## Test Report Template

After completing all tests, fill out this report:

```markdown
# Crustly Test Report

**Date:** YYYY-MM-DD
**Tester:** [Your Name]
**Version:** [Git commit hash or version]
**Environment:** [OS, Rust version]

## Test Results Summary

| Test # | Test Name | Status | Notes |
|--------|-----------|--------|-------|
| 1 | Simple Interactive Chat | ✅/❌ | |
| 2 | Non-Interactive Mode | ✅/❌ | |
| 3 | Session Management | ✅/❌ | |
| 4 | Cost/Token Tracking | ✅/❌ | |
| 5 | Error Handling | ✅/❌ | |
| 6 | Multi-Turn Context | ✅/❌ | |
| 7 | Keyboard Shortcuts | ✅/❌ | |
| 8 | Long Conversation | ✅/❌ | |
| 9 | Unicode/Special Chars | ✅/❌ | |

## Issues Found

1. **Issue:** [Description]
   - **Severity:** Critical/High/Medium/Low
   - **Steps to Reproduce:**
   - **Expected:** [What should happen]
   - **Actual:** [What actually happened]

## Performance Observations

- TUI Launch Time: [X seconds]
- Average Response Time: [X seconds]
- Memory Usage: [X MB]
- Database Size after tests: [X MB]

## API Usage

- Total Messages Sent: [X]
- Total Tokens Used: [X]
- Total Cost: $[X.XX]

## Recommendations

[Any suggestions for improvements]

## Screenshots/Logs

[Attach relevant screenshots or log files]
```

---

## Automated Testing

After manual testing, run the full automated test suite:

```bash
# Run all tests
cargo test

# Run only integration tests
cargo test --test integration_test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_end_to_end_simple_message
```

**Expected:** All tests pass (139 tests)

---

## Cleanup

After testing:

```bash
# Optional: Delete test database
rm ~/.local/share/crustly/crustly.db

# Optional: Remove config
rm ~/.config/crustly/config.toml
```

---

## Success Criteria for Release

Before considering the application production-ready:

- ✅ All 9 manual test scenarios pass
- ✅ All 139 automated tests pass
- ✅ No crashes or data corruption
- ✅ Error messages are helpful
- ✅ Performance is acceptable
- ✅ Unicode/internationalization works
- ✅ Keyboard shortcuts all work
- ✅ Session management reliable
- ✅ Cost tracking accurate
- ✅ Database persistence solid

---

## Next Steps After Testing

1. **Document any issues** found during testing
2. **Create GitHub issues** for bugs
3. **Update documentation** based on findings
4. **Add more automated tests** for edge cases discovered
5. **Consider CI/CD** integration for continuous testing
6. **Benchmark performance** under heavy load
7. **Security audit** of API key handling
8. **Accessibility testing** for terminal compatibility
