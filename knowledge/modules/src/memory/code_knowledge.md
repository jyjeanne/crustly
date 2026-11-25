---
type: Rust Module
title: code_knowledge
resource: src/memory/code_knowledge.rs#L1-L843
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-config-memorycodeknowledgeconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-mcp-client-mcpclient-mcptooldef
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/futures-future-boxfuture
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Identifier](../../../classes/src/memory/code_knowledge/Identifier.md)
- [identifiers](../../../functions/src/memory/code_knowledge/identifiers.md)
- [parse_code_span](../../../functions/src/memory/code_knowledge/parse_code_span.md)
- [is_test_concept](../../../functions/src/memory/code_knowledge/is_test_concept.md)
- [is_identifier](../../../functions/src/memory/code_knowledge/is_identifier.md)
- [is_snake_case](../../../functions/src/memory/code_knowledge/is_snake_case.md)
- [is_camel_case](../../../functions/src/memory/code_knowledge/is_camel_case.md)
- [OkfServer](../../../interfaces/src/memory/code_knowledge/OkfServer.md)
- [tools](../../../functions/src/memory/code_knowledge/MCPClient/okfserver/tools.md)
- [call](../../../functions/src/memory/code_knowledge/MCPClient/okfserver/call.md)
- [Status](../../../classes/src/memory/code_knowledge/Status.md)
- [Detection](../../../classes/src/memory/code_knowledge/Detection.md)
- [CodeKnowledge](../../../classes/src/memory/code_knowledge/CodeKnowledge.md)
- [bundle_dir](../../../functions/src/memory/code_knowledge/bundle_dir.md)
- [detect](../../../functions/src/memory/code_knowledge/CodeKnowledge/detect.md)
- [new](../../../functions/src/memory/code_knowledge/CodeKnowledge/new.md)
- [status](../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [set_status](../../../functions/src/memory/code_knowledge/CodeKnowledge/set_status.md)
- [is_ready](../../../functions/src/memory/code_knowledge/CodeKnowledge/is_ready.md)
- [status_line](../../../functions/src/memory/code_knowledge/CodeKnowledge/status_line.md)
- [bundle_shown](../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_shown.md)
- [bundle](../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle.md)
- [start](../../../functions/src/memory/code_knowledge/CodeKnowledge/start.md)
- [spawn_start](../../../functions/src/memory/code_knowledge/CodeKnowledge/spawn_start.md)
- [call_json](../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)
- [resolve](../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve.md)
- [resolve_text](../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve_text.md)
- [neighbours](../../../functions/src/memory/code_knowledge/CodeKnowledge/neighbours.md)
- [explore](../../../functions/src/memory/code_knowledge/CodeKnowledge/explore.md)
- [file_concepts](../../../functions/src/memory/code_knowledge/CodeKnowledge/file_concepts.md)
- [location](../../../functions/src/memory/code_knowledge/CodeKnowledge/location.md)
- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [closest](../../../functions/src/memory/code_knowledge/CodeKnowledge/closest.md)
- [bundle_time](../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_time.md)
- [bundle_is_older_than_sources](../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_is_older_than_sources.md)
- [newest_mtime](../../../functions/src/memory/code_knowledge/newest_mtime.md)
- [check_capabilities](../../../functions/src/memory/code_knowledge/check_capabilities.md)
- [names](../../../functions/src/memory/code_knowledge/names.md)
- [backticked_snake_and_camel_names_are_found](../../../functions/src/memory/code_knowledge/backticked_snake_and_camel_names_are_found.md)
- [a_qualified_name_keeps_its_type](../../../functions/src/memory/code_knowledge/a_qualified_name_keeps_its_type.md)
- [a_plain_word_needs_its_type_and_a_type_name_does_not](../../../functions/src/memory/code_knowledge/a_plain_word_needs_its_type_and_a_type_name_does_not.md)
- [test_code_is_recognised_by_its_path_or_name](../../../functions/src/memory/code_knowledge/test_code_is_recognised_by_its_path_or_name.md)
- [commands_paths_and_common_words_are_left_out](../../../functions/src/memory/code_knowledge/commands_paths_and_common_words_are_left_out.md)
- [capabilities_need_json_and_the_file_relation](../../../functions/src/memory/code_knowledge/capabilities_need_json_and_the_file_relation.md)
- [a_bundle_is_found_through_okf_toml_or_knowledge](../../../functions/src/memory/code_knowledge/a_bundle_is_found_through_okf_toml_or_knowledge.md)
- [detection_is_off_without_a_command_or_a_bundle](../../../functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle.md)

# Imports

- `crate::config::MemoryCodeKnowledgeConfig`
- `crate::mcp::client::{MCPClient, McpToolDef}`
- `anyhow::{anyhow, Result}`
- `async_trait::async_trait`
- `chrono::{DateTime, Utc}`
- `futures::future::BoxFuture`
- `serde_json::{json, Value}`
- `std::path::{Path, PathBuf}`
- `std::sync::{Arc, Mutex}`
- `std::time::Duration`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)