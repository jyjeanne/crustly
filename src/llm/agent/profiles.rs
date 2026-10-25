//! Sub-agent profiles: what a sub-agent may do and which model it runs on.
//!
//! A profile matches the model and tools to the job: an explorer can run on
//! a small fast model with read-only tools, an implementer on a coder model
//! with the editing tools. Four profiles are built in; `[subagents.profiles]`
//! adds more or overrides fields of the built-in ones.

use crate::config::AgentProfileConfig;
use std::collections::{BTreeMap, HashMap};

/// The profile used when a task names none.
pub const DEFAULT_PROFILE: &str = "general";

/// What a sub-agent runs with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentProfile {
    /// Model on the active provider; `None` = the parent's model.
    pub model: Option<String>,
    /// The only tools it may use; `None` = the parent's tool profile.
    pub tools: Option<Vec<String>>,
    /// Limited to read-only actions.
    pub read_only: bool,
    /// Rounds of tool calls before it must answer.
    pub max_rounds: usize,
    /// Role instructions added to its system prompt.
    pub instructions: String,
}

fn names(tools: &[&str]) -> Option<Vec<String>> {
    Some(tools.iter().map(|t| t.to_string()).collect())
}

/// The built-in profiles.
pub fn builtin_profiles() -> BTreeMap<String, AgentProfile> {
    let mut profiles = BTreeMap::new();
    profiles.insert(
        DEFAULT_PROFILE.to_string(),
        AgentProfile {
            model: None,
            tools: None,
            read_only: false,
            max_rounds: 15,
            instructions: "You are a sub-agent doing one task for another agent. Do the task, \
                           then answer with a concise report of what you found or changed, \
                           with file paths. Your answer is all the other agent will see."
                .to_string(),
        },
    );
    profiles.insert(
        "explorer".to_string(),
        AgentProfile {
            model: None,
            tools: names(&["read_file", "grep", "glob", "ls"]),
            read_only: true,
            max_rounds: 12,
            instructions: "You are an explorer sub-agent. Find what the task asks for with your \
                           read-only tools, then answer with your findings: the relevant file \
                           paths, line numbers and short quotes. Do not propose changes unless \
                           asked."
                .to_string(),
        },
    );
    profiles.insert(
        "implementer".to_string(),
        AgentProfile {
            model: None,
            tools: names(&[
                "read_file",
                "edit_file",
                "write_file",
                "bash",
                "grep",
                "glob",
                "ls",
            ]),
            read_only: false,
            max_rounds: 20,
            instructions: "You are an implementer sub-agent. Make the change the task asks for, \
                           check that it builds or that its tests pass, then answer with what \
                           you changed (each file and a one-line summary) and the check you ran."
                .to_string(),
        },
    );
    profiles.insert(
        "reviewer".to_string(),
        AgentProfile {
            model: None,
            tools: names(&["read_file", "grep", "glob", "ls", "bash"]),
            read_only: true,
            max_rounds: 12,
            instructions: "You are a reviewer sub-agent. Read the code the task points at (use \
                           `git diff` with the bash tool for recent changes) and answer with \
                           concrete findings: file, line, the problem and why it matters. \
                           Change nothing."
                .to_string(),
        },
    );
    profiles
}

/// The built-in profiles with `configured` applied: a configured field
/// replaces the built-in one; a new name starts from the `general` profile.
pub fn resolve_profiles(
    configured: &HashMap<String, AgentProfileConfig>,
) -> BTreeMap<String, AgentProfile> {
    let mut profiles = builtin_profiles();
    let general = profiles[DEFAULT_PROFILE].clone();
    for (name, config) in configured {
        let profile = profiles
            .entry(name.trim().to_string())
            .or_insert_with(|| general.clone());
        if let Some(model) = &config.model {
            profile.model = Some(model.clone());
        }
        if let Some(tools) = &config.tools {
            profile.tools = Some(tools.clone());
        }
        if let Some(read_only) = config.read_only {
            profile.read_only = read_only;
        }
        if let Some(max_rounds) = config.max_rounds {
            profile.max_rounds = max_rounds.max(1);
        }
        if let Some(instructions) = &config.instructions {
            profile.instructions = instructions.clone();
        }
    }
    profiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_built_in_profiles_exist_and_explorers_cannot_write() {
        let profiles = builtin_profiles();
        for name in ["general", "explorer", "implementer", "reviewer"] {
            assert!(profiles.contains_key(name), "{name}");
        }
        let explorer = &profiles["explorer"];
        assert!(explorer.read_only);
        let tools = explorer.tools.as_ref().unwrap();
        assert!(!tools
            .iter()
            .any(|t| t == "edit_file" || t == "write_file" || t == "bash"));
        assert!(!profiles["implementer"].read_only);
    }

    #[test]
    fn configuration_overrides_fields_and_adds_profiles() {
        let mut configured = HashMap::new();
        configured.insert(
            "explorer".to_string(),
            AgentProfileConfig {
                model: Some("qwen2.5-coder:3b".to_string()),
                ..Default::default()
            },
        );
        configured.insert(
            "tester".to_string(),
            AgentProfileConfig {
                tools: Some(vec!["bash".to_string(), "read_file".to_string()]),
                max_rounds: Some(0),
                ..Default::default()
            },
        );

        let profiles = resolve_profiles(&configured);
        let explorer = &profiles["explorer"];
        assert_eq!(explorer.model.as_deref(), Some("qwen2.5-coder:3b"));
        assert!(explorer.read_only, "unset fields keep the built-in value");

        let tester = &profiles["tester"];
        assert_eq!(
            tester.tools.as_deref(),
            Some(&["bash".to_string(), "read_file".to_string()][..])
        );
        assert_eq!(tester.max_rounds, 1, "at least one round");
        assert_eq!(tester.instructions, profiles["general"].instructions);
    }
}
