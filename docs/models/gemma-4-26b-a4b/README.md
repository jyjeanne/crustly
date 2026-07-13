# Gemma 4 26B A4B MoE

> Complete technical documentation for integrating Gemma 4 26B A4B with Ollama and Crustly.

---

# Overview

This documentation provides a complete developer reference for using **Gemma 4 26B A4B MoE** locally through **Ollama**.

The goal is to provide enough technical information to build an AI coding agent capable of:

- Repository analysis
- Code generation
- Refactoring
- Bug fixing
- Tool Calling
- Long context reasoning
- Multi-step planning
- Agentic software engineering

This documentation targets:

- Crustly
- Ollama
- OpenAI Compatible APIs
- Continue
- Roo Code
- OpenHands
- Aider
- Custom AI Agents

---

# Documentation Structure

| File | Description |
|---------|------------|
| 01-introduction.md | General presentation |
| 02-architecture.md | Internal architecture |
| 03-mixture-of-experts.md | Complete MoE explanation |
| 04-tokenizer.md | Tokenization |
| 05-chat-template.md | Conversation format |
| 06-system-prompts.md | System prompts |
| 07-thinking-mode.md | Reasoning mode |
| 08-tool-calling.md | Function Calling |
| 09-json-output.md | Structured outputs |
| 10-ollama-api.md | Ollama REST API |
| 11-openai-compatible-api.md | OpenAI compatibility |
| 12-streaming.md | Streaming protocol |
| 13-sampling.md | Sampling parameters |
| 14-context-management.md | Context engineering |
| 15-performance.md | Performance tuning |
| 16-vram-guide.md | GPU sizing |
| 17-modelfile.md | Ollama Modelfiles |
| 18-crustly-integration.md | Crustly integration |
| 19-best-practices.md | Coding workflows |
| 20-benchmarks.md | Evaluation |
| 21-troubleshooting.md | Troubleshooting |
| appendix.md | Reference |

> Only `01-introduction.md`, `02-architecture.md`, and `08-tool-calling.md` are written so far. The remaining chapters are planned and will be added incrementally; this table tracks the target structure.

---

# Model Summary

| Property | Value |
|------------|----------------|
| Name | Gemma 4 26B |
| Family | Gemma 4 |
| Architecture | Decoder Transformer |
| Type | Mixture of Experts |
| Parameters | 25.8B |
| Active Parameters | ~3.8B/token |
| Experts | 128 |
| Active Experts | 8 |
| Shared Expert | Yes |
| Context Window | 256k |
| Vocabulary | 262k |
| Attention | Sliding Window + Global |
| Quantization | GGUF |
| Ollama | Supported |
| Tool Calling | Supported |
| Vision | Supported |
| Thinking | Supported |

---

# Supported Tasks

## Programming

- Python
- JavaScript
- TypeScript
- Rust
- Go
- C#
- Java
- Kotlin
- PHP
- SQL
- Bash

---

## DevOps

- Docker
- Kubernetes
- Terraform
- GitHub Actions
- CI/CD

---

## Infrastructure

- Linux
- Nginx
- Apache
- HAProxy
- SSH
- Networking

---

## Data

- PostgreSQL
- MySQL
- SQLite
- MongoDB
- Redis

---

## Documentation

- Markdown
- Mermaid
- OpenAPI
- JSON Schema
- UML

---

# Why Gemma 4?

Gemma 4 introduces several major improvements over previous generations.

Compared to Gemma 3:

- Better reasoning
- Better instruction following
- Better coding
- Better long-context understanding
- Better multilingual capabilities
- Native Tool Calling
- Native Vision
- Better efficiency through Mixture of Experts

---

# Why MoE?

Instead of activating all parameters for every generated token, Gemma activates only a subset of experts.

Advantages:

- lower latency
- lower VRAM usage
- higher quality
- specialized experts

This makes Gemma particularly suitable for local inference.

---

# Recommended Hardware

## Minimum

- GPU: RTX 3060 12 GB
- RAM: 32 GB
- CPU: 8 Threads

## Recommended

- GPU: RTX 4070
- RAM: 64 GB
- CPU: Ryzen 9 / Intel i9

## Professional

- GPU: RTX 4090
- RAM: 128 GB

---

# Ollama Installation

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

Pull Gemma

```bash
ollama pull gemma4:26b
```

Run

```bash
ollama run gemma4:26b
```

List installed models

```bash
ollama list
```

---

# Basic API

Endpoint

```
POST http://localhost:11434/api/chat
```

Minimal request

```json
{
  "model": "gemma4:26b",
  "messages": [
    {
      "role": "user",
      "content": "Hello"
    }
  ]
}
```

---

# Recommended Parameters

| Parameter | Value |
|------------|----------|
| temperature | 0.2 |
| top_p | 0.9 |
| top_k | 20 |
| repeat_penalty | 1.05 |
| num_ctx | 65536 |

These values provide deterministic answers suitable for software development.

---

# Crustly Recommendations

Gemma 4 works particularly well for:

- Repository indexing
- Multi-file refactoring
- Test generation
- Bug fixing
- Architecture discussions
- Pull Request reviews
- Documentation generation

---

# Version Compatibility

| Component | Version |
|------------|----------|
| Ollama | Latest stable |
| GGUF | Supported |
| OpenAI API | Compatible |
| Tool Calling | Yes |
| Streaming | Yes |
| Vision | Yes |
| Thinking | Yes |

---

# Next Chapter

Continue with:

```
01-introduction.md
```

This chapter introduces Gemma 4 architecture and explains how Google designed the model before diving into implementation details.
