---
type: Rust Method
title: execute
resource: src/llm/tools/web_fetch.rs#L157-L242
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/check_url_not_blocked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/ssrf_guard/guard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/checked_redirect_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_fetch/html_to_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/web_fetch/execute_denies_cloud_metadata_endpoint
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/web_fetch/execute_denies_loopback_address
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_url_not_blocked](../../../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [guard](../../../../../../../functions/src/llm/tools/ssrf_guard/guard.md)
- [checked_redirect_policy](../../../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [status](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [html_to_text](../../../../../../../functions/src/llm/tools/web_fetch/html_to_text.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)

# Called by

- [execute_denies_cloud_metadata_endpoint](../../../../../../../functions/src/llm/tools/web_fetch/execute_denies_cloud_metadata_endpoint.md)
- [execute_denies_loopback_address](../../../../../../../functions/src/llm/tools/web_fetch/execute_denies_loopback_address.md)