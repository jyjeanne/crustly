//! Session-scoped file-read cache backing "prior read" enforcement for
//! mutating file tools (`edit_file`, `write_file`'s overwrite path,
//! `apply_patch`'s Update File operation).
//!
//! Verified against qwen-code's `priorReadEnforcement.ts`, which documents
//! this as deliberately matching "Claude Code's `readFileState`": a file
//! must be read (via `read_file`) at least once in the session before it
//! can be edited or overwritten, and its on-disk mtime/size must still
//! match what was read - any drift (an external process modified it since)
//! blocks the edit until the model re-reads it. Creating a brand-new file
//! needs no prior read (there is nothing to have read); the tool that
//! writes it seeds its own cache entry afterward, since the model authored
//! those bytes.
//!
//! Crustly's version is deliberately simpler than qwen-code's: mtime+size
//! only (no content-type/cacheable distinction, since none of Crustly's
//! mutating tools operate on binary content), and no config-level opt-out
//! - if that turns out to be needed in practice, it can be added later.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

/// A cheap fingerprint of a file's on-disk state, used to detect whether it
/// changed since it was last read. mtime + size (not a content hash) -
/// matches qwen-code's approach and costs only a `stat`, no read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileFingerprint {
    modified: Option<SystemTime>,
    size: u64,
}

impl FileFingerprint {
    pub fn of(metadata: &std::fs::Metadata) -> Self {
        Self {
            modified: metadata.modified().ok(),
            size: metadata.len(),
        }
    }
}

/// Outcome of checking whether a mutating tool may proceed against a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadGate {
    /// Cleared to proceed.
    Ok,
    /// Never read (or written) in this session.
    NeverRead,
    /// Read earlier, but the file has since changed on disk.
    Stale,
}

/// Shared, session-scoped record of "this session has seen these bytes of
/// this path." One instance is created per session (see
/// `AgentService::file_read_cache_for_session`) and threaded through every
/// `ToolExecutionContext` built for that session via
/// [`ToolExecutionContext::with_file_read_cache`], so reads recorded by one
/// tool call are visible to the next.
#[derive(Debug, Default)]
pub struct FileReadCache {
    entries: Mutex<HashMap<PathBuf, FileFingerprint>>,
}

impl FileReadCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that `path` was read (or authored, for a newly-written file)
    /// with the given fingerprint. Call this after every successful
    /// `read_file`, and after every successful mutation by `edit_file`,
    /// `write_file`, or `apply_patch` (using the *post*-write fingerprint,
    /// so a follow-up edit in the same session doesn't need an intervening
    /// re-read).
    pub fn record(&self, path: &Path, fingerprint: FileFingerprint) {
        self.entries
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(path.to_path_buf(), fingerprint);
    }

    /// Check whether a mutating tool may proceed against `path`, given its
    /// current on-disk fingerprint.
    pub fn check(&self, path: &Path, current: FileFingerprint) -> ReadGate {
        let entries = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        match entries.get(path) {
            None => ReadGate::NeverRead,
            Some(recorded) if *recorded == current => ReadGate::Ok,
            Some(_) => ReadGate::Stale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fp(size: u64) -> FileFingerprint {
        FileFingerprint {
            modified: None,
            size,
        }
    }

    #[test]
    fn never_read_path_is_rejected() {
        let cache = FileReadCache::new();
        assert_eq!(
            cache.check(Path::new("a.txt"), fp(1)),
            ReadGate::NeverRead
        );
    }

    #[test]
    fn matching_fingerprint_after_record_is_ok() {
        let cache = FileReadCache::new();
        cache.record(Path::new("a.txt"), fp(10));
        assert_eq!(cache.check(Path::new("a.txt"), fp(10)), ReadGate::Ok);
    }

    #[test]
    fn mismatched_fingerprint_is_stale() {
        let cache = FileReadCache::new();
        cache.record(Path::new("a.txt"), fp(10));
        assert_eq!(cache.check(Path::new("a.txt"), fp(20)), ReadGate::Stale);
    }

    #[test]
    fn distinct_paths_are_tracked_independently() {
        let cache = FileReadCache::new();
        cache.record(Path::new("a.txt"), fp(10));
        assert_eq!(
            cache.check(Path::new("b.txt"), fp(10)),
            ReadGate::NeverRead
        );
    }

    #[test]
    fn re_recording_updates_the_fingerprint() {
        let cache = FileReadCache::new();
        cache.record(Path::new("a.txt"), fp(10));
        cache.record(Path::new("a.txt"), fp(20));
        assert_eq!(cache.check(Path::new("a.txt"), fp(10)), ReadGate::Stale);
        assert_eq!(cache.check(Path::new("a.txt"), fp(20)), ReadGate::Ok);
    }
}
