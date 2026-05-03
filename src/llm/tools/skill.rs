//! Skill Tool
//!
//! Loads a named skill (slash command) from SKILL.md files on disk and returns
//! its full prompt content so the agent can execute it. Skills are discovered
//! in project-local and user-global directories, mirroring the Claw Code pattern.
//!
//! Lookup order (first match wins):
//!   1. `.crustly/skills/<name>/SKILL.md`  — project-local
//!   2. `.claude/skills/<name>/SKILL.md`   — project-local (Claude Code compat)
//!   3. `~/.config/crustly/skills/<name>/SKILL.md` — user-global
//!   4. `~/.claude/skills/<name>/SKILL.md` — user-global (Claude Code compat)
//!   5. Direct `.md` file in any of the above directories (legacy flat layout)

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub struct SkillTool;

#[derive(Debug, Deserialize)]
struct SkillInput {
    /// Skill name (e.g. "init", "review", or with leading slash "/init")
    skill: String,
    /// Optional extra arguments to pass through in the result
    args: Option<String>,
}

#[derive(Debug, Serialize)]
struct SkillOutput {
    skill: String,
    path: String,
    args: Option<String>,
    description: Option<String>,
    prompt: String,
}

#[async_trait]
impl Tool for SkillTool {
    fn name(&self) -> &str {
        "skill"
    }

    fn description(&self) -> &str {
        "Load a named skill (slash command) from a SKILL.md file. Returns the skill's prompt \
         content so the agent can execute it. Skills are looked up in project-local \
         (.crustly/skills/) and user-global (~/.config/crustly/skills/) directories."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "skill": {
                    "type": "string",
                    "description": "Name of the skill to load (e.g. \"init\", \"/review\")"
                },
                "args": {
                    "type": "string",
                    "description": "Optional arguments to pass to the skill"
                }
            },
            "required": ["skill"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ReadFiles]
    }

    fn requires_approval(&self) -> bool {
        false
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: SkillInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        let name = input.skill.trim().trim_start_matches('/');
        if name.is_empty() {
            return Err(ToolError::InvalidInput("skill name must not be empty".to_string()));
        }
        // Reject path traversal: ".." components or null bytes could escape the skills dir.
        if name.contains('\0') || name.split(['/', '\\']).any(|c| c == "..") {
            return Err(ToolError::InvalidInput(
                "skill name must not contain '..' path components or null bytes".to_string(),
            ));
        }
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: SkillInput = serde_json::from_value(input)?;
        let name = input.skill.trim().trim_start_matches('/').to_string();

        // Belt-and-suspenders: enforce the same constraint as validate_input so that
        // callers who skip validation cannot trigger path traversal via this tool.
        if name.is_empty() || name.contains('\0') || name.split(['/', '\\']).any(|c| c == "..") {
            return Err(ToolError::InvalidInput(
                "skill name must not contain '..' path components or null bytes".to_string(),
            ));
        }

        let skill_path = tokio::task::spawn_blocking({
            let name = name.clone();
            let cwd = context.working_directory.clone();
            move || resolve_skill_path(&name, &cwd)
        })
        .await
        .map_err(|e| ToolError::Execution(format!("task panicked: {e}")))?
        .map_err(ToolError::Execution)?;

        let contents = tokio::fs::read_to_string(&skill_path)
            .await
            .map_err(ToolError::Io)?;

        let description = parse_skill_frontmatter_value(&contents, "description");

        let output = SkillOutput {
            skill: name,
            path: skill_path.display().to_string(),
            args: input.args,
            description,
            prompt: contents,
        };

        let json = serde_json::to_string_pretty(&output)
            .map_err(ToolError::Json)?;

        Ok(ToolResult::success(json)
            .with_metadata("path".to_string(), output.path))
    }
}

/// Resolve the SKILL.md path for `name`, searching project-local then user-global roots.
fn resolve_skill_path(name: &str, cwd: &Path) -> std::result::Result<PathBuf, String> {
    let roots = skill_lookup_roots(cwd);

    for root in &roots {
        // Direct subdirectory: <root>/<name>/SKILL.md
        let dir_path = root.join(name).join("SKILL.md");
        if dir_path.is_file() {
            return Ok(dir_path);
        }

        // Case-insensitive scan
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                if !entry.path().is_dir() {
                    continue;
                }
                let skill_path = entry.path().join("SKILL.md");
                if !skill_path.is_file() {
                    continue;
                }
                let dir_name = entry.file_name().to_string_lossy().to_string();
                if dir_name.eq_ignore_ascii_case(name)
                    || frontmatter_name_matches(&skill_path, name)
                {
                    return Ok(skill_path);
                }
            }
        }

        // Legacy flat layout: <root>/<name>.md
        let flat_path = root.join(format!("{name}.md"));
        if flat_path.is_file() {
            return Ok(flat_path);
        }
    }

    Err(format!("unknown skill: {name}"))
}

/// Build the ordered list of skill lookup root directories.
fn skill_lookup_roots(cwd: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();

    // Project-local: walk ancestors
    for ancestor in cwd.ancestors() {
        let candidate = ancestor.join(".crustly").join("skills");
        push_if_dir(&mut roots, candidate);
        let candidate = ancestor.join(".claude").join("skills");
        push_if_dir(&mut roots, candidate);
    }

    // User-global via env or well-known home paths
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .ok();

    if let Some(home) = &home {
        let home = Path::new(home);
        push_if_dir(&mut roots, home.join(".config").join("crustly").join("skills"));
        push_if_dir(&mut roots, home.join(".claude").join("skills"));
    }

    roots
}

fn push_if_dir(roots: &mut Vec<PathBuf>, path: PathBuf) {
    if path.is_dir() && !roots.contains(&path) {
        roots.push(path);
    }
}

fn frontmatter_name_matches(path: &Path, requested: &str) -> bool {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|c| parse_skill_frontmatter_value(&c, "name"))
        .is_some_and(|n| n.eq_ignore_ascii_case(requested))
}

fn parse_skill_frontmatter_value(contents: &str, key: &str) -> Option<String> {
    let mut lines = contents.lines();
    if lines.next().map(str::trim) != Some("---") {
        return None;
    }
    // Build the prefix once to avoid allocating inside the loop.
    let prefix = format!("{key}:");
    for line in lines {
        let trimmed = line.trim();
        if trimmed == "---" {
            break;
        }
        if let Some(value) = trimmed.strip_prefix(prefix.as_str()) {
            let v = value.trim().trim_matches(|ch| matches!(ch, '"' | '\'')).trim();
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_description() {
        let content = "---\nname: my-skill\ndescription: Does something cool\n---\n\nPrompt body.";
        assert_eq!(
            parse_skill_frontmatter_value(content, "description"),
            Some("Does something cool".to_string())
        );
    }

    #[test]
    fn test_parse_frontmatter_no_frontmatter() {
        let content = "No frontmatter here.";
        assert!(parse_skill_frontmatter_value(content, "description").is_none());
    }

    #[test]
    fn test_parse_frontmatter_missing_key() {
        let content = "---\nname: test\n---\nBody.";
        assert!(parse_skill_frontmatter_value(content, "description").is_none());
    }

    #[test]
    fn test_validate_empty_skill_name() {
        let tool = SkillTool;
        let result = tool.validate_input(&serde_json::json!({ "skill": "/" }));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_skill_name() {
        let tool = SkillTool;
        let result = tool.validate_input(&serde_json::json!({ "skill": "init" }));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_rejects_dotdot_traversal() {
        let tool = SkillTool;
        for bad in &["../../etc/passwd", "../secret", "ok/../bad", "a/../../b"] {
            let result = tool.validate_input(&serde_json::json!({ "skill": bad }));
            assert!(result.is_err(), "expected error for: {bad}");
        }
    }

    #[test]
    fn test_validate_allows_namespaced_skill() {
        let tool = SkillTool;
        let result = tool.validate_input(&serde_json::json!({ "skill": "org/my-skill" }));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_rejects_null_byte() {
        let tool = SkillTool;
        let result = tool.validate_input(&serde_json::json!({ "skill": "evil\0name" }));
        assert!(result.is_err());
    }
}
