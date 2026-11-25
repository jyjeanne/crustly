---
type: Rust Function
title: execute_denies_cloud_metadata_endpoint
resource: src/llm/tools/http.rs#L338-L348
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute_denies_cloud_metadata_endpoint()`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_auto_approve](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve.md)
- [execute](../../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)