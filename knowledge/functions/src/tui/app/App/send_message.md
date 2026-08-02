---
type: Rust Method
title: send_message
resource: src/tui/app.rs#L1547-L1636
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform
  - functions/src/tui/app/App/event_sender
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming
---

# Signature

`async fn send_message(&mut self, content: String) -> Result<()>`

# Calls

- [analyze_and_transform](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/analyze_and_transform.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [send_message_with_tools_and_mode_streaming](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming.md)