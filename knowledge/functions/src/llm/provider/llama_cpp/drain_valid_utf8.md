---
type: Rust Function
title: drain_valid_utf8
resource: src/llm/provider/llama_cpp.rs#L1017-L1031
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/provider/llama_cpp/run_stream
  - functions/src/llm/provider/llama_cpp/drain_valid_utf8_multiple_tokens_reassemble_correctly
  - functions/src/llm/provider/llama_cpp/drain_valid_utf8_never_panics_on_arbitrary_bytes
---

# Signature

`fn drain_valid_utf8(buffer: &mut Vec<u8>) -> Option<String>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)
- [drain_valid_utf8_multiple_tokens_reassemble_correctly](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_multiple_tokens_reassemble_correctly.md)
- [drain_valid_utf8_never_panics_on_arbitrary_bytes](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_never_panics_on_arbitrary_bytes.md)