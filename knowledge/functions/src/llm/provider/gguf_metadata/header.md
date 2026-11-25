---
type: Rust Function
title: header
resource: src/llm/provider/gguf_metadata.rs#L576-L583
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/parses_architecture_name_context_length_and_chat_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/missing_optional_keys_still_parses_successfully
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_true_sets_is_vision_projector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_false_does_not_set_is_vision_projector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/clip_has_audio_encoder_true_sets_is_audio_projector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/neither_clip_key_present_leaves_both_projector_flags_false
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/general_file_type_maps_to_the_precise_quantization_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/tensor_type_mode_is_the_fallback_when_file_type_is_absent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/tensor_type_mode_breaks_a_tensor_count_tie_deterministically
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/parameter_count_sums_across_tensor_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/a_string_array_between_two_kv_pairs_is_skipped_correctly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/truncation_at_several_points_returns_none_not_a_panic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/oversized_declared_string_length_is_rejected_before_allocating
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/oversized_declared_array_length_is_rejected
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn header(tensor_count: u64, kv_count: u64) -> Vec<u8>`

# Called by

- [parses_architecture_name_context_length_and_chat_template](../../../../../functions/src/llm/provider/gguf_metadata/parses_architecture_name_context_length_and_chat_template.md)
- [missing_optional_keys_still_parses_successfully](../../../../../functions/src/llm/provider/gguf_metadata/missing_optional_keys_still_parses_successfully.md)
- [clip_has_vision_encoder_true_sets_is_vision_projector](../../../../../functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_true_sets_is_vision_projector.md)
- [clip_has_vision_encoder_false_does_not_set_is_vision_projector](../../../../../functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_false_does_not_set_is_vision_projector.md)
- [clip_has_audio_encoder_true_sets_is_audio_projector](../../../../../functions/src/llm/provider/gguf_metadata/clip_has_audio_encoder_true_sets_is_audio_projector.md)
- [neither_clip_key_present_leaves_both_projector_flags_false](../../../../../functions/src/llm/provider/gguf_metadata/neither_clip_key_present_leaves_both_projector_flags_false.md)
- [general_file_type_maps_to_the_precise_quantization_string](../../../../../functions/src/llm/provider/gguf_metadata/general_file_type_maps_to_the_precise_quantization_string.md)
- [tensor_type_mode_is_the_fallback_when_file_type_is_absent](../../../../../functions/src/llm/provider/gguf_metadata/tensor_type_mode_is_the_fallback_when_file_type_is_absent.md)
- [tensor_type_mode_breaks_a_tensor_count_tie_deterministically](../../../../../functions/src/llm/provider/gguf_metadata/tensor_type_mode_breaks_a_tensor_count_tie_deterministically.md)
- [parameter_count_sums_across_tensor_dimensions](../../../../../functions/src/llm/provider/gguf_metadata/parameter_count_sums_across_tensor_dimensions.md)
- [a_string_array_between_two_kv_pairs_is_skipped_correctly](../../../../../functions/src/llm/provider/gguf_metadata/a_string_array_between_two_kv_pairs_is_skipped_correctly.md)
- [truncation_at_several_points_returns_none_not_a_panic](../../../../../functions/src/llm/provider/gguf_metadata/truncation_at_several_points_returns_none_not_a_panic.md)
- [oversized_declared_string_length_is_rejected_before_allocating](../../../../../functions/src/llm/provider/gguf_metadata/oversized_declared_string_length_is_rejected_before_allocating.md)
- [oversized_declared_array_length_is_rejected](../../../../../functions/src/llm/provider/gguf_metadata/oversized_declared_array_length_is_rejected.md)