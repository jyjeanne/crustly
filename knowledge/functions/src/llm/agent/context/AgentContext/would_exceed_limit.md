---
type: Rust Method
title: would_exceed_limit
resource: src/llm/agent/context.rs#L111-L113
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/context/AgentContext/trim_to_fit
---

# Signature

`pub fn would_exceed_limit(&self, additional_tokens: usize) -> bool`

# Called by

- [trim_to_fit](../../../../../../functions/src/llm/agent/context/AgentContext/trim_to_fit.md)