---
type: Rust Module
title: cache
resource: src/llm/tools/cache.rs#L1-L198
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/dashmap-dashmap
  - external/serde-json-value
  - external/std-hash-hash-hasher
  - external/std-time-duration-instant
  - external/std-collections-hash-map-defaulthasher
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [CacheKey](../../../../classes/src/llm/tools/cache/CacheKey.md)
- [from_tool](../../../../functions/src/llm/tools/cache/CacheKey/from_tool.md)
- [CacheEntry](../../../../classes/src/llm/tools/cache/CacheEntry.md)
- [ToolTtlConfig](../../../../classes/src/llm/tools/cache/ToolTtlConfig.md)
- [default](../../../../functions/src/llm/tools/cache/ToolTtlConfig/default/default.md)
- [ttl_for](../../../../functions/src/llm/tools/cache/ToolTtlConfig/ttl_for.md)
- [ToolResultCache](../../../../classes/src/llm/tools/cache/ToolResultCache.md)
- [new](../../../../functions/src/llm/tools/cache/ToolResultCache/new.md)
- [get](../../../../functions/src/llm/tools/cache/ToolResultCache/get.md)
- [insert](../../../../functions/src/llm/tools/cache/ToolResultCache/insert.md)
- [insert_for_tool](../../../../functions/src/llm/tools/cache/ToolResultCache/insert_for_tool.md)
- [evict_expired](../../../../functions/src/llm/tools/cache/ToolResultCache/evict_expired.md)
- [invalidate_matching](../../../../functions/src/llm/tools/cache/ToolResultCache/invalidate_matching.md)
- [cache_hit_returns_same_result](../../../../functions/src/llm/tools/cache/cache_hit_returns_same_result.md)
- [cache_expires_after_ttl](../../../../functions/src/llm/tools/cache/cache_expires_after_ttl.md)
- [write_tool_not_cached](../../../../functions/src/llm/tools/cache/write_tool_not_cached.md)
- [invalidate_matching_drops_selected_tools_and_keeps_others](../../../../functions/src/llm/tools/cache/invalidate_matching_drops_selected_tools_and_keeps_others.md)
- [zero_ttl_insert_is_noop](../../../../functions/src/llm/tools/cache/zero_ttl_insert_is_noop.md)

# Imports

- `dashmap::DashMap`
- `serde_json::Value`
- `std::hash::{Hash, Hasher}`
- `std::time::{Duration, Instant}`
- `std::collections::hash_map::DefaultHasher`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)