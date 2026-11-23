//! Moving memories in and out: JSON export and import, and moving the
//! `## Memories` bullets of `.crustly/memory.md` into the database
//! (docs/improve-memory.md §15.1, tasks P1.26 and P1.27).

use super::model::{
    MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, RejectReason, SaveOutcome,
};
use super::project::{id_for_root, ProjectIdentity};
use super::MemoryStore;
use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Format version of the export file.
pub const EXPORT_VERSION: u32 = 1;

/// Heading of the section of `memory.md` that holds model-saved memories.
const NOTES_HEADING: &str = "## Memories";

/// One memory in an export file. Vectors are never exported; they are
/// recomputed on import.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportedMemory {
    pub id: String,
    /// "global" or "project".
    pub scope: String,
    /// Root of the project, for project memories.
    pub project_root: Option<String>,
    pub category: String,
    pub content: String,
    pub source: String,
    pub importance: u8,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportFile {
    pub version: u32,
    pub exported_at: DateTime<Utc>,
    pub memories: Vec<ExportedMemory>,
}

/// Which memories an export includes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportScope {
    All,
    Global,
    Project,
}

/// Counts from an import or a notes import.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TransferReport {
    pub created: usize,
    pub duplicates: usize,
    /// Memories not saved, with the reason, so nothing is lost silently.
    pub rejected: Vec<String>,
    pub skipped: usize,
}

/// Build the export file for `store`, restricted to `scope`.
pub async fn export(store: &MemoryStore, scope: ExportScope) -> Result<ExportFile> {
    let roots: std::collections::HashMap<String, String> = store
        .projects()
        .await?
        .into_iter()
        .map(|(id, root, _, _)| (id, root))
        .collect();
    let mut memories = Vec::new();
    for memory in store.all().await? {
        let (scope_name, project_root) = match &memory.scope {
            MemoryScope::Global => ("global", None),
            MemoryScope::Project(id) => ("project", roots.get(id).cloned()),
        };
        let wanted = match scope {
            ExportScope::All => true,
            ExportScope::Global => scope_name == "global",
            ExportScope::Project => scope_name == "project",
        };
        if !wanted {
            continue;
        }
        memories.push(ExportedMemory {
            id: memory.id.to_string(),
            scope: scope_name.to_string(),
            project_root,
            category: memory.category.as_str().to_string(),
            content: memory.content,
            source: match memory.source {
                MemorySource::User => "user",
                MemorySource::Tool => "tool",
                MemorySource::Extracted => "extracted",
                MemorySource::Imported => "imported",
                MemorySource::Consolidated => "consolidated",
            }
            .to_string(),
            importance: memory.importance,
            status: memory.status.as_str().to_string(),
            created_at: memory.created_at,
            updated_at: memory.updated_at,
        });
    }
    Ok(ExportFile {
        version: EXPORT_VERSION,
        exported_at: Utc::now(),
        memories,
    })
}

/// Save every active memory of an export. Project memories are filed under
/// the project whose root the export names, registered on the way.
pub async fn import(store: &MemoryStore, file: &ExportFile) -> Result<TransferReport> {
    if file.version != EXPORT_VERSION {
        bail!(
            "this export is version {}; this Crustly reads version {EXPORT_VERSION}",
            file.version
        );
    }
    let mut report = TransferReport::default();
    for item in &file.memories {
        if item.status != MemoryStatus::Active.as_str() {
            report.skipped += 1;
            continue;
        }
        let scope = match (item.scope.as_str(), &item.project_root) {
            ("global", _) => MemoryScope::Global,
            ("project", Some(root)) => {
                let root = PathBuf::from(root);
                let identity = ProjectIdentity {
                    id: id_for_root(&root),
                    name: root
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default(),
                    root,
                    remote_url: None,
                };
                store.register_project(&identity).await?;
                MemoryScope::Project(identity.id)
            }
            _ => {
                report.skipped += 1;
                continue;
            }
        };
        let Ok(category) = MemoryCategory::from_str(&item.category) else {
            report.rejected.push(format!(
                "unknown category {}: {}",
                item.category, item.content
            ));
            continue;
        };
        let outcome = store
            .save(NewMemory {
                content: item.content.clone(),
                scope,
                category,
                source: MemorySource::Imported,
                importance: item.importance,
                replaces: None,
                session_id: None,
                expires_at: None,
            })
            .await?;
        record(&mut report, outcome);
    }
    Ok(report)
}

/// Count an outcome. A rejection keeps only its reason: the text may be a
/// secret, and it must not be echoed to the terminal or logs.
fn record(report: &mut TransferReport, outcome: SaveOutcome) {
    match outcome {
        SaveOutcome::Created(_) => report.created += 1,
        SaveOutcome::Duplicate(_) => report.duplicates += 1,
        SaveOutcome::Rejected(reason) => report.rejected.push(reason_text(&reason)),
    }
}

fn reason_text(reason: &RejectReason) -> String {
    reason.to_string()
}

/// The parts of a `memory.md` file, split around its `## Memories` section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotesSplit {
    /// Bullet texts (without the leading "- ") in the section, in order.
    pub bullets: Vec<String>,
}

/// Find the bullets of the `## Memories` section. Lines outside the section,
/// and non-bullet lines inside it, are not returned.
pub fn split_notes(text: &str) -> NotesSplit {
    let mut bullets = Vec::new();
    let mut in_section = false;
    for line in text.lines() {
        if line.trim() == NOTES_HEADING {
            in_section = true;
            continue;
        }
        if line.starts_with("## ") {
            in_section = false;
            continue;
        }
        if in_section {
            if let Some(bullet) = line.strip_prefix("- ") {
                if !bullet.trim().is_empty() {
                    bullets.push(bullet.trim().to_string());
                }
            }
        }
    }
    NotesSplit { bullets }
}

/// Remove the bullets whose text is in `remove` from the `## Memories`
/// section. The heading goes too when no bullet is left under it. Everything
/// else is kept exactly as it was.
pub fn remove_bullets(text: &str, remove: &[String]) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let heading_at = lines.iter().position(|l| l.trim() == NOTES_HEADING);
    let section_end = heading_at.map(|h| {
        lines[h + 1..]
            .iter()
            .position(|l| l.starts_with("## "))
            .map(|i| h + 1 + i)
            .unwrap_or(lines.len())
    });

    let mut kept: Vec<&str> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let in_section = match (heading_at, section_end) {
            (Some(h), Some(end)) => index > h && index < end,
            _ => false,
        };
        if in_section {
            if let Some(bullet) = line.strip_prefix("- ") {
                if remove.iter().any(|r| r == bullet.trim()) {
                    continue;
                }
            }
        }
        kept.push(line);
    }

    // Drop the heading when its section now holds no bullets.
    if heading_at.is_some() {
        let section_has_bullet = kept
            .iter()
            .skip_while(|l| l.trim() != NOTES_HEADING)
            .skip(1)
            .take_while(|l| !l.starts_with("## "))
            .any(|l| l.starts_with("- "));
        if !section_has_bullet {
            if let Some(pos) = kept.iter().position(|l| l.trim() == NOTES_HEADING) {
                kept.remove(pos);
            }
        }
    }

    let mut out = kept.join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// Move the `## Memories` bullets of `notes` into the database, as project
/// memories, following §15.1: back the file up first (stopping if that
/// fails), save each bullet, then remove the saved and duplicate bullets from
/// the file. Rejected bullets stay in the file and are reported.
pub async fn import_from_notes(
    store: &MemoryStore,
    project: &ProjectIdentity,
    notes: &Path,
    backup_stamp: &str,
) -> Result<(TransferReport, Option<PathBuf>)> {
    let text = match std::fs::read_to_string(notes) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Ok((TransferReport::default(), None));
        }
        Err(err) => return Err(err).context("reading memory.md"),
    };
    let bullets = split_notes(&text).bullets;
    if bullets.is_empty() {
        return Ok((TransferReport::default(), None));
    }

    let backup = notes.with_file_name(format!(
        "{}.bak-{backup_stamp}",
        notes
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    ));
    std::fs::copy(notes, &backup).context("backing up memory.md; nothing was changed")?;

    store.register_project(project).await?;
    let mut report = TransferReport::default();
    let mut moved: Vec<String> = Vec::new();
    for bullet in &bullets {
        let outcome = store
            .save(NewMemory {
                content: bullet.clone(),
                scope: MemoryScope::Project(project.id.clone()),
                category: MemoryCategory::Fact,
                source: MemorySource::Imported,
                importance: 3,
                replaces: None,
                session_id: None,
                expires_at: None,
            })
            .await?;
        match &outcome {
            SaveOutcome::Created(_) | SaveOutcome::Duplicate(_) => moved.push(bullet.clone()),
            SaveOutcome::Rejected(_) => {}
        }
        record(&mut report, outcome);
    }

    std::fs::write(notes, remove_bullets(&text, &moved)).context("updating memory.md")?;
    Ok((report, Some(backup)))
}

/// A backup-file timestamp, e.g. `20261123-093015`.
pub fn backup_stamp(now: DateTime<Utc>) -> String {
    now.format("%Y%m%d-%H%M%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTES: &str = "# Notes\n\nIntro text.\n\n## Memories\n- Prefers tabs\n- Uses cargo fmt\n\n## Other\n- not a memory\n";

    #[test]
    fn split_finds_only_the_bullets_of_the_memories_section() {
        assert_eq!(
            split_notes(NOTES).bullets,
            vec!["Prefers tabs".to_string(), "Uses cargo fmt".to_string()]
        );
    }

    #[test]
    fn remove_drops_only_the_named_bullets_and_keeps_the_rest() {
        let out = remove_bullets(NOTES, &["Prefers tabs".to_string()]);
        assert!(!out.contains("Prefers tabs"));
        assert!(out.contains("- Uses cargo fmt"));
        assert!(
            out.contains("- not a memory"),
            "bullets outside the section are kept"
        );
        assert!(out.contains("Intro text."));
        assert!(
            out.contains("## Memories"),
            "the heading stays while a bullet remains"
        );
    }

    #[test]
    fn the_heading_goes_when_no_bullet_is_left() {
        let out = remove_bullets(
            NOTES,
            &["Prefers tabs".to_string(), "Uses cargo fmt".to_string()],
        );
        assert!(!out.contains("## Memories"));
        assert!(out.contains("## Other"));
        assert!(out.contains("Intro text."));
    }

    #[test]
    fn a_file_without_the_section_is_unchanged() {
        let text = "# Notes\n- a bullet elsewhere\n";
        assert_eq!(
            remove_bullets(text, &["a bullet elsewhere".to_string()]),
            text
        );
        assert!(split_notes(text).bullets.is_empty());
    }

    #[test]
    fn backup_stamps_sort_by_time() {
        let early = DateTime::from_timestamp(1_800_000_000, 0).unwrap();
        let later = DateTime::from_timestamp(1_800_000_060, 0).unwrap();
        assert!(backup_stamp(early) < backup_stamp(later));
    }
}
