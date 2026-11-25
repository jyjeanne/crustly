---
type: Rust Module
title: model
resource: src/memory/model.rs#L1-L414
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fmt
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-str-fromstr
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

- [MemoryCategory](../../../classes/src/memory/model/MemoryCategory.md)
- [as_str](../../../functions/src/memory/model/MemoryCategory/as_str.md)
- [from_str](../../../functions/src/memory/model/MemoryCategory/fromstr/from_str.md)
- [MemorySource](../../../classes/src/memory/model/MemorySource.md)
- [as_str](../../../functions/src/memory/model/MemorySource/as_str.md)
- [from_str](../../../functions/src/memory/model/MemorySource/fromstr/from_str.md)
- [MemoryStatus](../../../classes/src/memory/model/MemoryStatus.md)
- [as_str](../../../functions/src/memory/model/MemoryStatus/as_str.md)
- [from_str](../../../functions/src/memory/model/MemoryStatus/fromstr/from_str.md)
- [MemoryScope](../../../classes/src/memory/model/MemoryScope.md)
- [ParseEnumError](../../../classes/src/memory/model/ParseEnumError.md)
- [new](../../../functions/src/memory/model/ParseEnumError/new.md)
- [fmt](../../../functions/src/memory/model/ParseEnumError/fmt-display/fmt.md)
- [Memory](../../../classes/src/memory/model/Memory.md)
- [NewMemory](../../../classes/src/memory/model/NewMemory.md)
- [SaveOutcome](../../../classes/src/memory/model/SaveOutcome.md)
- [RejectReason](../../../classes/src/memory/model/RejectReason.md)
- [fmt](../../../functions/src/memory/model/RejectReason/fmt-display/fmt.md)
- [normalize](../../../functions/src/memory/model/normalize.md)
- [content_hash](../../../functions/src/memory/model/content_hash.md)
- [validate](../../../functions/src/memory/model/validate.md)
- [category_strings_round_trip](../../../functions/src/memory/model/category_strings_round_trip.md)
- [source_and_status_strings_round_trip](../../../functions/src/memory/model/source_and_status_strings_round_trip.md)
- [strings_match_the_migration_check_lists](../../../functions/src/memory/model/strings_match_the_migration_check_lists.md)
- [content_hash_matches_a_fixed_digest](../../../functions/src/memory/model/content_hash_matches_a_fixed_digest.md)
- [content_hash_ignores_case_spacing_and_one_final_period](../../../functions/src/memory/model/content_hash_ignores_case_spacing_and_one_final_period.md)
- [normalize_collapses_whitespace](../../../functions/src/memory/model/normalize_collapses_whitespace.md)
- [validate_accepts_a_single_fact](../../../functions/src/memory/model/validate_accepts_a_single_fact.md)
- [validate_rejects_empty_text](../../../functions/src/memory/model/validate_rejects_empty_text.md)
- [validate_rejects_long_text](../../../functions/src/memory/model/validate_rejects_long_text.md)
- [validate_rejects_several_paragraphs](../../../functions/src/memory/model/validate_rejects_several_paragraphs.md)
- [validate_rejects_code_blocks](../../../functions/src/memory/model/validate_rejects_code_blocks.md)
- [reject_messages_are_actionable](../../../functions/src/memory/model/reject_messages_are_actionable.md)

# Imports

- `chrono::{DateTime, Utc}`
- `sha2::{Digest, Sha256}`
- `std::fmt`
- `std::str::FromStr`
- `uuid::Uuid`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)