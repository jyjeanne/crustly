---
type: Rust Module
title: types
resource: src/llm/provider/types.rs#L1-L611
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/serde-deserialize-serialize
  - external/std-collections-hashmap
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [Role](../../../../classes/src/llm/provider/types/Role.md)
- [Message](../../../../classes/src/llm/provider/types/Message.md)
- [user](../../../../functions/src/llm/provider/types/Message/user.md)
- [assistant](../../../../functions/src/llm/provider/types/Message/assistant.md)
- [system](../../../../functions/src/llm/provider/types/Message/system.md)
- [ContentBlock](../../../../classes/src/llm/provider/types/ContentBlock.md)
- [ImageSource](../../../../classes/src/llm/provider/types/ImageSource.md)
- [ThinkingConfig](../../../../classes/src/llm/provider/types/ThinkingConfig.md)
- [LLMRequest](../../../../classes/src/llm/provider/types/LLMRequest.md)
- [new](../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_thinking](../../../../functions/src/llm/provider/types/LLMRequest/with_thinking.md)
- [with_system](../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_tools](../../../../functions/src/llm/provider/types/LLMRequest/with_tools.md)
- [with_temperature](../../../../functions/src/llm/provider/types/LLMRequest/with_temperature.md)
- [with_top_p](../../../../functions/src/llm/provider/types/LLMRequest/with_top_p.md)
- [with_seed](../../../../functions/src/llm/provider/types/LLMRequest/with_seed.md)
- [with_stop](../../../../functions/src/llm/provider/types/LLMRequest/with_stop.md)
- [with_frequency_penalty](../../../../functions/src/llm/provider/types/LLMRequest/with_frequency_penalty.md)
- [with_presence_penalty](../../../../functions/src/llm/provider/types/LLMRequest/with_presence_penalty.md)
- [with_response_format](../../../../functions/src/llm/provider/types/LLMRequest/with_response_format.md)
- [with_max_tokens](../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [with_streaming](../../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [Tool](../../../../classes/src/llm/provider/types/Tool.md)
- [CacheMetrics](../../../../classes/src/llm/provider/types/CacheMetrics.md)
- [hit_rate](../../../../functions/src/llm/provider/types/CacheMetrics/hit_rate.md)
- [LLMResponse](../../../../classes/src/llm/provider/types/LLMResponse.md)
- [PerfMetrics](../../../../classes/src/llm/provider/types/PerfMetrics.md)
- [tokens_per_second](../../../../functions/src/llm/provider/types/PerfMetrics/tokens_per_second.md)
- [StopReason](../../../../classes/src/llm/provider/types/StopReason.md)
- [TokenUsage](../../../../classes/src/llm/provider/types/TokenUsage.md)
- [total](../../../../functions/src/llm/provider/types/TokenUsage/total.md)
- [StreamEvent](../../../../classes/src/llm/provider/types/StreamEvent.md)
- [StreamMessage](../../../../classes/src/llm/provider/types/StreamMessage.md)
- [ContentDelta](../../../../classes/src/llm/provider/types/ContentDelta.md)
- [MessageDelta](../../../../classes/src/llm/provider/types/MessageDelta.md)
- [extract_think_tags](../../../../functions/src/llm/provider/types/extract_think_tags.md)
- [test_message_creation](../../../../functions/src/llm/provider/types/test_message_creation.md)
- [test_llm_request_builder](../../../../functions/src/llm/provider/types/test_llm_request_builder.md)
- [test_token_usage](../../../../functions/src/llm/provider/types/test_token_usage.md)
- [with_thinking_sets_temperature_and_config](../../../../functions/src/llm/provider/types/with_thinking_sets_temperature_and_config.md)
- [with_thinking_sets_temperature](../../../../functions/src/llm/provider/types/with_thinking_sets_temperature.md)
- [with_thinking_zero_budget_is_noop](../../../../functions/src/llm/provider/types/with_thinking_zero_budget_is_noop.md)
- [cache_metrics_hit_rate](../../../../functions/src/llm/provider/types/cache_metrics_hit_rate.md)
- [perf_metrics_tokens_per_second](../../../../functions/src/llm/provider/types/perf_metrics_tokens_per_second.md)
- [perf_metrics_tokens_per_second_missing_duration](../../../../functions/src/llm/provider/types/perf_metrics_tokens_per_second_missing_duration.md)
- [perf_metrics_tokens_per_second_zero_duration](../../../../functions/src/llm/provider/types/perf_metrics_tokens_per_second_zero_duration.md)
- [extract_think_tags_single_block](../../../../functions/src/llm/provider/types/extract_think_tags_single_block.md)
- [extract_think_tags_multiple_blocks](../../../../functions/src/llm/provider/types/extract_think_tags_multiple_blocks.md)
- [extract_think_tags_no_tags](../../../../functions/src/llm/provider/types/extract_think_tags_no_tags.md)
- [extract_think_tags_unclosed](../../../../functions/src/llm/provider/types/extract_think_tags_unclosed.md)
- [extract_think_tags_only_block](../../../../functions/src/llm/provider/types/extract_think_tags_only_block.md)

# Imports

- `serde::{Deserialize, Serialize}`
- `std::collections::HashMap`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)