---
type: Rust Function
title: system_prompt_tells_the_model_the_working_directory
resource: src/llm/agent/service.rs#L2098-L2118
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/system_prompt_with_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn system_prompt_tells_the_model_the_working_directory()`

# Calls

- [system_prompt_with_env](../../../../../functions/src/llm/agent/service/AgentService/system_prompt_with_env.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)