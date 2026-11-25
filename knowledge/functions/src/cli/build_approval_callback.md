---
type: Rust Function
title: build_approval_callback
resource: src/cli/mod.rs#L1789-L1845
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/auto_mode_bypasses_approval
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_approval_callback( event_sender: tokio::sync::mpsc::UnboundedSender<crate::tui::events::TuiEvent>, auto_mode: Arc<std::sync::Mutex<crate::config::PlanExecMode>>, ) -> crate::llm::agent::ApprovalCallback`

# Calls

- [auto_mode_bypasses_approval](../../../functions/src/cli/auto_mode_bypasses_approval.md)

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)