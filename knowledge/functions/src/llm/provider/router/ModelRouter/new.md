---
type: Rust Method
title: new
resource: src/llm/provider/router.rs#L26-L39
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/the_router_picks_the_model_for_the_request
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/router/ModelRouter/default_anthropic
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/router/ModelRouter/default_for_test
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new( fast_provider: &str, fast_model: &str, balanced_provider: &str, balanced_model: &str, powerful_provider: &str, powerful_model: &str, ) -> Self`

# Called by

- [configure_agent](../../../../../../functions/src/cli/configure_agent.md)
- [the_router_picks_the_model_for_the_request](../../../../../../functions/src/llm/agent/service/the_router_picks_the_model_for_the_request.md)
- [default_anthropic](../../../../../../functions/src/llm/provider/router/ModelRouter/default_anthropic.md)
- [default_for_test](../../../../../../functions/src/llm/provider/router/ModelRouter/default_for_test.md)