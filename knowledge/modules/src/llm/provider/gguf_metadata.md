---
type: Rust Module
title: gguf_metadata
resource: src/llm/provider/gguf_metadata.rs#L1-L910
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-bufreader-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write-as
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [GgufMetadata](../../../../classes/src/llm/provider/gguf_metadata/GgufMetadata.md)
- [read_gguf_metadata](../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)
- [ValueType](../../../../classes/src/llm/provider/gguf_metadata/ValueType.md)
- [from_tag](../../../../functions/src/llm/provider/gguf_metadata/ValueType/from_tag.md)
- [scalar_width](../../../../functions/src/llm/provider/gguf_metadata/ValueType/scalar_width.md)
- [DecodedValue](../../../../classes/src/llm/provider/gguf_metadata/DecodedValue.md)
- [BoundedReader](../../../../classes/src/llm/provider/gguf_metadata/BoundedReader.md)
- [new](../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/new.md)
- [read_exact](../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact.md)
- [read_u32](../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u32.md)
- [read_u64](../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64.md)
- [read_string](../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_string.md)
- [read_value](../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [parse](../../../../functions/src/llm/provider/gguf_metadata/parse.md)
- [ftype_to_string](../../../../functions/src/llm/provider/gguf_metadata/ftype_to_string.md)
- [ggml_type_to_string](../../../../functions/src/llm/provider/gguf_metadata/ggml_type_to_string.md)
- [MemoryEstimate](../../../../classes/src/llm/provider/gguf_metadata/MemoryEstimate.md)
- [bits_per_weight](../../../../functions/src/llm/provider/gguf_metadata/bits_per_weight.md)
- [estimate_memory_bytes](../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes.md)
- [push_string](../../../../functions/src/llm/provider/gguf_metadata/push_string.md)
- [push_string_kv](../../../../functions/src/llm/provider/gguf_metadata/push_string_kv.md)
- [push_u32_kv](../../../../functions/src/llm/provider/gguf_metadata/push_u32_kv.md)
- [push_bool_kv](../../../../functions/src/llm/provider/gguf_metadata/push_bool_kv.md)
- [push_tensor](../../../../functions/src/llm/provider/gguf_metadata/push_tensor.md)
- [header](../../../../functions/src/llm/provider/gguf_metadata/header.md)
- [write_temp_gguf](../../../../functions/src/llm/provider/gguf_metadata/write_temp_gguf.md)
- [parses_architecture_name_context_length_and_chat_template](../../../../functions/src/llm/provider/gguf_metadata/parses_architecture_name_context_length_and_chat_template.md)
- [missing_optional_keys_still_parses_successfully](../../../../functions/src/llm/provider/gguf_metadata/missing_optional_keys_still_parses_successfully.md)
- [clip_has_vision_encoder_true_sets_is_vision_projector](../../../../functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_true_sets_is_vision_projector.md)
- [clip_has_vision_encoder_false_does_not_set_is_vision_projector](../../../../functions/src/llm/provider/gguf_metadata/clip_has_vision_encoder_false_does_not_set_is_vision_projector.md)
- [clip_has_audio_encoder_true_sets_is_audio_projector](../../../../functions/src/llm/provider/gguf_metadata/clip_has_audio_encoder_true_sets_is_audio_projector.md)
- [neither_clip_key_present_leaves_both_projector_flags_false](../../../../functions/src/llm/provider/gguf_metadata/neither_clip_key_present_leaves_both_projector_flags_false.md)
- [general_file_type_maps_to_the_precise_quantization_string](../../../../functions/src/llm/provider/gguf_metadata/general_file_type_maps_to_the_precise_quantization_string.md)
- [tensor_type_mode_is_the_fallback_when_file_type_is_absent](../../../../functions/src/llm/provider/gguf_metadata/tensor_type_mode_is_the_fallback_when_file_type_is_absent.md)
- [tensor_type_mode_breaks_a_tensor_count_tie_deterministically](../../../../functions/src/llm/provider/gguf_metadata/tensor_type_mode_breaks_a_tensor_count_tie_deterministically.md)
- [parameter_count_sums_across_tensor_dimensions](../../../../functions/src/llm/provider/gguf_metadata/parameter_count_sums_across_tensor_dimensions.md)
- [a_string_array_between_two_kv_pairs_is_skipped_correctly](../../../../functions/src/llm/provider/gguf_metadata/a_string_array_between_two_kv_pairs_is_skipped_correctly.md)
- [rejects_bad_magic](../../../../functions/src/llm/provider/gguf_metadata/rejects_bad_magic.md)
- [rejects_unsupported_version](../../../../functions/src/llm/provider/gguf_metadata/rejects_unsupported_version.md)
- [truncation_at_several_points_returns_none_not_a_panic](../../../../functions/src/llm/provider/gguf_metadata/truncation_at_several_points_returns_none_not_a_panic.md)
- [oversized_declared_string_length_is_rejected_before_allocating](../../../../functions/src/llm/provider/gguf_metadata/oversized_declared_string_length_is_rejected_before_allocating.md)
- [oversized_declared_array_length_is_rejected](../../../../functions/src/llm/provider/gguf_metadata/oversized_declared_array_length_is_rejected.md)
- [oversized_declared_kv_count_is_rejected_before_reading_any_of_them](../../../../functions/src/llm/provider/gguf_metadata/oversized_declared_kv_count_is_rejected_before_reading_any_of_them.md)
- [nonexistent_file_returns_none](../../../../functions/src/llm/provider/gguf_metadata/nonexistent_file_returns_none.md)
- [estimate_memory_bytes_is_none_without_parameter_count_or_quantization](../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_is_none_without_parameter_count_or_quantization.md)
- [estimate_memory_bytes_covers_weights_only_when_geometry_is_missing](../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_covers_weights_only_when_geometry_is_missing.md)
- [estimate_memory_bytes_includes_kv_cache_when_geometry_is_present](../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_includes_kv_cache_when_geometry_is_present.md)
- [estimate_memory_bytes_falls_back_to_head_count_without_gqa_kv_heads](../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_falls_back_to_head_count_without_gqa_kv_heads.md)

# Imports

- `std::collections::HashMap`
- `std::fs::File`
- `std::io::{BufReader, Read}`
- `std::path::Path`
- `super::*`
- `std::io::Write as _`

# Member of

- [crustly](../../../../packages/crustly.md)