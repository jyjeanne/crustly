---
type: Rust Method
title: parse
resource: src/db/models.rs#L136-L151
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/Config/apply_env_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/load_provider_api_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTask/exec_status
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/pending_replaces
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/folded_sources
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/row_to_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/row_to_plan_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/headers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/headers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_size_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/headers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/headers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_loopback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_cloud_metadata_link_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_rfc1918_private_ranges
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_carrier_grade_nat_range
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_ipv6_unique_local_and_link_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_ipv4_mapped_blocked_address
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/allows_public_addresses
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_debug_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_minimal_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/debug_filter_is_scoped_to_crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse(s: &str) -> Self`

# Called by

- [apply_env_overrides](../../../../../functions/src/config/Config/apply_env_overrides.md)
- [load_provider_api_keys](../../../../../functions/src/config/Config/load_provider_api_keys.md)
- [exec_status](../../../../../functions/src/db/models/PlanTask/exec_status.md)
- [list_for_session](../../../../../functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session.md)
- [pending_replaces](../../../../../functions/src/db/repository/memory/MemoryRepository/pending_replaces.md)
- [folded_sources](../../../../../functions/src/db/repository/memory/MemoryRepository/folded_sources.md)
- [row_to_memory](../../../../../functions/src/db/repository/memory/row_to_memory.md)
- [row_to_plan_task](../../../../../functions/src/db/repository/plan/row_to_plan_task.md)
- [headers](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/headers.md)
- [headers](../../../../../functions/src/llm/provider/gemini/GeminiProvider/headers.md)
- [parse_nvidia_smi_csv](../../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)
- [parse_size_string](../../../../../functions/src/llm/provider/hardware_detect/parse_size_string.md)
- [parse_split_gguf_filename](../../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename.md)
- [mock_resumable_http_server](../../../../../functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server.md)
- [headers](../../../../../functions/src/llm/provider/openai/OpenAIProvider/headers.md)
- [headers](../../../../../functions/src/llm/provider/qwen/QwenProvider/headers.md)
- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [save_to_store](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [blocks_loopback](../../../../../functions/src/llm/tools/ssrf_guard/blocks_loopback.md)
- [blocks_cloud_metadata_link_local](../../../../../functions/src/llm/tools/ssrf_guard/blocks_cloud_metadata_link_local.md)
- [blocks_rfc1918_private_ranges](../../../../../functions/src/llm/tools/ssrf_guard/blocks_rfc1918_private_ranges.md)
- [blocks_carrier_grade_nat_range](../../../../../functions/src/llm/tools/ssrf_guard/blocks_carrier_grade_nat_range.md)
- [blocks_ipv6_unique_local_and_link_local](../../../../../functions/src/llm/tools/ssrf_guard/blocks_ipv6_unique_local_and_link_local.md)
- [blocks_ipv4_mapped_blocked_address](../../../../../functions/src/llm/tools/ssrf_guard/blocks_ipv4_mapped_blocked_address.md)
- [allows_public_addresses](../../../../../functions/src/llm/tools/ssrf_guard/allows_public_addresses.md)
- [init_debug_logging](../../../../../functions/src/logging/init_debug_logging.md)
- [init_minimal_logging](../../../../../functions/src/logging/init_minimal_logging.md)
- [debug_filter_is_scoped_to_crustly](../../../../../functions/src/logging/debug_filter_is_scoped_to_crustly.md)