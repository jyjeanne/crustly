---
type: Rust Function
title: guard
resource: src/llm/tools/ssrf_guard.rs#L147-L149
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn guard(builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder`

# Called by

- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)