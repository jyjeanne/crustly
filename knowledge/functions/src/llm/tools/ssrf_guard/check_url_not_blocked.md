---
type: Rust Function
title: check_url_not_blocked
resource: src/llm/tools/ssrf_guard.rs#L132-L147
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/llm/tools/ssrf_guard/is_blocked_ip
  called_by:
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
  - functions/src/llm/tools/ssrf_guard/checked_redirect_policy
  - functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
---

# Signature

`pub fn check_url_not_blocked(url: &str) -> Result<(), String>`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [checked_redirect_policy](../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)