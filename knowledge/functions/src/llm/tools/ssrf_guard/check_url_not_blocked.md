---
type: Rust Function
title: check_url_not_blocked
resource: src/llm/tools/ssrf_guard.rs#L129-L144
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ip
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/checked_redirect_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/check_url_not_blocked_rejects_ip_literal_metadata_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/check_url_not_blocked_allows_normal_domain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn check_url_not_blocked(url: &str) -> Result<(), String>`

# Calls

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [checked_redirect_policy](../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [check_url_not_blocked_rejects_ip_literal_metadata_url](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked_rejects_ip_literal_metadata_url.md)
- [check_url_not_blocked_allows_normal_domain](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked_allows_normal_domain.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)