---
type: Rust Method
title: new
resource: src/tui/app.rs#L405-L500
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/plain_textarea
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(agent_service: Arc<AgentService>, context: ServiceContext) -> Self`

# Calls

- [context_usage_preview](../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [plain_textarea](../../../../../functions/src/tui/app/plain_textarea.md)
- [new](../../../../../functions/src/services/session/SessionService/new.md)
- [new](../../../../../functions/src/services/message/MessageService/new.md)
- [new](../../../../../functions/src/services/plan/PlanService/new.md)
- [new](../../../../../functions/src/tui/events/EventHandler/new.md)
- [new](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/new.md)