---
type: Rust Method
title: build_client
resource: src/llm/provider/qwen.rs#L213-L221
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/with_base_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_client() -> Client`

# Calls

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [local](../../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_base_url](../../../../../../functions/src/llm/provider/qwen/QwenProvider/with_base_url.md)