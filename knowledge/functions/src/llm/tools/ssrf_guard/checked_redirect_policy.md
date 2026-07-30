---
type: Rust Function
title: checked_redirect_policy
resource: src/llm/tools/ssrf_guard.rs#L165-L179
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/ssrf_guard/check_url_not_blocked
  called_by:
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
  - functions/src/llm/tools/ssrf_guard/checked_redirect_policy_blocks_redirect_to_blocked_address
  - functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
---

# Signature

`pub fn checked_redirect_policy(max_redirects: usize) -> reqwest::redirect::Policy`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [check_url_not_blocked](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [checked_redirect_policy_blocks_redirect_to_blocked_address](../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy_blocks_redirect_to_blocked_address.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)