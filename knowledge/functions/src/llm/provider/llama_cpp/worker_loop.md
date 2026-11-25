---
type: Rust Function
title: worker_loop
resource: src/llm/provider/llama_cpp.rs#L265-L442
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/dispatch_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/LlamaCppProvider/new
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn worker_loop(init: WorkerInit)`

# Calls

- [dispatch_job](../../../../../functions/src/llm/provider/llama_cpp/dispatch_job.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [new](../../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/new.md)