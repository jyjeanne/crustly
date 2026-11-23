//! Project identity for long-term memory (docs/improve-memory.md §7).
//!
//! A project is the git repository that contains the working directory. A
//! linked worktree belongs to its main repository, so every worktree shares
//! one set of project memories. A directory outside any repository is its own
//! project.

use super::model::ProjectId;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// Who the current directory belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectIdentity {
    /// Hex SHA-256 of the normalised root path. Never the remote URL.
    pub id: ProjectId,
    /// Canonical root directory of the project.
    pub root: PathBuf,
    /// Last component of the root, for display.
    pub name: String,
    /// The `origin` remote URL, for display only.
    pub remote_url: Option<String>,
}

/// Identify the project that `working_dir` belongs to.
pub fn identify(working_dir: &Path) -> ProjectIdentity {
    let start = fs::canonicalize(working_dir).unwrap_or_else(|_| working_dir.to_path_buf());
    let (root, git_dir) = find_root(&start).unwrap_or_else(|| (start.clone(), None));
    let root_text = normalize_root(&root.to_string_lossy());
    ProjectIdentity {
        id: sha256_hex(root_text.as_bytes()),
        name: root
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| root_text.clone()),
        remote_url: git_dir.as_deref().and_then(origin_url),
        root,
    }
}

/// Walk up from `start` to the nearest repository. Returns the project's root
/// and, when one was found, the git directory whose config holds the remote.
fn find_root(start: &Path) -> Option<(PathBuf, Option<PathBuf>)> {
    for dir in start.ancestors() {
        let dot_git = dir.join(".git");
        if dot_git.is_dir() {
            return Some((dir.to_path_buf(), Some(dot_git)));
        }
        if dot_git.is_file() {
            // A linked worktree: ".git" holds "gitdir: <main>/.git/worktrees/<name>".
            let text = fs::read_to_string(&dot_git).ok()?;
            let linked = text.trim().strip_prefix("gitdir:")?.trim();
            let linked = dir.join(linked);
            // <main>/.git/worktrees/<name> -> <main>/.git -> <main>
            let main_git = linked.parent()?.parent()?.to_path_buf();
            let main_root = fs::canonicalize(main_git.parent()?).ok()?;
            return Some((main_root, Some(main_git)));
        }
    }
    None
}

/// Normalise a path for hashing: strip a Windows `\\?\` prefix and lower-case
/// the drive letter, so one directory always produces one id.
pub fn normalize_root(path: &str) -> String {
    let stripped = path.strip_prefix(r"\\?\").unwrap_or(path);
    let mut chars = stripped.chars();
    match (chars.next(), chars.next()) {
        (Some(drive), Some(':')) if drive.is_ascii_alphabetic() => {
            format!("{}{}", drive.to_ascii_lowercase(), &stripped[1..])
        }
        _ => stripped.to_string(),
    }
}

/// The `url` of `[remote "origin"]` in a git config file, if present.
fn origin_url(git_dir: &Path) -> Option<String> {
    let config = fs::read_to_string(git_dir.join("config")).ok()?;
    let mut in_origin = false;
    for line in config.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_origin = line == r#"[remote "origin"]"#;
        } else if in_origin {
            if let Some(value) = line.strip_prefix("url") {
                if let Some(value) = value.trim_start().strip_prefix('=') {
                    return Some(value.trim().to_string());
                }
            }
        }
    }
    None
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, text: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn a_plain_directory_is_its_own_project() {
        let dir = tempfile::tempdir().unwrap();
        let id = identify(dir.path());
        assert_eq!(id.name, dir.path().file_name().unwrap().to_string_lossy());
        assert_eq!(id.remote_url, None);
        assert_eq!(id.id.len(), 64, "the id is a hex SHA-256");
    }

    #[test]
    fn a_subdirectory_resolves_to_its_repository_root() {
        let repo = tempfile::tempdir().unwrap();
        fs::create_dir_all(repo.path().join(".git")).unwrap();
        let deep = repo.path().join("src").join("deep");
        fs::create_dir_all(&deep).unwrap();

        let from_root = identify(repo.path());
        let from_deep = identify(&deep);
        assert_eq!(from_deep.root, from_root.root);
        assert_eq!(from_deep.id, from_root.id);
    }

    #[test]
    fn a_linked_worktree_belongs_to_the_main_repository() {
        let main = tempfile::tempdir().unwrap();
        let main_git = main.path().join(".git");
        write(&main_git.join("config"), "[core]\n\tbare = false\n");
        let worktree_git = main_git.join("worktrees").join("feature");
        fs::create_dir_all(&worktree_git).unwrap();

        let worktree = tempfile::tempdir().unwrap();
        write(
            &worktree.path().join(".git"),
            &format!("gitdir: {}\n", worktree_git.display()),
        );

        let from_main = identify(main.path());
        let from_worktree = identify(worktree.path());
        assert_eq!(from_worktree.id, from_main.id);
        assert_eq!(from_worktree.root, from_main.root);
    }

    #[test]
    fn the_origin_url_is_read_for_display() {
        let repo = tempfile::tempdir().unwrap();
        write(
            &repo.path().join(".git").join("config"),
            "[remote \"upstream\"]\n\turl = https://example.com/other.git\n\
             [remote \"origin\"]\n\turl = https://example.com/project.git\n",
        );
        let id = identify(repo.path());
        assert_eq!(
            id.remote_url.as_deref(),
            Some("https://example.com/project.git")
        );
    }

    #[test]
    fn two_clones_of_one_remote_are_two_projects() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        for dir in [&first, &second] {
            write(
                &dir.path().join(".git").join("config"),
                "[remote \"origin\"]\n\turl = https://example.com/same.git\n",
            );
        }
        assert_ne!(identify(first.path()).id, identify(second.path()).id);
    }

    #[test]
    fn normalize_root_strips_the_verbatim_prefix_and_lowers_the_drive() {
        assert_eq!(normalize_root(r"\\?\C:\Work\Repo"), r"c:\Work\Repo");
        assert_eq!(normalize_root(r"D:\data"), r"d:\data");
        assert_eq!(normalize_root("/home/user/repo"), "/home/user/repo");
    }
}
