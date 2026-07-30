---
type: Rust Function
title: guard
resource: src/llm/tools/ssrf_guard.rs#L150-L152
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/http/HttpClientTool/tool/execute
  - functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
---

# Signature

`pub fn guard(builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder`

# Called by

- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)