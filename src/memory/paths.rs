//! Path-scoped memories (docs/improve-memory.md §13.5; tasks P4.2 and P4.3).
//!
//! A project memory can name the files it is about with up to five globs,
//! relative to the project root ("src/tui/**", "migrations/*.sql"). Such a
//! memory is recalled when a request mentions a matching path.

use glob::{MatchOptions, Pattern};

/// Most globs one memory may carry.
pub const MAX_GLOBS: usize = 5;

/// Longest glob, in characters.
const MAX_GLOB_CHARS: usize = 200;

/// Check and normalise one glob. Backslashes become forward slashes. Absolute
/// paths, drive letters and `..` are refused, so a glob cannot point outside
/// the project.
pub fn validate_glob(glob: &str) -> Result<String, String> {
    let normalised = glob.trim().replace('\\', "/");
    if normalised.is_empty() {
        return Err("a path cannot be empty".to_string());
    }
    if normalised.chars().count() > MAX_GLOB_CHARS {
        return Err(format!("a path can be at most {MAX_GLOB_CHARS} characters"));
    }
    if normalised.starts_with('/') || normalised.contains(':') {
        return Err(format!(
            "{normalised}: paths are relative to the project root"
        ));
    }
    if normalised.split('/').any(|part| part == "..") {
        return Err(format!("{normalised}: a path cannot leave the project"));
    }
    Pattern::new(&normalised).map_err(|e| format!("{normalised}: not a valid pattern ({e})"))?;
    Ok(normalised)
}

/// Paths a message mentions: words with a slash or a file extension, with
/// surrounding punctuation removed and backslashes made forward slashes.
pub fn mentioned_paths(message: &str) -> Vec<String> {
    let mut found = Vec::new();
    for word in message.split_whitespace() {
        let trimmed = word.trim_matches(|c: char| {
            matches!(
                c,
                '`' | '\'' | '"' | ',' | ';' | ':' | '(' | ')' | '[' | ']' | '!' | '?' | '.'
            )
        });
        let looks_like_path = trimmed.contains('/')
            || trimmed.contains('\\')
            || trimmed.rsplit_once('.').is_some_and(|(stem, ext)| {
                !stem.is_empty()
                    && !ext.is_empty()
                    && ext.chars().all(|c| c.is_ascii_alphanumeric())
            });
        if looks_like_path && !trimmed.is_empty() {
            let path = trimmed.replace('\\', "/");
            if !found.contains(&path) {
                found.push(path);
            }
        }
    }
    found
}

/// Whether any of `globs` matches `path`. A `*` does not cross a directory
/// boundary; `**` does.
pub fn matches(globs: &[String], path: &str) -> bool {
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: true,
        require_literal_leading_dot: false,
    };
    let path = path.trim_start_matches("./").replace('\\', "/");
    globs.iter().any(|g| {
        Pattern::new(g)
            .map(|p| p.matches_with(&path, options))
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_relative_globs_are_normalised() {
        assert_eq!(validate_glob("src\\tui\\**").unwrap(), "src/tui/**");
        assert_eq!(
            validate_glob("  migrations/*.sql ").unwrap(),
            "migrations/*.sql"
        );
    }

    #[test]
    fn globs_that_leave_the_project_are_refused() {
        assert!(validate_glob("/etc/passwd").is_err());
        assert!(validate_glob("C:/secrets").is_err());
        assert!(validate_glob("src/../../etc").is_err());
        assert!(validate_glob("").is_err());
        assert!(validate_glob("src/[unclosed").is_err());
    }

    #[test]
    fn paths_are_found_in_a_message() {
        let paths =
            mentioned_paths("Why does src/tui/app.rs panic? Also check `migrations/x.sql`.");
        assert_eq!(
            paths,
            vec!["src/tui/app.rs".to_string(), "migrations/x.sql".to_string()]
        );
        assert!(mentioned_paths("no paths in this sentence").is_empty());
    }

    #[test]
    fn globs_match_paths_and_stop_at_directories() {
        let globs = vec!["src/tui/**".to_string()];
        assert!(matches(&globs, "src/tui/app.rs"));
        assert!(matches(&globs, "./src/tui/deep/x.rs"));
        assert!(!matches(&globs, "src/llm/x.rs"));
        let single = vec!["src/*.rs".to_string()];
        assert!(matches(&single, "src/lib.rs"));
        assert!(
            !matches(&single, "src/tui/app.rs"),
            "* does not cross a directory"
        );
    }
}
