//! Apply Patch Tool (Codex-compatible)
//!
//! Implements OpenAI Codex's `apply_patch` tool: a single string argument
//! containing a custom multi-file patch script (verified against
//! `codex-rs/apply-patch/src/lib.rs`), distinct from `edit_file`'s
//! old_string/new_string format:
//!
//! ```text
//! *** Begin Patch
//! *** Update File: path/to/file.rs
//! *** Move to: path/to/renamed.rs
//! @@ optional context marker (e.g. a function name), purely for readability
//!  unchanged context line
//! -line to remove
//! +line to add
//!  unchanged context line
//! *** Add File: path/to/new_file.rs
//! +line of the new file
//! +another line
//! *** Delete File: path/to/old_file.rs
//! *** End Patch
//! ```
//!
//! A single call may touch multiple files. `Update File` hunks are matched
//! by content (the contiguous run of context + removed lines), not line
//! numbers - the same context-based matching `edit_file`'s `old_string`
//! uses, just scoped to a hunk instead of the whole file. All hunks across
//! all files are validated (and, for updates, matched against their
//! target's current content) before anything is written, so a patch either
//! applies completely or leaves the filesystem untouched - a model that
//! sends a five-file patch shouldn't end up with three files changed and
//! two left stale because hunk four didn't match.
//!
//! Deliberately not implemented (real Codex patches rarely need them, and
//! getting them wrong silently would be worse than not having them):
//! whitespace-fuzzy hunk matching, and the `*** End of File` hunk marker.

use super::error::{validate_file_path, validate_path_safety, Result, ToolError};
use super::file_read_cache::{FileFingerprint, ReadGate};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::path::PathBuf;
use tokio::fs;

pub struct ApplyPatchTool;

#[derive(Debug, Deserialize)]
struct ApplyPatchInput {
    /// The entire patch script, including the `*** Begin Patch`/`*** End Patch` wrapper.
    input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HunkLine {
    Context(String),
    Remove(String),
    Add(String),
}

#[derive(Debug, Clone, Default)]
struct Hunk {
    lines: Vec<HunkLine>,
}

#[derive(Debug, Clone)]
enum FileOp {
    Add {
        path: String,
        content: String,
    },
    Delete {
        path: String,
    },
    Update {
        path: String,
        move_to: Option<String>,
        hunks: Vec<Hunk>,
    },
}

const BEGIN: &str = "*** Begin Patch";
const END: &str = "*** End Patch";
const ADD_PREFIX: &str = "*** Add File: ";
const DELETE_PREFIX: &str = "*** Delete File: ";
const UPDATE_PREFIX: &str = "*** Update File: ";
const MOVE_PREFIX: &str = "*** Move to: ";

/// Parse a full `*** Begin Patch` ... `*** End Patch` script into a list of
/// file operations, in the order they appear.
fn parse_patch(text: &str) -> std::result::Result<Vec<FileOp>, String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;

    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    if i >= lines.len() || lines[i].trim() != BEGIN {
        return Err(format!("Patch must start with '{BEGIN}'"));
    }
    i += 1;

    let mut ops = Vec::new();

    loop {
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        if i >= lines.len() {
            return Err(format!("Patch is missing the closing '{END}' marker"));
        }
        if lines[i].trim() == END {
            break;
        }

        if let Some(path) = lines[i].strip_prefix(ADD_PREFIX) {
            let path = path.trim().to_string();
            i += 1;
            let mut content_lines = Vec::new();
            while i < lines.len() && !lines[i].starts_with("*** ") {
                let l = lines[i];
                if let Some(rest) = l.strip_prefix('+') {
                    content_lines.push(rest.to_string());
                } else if l.is_empty() {
                    content_lines.push(String::new());
                } else {
                    return Err(format!(
                        "Add File '{path}' body line must start with '+': {l:?}"
                    ));
                }
                i += 1;
            }
            ops.push(FileOp::Add {
                path,
                content: content_lines.join("\n"),
            });
        } else if let Some(path) = lines[i].strip_prefix(DELETE_PREFIX) {
            let path = path.trim().to_string();
            i += 1;
            ops.push(FileOp::Delete { path });
        } else if let Some(path) = lines[i].strip_prefix(UPDATE_PREFIX) {
            let path = path.trim().to_string();
            i += 1;

            let move_to = if i < lines.len() {
                lines[i]
                    .strip_prefix(MOVE_PREFIX)
                    .map(|p| p.trim().to_string())
            } else {
                None
            };
            if move_to.is_some() {
                i += 1;
            }

            let mut hunks = Vec::new();
            while i < lines.len() && !lines[i].starts_with("*** ") {
                if !lines[i].starts_with("@@") {
                    return Err(format!(
                        "Update File '{path}': expected a '@@' hunk header, got {:?}",
                        lines[i]
                    ));
                }
                i += 1;
                let mut hunk = Hunk::default();
                while i < lines.len() && !lines[i].starts_with("@@") && !lines[i].starts_with("*** ")
                {
                    let l = lines[i];
                    if let Some(rest) = l.strip_prefix('+') {
                        hunk.lines.push(HunkLine::Add(rest.to_string()));
                    } else if let Some(rest) = l.strip_prefix('-') {
                        hunk.lines.push(HunkLine::Remove(rest.to_string()));
                    } else if let Some(rest) = l.strip_prefix(' ') {
                        hunk.lines.push(HunkLine::Context(rest.to_string()));
                    } else if l.is_empty() {
                        hunk.lines.push(HunkLine::Context(String::new()));
                    } else {
                        return Err(format!(
                            "Update File '{path}': hunk line must start with ' ', '-', or '+': {l:?}"
                        ));
                    }
                    i += 1;
                }
                if hunk.lines.is_empty() {
                    return Err(format!("Update File '{path}': empty hunk"));
                }
                hunks.push(hunk);
            }
            if hunks.is_empty() {
                return Err(format!(
                    "Update File '{path}' has no hunks (expected at least one '@@' block)"
                ));
            }
            ops.push(FileOp::Update {
                path,
                move_to,
                hunks,
            });
        } else {
            return Err(format!(
                "Expected '*** Add File:', '*** Delete File:', '*** Update File:', or '{END}', got: {:?}",
                lines[i]
            ));
        }
    }

    if ops.is_empty() {
        return Err("Patch contains no file operations".to_string());
    }

    Ok(ops)
}

/// Find `needle` as a contiguous run within `haystack`, searching from
/// `start` onward (hunks within one file are matched in order, so later
/// hunks never match earlier text).
fn find_subsequence(haystack: &[String], needle: &[&str], start: usize) -> Option<usize> {
    if needle.is_empty() {
        return Some(start.min(haystack.len()));
    }
    if start + needle.len() > haystack.len() {
        return None;
    }
    (start..=haystack.len() - needle.len())
        .find(|&pos| (0..needle.len()).all(|k| haystack[pos + k] == needle[k]))
}

/// Apply a sequence of hunks to `original`, returning the new content.
/// Hunks are applied in order, each searched for starting at the position
/// the previous hunk's replacement ended - so identical text appearing
/// earlier in the file is never matched by a later hunk.
fn apply_hunks(original: &str, hunks: &[Hunk]) -> std::result::Result<String, String> {
    let mut lines: Vec<String> = original.lines().map(String::from).collect();
    let had_trailing_newline = original.is_empty() || original.ends_with('\n');
    let mut cursor = 0usize;

    for (idx, hunk) in hunks.iter().enumerate() {
        let old_seq: Vec<&str> = hunk
            .lines
            .iter()
            .filter_map(|l| match l {
                HunkLine::Context(s) | HunkLine::Remove(s) => Some(s.as_str()),
                HunkLine::Add(_) => None,
            })
            .collect();
        let new_seq: Vec<String> = hunk
            .lines
            .iter()
            .filter_map(|l| match l {
                HunkLine::Context(s) => Some(s.clone()),
                HunkLine::Add(s) => Some(s.clone()),
                HunkLine::Remove(_) => None,
            })
            .collect();

        if old_seq.is_empty() {
            // Pure insertion (no context/removed lines) - insert at cursor.
            for (offset, s) in new_seq.iter().enumerate() {
                lines.insert(cursor + offset, s.clone());
            }
            cursor += new_seq.len();
            continue;
        }

        let pos = find_subsequence(&lines, &old_seq, cursor).ok_or_else(|| {
            format!(
                "hunk #{} could not be matched (its context/removed lines were not found \
                 in order from where the previous hunk left off)",
                idx + 1
            )
        })?;

        lines.splice(pos..pos + old_seq.len(), new_seq.iter().cloned());
        cursor = pos + new_seq.len();
    }

    let mut result = lines.join("\n");
    if had_trailing_newline && !result.is_empty() {
        result.push('\n');
    }
    Ok(result)
}

/// A validated, ready-to-apply filesystem action, computed in a read-only
/// pass over every operation in the patch before anything is written.
enum PlannedAction {
    Write { path: PathBuf, content: String },
    Delete { path: PathBuf },
}

#[async_trait]
impl Tool for ApplyPatchTool {
    fn name(&self) -> &str {
        "apply_patch"
    }

    fn description(&self) -> &str {
        "Apply a multi-file patch (OpenAI Codex's apply_patch format). The `input` argument is \
         the entire patch script: `*** Begin Patch`, one or more `*** Add File: <path>` / \
         `*** Delete File: <path>` / `*** Update File: <path>` sections (Update sections contain \
         `@@` hunks with ' ' context, '-' removed, and '+' added lines; may be followed by \
         `*** Move to: <new path>` to rename), then `*** End Patch`. The whole patch is validated \
         against the current file contents before anything is written - if any hunk fails to \
         match, no files are changed. Every `Update File` target must have been read with \
         read_file at least once in this session first; `Add File`/`Delete File` need no prior read."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "The entire apply_patch script, including the '*** Begin Patch' / '*** End Patch' wrapper."
                }
            },
            "required": ["input"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![
            ToolCapability::ReadFiles,
            ToolCapability::WriteFiles,
            ToolCapability::SystemModification,
        ]
    }

    fn requires_approval(&self) -> bool {
        true
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: ApplyPatchInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        parse_patch(&input.input).map_err(ToolError::InvalidInput)?;
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        if context.read_only_mode {
            return Ok(ToolResult::error(
                "apply_patch is not allowed in Plan mode. Please approve the plan and switch to \
                 execution mode (Ctrl+A) to modify files."
                    .to_string(),
            ));
        }

        let input: ApplyPatchInput = serde_json::from_value(input)?;
        let ops = match parse_patch(&input.input) {
            Ok(ops) => ops,
            Err(msg) => return Ok(ToolResult::error(format!("Invalid patch: {msg}"))),
        };

        // Phase 1: validate every operation and compute its resulting
        // content without writing anything, so a bad hunk in file N of an
        // N-file patch can't leave files 1..N-1 already modified.
        let mut planned = Vec::with_capacity(ops.len());
        for op in &ops {
            match op {
                FileOp::Add { path, content } => {
                    if let Err(reason) =
                        crate::llm::tools::sandbox::check_path(path, &context.working_directory)
                    {
                        return Ok(ToolResult::error(reason));
                    }
                    let full = match validate_path_safety(path, &context.working_directory) {
                        Ok(p) => p,
                        Err(e) => {
                            return Ok(ToolResult::error(format!("Add File '{path}': {e}")))
                        }
                    };
                    if full.exists() {
                        return Ok(ToolResult::error(format!(
                            "Add File '{path}': already exists - use Update File to modify it"
                        )));
                    }
                    planned.push(PlannedAction::Write {
                        path: full,
                        content: content.clone(),
                    });
                }
                FileOp::Delete { path } => {
                    if let Err(reason) =
                        crate::llm::tools::sandbox::check_path(path, &context.working_directory)
                    {
                        return Ok(ToolResult::error(reason));
                    }
                    let full = match validate_file_path(path, &context.working_directory) {
                        Ok(p) => p,
                        Err(msg) => {
                            return Ok(ToolResult::error(format!("Delete File '{path}': {msg}")))
                        }
                    };
                    planned.push(PlannedAction::Delete { path: full });
                }
                FileOp::Update {
                    path,
                    move_to,
                    hunks,
                } => {
                    if let Err(reason) =
                        crate::llm::tools::sandbox::check_path(path, &context.working_directory)
                    {
                        return Ok(ToolResult::error(reason));
                    }
                    let full = match validate_file_path(path, &context.working_directory) {
                        Ok(p) => p,
                        Err(msg) => {
                            return Ok(ToolResult::error(format!("Update File '{path}': {msg}")))
                        }
                    };

                    // Prior-read enforcement (matches Claude Code's/
                    // qwen-code's edit tools): only applies to Update File,
                    // not Add File (nothing existing to have read) or
                    // Delete File (no content is being trusted/modified).
                    let metadata_before = fs::metadata(&full).await.map_err(ToolError::Io)?;
                    match context
                        .file_read_cache
                        .check(&full, FileFingerprint::of(&metadata_before))
                    {
                        ReadGate::NeverRead => {
                            return Ok(ToolResult::error(format!(
                                "Update File '{path}': you must read this file with read_file \
                                 before editing it."
                            )));
                        }
                        ReadGate::Stale => {
                            return Ok(ToolResult::error(format!(
                                "Update File '{path}': it has changed on disk since it was \
                                 last read. Re-read it with read_file before editing."
                            )));
                        }
                        ReadGate::Ok => {}
                    }

                    let original = fs::read_to_string(&full).await.map_err(ToolError::Io)?;
                    let new_content = match apply_hunks(&original, hunks) {
                        Ok(c) => c,
                        Err(msg) => {
                            return Ok(ToolResult::error(format!("Update File '{path}': {msg}")))
                        }
                    };

                    match move_to {
                        Some(new_path) => {
                            if let Err(reason) = crate::llm::tools::sandbox::check_path(
                                new_path,
                                &context.working_directory,
                            ) {
                                return Ok(ToolResult::error(reason));
                            }
                            let new_full =
                                match validate_path_safety(new_path, &context.working_directory) {
                                    Ok(p) => p,
                                    Err(e) => {
                                        return Ok(ToolResult::error(format!(
                                            "Update File '{path}' Move to '{new_path}': {e}"
                                        )))
                                    }
                                };
                            if new_full.exists() {
                                return Ok(ToolResult::error(format!(
                                    "Update File '{path}' Move to '{new_path}': target already exists"
                                )));
                            }
                            planned.push(PlannedAction::Delete { path: full });
                            planned.push(PlannedAction::Write {
                                path: new_full,
                                content: new_content,
                            });
                        }
                        None => {
                            planned.push(PlannedAction::Write {
                                path: full,
                                content: new_content,
                            });
                        }
                    }
                }
            }
        }

        // Phase 2: everything validated - apply it.
        let mut summary = Vec::with_capacity(planned.len());
        for action in planned {
            match action {
                PlannedAction::Write { path, content } => {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent).await.map_err(ToolError::Io)?;
                    }
                    fs::write(&path, &content).await.map_err(ToolError::Io)?;

                    // Seed the cache with the post-write fingerprint (new
                    // file, updated file, or a rename's destination) so a
                    // follow-up edit in the same session doesn't need an
                    // intervening re-read.
                    let metadata_after = fs::metadata(&path).await.map_err(ToolError::Io)?;
                    context
                        .file_read_cache
                        .record(&path, FileFingerprint::of(&metadata_after));

                    summary.push(format!("wrote {}", path.display()));
                }
                PlannedAction::Delete { path } => {
                    fs::remove_file(&path).await.map_err(ToolError::Io)?;
                    summary.push(format!("deleted {}", path.display()));
                }
            }
        }

        Ok(ToolResult::success(format!(
            "Applied patch:\n{}",
            summary.join("\n")
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn context(temp_dir: &TempDir) -> ToolExecutionContext {
        ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf())
    }

    /// A context with `relative_path` already recorded in the file-read
    /// cache, simulating "the model read this file earlier in the
    /// session" - needed before any `Update File` op, which (like
    /// edit_file) requires a prior read. `Add`/`Delete File` need no such
    /// seeding.
    async fn seeded_context(temp_dir: &TempDir, relative_path: &str) -> ToolExecutionContext {
        let ctx = context(temp_dir);
        let full_path = temp_dir.path().join(relative_path);
        let metadata = fs::metadata(&full_path).await.unwrap();
        ctx.file_read_cache
            .record(&full_path, FileFingerprint::of(&metadata));
        ctx
    }

    #[test]
    fn parse_rejects_missing_begin_marker() {
        let err = parse_patch("*** Update File: a.txt\n*** End Patch").unwrap_err();
        assert!(err.contains("Begin Patch"));
    }

    #[test]
    fn parse_rejects_missing_end_marker() {
        let err = parse_patch("*** Begin Patch\n*** Add File: a.txt\n+hi").unwrap_err();
        assert!(err.contains("End Patch"));
    }

    #[test]
    fn parse_add_file_collects_plus_prefixed_lines() {
        let text = "*** Begin Patch\n*** Add File: a.txt\n+line one\n+line two\n*** End Patch";
        let ops = parse_patch(text).unwrap();
        match &ops[0] {
            FileOp::Add { path, content } => {
                assert_eq!(path, "a.txt");
                assert_eq!(content, "line one\nline two");
            }
            other => panic!("expected Add, got {other:?}"),
        }
    }

    #[test]
    fn parse_multiple_file_ops_in_one_patch() {
        let text = "*** Begin Patch\n\
             *** Add File: new.txt\n+hello\n\
             *** Delete File: old.txt\n\
             *** Update File: existing.txt\n@@\n-old\n+new\n\
             *** End Patch";
        let ops = parse_patch(text).unwrap();
        assert_eq!(ops.len(), 3);
        assert!(matches!(&ops[0], FileOp::Add { .. }));
        assert!(matches!(&ops[1], FileOp::Delete { .. }));
        assert!(matches!(&ops[2], FileOp::Update { .. }));
    }

    #[test]
    fn parse_update_with_move_to() {
        let text =
            "*** Begin Patch\n*** Update File: old.txt\n*** Move to: new.txt\n@@\n-a\n+b\n*** End Patch";
        let ops = parse_patch(text).unwrap();
        match &ops[0] {
            FileOp::Update { path, move_to, .. } => {
                assert_eq!(path, "old.txt");
                assert_eq!(move_to.as_deref(), Some("new.txt"));
            }
            other => panic!("expected Update, got {other:?}"),
        }
    }

    #[test]
    fn apply_hunks_replaces_matched_context() {
        let original = "one\ntwo\nthree\n";
        let hunks = vec![Hunk {
            lines: vec![
                HunkLine::Context("one".to_string()),
                HunkLine::Remove("two".to_string()),
                HunkLine::Add("TWO".to_string()),
                HunkLine::Context("three".to_string()),
            ],
        }];
        let result = apply_hunks(original, &hunks).unwrap();
        assert_eq!(result, "one\nTWO\nthree\n");
    }

    #[test]
    fn apply_hunks_second_hunk_searches_after_first() {
        // "dup" appears twice; two hunks target each occurrence in order.
        let original = "dup\na\ndup\nb\n";
        let hunks = vec![
            Hunk {
                lines: vec![
                    HunkLine::Remove("dup".to_string()),
                    HunkLine::Add("FIRST".to_string()),
                ],
            },
            Hunk {
                lines: vec![
                    HunkLine::Context("a".to_string()),
                    HunkLine::Remove("dup".to_string()),
                    HunkLine::Add("SECOND".to_string()),
                ],
            },
        ];
        let result = apply_hunks(original, &hunks).unwrap();
        assert_eq!(result, "FIRST\na\nSECOND\nb\n");
    }

    #[test]
    fn apply_hunks_errors_when_context_not_found() {
        let original = "one\ntwo\nthree\n";
        let hunks = vec![Hunk {
            lines: vec![HunkLine::Remove("nonexistent".to_string())],
        }];
        let err = apply_hunks(original, &hunks).unwrap_err();
        assert!(err.contains("hunk #1"));
    }

    #[tokio::test]
    async fn execute_updates_an_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("a.txt"), "hello\nworld\n")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n*** Update File: a.txt\n@@\n-hello\n+goodbye\n world\n*** End Patch"
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "a.txt").await)
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        let contents = tokio::fs::read_to_string(temp_dir.path().join("a.txt"))
            .await
            .unwrap();
        assert_eq!(contents, "goodbye\nworld\n");
    }

    #[tokio::test]
    async fn execute_adds_a_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n*** Add File: new.txt\n+line one\n+line two\n*** End Patch"
        });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        let contents = tokio::fs::read_to_string(temp_dir.path().join("new.txt"))
            .await
            .unwrap();
        assert_eq!(contents, "line one\nline two");
    }

    #[tokio::test]
    async fn execute_add_file_that_already_exists_fails() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("exists.txt"), "already here")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n*** Add File: exists.txt\n+new content\n*** End Patch"
        });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("already exists"));
        // Must not have overwritten the file.
        assert_eq!(
            tokio::fs::read_to_string(temp_dir.path().join("exists.txt"))
                .await
                .unwrap(),
            "already here"
        );
    }

    #[tokio::test]
    async fn execute_deletes_a_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("gone.txt");
        tokio::fs::write(&path, "bye").await.unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n*** Delete File: gone.txt\n*** End Patch"
        });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn execute_renames_via_move_to() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("old.txt"), "content\n")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n*** Update File: old.txt\n*** Move to: new.txt\n@@\n-content\n+updated\n*** End Patch"
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "old.txt").await)
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(!temp_dir.path().join("old.txt").exists());
        assert_eq!(
            tokio::fs::read_to_string(temp_dir.path().join("new.txt"))
                .await
                .unwrap(),
            "updated\n"
        );
    }

    #[tokio::test]
    async fn execute_applies_multiple_file_ops_in_one_patch() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("update.txt"), "before\n")
            .await
            .unwrap();
        tokio::fs::write(temp_dir.path().join("delete.txt"), "x")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n\
                 *** Add File: add.txt\n+new\n\
                 *** Delete File: delete.txt\n\
                 *** Update File: update.txt\n@@\n-before\n+after\n\
                 *** End Patch"
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "update.txt").await)
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(temp_dir.path().join("add.txt").exists());
        assert!(!temp_dir.path().join("delete.txt").exists());
        assert_eq!(
            tokio::fs::read_to_string(temp_dir.path().join("update.txt"))
                .await
                .unwrap(),
            "after\n"
        );
    }

    /// Regression: a patch touching multiple files where a later hunk
    /// fails to match must leave EVERY file untouched, not just the one
    /// that failed - otherwise a partially-applied multi-file patch leaves
    /// the tree in a state the model never asked for and doesn't know
    /// about.
    #[tokio::test]
    async fn execute_is_atomic_across_files_on_failure() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("first.txt"), "before\n")
            .await
            .unwrap();
        tokio::fs::write(temp_dir.path().join("second.txt"), "unrelated\n")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n\
                 *** Update File: first.txt\n@@\n-before\n+after\n\
                 *** Update File: second.txt\n@@\n-this text does not exist\n+replacement\n\
                 *** End Patch"
        });

        let ctx = seeded_context(&temp_dir, "first.txt").await;
        let metadata = fs::metadata(temp_dir.path().join("second.txt"))
            .await
            .unwrap();
        ctx.file_read_cache
            .record(&temp_dir.path().join("second.txt"), FileFingerprint::of(&metadata));

        let result = tool.execute(input, &ctx).await.unwrap();
        assert!(!result.success);
        // Neither file may have been modified.
        assert_eq!(
            tokio::fs::read_to_string(temp_dir.path().join("first.txt"))
                .await
                .unwrap(),
            "before\n",
            "first.txt must be untouched when a later hunk in the same patch fails"
        );
        assert_eq!(
            tokio::fs::read_to_string(temp_dir.path().join("second.txt"))
                .await
                .unwrap(),
            "unrelated\n"
        );
    }

    #[tokio::test]
    async fn execute_blocked_in_read_only_mode() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("a.txt"), "x\n")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let mut ctx = context(&temp_dir);
        ctx.read_only_mode = true;

        let input = serde_json::json!({
            "input": "*** Begin Patch\n*** Update File: a.txt\n@@\n-x\n+y\n*** End Patch"
        });

        let result = tool.execute(input, &ctx).await.unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("Plan mode"));
    }

    #[test]
    fn validate_input_rejects_malformed_patch() {
        let tool = ApplyPatchTool;
        let input = serde_json::json!({ "input": "not a patch at all" });
        assert!(tool.validate_input(&input).is_err());
    }

    /// Regression: Update File must not blindly rewrite a file this
    /// session never read - matches edit_file's own prior-read enforcement.
    #[tokio::test]
    async fn execute_update_rejects_a_file_never_read_this_session() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("a.txt"), "hello\n")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n*** Update File: a.txt\n@@\n-hello\n+goodbye\n*** End Patch"
        });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("must read"));
        assert_eq!(
            tokio::fs::read_to_string(temp_dir.path().join("a.txt"))
                .await
                .unwrap(),
            "hello\n"
        );
    }

    /// Add File and Delete File need no prior read - only Update File does.
    #[tokio::test]
    async fn execute_add_and_delete_need_no_prior_read() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("gone.txt"), "bye")
            .await
            .unwrap();

        let tool = ApplyPatchTool;
        let input = serde_json::json!({
            "input": "*** Begin Patch\n\
                 *** Add File: new.txt\n+hi\n\
                 *** Delete File: gone.txt\n\
                 *** End Patch"
        });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(result.success, "{:?}", result.error);
    }
}
