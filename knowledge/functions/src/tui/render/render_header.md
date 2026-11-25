---
type: Rust Function
title: render_header
resource: src/tui/render.rs#L201-L307
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/total_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/total_cost
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/tail_chars
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/context_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/context_usage_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/provider_icon
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/short_token_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_header(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [total_tokens](../../../../functions/src/tui/app/App/total_tokens.md)
- [total_cost](../../../../functions/src/tui/app/App/total_cost.md)
- [tail_chars](../../../../functions/src/tui/render/tail_chars.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [context_usage](../../../../functions/src/tui/app/App/context_usage.md)
- [context_usage_color](../../../../functions/src/tui/render/context_usage_color.md)
- [provider_icon](../../../../functions/src/tui/render/provider_icon.md)
- [short_token_count](../../../../functions/src/tui/render/short_token_count.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)