---
type: Rust Function
title: simple_prompt_routes_to_fast_tier
resource: tests/model_routing_test.rs#L77-L93
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn simple_prompt_routes_to_fast_tier()`

# Calls

- [new](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)
- [classify_tier](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier.md)