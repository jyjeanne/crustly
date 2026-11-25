---
type: Rust Method
title: has_tool
resource: src/llm/tools/registry.rs#L134-L136
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/canonical_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/autoplan_prompts_for_the_registered_code_and_shell_tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/build_tool_registry_registers_every_built_in_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/test_register_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/has_tool_is_false_for_an_alias_whose_target_is_not_registered
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn has_tool(&self, name: &str) -> bool`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)

# Called by

- [autoplan_prompts_for_the_registered_code_and_shell_tool_names](../../../../../../functions/src/cli/autoplan_prompts_for_the_registered_code_and_shell_tool_names.md)
- [build_tool_registry_registers_every_built_in_tool](../../../../../../functions/src/cli/build_tool_registry_registers_every_built_in_tool.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)
- [test_register_tool](../../../../../../functions/src/llm/tools/registry/test_register_tool.md)
- [get_resolves_a_known_alias_to_the_registered_canonical_tool](../../../../../../functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool.md)
- [has_tool_is_false_for_an_alias_whose_target_is_not_registered](../../../../../../functions/src/llm/tools/registry/has_tool_is_false_for_an_alias_whose_target_is_not_registered.md)