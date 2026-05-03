//! Permission policy engine and path boundary enforcement.
//!
//! All path-handling tools must call `check_path` before any filesystem operation.
//! The policy tree is evaluated before every tool call in `ToolRegistry::execute`.

use serde_json::Value;
use std::path::{Path, PathBuf};

// ── Policy decision ───────────────────────────────────────────────────────────

/// Result of evaluating a permission policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
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
        Self { pattern: glob::Pattern::new(pattern).expect("invalid glob pattern") }
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
        Self { pattern: glob::Pattern::new(pattern).expect("invalid glob pattern") }
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
        Self { prefix: PathBuf::from(raw) }
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
            let root_canonical = std::fs::canonicalize(&self.root)
                .unwrap_or_else(|_| normalize_path(&self.root));
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
            // Non-existent path: normalize both sides (no canonicalize → no UNC prefix issues).
            let resolved = normalize_path(candidate);
            let root_normalized = normalize_path(&self.root);
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
        let cmd = inputs
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let program = cmd.split_whitespace().next().unwrap_or("");
        if self.allowed_programs.iter().any(|p| p == program) {
            PolicyDecision::Allow
        } else {
            PolicyDecision::Deny(format!(
                "bash command '{}' is not in the allowlist {:?}",
                program, self.allowed_programs
            ))
        }
    }
}

// ── Combinators ───────────────────────────────────────────────────────────────

/// All children must Allow; short-circuits on first Deny.
pub struct AndPolicy(pub Vec<Box<dyn PermissionPolicy>>);

impl PermissionPolicy for AndPolicy {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision {
        for rule in &self.0 {
            let d = rule.evaluate(tool_name, inputs);
            if d != PolicyDecision::Allow {
                return d;
            }
        }
        PolicyDecision::Allow
    }
}

/// First Allow wins; short-circuits on first Allow.
pub struct OrPolicy(pub Vec<Box<dyn PermissionPolicy>>);

impl PermissionPolicy for OrPolicy {
    fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision {
        let mut last_deny = PolicyDecision::Deny("no rules matched".to_string());
        for rule in &self.0 {
            let d = rule.evaluate(tool_name, inputs);
            if d == PolicyDecision::Allow {
                return PolicyDecision::Allow;
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
            PolicyDecision::Allow => PolicyDecision::Deny("negated allow".to_string()),
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
    let rule = PathBoundaryRule { root: canonical_root };
    let inputs = serde_json::json!({ "path": resolved });
    match rule.evaluate("", &inputs) {
        PolicyDecision::Allow => Ok(()),
        PolicyDecision::Deny(reason) => Err(reason),
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

    #[test]
    fn valid_path_inside_root_allowed() {
        let (_tmp, root) = make_root();
        let file = root.join("main.rs");
        std::fs::write(&file, "").unwrap();
        let rule = PathBoundaryRule { root };
        let d = rule.evaluate("read_file", &serde_json::json!({ "path": file.to_str().unwrap() }));
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
        assert!(!called.load(Ordering::SeqCst), "second rule must not be evaluated");
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
        assert!(!called.load(Ordering::SeqCst), "second rule must not be evaluated");
    }

    #[test]
    fn bash_allowlist_permits_cargo_denies_rm() {
        let rule = BashCommandAllowlist {
            allowed_programs: vec!["cargo".to_string(), "git".to_string()],
        };
        assert_eq!(rule.evaluate("bash", &serde_json::json!({ "command": "cargo test" })), PolicyDecision::Allow);
        assert!(matches!(rule.evaluate("bash", &serde_json::json!({ "command": "rm -rf ." })), PolicyDecision::Deny(_)));
    }

    #[test]
    fn not_policy_inverts_allow() {
        let policy = NotPolicy(Box::new(AllowAll));
        assert!(matches!(policy.evaluate("read_file", &serde_json::json!({})), PolicyDecision::Deny(_)));
    }

    #[test]
    fn path_traversal_denied() {
        let (_tmp, root) = make_root();
        let rule = PathBoundaryRule { root: root.clone() };
        // Construct a traversal path: root/../../etc/passwd
        let traversal = root.join("..").join("..").join("etc").join("passwd");
        let d = rule.evaluate("read_file", &serde_json::json!({ "path": traversal.to_str().unwrap() }));
        assert!(matches!(d, PolicyDecision::Deny(_)), "path traversal must be denied");
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
        let d = rule.evaluate("read_file", &serde_json::json!({ "path": file.to_str().unwrap() }));
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
        assert!(matches!(d, PolicyDecision::Deny(_)), "symlink outside root must be denied");
    }
}
