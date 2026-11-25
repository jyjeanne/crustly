---
type: Rust Method
title: resolve_name
resource: src/llm/tools/registry.rs#L97-L99
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/canonical_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/resolve_name_maps_aliases_and_keeps_unknown_names
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_name(&self, name: &str) -> String`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)

# Called by

- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [resolve_name_maps_aliases_and_keeps_unknown_names](../../../../../../functions/src/llm/tools/registry/resolve_name_maps_aliases_and_keeps_unknown_names.md)