# Crabrace Integration in Crustly

This document describes how Crustly integrates with Crabrace (the Rust port of Catwalk) for automatic AI provider discovery and management.

## Overview

**Crabrace** replaces the originally planned Catwalk integration. It provides:

- **Automatic Provider Discovery** - Fetch available AI providers dynamically
- **Model Information** - Get up-to-date model capabilities, pricing, and limits
- **Local & Cloud Support** - Works with both cloud APIs and local LLMs (Ollama, LM Studio)
- **High Performance** - Built in Rust with <15ms response times
- **Self-Hosted** - Run your own registry server for privacy and control

## Architecture

```
┌─────────────────────────────────────┐
│     Crustly Application             │
│                                     │
│  ┌──────────────────────────────┐  │
│  │   config.json                 │  │
│  │   - API Keys                  │  │
│  │   - Crabrace Settings         │  │
│  └──────────────────────────────┘  │
│              ↓                      │
│  ┌──────────────────────────────┐  │
│  │   CrabraceIntegration         │  │
│  │   (Auto-Update)              │  │
│  └──────────────────────────────┘  │
│              ↓                      │
└──────────────┼──────────────────────┘
               │ HTTP
               ↓
┌──────────────────────────────────────┐
│   Crabrace Server                    │
│   (Provider Registry)                │
│                                      │
│  GET /providers → All providers      │
│  GET /health    → Health check       │
│  GET /metrics   → Prometheus metrics │
└──────────────────────────────────────┘
```

## Configuration

### 1. Start Crabrace Server

**Option A: Docker (Recommended)**
```bash
docker run -d \
  --name crabrace \
  -p 8080:8080 \
  ghcr.io/jyjeanne/crabrace:v0.1.0
```

**Option B: From Source**
```bash
cd ../crabrace
cargo run --release
```

**Option C: Docker Compose**
```bash
cd ../crabrace
docker-compose up -d
```

### 2. Configure Crustly

Edit `~/.config/crustly/config.json`:

```json
{
  "crabrace": {
    "enabled": true,
    "base_url": "http://localhost:8080",
    "auto_update": true,
    "update_interval_seconds": 3600
  }
}
```

**Configuration Options:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | bool | `true` | Enable Crabrace integration |
| `base_url` | string | `"http://localhost:8080"` | Crabrace server URL |
| `auto_update` | bool | `true` | Auto-update providers on startup |
| `update_interval_seconds` | u64 | `3600` | Update interval (0 = only on startup) |

### 3. Verify Integration

```bash
# Check Crabrace is running
curl http://localhost:8080/health

# List available providers
curl http://localhost:8080/providers | jq .

# Start Crustly (will auto-connect to Crabrace)
crustly
```

## Usage in Code

### Fetching Providers

```rust
use crustly::config::{CrabraceConfig, CrabraceIntegration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create Crabrace integration
    let config = CrabraceConfig::default();
    let crabrace = CrabraceIntegration::new(config)?;

    // Health check
    if crabrace.health_check().await? {
        println!("✅ Crabrace server is healthy");
    }

    // Fetch all providers
    let providers = crabrace.fetch_providers().await?;
    println!("Found {} providers", providers.len());

    for provider in providers {
        println!("  - {} ({} models)", provider.name, provider.models.len());
    }

    Ok(())
}
```

### Getting Specific Provider

```rust
// Get Anthropic provider
if let Some(provider) = crabrace.get_provider("anthropic").await? {
    println!("Provider: {}", provider.name);

    for model in &provider.models {
        println!("  Model: {} ({})", model.name, model.id);
        println!("    Context: {} tokens", model.context_window);
        println!("    Cost: ${:.2}/1M in, ${:.2}/1M out",
                 model.cost_per_1m_in, model.cost_per_1m_out);
    }
}
```

### Checking Provider Availability

```rust
// Check if Ollama is available
if crabrace.is_provider_available("ollama").await? {
    println!("✅ Ollama provider is available");
} else {
    println!("❌ Ollama not found in registry");
}
```

## Benefits Over Manual Configuration

### Without Crabrace (Manual)
```json
{
  "providers": {
    "anthropic": {
      "models": ["claude-3-5-sonnet-20241022"],  // Outdated!
      "endpoint": "https://api.anthropic.com"
    }
  }
}
```

**Problems:**
- ❌ Models become outdated
- ❌ New providers require code changes
- ❌ No cost information
- ❌ Manual maintenance required

### With Crabrace (Automatic)
```json
{
  "crabrace": {
    "enabled": true,
    "base_url": "http://localhost:8080"
  }
}
```

**Benefits:**
- ✅ Always up-to-date models
- ✅ Automatic new provider discovery
- ✅ Cost and capability metadata
- ✅ Zero maintenance
- ✅ 18 providers, 354+ models

## Supported Providers (via Crabrace)

### Cloud Providers (16)
- Anthropic (Claude)
- OpenAI (GPT)
- Google Gemini
- Azure OpenAI
- AWS Bedrock
- VertexAI
- xAI (Grok)
- Zhipu AI (zAI)
- GROQ
- OpenRouter (206+ models)
- Cerebras
- Venice
- Chutes
- DeepSeek
- HuggingFace
- AIHubMix

### Local Providers (2)
- **Ollama** - Run models locally (Llama, Mistral, Phi, etc.)
- **LM Studio** - Desktop app for local LLM inference

## Deployment Scenarios

### Development (Local)
```bash
# Terminal 1: Start Crabrace
cd ../crabrace
cargo run --release

# Terminal 2: Start Crustly
crustly
```

### Production (Docker)
```yaml
# docker-compose.yml
version: '3.8'
services:
  crabrace:
    image: ghcr.io/jyjeanne/crabrace:v0.1.0
    ports:
      - "8080:8080"
    restart: always

  crustly:
    image: crustly:latest
    depends_on:
      - crabrace
    environment:
      - CRUSTLY_CRABRACE_BASE_URL=http://crabrace:8080
```

### Kubernetes
```yaml
apiVersion: v1
kind: Service
metadata:
  name: crabrace
spec:
  selector:
    app: crabrace
  ports:
    - port: 8080
---
# Crustly deployment can reference: http://crabrace:8080
```

## Troubleshooting

### Crabrace Connection Failed

```bash
# Check if Crabrace is running
curl http://localhost:8080/health

# Expected: 200 OK
```

**Solution:**
1. Start Crabrace server: `cd ../crabrace && cargo run --release`
2. Or disable Crabrace: Set `"enabled": false` in config

### No Providers Found

```bash
# Check providers endpoint
curl http://localhost:8080/providers | jq 'length'

# Expected: 18
```

**Solution:**
1. Verify Crabrace server is v0.1.0 or later
2. Check server logs for errors
3. Restart Crabrace server

### Provider Not Available

```rust
// List all available providers
let providers = crabrace.fetch_providers().await?;
for provider in providers {
    println!("{}", provider.id);
}
```

**Common Issues:**
- **Ollama not found**: Install Ollama and run `ollama serve`
- **LM Studio not found**: Start LM Studio local server
- **Cloud provider missing**: Check Crabrace version (should be v0.1.0+)

## Performance

Crabrace is designed for high performance:

- **25,000+ req/s** throughput
- **<15ms P99 latency** for provider queries
- **~6MB memory** usage (idle)
- **~50ms startup** time

This means Crustly can query provider information with negligible overhead.

## Migration from Catwalk

If you were planning to use Catwalk, Crabrace is a drop-in replacement:

```diff
- base_url: "http://catwalk-server:8080"
+ base_url: "http://crabrace-server:8080"
```

**Advantages over Catwalk:**
- 2.5x higher throughput
- 2x faster response times
- Written in Rust (memory-safe)
- Includes local LLM support (Ollama, LM Studio)

## Further Reading

- [Crabrace README](../crabrace/README.md)
- [Crabrace API Documentation](../crabrace/docs/CRABRACE_SPECIFICATION.md)
- [Performance Benchmarks](../crabrace/BENCHMARK_RESULTS.md)
- [Docker Deployment Guide](../crabrace/DOCKER_DEPLOYMENT.md)
- [Kubernetes Guide](../crabrace/KUBERNETES.md)
