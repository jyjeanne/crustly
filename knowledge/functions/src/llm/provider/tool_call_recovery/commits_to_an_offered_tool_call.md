---
type: Rust Function
title: commits_to_an_offered_tool_call
resource: src/llm/provider/tool_call_recovery.rs#L71-L80
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/tool_call_recovery/leading_window
  - functions/src/llm/provider/tool_call_recovery/top_level_name_key_matches
  called_by:
  - functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler
---

# Signature

`pub fn commits_to_an_offered_tool_call(text: &str, offered: &[Tool]) -> bool`

# Calls

- [leading_window](../../../../../functions/src/llm/provider/tool_call_recovery/leading_window.md)
- [top_level_name_key_matches](../../../../../functions/src/llm/provider/tool_call_recovery/top_level_name_key_matches.md)

# Called by

- [maybe_swap_to_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler.md)