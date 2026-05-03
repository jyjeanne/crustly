//! Session-scoped tool result cache with per-tool TTL.

use dashmap::DashMap;
use serde_json::Value;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

/// Key for a cache entry: (tool_name, inputs_hash).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub tool_name: String,
    pub inputs_hash: u64,
}

impl CacheKey {
    /// Build a cache key by hashing the serialised inputs JSON.
    pub fn from_tool(tool_name: &str, inputs: &Value) -> Self {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        inputs.to_string().hash(&mut hasher);
        Self {
            tool_name: tool_name.to_string(),
            inputs_hash: hasher.finish(),
        }
    }
}

struct CacheEntry {
    result: String,
    expires_at: Instant,
}

/// Per-tool TTL configuration.
#[derive(Debug, Clone)]
pub struct ToolTtlConfig {
    pub read_file: Duration,
    pub glob: Duration,
    pub grep: Duration,
    pub ls: Duration,
    pub web_search: Duration,
    pub http_get: Duration,
    /// Default TTL for unlisted tools (0 = no cache).
    pub default: Duration,
}

impl Default for ToolTtlConfig {
    fn default() -> Self {
        Self {
            read_file: Duration::from_secs(60),
            glob: Duration::from_secs(30),
            grep: Duration::from_secs(30),
            ls: Duration::from_secs(30),
            web_search: Duration::from_secs(300),
            http_get: Duration::from_secs(60),
            default: Duration::ZERO,
        }
    }
}

impl ToolTtlConfig {
    pub fn ttl_for(&self, tool_name: &str) -> Duration {
        match tool_name {
            "read_file" => self.read_file,
            "glob" => self.glob,
            "grep" => self.grep,
            "ls" => self.ls,
            "web_search" => self.web_search,
            "http_get" => self.http_get,
            _ => self.default,
        }
    }
}

/// Session-scoped in-memory tool result cache.
pub struct ToolResultCache {
    entries: DashMap<CacheKey, CacheEntry>,
    ttl: ToolTtlConfig,
}

impl ToolResultCache {
    pub fn new(ttl: ToolTtlConfig) -> Self {
        Self {
            entries: DashMap::new(),
            ttl,
        }
    }

    /// Get a cached result. Returns `None` if absent or expired.
    pub fn get(&self, key: &CacheKey) -> Option<String> {
        if let Some(entry) = self.entries.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.result.clone());
            }
        }
        self.entries.remove(key);
        None
    }

    /// Insert a result with the given TTL. If TTL is zero, the entry is not stored.
    pub fn insert(&self, key: CacheKey, result: String, ttl: Duration) {
        if ttl == Duration::ZERO {
            return;
        }
        self.entries.insert(
            key,
            CacheEntry {
                result,
                expires_at: Instant::now() + ttl,
            },
        );
    }

    /// Convenience: insert using the tool name to determine TTL.
    pub fn insert_for_tool(&self, key: CacheKey, result: String) {
        let ttl = self.ttl.ttl_for(&key.tool_name);
        self.insert(key, result, ttl);
    }

    /// Remove all expired entries.
    pub fn evict_expired(&self) {
        let now = Instant::now();
        self.entries.retain(|_, v| now < v.expires_at);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_hit_returns_same_result() {
        let cache = ToolResultCache::new(ToolTtlConfig::default());
        let key = CacheKey::from_tool("read_file", &serde_json::json!({ "path": "src/main.rs" }));
        cache.insert(
            key.clone(),
            "fn main() {}".to_string(),
            Duration::from_secs(60),
        );
        assert_eq!(cache.get(&key), Some("fn main() {}".to_string()));
    }

    #[tokio::test]
    async fn cache_expires_after_ttl() {
        let cache = ToolResultCache::new(ToolTtlConfig::default());
        let key = CacheKey::from_tool("read_file", &serde_json::json!({ "path": "x" }));
        cache.insert(
            key.clone(),
            "content".to_string(),
            Duration::from_millis(10),
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert!(cache.get(&key).is_none(), "entry must be expired");
    }

    #[test]
    fn write_tool_not_cached() {
        let cfg = ToolTtlConfig::default();
        assert_eq!(cfg.ttl_for("write_file"), Duration::ZERO);
        assert_eq!(cfg.ttl_for("edit_file"), Duration::ZERO);
        assert_eq!(cfg.ttl_for("bash"), Duration::ZERO);
    }

    #[test]
    fn zero_ttl_insert_is_noop() {
        let cache = ToolResultCache::new(ToolTtlConfig::default());
        let key = CacheKey::from_tool("write_file", &serde_json::json!({}));
        cache.insert(key.clone(), "written".to_string(), Duration::ZERO);
        assert!(cache.get(&key).is_none());
    }
}
