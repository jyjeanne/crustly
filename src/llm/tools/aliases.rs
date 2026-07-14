//! Tool name aliases.
//!
//! Coding agents don't agree on tool names for the same capability, and a
//! model trained against one agent's tool surface calls it by that agent's
//! name regardless of which agent is actually running it. `ToolRegistry`
//! falls back to this table when an exact name match misses, so a call for
//! e.g. `list_directory` (qwen-code's name for what Crustly calls `ls`)
//! still resolves instead of failing with `ToolError::NotFound`.
//!
//! Entries are `(alias, canonical)`, matched case-insensitively so both
//! qwen-code's snake_case (`list_directory`) and Claude Code's PascalCase
//! (`Read`, `TodoWrite`) forms work without listing every case variant.
//!
//! Only maps to tools Crustly actually has a real equivalent for, and only
//! where the argument *shape* AND *behavior* also line up (verified against
//! the source of qwen-code and Crustly's own tool schemas) - a name for a
//! capability Crustly doesn't implement, or whose input format or semantics
//! genuinely differ, is intentionally left out rather than pointed at
//! something that would deserialize fine but quietly do the wrong thing.
//!
//! Two tools are absent from this table entirely, by design, because they
//! turned out to need real implementations rather than a rename:
//! - Codex's `apply_patch` - not a renamed `edit_file`. It's a distinct
//!   multi-file patch-script format (`*** Begin Patch` / `*** Update File:`
//!   / `@@` hunks / `*** End Patch`) that `edit_file` can't parse.
//!   Registered as its own real tool (`apply_patch.rs`) under the exact
//!   name Codex uses, so no alias is needed.
//! - qwen-code's `save_memory` - not the same thing as `session_context`'s
//!   `add_fact` operation, which is scoped to `context_{session_id}.json`
//!   and disappears once the session ends. `save_memory`'s whole point is
//!   that the fact outlives the session, so aliasing it to `add_fact` would
//!   make the call succeed while silently breaking that guarantee.
//!   Registered as its own real tool (`save_memory.rs`) that persists to a
//!   working-directory-keyed file instead, so no alias is needed here
//!   either.
pub const TOOL_ALIASES: &[(&str, &str)] = &[
    // qwen-code (packages/core/src/tools/tool-names.ts and per-tool
    // schemas) - verified against source, not guessed. Field-shape
    // compatibility confirmed (and where it wasn't, fixed alongside this
    // table: edit_file's file_path/old_string/new_string/replace_all,
    // bash's directory/timeout/description/is_background, grep's glob).
    ("list_directory", "ls"),
    ("grep_search", "grep"),
    ("search_file_content", "grep"), // qwen-code's legacy pre-rename name
    ("edit", "edit_file"), // also Claude Code's name for the same tool (case-insensitive match covers `Edit` too)
    ("run_shell_command", "bash"),

    // Claude Code's documented/observed tool names. Crustly's own file/
    // search/shell tools were deliberately modeled on these (see
    // CLAUDE.md's "Claw Code parity" phase), so shape already matches.
    // (`Edit` is covered by the `edit` entry above - case-insensitive.)
    ("Bash", "bash"),
    ("Read", "read_file"),
    ("Write", "write_file"),
    ("Glob", "glob"),
    ("Grep", "grep"),
    ("WebFetch", "web_fetch"),
    ("WebSearch", "web_search"),
    ("TodoWrite", "todo_write"),
    ("NotebookEdit", "notebook_edit"),
    ("Task", "agent"), // Claude Code's subagent-launch tool; same description/prompt/subagent_type shape as Crustly's `agent`.

    // Generic names models commonly reach for regardless of training
    // source, for single-argument tools where a shape mismatch isn't a
    // real risk.
    ("list_files", "ls"),
    ("list_dir", "ls"),
    ("dir", "ls"),
    ("cat", "read_file"),
    ("open_file", "read_file"),
    ("save_file", "write_file"),
    ("find_files", "glob"),
    ("rg", "grep"),
    ("shell", "bash"),
    ("execute", "bash"),
    ("run_command", "bash"),
    ("terminal", "bash"),
    ("exec", "bash"),
];

/// Resolve `name` to its canonical tool name via [`TOOL_ALIASES`], matched
/// case-insensitively. Returns `None` if `name` isn't a known alias -
/// callers should fall back to treating `name` as already canonical (or
/// genuinely unknown).
pub fn resolve(name: &str) -> Option<&'static str> {
    TOOL_ALIASES
        .iter()
        .find(|(alias, _)| alias.eq_ignore_ascii_case(name))
        .map(|(_, canonical)| *canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_known_qwen_code_alias() {
        assert_eq!(resolve("list_directory"), Some("ls"));
    }

    #[test]
    fn resolves_known_claude_code_alias() {
        assert_eq!(resolve("TodoWrite"), Some("todo_write"));
    }

    #[test]
    fn resolution_is_case_insensitive() {
        assert_eq!(resolve("LIST_DIRECTORY"), Some("ls"));
        assert_eq!(resolve("list_Directory"), Some("ls"));
        assert_eq!(resolve("READ"), Some("read_file"));
    }

    #[test]
    fn unknown_name_resolves_to_none() {
        assert_eq!(resolve("definitely_not_a_real_tool"), None);
    }

    /// Resolution must be a single hop: an alias's target must not itself
    /// resolve to something else, or a caller doing one `resolve()` call
    /// (as `ToolRegistry` does) would stop short of the real tool. A target
    /// that case-insensitively matches its own alias entry (e.g. `Bash` ->
    /// `bash`) is fine - that's a no-op, not a chain.
    #[test]
    fn alias_resolution_is_a_single_hop() {
        for (alias, canonical) in TOOL_ALIASES {
            if let Some(next) = resolve(canonical) {
                assert!(
                    next.eq_ignore_ascii_case(canonical),
                    "alias '{alias}' -> '{canonical}' resolves further to '{next}' - \
                     resolution must be a single hop"
                );
            }
        }
    }

    #[test]
    fn no_duplicate_alias_entries() {
        let mut seen = std::collections::HashSet::new();
        for (alias, _) in TOOL_ALIASES {
            let lower = alias.to_ascii_lowercase();
            assert!(seen.insert(lower), "duplicate alias entry: {alias}");
        }
    }
}
