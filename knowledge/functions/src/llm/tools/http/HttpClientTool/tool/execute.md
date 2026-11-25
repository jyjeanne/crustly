---
type: Rust Method
title: execute
resource: src/llm/tools/http.rs#L171-L327
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/http/parse_method
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiPart/text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/utils/truncate_at_char_boundary
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/http/execute_denies_cloud_metadata_endpoint
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/http/execute_denies_loopback_address
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [parse_method](../../../../../../../functions/src/llm/tools/http/parse_method.md)
- [check_url_not_blocked](../../../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [guard](../../../../../../../functions/src/llm/tools/ssrf_guard/guard.md)
- [checked_redirect_policy](../../../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [parse](../../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [text](../../../../../../../functions/src/llm/provider/gemini/GeminiPart/text.md)
- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [truncate_at_char_boundary](../../../../../../../functions/src/utils/truncate_at_char_boundary.md)

# Called by

- [execute_denies_cloud_metadata_endpoint](../../../../../../../functions/src/llm/tools/http/execute_denies_cloud_metadata_endpoint.md)
- [execute_denies_loopback_address](../../../../../../../functions/src/llm/tools/http/execute_denies_loopback_address.md)