---
type: Rust Method
title: export_plan_to_markdown
resource: src/tui/app.rs#L2902-L2979
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_plan_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn export_plan_to_markdown(&self, filename: &str) -> Result<()>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [write](../../../../../functions/src/memory/project/write.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)