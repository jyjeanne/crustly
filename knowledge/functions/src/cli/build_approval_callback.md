---
type: Rust Function
title: build_approval_callback
resource: src/cli/mod.rs#L696-L751
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/cli/auto_mode_bypasses_approval
  called_by:
  - functions/src/cli/cmd_chat
---

# Signature

`fn build_approval_callback( event_sender: tokio::sync::mpsc::UnboundedSender<crate::tui::events::TuiEvent>, auto_mode: Arc<std::sync::Mutex<crate::config::PlanExecMode>>, ) -> crate::llm::agent::ApprovalCallback`

# Calls

- [auto_mode_bypasses_approval](../../../functions/src/cli/auto_mode_bypasses_approval.md)

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)