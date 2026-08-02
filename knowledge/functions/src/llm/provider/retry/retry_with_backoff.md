---
type: Rust Function
title: retry_with_backoff
resource: src/llm/provider/retry.rs#L112-L166
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete
  - functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream
  - functions/src/llm/provider/gemini/GeminiProvider/provider/complete
  - functions/src/llm/provider/gemini/GeminiProvider/provider/stream
  - functions/src/llm/provider/openai/OpenAIProvider/provider/complete
  - functions/src/llm/provider/openai/OpenAIProvider/provider/stream
  - functions/src/llm/provider/qwen/QwenProvider/provider/complete
  - functions/src/llm/provider/qwen/QwenProvider/provider/stream
  - functions/src/llm/provider/retry/retry_with_rate_limit
  - functions/src/llm/provider/retry/test_retry_success_immediate
  - functions/src/llm/provider/retry/test_retry_success_after_retries
  - functions/src/llm/provider/retry/test_retry_max_attempts_exceeded
  - functions/src/llm/provider/retry/test_retry_non_retryable_error
---

# Signature

`pub async fn retry_with_backoff<F, Fut, T>(mut operation: F, config: &RetryConfig) -> Result<T> where F: FnMut() -> Fut, Fut: Future<Output = Result<T>>,`

# Called by

- [complete](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)
- [complete](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [complete](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [complete](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/complete.md)
- [stream](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [retry_with_rate_limit](../../../../../functions/src/llm/provider/retry/retry_with_rate_limit.md)
- [test_retry_success_immediate](../../../../../functions/src/llm/provider/retry/test_retry_success_immediate.md)
- [test_retry_success_after_retries](../../../../../functions/src/llm/provider/retry/test_retry_success_after_retries.md)
- [test_retry_max_attempts_exceeded](../../../../../functions/src/llm/provider/retry/test_retry_max_attempts_exceeded.md)
- [test_retry_non_retryable_error](../../../../../functions/src/llm/provider/retry/test_retry_non_retryable_error.md)