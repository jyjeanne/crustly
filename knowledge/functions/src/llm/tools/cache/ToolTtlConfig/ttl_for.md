---
type: Rust Method
title: ttl_for
resource: src/llm/tools/cache.rs#L61-L71
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/cache/ToolResultCache/insert_for_tool
---

# Signature

`pub fn ttl_for(&self, tool_name: &str) -> Duration`

# Called by

- [insert_for_tool](../../../../../../functions/src/llm/tools/cache/ToolResultCache/insert_for_tool.md)