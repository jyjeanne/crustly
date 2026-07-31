---
type: Rust Function
title: build_gemini_error
resource: src/llm/provider/gemini.rs#L355-L397
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/gemini/GeminiProvider/handle_error
  - functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_with_retry_after
  - functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_without_retry_after
  - functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_no_body
  - functions/src/llm/provider/gemini/test_build_gemini_error_api_error_with_body
  - functions/src/llm/provider/gemini/test_build_gemini_error_no_body_falls_back_to_unknown
---

# Signature

`fn build_gemini_error( status: u16, retry_after: Option<u64>, error_body: Option<GeminiErrorResponse>, ) -> ProviderError`

# Called by

- [handle_error](../../../../../functions/src/llm/provider/gemini/GeminiProvider/handle_error.md)
- [test_build_gemini_error_rate_limit_with_retry_after](../../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_with_retry_after.md)
- [test_build_gemini_error_rate_limit_without_retry_after](../../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_without_retry_after.md)
- [test_build_gemini_error_rate_limit_no_body](../../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_no_body.md)
- [test_build_gemini_error_api_error_with_body](../../../../../functions/src/llm/provider/gemini/test_build_gemini_error_api_error_with_body.md)
- [test_build_gemini_error_no_body_falls_back_to_unknown](../../../../../functions/src/llm/provider/gemini/test_build_gemini_error_no_body_falls_back_to_unknown.md)