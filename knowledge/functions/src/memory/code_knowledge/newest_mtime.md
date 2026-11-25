---
type: Rust Function
title: newest_mtime
resource: src/memory/code_knowledge.rs#L633-L652
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/bundle_time
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/bundle_is_older_than_sources
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn newest_mtime(dir: &Path) -> Option<std::time::SystemTime>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [bundle_time](../../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_time.md)
- [bundle_is_older_than_sources](../../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_is_older_than_sources.md)