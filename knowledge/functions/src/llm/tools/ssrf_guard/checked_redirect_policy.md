---
type: Rust Function
title: checked_redirect_policy
resource: src/llm/tools/ssrf_guard.rs#L162-L176
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/check_url_not_blocked
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/checked_redirect_policy_blocks_redirect_to_blocked_address
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn checked_redirect_policy(max_redirects: usize) -> reqwest::redirect::Policy`

# Calls

- [check_url_not_blocked](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [checked_redirect_policy_blocks_redirect_to_blocked_address](../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy_blocks_redirect_to_blocked_address.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)