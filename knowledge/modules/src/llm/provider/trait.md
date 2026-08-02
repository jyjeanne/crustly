---
type: Rust Module
title: trait
resource: src/llm/provider/trait.rs#L1-L142
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result
  - external/super-types-llmrequest-llmresponse-streamevent
  - external/async-trait-async-trait
  - external/futures-stream
  - external/std-pin-pin
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [Provider](../../../../interfaces/src/llm/provider/trait/Provider.md)
- [supports_streaming](../../../../functions/src/llm/provider/trait/Provider/supports_streaming.md)
- [supports_tools](../../../../functions/src/llm/provider/trait/Provider/supports_tools.md)
- [supports_vision](../../../../functions/src/llm/provider/trait/Provider/supports_vision.md)
- [validate_model](../../../../functions/src/llm/provider/trait/Provider/validate_model.md)
- [ProviderCapabilities](../../../../classes/src/llm/provider/trait/ProviderCapabilities.md)
- [for_provider](../../../../functions/src/llm/provider/trait/ProviderCapabilities/for_provider.md)
- [MockProvider](../../../../classes/src/llm/provider/trait/MockProvider.md)
- [complete](../../../../functions/src/llm/provider/trait/MockProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/trait/MockProvider/provider/stream.md)
- [name](../../../../functions/src/llm/provider/trait/MockProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/trait/MockProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/trait/MockProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/provider/trait/MockProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/trait/MockProvider/provider/calculate_cost.md)
- [test_provider_validate_model](../../../../functions/src/llm/provider/trait/test_provider_validate_model.md)
- [test_provider_capabilities](../../../../functions/src/llm/provider/trait/test_provider_capabilities.md)

# Imports

- `super::error::Result`
- `super::types::{LLMRequest, LLMResponse, StreamEvent}`
- `async_trait::async_trait`
- `futures::Stream`
- `std::pin::Pin`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)