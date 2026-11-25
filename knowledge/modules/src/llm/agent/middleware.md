---
type: Rust Module
title: middleware
resource: src/llm/agent/middleware.rs#L1-L780
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-llm-tools-toolcapability
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hash-map-defaulthasher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-hash-hash-hasher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Decision](../../../../classes/src/llm/agent/middleware/Decision.md)
- [ToolCallView](../../../../classes/src/llm/agent/middleware/ToolCallView.md)
- [ToolCallRecord](../../../../classes/src/llm/agent/middleware/ToolCallRecord.md)
- [RunState](../../../../classes/src/llm/agent/middleware/RunState.md)
- [RoundMiddleware](../../../../interfaces/src/llm/agent/middleware/RoundMiddleware.md)
- [before_round](../../../../functions/src/llm/agent/middleware/RoundMiddleware/before_round.md)
- [on_tool_calls](../../../../functions/src/llm/agent/middleware/RoundMiddleware/on_tool_calls.md)
- [after_tools](../../../../functions/src/llm/agent/middleware/RoundMiddleware/after_tools.md)
- [on_final_answer](../../../../functions/src/llm/agent/middleware/RoundMiddleware/on_final_answer.md)
- [Pipeline](../../../../classes/src/llm/agent/middleware/Pipeline.md)
- [new](../../../../functions/src/llm/agent/middleware/Pipeline/new.md)
- [with](../../../../functions/src/llm/agent/middleware/Pipeline/with.md)
- [first_decision](../../../../functions/src/llm/agent/middleware/Pipeline/first_decision.md)
- [before_round](../../../../functions/src/llm/agent/middleware/Pipeline/before_round.md)
- [on_tool_calls](../../../../functions/src/llm/agent/middleware/Pipeline/on_tool_calls.md)
- [after_tools](../../../../functions/src/llm/agent/middleware/Pipeline/after_tools.md)
- [on_final_answer](../../../../functions/src/llm/agent/middleware/Pipeline/on_final_answer.md)
- [has_mutating_capability](../../../../functions/src/llm/agent/middleware/has_mutating_capability.md)
- [tool_call_signature](../../../../functions/src/llm/agent/middleware/tool_call_signature.md)
- [LoopGuard](../../../../classes/src/llm/agent/middleware/LoopGuard.md)
- [new](../../../../functions/src/llm/agent/middleware/LoopGuard/new.md)
- [on_tool_calls](../../../../functions/src/llm/agent/middleware/LoopGuard/roundmiddleware/on_tool_calls.md)
- [Limits](../../../../classes/src/llm/agent/middleware/Limits.md)
- [default](../../../../functions/src/llm/agent/middleware/Limits/default/default.md)
- [finish_at_limit](../../../../functions/src/llm/agent/middleware/finish_at_limit.md)
- [before_round](../../../../functions/src/llm/agent/middleware/Limits/roundmiddleware/before_round.md)
- [is_file_edit](../../../../functions/src/llm/agent/middleware/is_file_edit.md)
- [Verification](../../../../classes/src/llm/agent/middleware/Verification.md)
- [new](../../../../functions/src/llm/agent/middleware/Verification/new.md)
- [detect_command](../../../../functions/src/llm/agent/middleware/Verification/detect_command.md)
- [is_check](../../../../functions/src/llm/agent/middleware/Verification/is_check.md)
- [after_tools](../../../../functions/src/llm/agent/middleware/Verification/roundmiddleware/after_tools.md)
- [on_final_answer](../../../../functions/src/llm/agent/middleware/Verification/roundmiddleware/on_final_answer.md)
- [state](../../../../functions/src/llm/agent/middleware/state.md)
- [call](../../../../functions/src/llm/agent/middleware/call.md)
- [record](../../../../functions/src/llm/agent/middleware/record.md)
- [loop_guard_stops_a_repeated_write_on_the_second_call](../../../../functions/src/llm/agent/middleware/loop_guard_stops_a_repeated_write_on_the_second_call.md)
- [loop_guard_lets_exploration_revisit_a_path](../../../../functions/src/llm/agent/middleware/loop_guard_lets_exploration_revisit_a_path.md)
- [loop_guard_stops_drift_but_not_silent_plan_building](../../../../functions/src/llm/agent/middleware/loop_guard_stops_drift_but_not_silent_plan_building.md)
- [limits_finish_with_a_notice_after_the_last_round](../../../../functions/src/llm/agent/middleware/limits_finish_with_a_notice_after_the_last_round.md)
- [limits_finish_on_tokens_and_wall_time](../../../../functions/src/llm/agent/middleware/limits_finish_on_tokens_and_wall_time.md)
- [verification_nudges_after_an_edit_until_a_check_passes](../../../../functions/src/llm/agent/middleware/verification_nudges_after_an_edit_until_a_check_passes.md)
- [verification_gives_up_after_two_nudges](../../../../functions/src/llm/agent/middleware/verification_gives_up_after_two_nudges.md)
- [verification_detects_the_check_command_from_build_files](../../../../functions/src/llm/agent/middleware/verification_detects_the_check_command_from_build_files.md)
- [pipeline_consults_everyone_and_the_first_decision_wins](../../../../functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins.md)
- [Fixed](../../../../classes/src/llm/agent/middleware/Fixed.md)
- [before_round](../../../../functions/src/llm/agent/middleware/Fixed/roundmiddleware/before_round.md)

# Imports

- `crate::llm::tools::ToolCapability`
- `serde_json::Value`
- `std::path::Path`
- `std::time::{Duration, Instant}`
- `std::collections::hash_map::DefaultHasher`
- `std::hash::{Hash, Hasher}`
- `super::*`
- `serde_json::json`

# Member of

- [crustly](../../../../packages/crustly.md)