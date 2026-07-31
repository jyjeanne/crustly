---
type: Rust Function
title: create_provider
resource: src/llm/provider/factory.rs#L141-L169
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/factory/try_create_qwen
  - functions/src/llm/provider/factory/try_create_openai
  - functions/src/llm/provider/factory/try_create_gemini
  - functions/src/llm/provider/factory/try_create_azure
  - functions/src/llm/provider/factory/create_anthropic
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_run
  - functions/src/llm/provider/factory/test_create_provider_with_anthropic
  - functions/src/llm/provider/factory/test_create_provider_with_openai
  - functions/src/llm/provider/factory/test_create_provider_with_azure
  - functions/src/llm/provider/factory/test_disabled_azure_falls_through_to_anthropic
  - functions/src/llm/provider/factory/test_create_provider_with_gemini
  - functions/src/llm/provider/factory/test_create_provider_with_gemini_custom_base_url_and_model
  - functions/src/llm/provider/factory/gemini_without_api_key_falls_through_to_anthropic
  - functions/src/llm/provider/factory/disabled_gemini_is_skipped_in_favour_of_the_next_provider
  - functions/src/llm/provider/factory/disabled_openai_is_skipped_in_favour_of_the_next_provider
  - functions/src/llm/provider/factory/disabled_qwen_is_skipped_in_favour_of_the_next_provider
  - functions/src/llm/provider/factory/disabled_anthropic_fallback_fails_with_a_clear_message
  - functions/src/llm/provider/factory/test_create_provider_with_qwen
  - functions/src/llm/provider/factory/test_create_provider_no_credentials
---

# Signature

`pub fn create_provider(config: &Config) -> Result<Arc<dyn Provider>>`

# Calls

- [try_create_qwen](../../../../../functions/src/llm/provider/factory/try_create_qwen.md)
- [try_create_openai](../../../../../functions/src/llm/provider/factory/try_create_openai.md)
- [try_create_gemini](../../../../../functions/src/llm/provider/factory/try_create_gemini.md)
- [try_create_azure](../../../../../functions/src/llm/provider/factory/try_create_azure.md)
- [create_anthropic](../../../../../functions/src/llm/provider/factory/create_anthropic.md)

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../../functions/src/cli/cmd_run.md)
- [test_create_provider_with_anthropic](../../../../../functions/src/llm/provider/factory/test_create_provider_with_anthropic.md)
- [test_create_provider_with_openai](../../../../../functions/src/llm/provider/factory/test_create_provider_with_openai.md)
- [test_create_provider_with_azure](../../../../../functions/src/llm/provider/factory/test_create_provider_with_azure.md)
- [test_disabled_azure_falls_through_to_anthropic](../../../../../functions/src/llm/provider/factory/test_disabled_azure_falls_through_to_anthropic.md)
- [test_create_provider_with_gemini](../../../../../functions/src/llm/provider/factory/test_create_provider_with_gemini.md)
- [test_create_provider_with_gemini_custom_base_url_and_model](../../../../../functions/src/llm/provider/factory/test_create_provider_with_gemini_custom_base_url_and_model.md)
- [gemini_without_api_key_falls_through_to_anthropic](../../../../../functions/src/llm/provider/factory/gemini_without_api_key_falls_through_to_anthropic.md)
- [disabled_gemini_is_skipped_in_favour_of_the_next_provider](../../../../../functions/src/llm/provider/factory/disabled_gemini_is_skipped_in_favour_of_the_next_provider.md)
- [disabled_openai_is_skipped_in_favour_of_the_next_provider](../../../../../functions/src/llm/provider/factory/disabled_openai_is_skipped_in_favour_of_the_next_provider.md)
- [disabled_qwen_is_skipped_in_favour_of_the_next_provider](../../../../../functions/src/llm/provider/factory/disabled_qwen_is_skipped_in_favour_of_the_next_provider.md)
- [disabled_anthropic_fallback_fails_with_a_clear_message](../../../../../functions/src/llm/provider/factory/disabled_anthropic_fallback_fails_with_a_clear_message.md)
- [test_create_provider_with_qwen](../../../../../functions/src/llm/provider/factory/test_create_provider_with_qwen.md)
- [test_create_provider_no_credentials](../../../../../functions/src/llm/provider/factory/test_create_provider_no_credentials.md)