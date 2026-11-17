# crustly-llm

The LLM provider layer of [Crustly](https://github.com/jyjeanne/crustly), as a crate other projects can use.

One `Provider` trait over:

- **Anthropic** (Claude)
- **OpenAI** and OpenAI-compatible servers: LM Studio, Ollama's `/v1` endpoint, LocalAI
- **Ollama**, native API (feature `ollama`)
- **Gemini**, **Azure OpenAI**, **Qwen** (local or DashScope)
- **llama.cpp** in-process, loading a `.gguf` file directly (feature `llama-cpp`)

Every provider supports token-by-token streaming (`Provider::stream`) and tool calls. Tool calls that a local model prints as JSON instead of returning natively are recovered (`tool_call_recovery`).

## Features

| Feature | Adds | Build needs |
|---|---|---|
| `ollama` | Native Ollama provider and model management (list, pull, delete) | Nothing extra |
| `gguf-management` | Local `.gguf` files: listing, resumable downloads with SHA-256, hardware detection | Nothing extra |
| `llama-cpp` | In-process llama.cpp with grammar-constrained tool calls | CMake and a C/C++ toolchain |
| `llama-cpp-cuda`, `-metal`, `-vulkan`, `-rocm`, `-opencl`, `-mkl` | GPU offload for llama.cpp | The matching SDK |

No feature is enabled by default.

## Usage

```toml
[dependencies]
crustly-llm = { git = "https://github.com/jyjeanne/crustly", tag = "crustly-llm-v0.1.0", features = ["ollama"] }
```

```rust,ignore
use crustly_llm::{LLMRequest, Message, OllamaProvider, Provider, StreamEvent, ContentDelta};
use futures::StreamExt;

let provider = OllamaProvider::new("http://127.0.0.1:11434").with_default_model("qwen3.5:4b");
let request = LLMRequest::new("qwen3.5:4b", vec![Message::user("Hello!")]);

let mut stream = provider.stream(request).await?;
while let Some(event) = stream.next().await {
    if let StreamEvent::ContentBlockDelta { delta: ContentDelta::TextDelta { text }, .. } = event? {
        print!("{text}");
    }
}
```

## Licence

FSL-1.1-MIT, like Crustly.
