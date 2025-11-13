# Final Diagnostic Test for Plan Mode

## Your Symptoms

1. ✅ Ctrl+P works - switches to Plan Mode
2. ❌ In Plan Mode, Ctrl+A and Ctrl+R type 'a' and 'r' instead of working

This is strange because Ctrl+P works but Ctrl+A/R don't!

## Test to Run

```bash
cd /home/user/crustly
RUST_LOG=crustly=debug ./target/release/crustly chat 2>&1 | tee /tmp/plan_diagnostic.log
```

## Steps:

### 1. Create a Plan
```
create a plan to add hello world
```
Press **Ctrl+Enter**

### 2. Switch to Plan Mode
Press **Ctrl+P**

### 3. Check What You See
- Status bar should show: `[PLAN]`
- You should see your plan with tasks listed
- **Is there an input box visible?** (There shouldn't be!)

### 4. Try the Shortcuts
While in Plan Mode (`[PLAN]` in status bar):

**A. Press Ctrl+A**
- Watch the console for: `🔑 Plan Mode Key: code=Char('a'), modifiers=CONTROL`
- Watch for: `✅ Ctrl+A pressed - Approving plan`
- **WHERE does the letter 'a' appear?** (input box? somewhere else?)

**B. Press Ctrl+R**
- Watch for similar messages
- **WHERE does the letter 'r' appear?**

### 5. Exit
Press **Ctrl+C**

## Questions to Answer

After running the test, tell me:

1. **In Plan Mode, do you see an input box?**
   - Yes / No
   - If yes, where? (bottom of screen?)

2. **When pressing Ctrl+A in Plan Mode:**
   - Do you see `🔑 Plan Mode Key:` in the log?
   - Do you see `✅ Ctrl+A pressed` in the log?
   - Where does the 'a' appear on screen?

3. **Log output:**
   ```bash
   grep -E "🔑|✅|❌|🔄|Plan Mode" /tmp/plan_diagnostic.log | tail -20
   ```
   Share this output

## What This Will Tell Us

**Scenario A: Keys not detected**
- No `🔑 Plan Mode Key` messages
- → Terminal/OS issue (but then why does Ctrl+P work?)

**Scenario B: Keys detected but not matched**
- See `🔑 Plan Mode Key: code=Char('a'), modifiers=CONTROL`
- But NO `✅ Ctrl+A pressed`
- → Key matching logic issue

**Scenario C: Keys detected and matched**
- See both `🔑` and `✅` messages
- But still types 'a'
- → Something else is capturing the key after handling

**Scenario D: Wrong mode**
- See `🔑 Plan Mode Key` with modifiers=empty (no CONTROL)
- → Ctrl not being passed through for some keys

## Expected Behavior

In Plan Mode, you should:
- See NO input box
- Keys should be logged with 🔑
- Ctrl+A should show ✅ and approve the plan
- NO text should be typed anywhere

---

Run this test and share the results!
