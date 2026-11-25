---
type: Rust Method
title: closest
resource: src/memory/code_knowledge.rs#L570-L586
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/call_json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/set_anchors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/drifted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn closest(&self, id: &str) -> Option<String>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [call_json](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)

# Called by

- [set_anchors](../../../../../functions/src/memory/anchors/MemoryStore/set_anchors.md)
- [drifted](../../../../../functions/src/memory/anchors/MemoryStore/drifted.md)