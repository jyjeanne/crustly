---
type: Rust Method
title: get
resource: src/llm/tools/registry.rs#L104-L106
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/registry/ToolRegistry/canonical_name
---

# Signature

`pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>>`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)