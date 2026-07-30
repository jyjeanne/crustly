---
type: Rust Method
title: parse
resource: src/db/models.rs#L133-L148
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/cli/run
  - functions/src/config/Config/apply_env_overrides
  - functions/src/config/Config/load_provider_api_keys
  - functions/src/db/models/PlanTask/exec_status
  - functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session
  - functions/src/db/repository/memory/EpisodicMemoryRepository/list_recent
  - functions/src/db/repository/plan/row_to_plan_task
  - functions/src/llm/agent/memory/row_to_entry
  - functions/src/llm/provider/anthropic/AnthropicProvider/headers
  - functions/src/llm/provider/gemini/GeminiProvider/headers
  - functions/src/llm/provider/openai/OpenAIProvider/headers
  - functions/src/llm/provider/qwen/QwenProvider/headers
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
  - functions/src/llm/tools/ssrf_guard/check_url_not_blocked
  - functions/src/logging/init_debug_logging
  - functions/src/logging/init_minimal_logging
  - functions/src/logging/debug_filter_is_scoped_to_crustly
  - functions/src/main
---

# Signature

`pub fn parse(s: &str) -> Self`

# Called by

- [run](../../../../../functions/src/cli/run.md)
- [apply_env_overrides](../../../../../functions/src/config/Config/apply_env_overrides.md)
- [load_provider_api_keys](../../../../../functions/src/config/Config/load_provider_api_keys.md)
- [exec_status](../../../../../functions/src/db/models/PlanTask/exec_status.md)
- [list_for_session](../../../../../functions/src/db/repository/compaction/CompactionRecordRepository/list_for_session.md)
- [list_recent](../../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/list_recent.md)
- [row_to_plan_task](../../../../../functions/src/db/repository/plan/row_to_plan_task.md)
- [row_to_entry](../../../../../functions/src/llm/agent/memory/row_to_entry.md)
- [headers](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/headers.md)
- [headers](../../../../../functions/src/llm/provider/gemini/GeminiProvider/headers.md)
- [headers](../../../../../functions/src/llm/provider/openai/OpenAIProvider/headers.md)
- [headers](../../../../../functions/src/llm/provider/qwen/QwenProvider/headers.md)
- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [check_url_not_blocked](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
- [init_debug_logging](../../../../../functions/src/logging/init_debug_logging.md)
- [init_minimal_logging](../../../../../functions/src/logging/init_minimal_logging.md)
- [debug_filter_is_scoped_to_crustly](../../../../../functions/src/logging/debug_filter_is_scoped_to_crustly.md)
- [main](../../../../../functions/src/main.md)