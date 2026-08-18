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
//!   6. A built-in skill compiled into the binary (see [`builtin`]) — works with
//!      no project setup at all, and is shadowed by any of the above.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Skills shipped with crustly itself, embedded at compile time so `/review`
/// (and future built-ins) work in any project without a `.crustly/skills/`
/// directory. A project or user skill of the same name always takes
/// precedence — see `resolve_skill` — so these are defaults, not overrides.
mod builtin {
    /// `(name, SKILL.md contents)` pairs, checked case-insensitively.
    const SKILLS: &[(&str, &str)] = &[("review", include_str!("builtin_skills/review.md"))];

    pub(super) fn lookup(name: &str) -> Option<&'static str> {
        SKILLS
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, content)| *content)
    }

    pub(super) fn names() -> impl Iterator<Item = &'static str> {
        SKILLS.iter().map(|(n, _)| *n)
    }
}

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
            return Err(ToolError::InvalidInput(
                "skill name must not be empty".to_string(),
            ));
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

        let resolved = tokio::task::spawn_blocking({
            let name = name.clone();
            let cwd = context.working_directory.clone();
            move || resolve_skill(&name, &cwd)
        })
        .await
        .map_err(|e| ToolError::Execution(format!("task panicked: {e}")))?
        .ok_or_else(|| ToolError::Execution(format!("unknown skill: {name}")))?;

        let path_display = resolved.display();
        let contents = match resolved {
            SkillSource::File(path) => tokio::fs::read_to_string(&path)
                .await
                .map_err(ToolError::Io)?,
            SkillSource::Builtin(content) => content.to_string(),
        };

        let description = parse_skill_frontmatter_value(&contents, "description");

        let output = SkillOutput {
            skill: name,
            path: path_display,
            args: input.args,
            description,
            prompt: contents,
        };

        let json = serde_json::to_string_pretty(&output).map_err(ToolError::Json)?;

        Ok(ToolResult::success(json).with_metadata("path".to_string(), output.path.clone()))
    }
}

/// Where a resolved skill's content came from — a real file, or a built-in
/// compiled into the binary. Kept alongside the borrowed `&'static str` so
/// callers don't need to re-run resolution just to read the content.
enum SkillSource {
    File(PathBuf),
    Builtin(&'static str),
}

impl SkillSource {
    fn display(&self) -> String {
        match self {
            SkillSource::File(path) => path.display().to_string(),
            SkillSource::Builtin(_) => "<builtin>".to_string(),
        }
    }
}

/// Resolve `name` to its content, trying file-based lookup first (project-local,
/// then user-global) and falling back to a built-in skill. Returns `None` if
/// neither has it.
fn resolve_skill(name: &str, cwd: &Path) -> Option<SkillSource> {
    if let Ok(path) = resolve_skill_path(name, cwd) {
        return Some(SkillSource::File(path));
    }
    builtin::lookup(name).map(SkillSource::Builtin)
}

/// Synchronous convenience wrapper for non-tool callers (e.g. the TUI resolving
/// a `/name` slash command directly) that just want the content and
/// description, without going through the `Tool` trait. Reads file-based
/// skills eagerly, so should not be called from a context where blocking I/O
/// is unacceptable — the TUI's key-handling path already does equivalent
/// synchronous reads for `list_skills`.
pub(crate) fn resolve_skill_content(name: &str, cwd: &Path) -> Option<(String, Option<String>)> {
    let name = name.trim().trim_start_matches('/');
    if name.is_empty() || name.contains('\0') || name.split(['/', '\\']).any(|c| c == "..") {
        return None;
    }
    let content = match resolve_skill(name, cwd)? {
        SkillSource::File(path) => std::fs::read_to_string(path).ok()?,
        SkillSource::Builtin(content) => content.to_string(),
    };
    let description = parse_skill_frontmatter_value(&content, "description");
    Some((content, description))
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
        push_if_dir(
            &mut roots,
            home.join(".config").join("crustly").join("skills"),
        );
        push_if_dir(&mut roots, home.join(".claude").join("skills"));
    }

    roots
}

fn push_if_dir(roots: &mut Vec<PathBuf>, path: PathBuf) {
    if path.is_dir() && !roots.contains(&path) {
        roots.push(path);
    }
}

/// One discoverable skill, for the `/skills` TUI list view.
#[derive(Debug, Clone)]
pub struct SkillListing {
    pub name: String,
    pub description: Option<String>,
    pub root: PathBuf,
}

/// Enumerate every skill discoverable from `cwd`, across all lookup roots
/// (project-local and user-global, `.crustly` and `.claude`). Deduplicated
/// by name using the same first-root-wins precedence `resolve_skill_path`
/// uses, so this list matches what invoking each name would actually
/// resolve to. Sorted alphabetically (case-insensitive) for stable display.
pub(crate) fn list_skills(cwd: &Path) -> Vec<SkillListing> {
    let mut seen = std::collections::HashSet::new();
    let mut skills = Vec::new();

    for root in skill_lookup_roots(cwd) {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let skill_md = path.join("SKILL.md");
                if !skill_md.is_file() {
                    continue;
                }
                let dir_name = entry.file_name().to_string_lossy().to_string();
                let contents = std::fs::read_to_string(&skill_md).unwrap_or_default();
                let name = parse_skill_frontmatter_value(&contents, "name").unwrap_or(dir_name);
                if !seen.insert(name.to_lowercase()) {
                    continue;
                }
                skills.push(SkillListing {
                    description: parse_skill_frontmatter_value(&contents, "description"),
                    name,
                    root: root.clone(),
                });
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                // Legacy flat layout: <root>/<name>.md
                let Some(stem) = path.file_stem().map(|s| s.to_string_lossy().to_string()) else {
                    continue;
                };
                if !seen.insert(stem.to_lowercase()) {
                    continue;
                }
                let contents = std::fs::read_to_string(&path).unwrap_or_default();
                skills.push(SkillListing {
                    description: parse_skill_frontmatter_value(&contents, "description"),
                    name: stem,
                    root: root.clone(),
                });
            }
        }
    }

    // Built-ins fill in any name not already provided by a project/user file,
    // matching the precedence `resolve_skill` uses.
    for name in builtin::names() {
        if seen.insert(name.to_lowercase()) {
            let content = builtin::lookup(name).unwrap_or_default();
            skills.push(SkillListing {
                description: parse_skill_frontmatter_value(content, "description"),
                name: name.to_string(),
                root: PathBuf::from("<builtin>"),
            });
        }
    }

    skills.sort_by_key(|a| a.name.to_lowercase());
    skills
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
            let v = value
                .trim()
                .trim_matches(|ch| matches!(ch, '"' | '\''))
                .trim();
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

    #[test]
    fn list_skills_discovers_project_local_skills_with_frontmatter() {
        let tmp = tempfile::tempdir().unwrap();
        let skill_dir = tmp.path().join(".crustly").join("skills").join("my-skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: my-skill\ndescription: Does something cool\n---\n\nBody.",
        )
        .unwrap();

        let skills = list_skills(tmp.path());

        let found = skills
            .iter()
            .find(|s| s.name == "my-skill")
            .expect("my-skill should be discovered");
        assert_eq!(found.description.as_deref(), Some("Does something cool"));
    }

    #[test]
    fn list_skills_falls_back_to_directory_name_without_frontmatter_name() {
        let tmp = tempfile::tempdir().unwrap();
        let skill_dir = tmp
            .path()
            .join(".crustly")
            .join("skills")
            .join("no-frontmatter-name");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "Just a body, no frontmatter.").unwrap();

        let skills = list_skills(tmp.path());
        assert!(skills.iter().any(|s| s.name == "no-frontmatter-name"));
    }

    #[test]
    fn list_skills_discovers_legacy_flat_md_files() {
        let tmp = tempfile::tempdir().unwrap();
        let skills_dir = tmp.path().join(".crustly").join("skills");
        std::fs::create_dir_all(&skills_dir).unwrap();
        std::fs::write(
            skills_dir.join("legacy.md"),
            "---\ndescription: Old style\n---\nBody.",
        )
        .unwrap();

        let skills = list_skills(tmp.path());
        let found = skills
            .iter()
            .find(|s| s.name == "legacy")
            .expect("legacy.md skill should be discovered");
        assert_eq!(found.description.as_deref(), Some("Old style"));
    }

    #[test]
    fn list_skills_is_sorted_alphabetically_case_insensitive() {
        let tmp = tempfile::tempdir().unwrap();
        let skills_dir = tmp.path().join(".crustly").join("skills");
        for name in ["Zebra", "apple", "Banana"] {
            let dir = skills_dir.join(name);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("SKILL.md"), "Body.").unwrap();
        }

        let skills = list_skills(tmp.path());
        let names: Vec<_> = skills
            .iter()
            .map(|s| s.name.clone())
            .filter(|n| ["Zebra", "apple", "Banana"].contains(&n.as_str()))
            .collect();
        assert_eq!(names, vec!["apple", "Banana", "Zebra"]);
    }

    #[test]
    fn list_skills_deduplicates_same_name_across_roots() {
        let tmp = tempfile::tempdir().unwrap();
        for dir_name in [".crustly", ".claude"] {
            let skill_dir = tmp.path().join(dir_name).join("skills").join("dup");
            std::fs::create_dir_all(&skill_dir).unwrap();
            std::fs::write(skill_dir.join("SKILL.md"), "Body.").unwrap();
        }

        let skills = list_skills(tmp.path());
        assert_eq!(skills.iter().filter(|s| s.name == "dup").count(), 1);
    }

    #[test]
    fn list_skills_does_not_panic_on_a_directory_with_no_skills_dir() {
        let tmp = tempfile::tempdir().unwrap();
        // No .crustly/ or .claude/ under tmp.path() at all - just confirms
        // this doesn't panic when there's nothing to find.
        let _ = list_skills(tmp.path());
    }

    #[test]
    fn resolve_skill_content_falls_back_to_builtin_review_skill() {
        let tmp = tempfile::tempdir().unwrap();
        // No .crustly/skills/review anywhere under tmp - only the built-in exists.
        let (content, description) = resolve_skill_content("review", tmp.path())
            .expect("built-in review skill should resolve with no project skill present");
        assert!(content.contains("name: review"));
        assert!(description.is_some());
    }

    #[test]
    fn resolve_skill_content_prefers_project_file_over_builtin() {
        let tmp = tempfile::tempdir().unwrap();
        let skill_dir = tmp.path().join(".crustly").join("skills").join("review");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\ndescription: Project override\n---\nCustom body.",
        )
        .unwrap();

        let (content, description) = resolve_skill_content("review", tmp.path()).unwrap();
        assert_eq!(description.as_deref(), Some("Project override"));
        assert!(content.contains("Custom body."));
    }

    #[test]
    fn resolve_skill_content_returns_none_for_unknown_skill() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(resolve_skill_content("not-a-real-skill", tmp.path()).is_none());
    }

    #[test]
    fn list_skills_includes_builtin_review_skill() {
        let tmp = tempfile::tempdir().unwrap();
        let skills = list_skills(tmp.path());
        assert!(skills.iter().any(|s| s.name == "review"));
    }

    #[test]
    fn list_skills_lets_project_skill_shadow_builtin_of_same_name() {
        let tmp = tempfile::tempdir().unwrap();
        let skill_dir = tmp.path().join(".crustly").join("skills").join("review");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("SKILL.md"),
            "---\ndescription: Project override\n---\nCustom body.",
        )
        .unwrap();

        let skills = list_skills(tmp.path());
        let matches: Vec<_> = skills.iter().filter(|s| s.name == "review").collect();
        assert_eq!(
            matches.len(),
            1,
            "builtin should be shadowed, not duplicated"
        );
        assert_eq!(matches[0].description.as_deref(), Some("Project override"));
    }
}
