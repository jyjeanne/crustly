//! Which tool schemas a request carries (`[tools] profile`).
//!
//! The 23 built-in tool schemas take about 4,800 tokens on every request. On
//! a large window that is noise; on the 8,192-token default of the local
//! providers it is most of the window. The `core` profile sends only the
//! tools needed to read, search, edit and run code, plus any configured
//! `extra` tools and MCP tools (which the user configured explicitly).

use crate::config::ToolProfile;

/// Tools every profile exposes.
pub const CORE_TOOLS: &[&str] = &[
    "read_file",
    "edit_file",
    "write_file",
    "bash",
    "grep",
    "glob",
    "ls",
];

/// Context window at or above which the `auto` profile exposes every tool.
pub const AUTO_FULL_MIN_WINDOW: u32 = 32_768;

/// A tool profile resolved against one model's context window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSelection {
    core_only: bool,
    extra: Vec<String>,
    /// When set, exactly these tools and nothing else (a sub-agent profile).
    only: Option<Vec<String>>,
}

impl ToolSelection {
    /// Every tool.
    pub fn full() -> Self {
        Self {
            core_only: false,
            extra: Vec::new(),
            only: None,
        }
    }

    /// Exactly the tools in `names`. Uses the short system prompt, like the
    /// core profile.
    pub fn only(names: &[String]) -> Self {
        Self {
            core_only: true,
            extra: Vec::new(),
            only: Some(names.to_vec()),
        }
    }

    /// Resolve `profile` for a model with `context_window` tokens.
    pub fn resolve(profile: ToolProfile, extra: &[String], context_window: u32) -> Self {
        let core_only = match profile {
            ToolProfile::Full => false,
            ToolProfile::Core => true,
            ToolProfile::Auto => context_window < AUTO_FULL_MIN_WINDOW,
        };
        Self {
            core_only,
            extra: if core_only {
                extra.to_vec()
            } else {
                Vec::new()
            },
            only: None,
        }
    }

    /// Whether only the core set (plus extras and MCP tools) is exposed.
    pub fn is_core_only(&self) -> bool {
        self.core_only
    }

    /// Whether the tool registered as `name` is exposed. `name` must be the
    /// canonical registered name, not an alias.
    pub fn exposes(&self, name: &str) -> bool {
        if let Some(only) = &self.only {
            return only.iter().any(|n| n == name);
        }
        !self.core_only
            || CORE_TOOLS.contains(&name)
            || name.starts_with("mcp__")
            || self.extra.iter().any(|e| e == name)
    }

    /// `"core"` or `"full"`, for messages to the model and the user.
    pub fn label(&self) -> &'static str {
        if self.only.is_some() {
            "sub-agent"
        } else if self.core_only {
            "core"
        } else {
            "full"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_uses_core_tools_below_32k_and_every_tool_above() {
        assert!(ToolSelection::resolve(ToolProfile::Auto, &[], 8192).is_core_only());
        assert!(ToolSelection::resolve(ToolProfile::Auto, &[], 32_767).is_core_only());
        assert!(!ToolSelection::resolve(ToolProfile::Auto, &[], 32_768).is_core_only());
        assert!(!ToolSelection::resolve(ToolProfile::Auto, &[], 200_000).is_core_only());
    }

    #[test]
    fn explicit_profiles_ignore_the_window() {
        assert!(ToolSelection::resolve(ToolProfile::Core, &[], 200_000).is_core_only());
        assert!(!ToolSelection::resolve(ToolProfile::Full, &[], 4096).is_core_only());
    }

    #[test]
    fn core_exposes_core_extra_and_mcp_tools_only() {
        let extra = vec!["plan".to_string()];
        let s = ToolSelection::resolve(ToolProfile::Core, &extra, 8192);
        for name in CORE_TOOLS {
            assert!(s.exposes(name), "{name} is a core tool");
        }
        assert!(s.exposes("plan"), "extra tools are exposed");
        assert!(
            s.exposes("mcp__github__list_issues"),
            "MCP tools are exposed"
        );
        assert!(!s.exposes("web_fetch"));
        assert!(!s.exposes("execute_code"));
    }

    #[test]
    fn an_allow_list_exposes_exactly_its_tools() {
        let s = ToolSelection::only(&["read_file".to_string(), "grep".to_string()]);
        assert!(s.exposes("read_file"));
        assert!(s.exposes("grep"));
        assert!(
            !s.exposes("edit_file"),
            "core tools outside the list are hidden"
        );
        assert!(!s.exposes("mcp__github__list_issues"), "MCP tools too");
        assert!(s.is_core_only(), "uses the short system prompt");
    }

    #[test]
    fn full_exposes_everything() {
        let s = ToolSelection::full();
        assert!(s.exposes("web_fetch"));
        assert!(s.exposes("plan"));
        assert_eq!(s.label(), "full");
    }
}
