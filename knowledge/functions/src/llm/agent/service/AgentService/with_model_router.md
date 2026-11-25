---
type: Rust Method
title: with_model_router
resource: src/llm/agent/service.rs#L726-L729
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/the_router_picks_the_model_for_the_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_model_router(mut self, router: ModelRouter) -> Self`

# Called by

- [configure_agent](../../../../../../functions/src/cli/configure_agent.md)
- [the_router_picks_the_model_for_the_request](../../../../../../functions/src/llm/agent/service/the_router_picks_the_model_for_the_request.md)