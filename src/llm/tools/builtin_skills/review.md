---
name: review
description: Multi-pass automated code review of the current diff, a PR, a branch, or a path — checks for correctness bugs and CLAUDE.md/AGENTS.md compliance, filters findings by confidence, and optionally posts a summary or applies fixes.
---

# Review

Review a change for real defects and project-guideline violations, using
independent passes so a single agent's blind spot doesn't become the whole
review. Only report findings that survive a second, skeptical look — a
review that cries wolf gets ignored.

## Step 1: Determine the target

Figure out what to review, in this priority order:

1. An explicit argument was given — a PR number, a branch name, or a file
   path. Resolve a PR number with `bash` (`gh pr view <n> --json ...`,
   `gh pr diff <n>`); resolve a branch with `git diff <base>...<branch>`.
2. No argument, but the current branch has an open PR against its
   upstream — use `git diff <upstream>...HEAD`.
3. Otherwise fall back to whatever is actually different: working-tree
   changes (`git diff HEAD`), or if that's empty, the last commit
   (`git diff HEAD~1`).

If none of these produce a non-empty diff, say so plainly and stop — do
not review unrelated code just to have something to say.

## Step 2: Gather project guidelines

Use `glob` to find `CLAUDE.md` and `AGENTS.md` at the repo root and in any
directory that contains a changed file (or one of its parents). Read the
ones that exist with `read`. These are the compliance bar for Step 4 —
skip this step's findings entirely if no such files exist.

## Step 3: Summarize the change

Read the diff and write a short (3-6 sentence) summary of what changed and
why, inferred from the diff and any PR title/description. This summary is
handed to every review pass in Step 4 so agents that only see a diff still
have intent to check against.

## Step 4: Independent review passes

Dispatch review passes in parallel with the `agent` tool rather than doing
them serially in this same context — each pass should see only the diff,
the Step 3 summary, and (for the guideline pass) the Step 2 files, so its
verdict isn't anchored by another pass's framing:

- **Correctness pass** — read the diff for logic errors, off-by-one bugs,
  unhandled error cases, and anything that would not compile or would
  panic. Only flag problems inside the changed lines; pre-existing issues
  nearby are out of scope.
- **Guideline-compliance pass** — check the diff against the CLAUDE.md /
  AGENTS.md files gathered in Step 2. Only flag a violation you can quote
  the exact rule for; skip anything you'd have to infer or paraphrase the
  rule to justify.
- **Security pass** — check for injection, path traversal, unsafe
  deserialization, secrets in code, and missing input validation in the
  changed lines only.

Run two or three passes when the diff is small; for a large diff, split
it by file or module across more parallel agents instead of asking one
agent to hold the whole thing in mind.

Every pass reports findings as `{file, line, summary, why}` — no finding
without a concrete file and line.

## Step 5: Filter by confidence

Findings are cheap to produce and expensive to trust. For each finding
from Step 4, spawn a second, independent agent whose only job is to try to
disprove it — given just the finding and the relevant few lines of diff,
does it hold up? Drop anything that doesn't survive this pass, and drop
anything where the two passes disagree on severity. Prefer reporting
nothing over reporting a plausible-sounding false positive.

## Step 6: Report

- No findings survived: say so in one line — do not manufacture minor
  nitpicks to justify the review having run.
- Findings survived: list each with its file:line, a one-sentence summary,
  and the concrete failure it causes (bad input, wrong output, crash) —
  not a vague "could be improved."

Stop here unless `--comment` or `--fix` was passed.

## Optional: `--comment`

Post the Step 6 report as a single summary comment via `bash` (`gh pr
comment <n> --body-file -`), scoped to whatever `gh pr` subcommands are on
the `security.allow_bash` allowlist. This posts one summary comment, not
per-line inline comments — do not attempt line-anchored comments without a
dedicated GitHub review-comment tool.

## Optional: `--fix`

For findings from Step 6 that have an unambiguous, self-contained fix, apply
it with `edit` or `apply_patch` and note in the final summary which
findings were fixed automatically versus left for manual attention.
Findings that require an architectural decision are never auto-fixed —
report them instead.
