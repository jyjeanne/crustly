---
type: Rust Function
title: neutral_prompt_routes_to_balanced_tier
resource: tests/model_routing_test.rs#L97-L102
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn neutral_prompt_routes_to_balanced_tier()`

# Calls

- [new](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)
- [classify_tier](../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier.md)