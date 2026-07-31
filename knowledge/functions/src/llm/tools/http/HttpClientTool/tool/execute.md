---
type: Rust Method
title: execute
resource: src/llm/tools/http.rs#L171-L327
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/http/parse_method
  - functions/src/llm/tools/ssrf_guard/check_url_not_blocked
  - functions/src/llm/tools/ssrf_guard/guard
  - functions/src/llm/tools/ssrf_guard/checked_redirect_policy
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/llm/provider/gemini/GeminiPart/text
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [parse_method](../../../../../../../functions/src/llm/tools/http/parse_method.md)
- [check_url_not_blocked](../../../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
- [guard](../../../../../../../functions/src/llm/tools/ssrf_guard/guard.md)
- [checked_redirect_policy](../../../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [parse](../../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [text](../../../../../../../functions/src/llm/provider/gemini/GeminiPart/text.md)
- [from_str](../../../../../../../functions/src/config/secrets/SecretString/from_str.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)