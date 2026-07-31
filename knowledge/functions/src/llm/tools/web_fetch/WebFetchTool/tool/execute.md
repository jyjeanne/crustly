---
type: Rust Method
title: execute
resource: src/llm/tools/web_fetch.rs#L157-L242
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/ssrf_guard/check_url_not_blocked
  - functions/src/llm/tools/ssrf_guard/guard
  - functions/src/llm/tools/ssrf_guard/checked_redirect_policy
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/web_fetch/html_to_text
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_url_not_blocked](../../../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
- [guard](../../../../../../../functions/src/llm/tools/ssrf_guard/guard.md)
- [checked_redirect_policy](../../../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [html_to_text](../../../../../../../functions/src/llm/tools/web_fetch/html_to_text.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)