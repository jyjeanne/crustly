---
type: Rust Function
title: cmd_init
resource: src/cli/mod.rs#L977-L1006
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/Config/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_init(_config: &crate::config::Config, force: bool) -> Result<()>`

# Calls

- [context](../../../functions/src/services/ServiceManager/context.md)
- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [default](../../../functions/src/config/Config/default/default.md)
- [save](../../../functions/src/config/Config/save.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [run](../../../functions/src/cli/run.md)