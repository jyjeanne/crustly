---
type: Rust Method
title: block
resource: src/plan/mod.rs#L692-L694
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/plan/plan_tests/test_task_blocking
  - functions/src/tui/render/render_header
  - functions/src/tui/render/render_chat
  - functions/src/tui/render/render_sessions
  - functions/src/tui/render/render_skills
  - functions/src/tui/render/render_mcp
  - functions/src/tui/render/render_help
  - functions/src/tui/render/render_plan_help
  - functions/src/tui/render/render_plan
  - functions/src/tui/render/render_settings
  - functions/src/tui/render/render_approval
  - functions/src/tui/render/render_file_picker
  - functions/src/tui/render/render_model_info
  - functions/src/tui/render/render_provider_switch
  - functions/src/tui/render/render_model_download
  - functions/src/tui/render/render_model_download_progress
  - functions/src/tui/render/render_model_download_confirm_delete
  - functions/src/tui/render/render_model_download_deleting
  - functions/src/tui/splash/render_splash_content
---

# Signature

`pub fn block(&mut self, reason: String)`

# Called by

- [test_task_blocking](../../../../functions/src/plan/plan_tests/test_task_blocking.md)
- [render_header](../../../../functions/src/tui/render/render_header.md)
- [render_chat](../../../../functions/src/tui/render/render_chat.md)
- [render_sessions](../../../../functions/src/tui/render/render_sessions.md)
- [render_skills](../../../../functions/src/tui/render/render_skills.md)
- [render_mcp](../../../../functions/src/tui/render/render_mcp.md)
- [render_help](../../../../functions/src/tui/render/render_help.md)
- [render_plan_help](../../../../functions/src/tui/render/render_plan_help.md)
- [render_plan](../../../../functions/src/tui/render/render_plan.md)
- [render_settings](../../../../functions/src/tui/render/render_settings.md)
- [render_approval](../../../../functions/src/tui/render/render_approval.md)
- [render_file_picker](../../../../functions/src/tui/render/render_file_picker.md)
- [render_model_info](../../../../functions/src/tui/render/render_model_info.md)
- [render_provider_switch](../../../../functions/src/tui/render/render_provider_switch.md)
- [render_model_download](../../../../functions/src/tui/render/render_model_download.md)
- [render_model_download_progress](../../../../functions/src/tui/render/render_model_download_progress.md)
- [render_model_download_confirm_delete](../../../../functions/src/tui/render/render_model_download_confirm_delete.md)
- [render_model_download_deleting](../../../../functions/src/tui/render/render_model_download_deleting.md)
- [render_splash_content](../../../../functions/src/tui/splash/render_splash_content.md)