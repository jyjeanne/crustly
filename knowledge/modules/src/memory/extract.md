---
type: Rust Module
title: extract
resource: src/memory/extract.rs#L1-L406
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-model-validate-memorycategory-max-content-chars
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-secret-scan
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Turn](../../../classes/src/memory/extract/Turn.md)
- [transcript](../../../functions/src/memory/extract/transcript.md)
- [redact_and_omit](../../../functions/src/memory/extract/redact_and_omit.md)
- [redact_line](../../../functions/src/memory/extract/redact_line.md)
- [should_extract](../../../functions/src/memory/extract/should_extract.md)
- [build_prompt](../../../functions/src/memory/extract/build_prompt.md)
- [Candidate](../../../classes/src/memory/extract/Candidate.md)
- [Raw](../../../classes/src/memory/extract/Raw.md)
- [ParseError](../../../classes/src/memory/extract/ParseError.md)
- [parse](../../../functions/src/memory/extract/parse.md)
- [check](../../../functions/src/memory/extract/check.md)
- [turn](../../../functions/src/memory/extract/turn.md)
- [long_user_messages_are_cut_and_answers_cut_shorter](../../../functions/src/memory/extract/long_user_messages_are_cut_and_answers_cut_shorter.md)
- [long_code_is_omitted_and_short_code_is_kept](../../../functions/src/memory/extract/long_code_is_omitted_and_short_code_is_kept.md)
- [secret_lines_are_redacted_before_the_model_sees_them](../../../functions/src/memory/extract/secret_lines_are_redacted_before_the_model_sees_them.md)
- [extraction_runs_after_enough_messages_or_at_session_end](../../../functions/src/memory/extract/extraction_runs_after_enough_messages_or_at_session_end.md)
- [the_prompt_carries_the_saved_memories_and_the_transcript](../../../functions/src/memory/extract/the_prompt_carries_the_saved_memories_and_the_transcript.md)
- [the_spec_example_parses](../../../functions/src/memory/extract/the_spec_example_parses.md)
- [invalid_candidates_are_dropped_one_by_one](../../../functions/src/memory/extract/invalid_candidates_are_dropped_one_by_one.md)
- [an_episode_only_at_session_end_and_only_one](../../../functions/src/memory/extract/an_episode_only_at_session_end_and_only_one.md)
- [the_per_run_limit_applies_to_normal_memories](../../../functions/src/memory/extract/the_per_run_limit_applies_to_normal_memories.md)
- [an_answer_without_json_is_a_failure](../../../functions/src/memory/extract/an_answer_without_json_is_a_failure.md)

# Imports

- `super::model::{validate, MemoryCategory, MAX_CONTENT_CHARS}`
- `super::secret_scan`
- `serde::Deserialize`
- `uuid::Uuid`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)