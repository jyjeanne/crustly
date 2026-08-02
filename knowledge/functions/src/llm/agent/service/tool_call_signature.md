---
type: Rust Function
title: tool_call_signature
resource: src/llm/agent/service.rs#L71-L140
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/signature_uses_path_key_so_different_edits_do_not_collide
  - functions/src/llm/agent/service/signature_accepts_file_path_alias
---

# Signature

`fn tool_call_signature(name: &str, input: &Value) -> String`

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [signature_uses_path_key_so_different_edits_do_not_collide](../../../../../functions/src/llm/agent/service/signature_uses_path_key_so_different_edits_do_not_collide.md)
- [signature_accepts_file_path_alias](../../../../../functions/src/llm/agent/service/signature_accepts_file_path_alias.md)