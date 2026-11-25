---
type: Rust Method
title: detect_command
resource: src/llm/agent/middleware.rs#L493-L506
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/verification_detects_the_check_command_from_build_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn detect_command(dir: &Path, configured: Option<&str>) -> Option<String>`

# Called by

- [verification_detects_the_check_command_from_build_files](../../../../../../functions/src/llm/agent/middleware/verification_detects_the_check_command_from_build_files.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)