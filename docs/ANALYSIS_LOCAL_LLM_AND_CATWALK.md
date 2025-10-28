# Analysis: Local LLM Support & Catwalk Integration for Crustly

**Date:** October 26, 2025
**Version:** 1.0
**Status:** Analysis Complete

---

## Executive Summary

After analyzing the complete specification files in the `crusty/docs/` directory, here are the findings:

### 🔍 Key Findings:

1. **❌ Local LLM Support (Ollama & LM Studio) - NOT in Specification**
   - Ollama is **NOT mentioned** anywhere in the specification
   - LM Studio is **NOT mentioned** anywhere in the specification
   - This is a **NEW REQUIREMENT** not part of the original spec

2. **✅ Catwalk Integration - REQUIRED in Specification**
   - Catwalk is mentioned as **Priority 1 (CRITICAL)** enhancement
   - Already documented with implementation details
   - Required for provider auto-update functionality

---

## 1. Current LLM Provider Support in Specification

### Providers Specified (6 total):

| Provider | File Location | Status in Spec |
|----------|---------------|----------------|
| **Anthropic/Claude** | `src/llm/provider/anthropic.rs` | ✅ Required |
| **OpenAI** | `src/llm/provider/openai.rs` | ✅ Required |
| **Google Gemini** | `src/llm/provider/gemini.rs` | ✅ Required |
| **AWS Bedrock** | `src/llm/provider/bedrock.rs` | ✅ Required |
| **Azure OpenAI** | `src/llm/provider/azure.rs` | ✅ Required |
| **VertexAI** | `src/llm/provider/vertexai.rs` | ✅ Required |

### Dependencies Listed in Cargo.toml:

```toml
# HTTP & LLM Clients
reqwest = { version = "0.11", features = ["json", "rustls-tls", "stream"] }
async-openai = "0.20"
aws-sdk-bedrockruntime = "1.15"
```

**⚠️ FINDING:** No dependencies for local LLM inference (no Ollama client, no LM Studio client)

---

## 2. Local LLM Support Analysis (Ollama & LM Studio)

### 2.1 Ollama Support - NOT in Specification

**What is Ollama?**
- Open-source local LLM runtime
- Allows running Llama, Mistral, CodeLlama, and other models locally
- REST API compatible with OpenAI API format
- Benefits: Privacy, offline operation, no API costs

**Required Implementation:**

```rust
// src/llm/provider/ollama.rs - NEW FILE NEEDED
pub struct OllamaProvider {
    base_url: String,          // Default: http://localhost:11434
    http_client: reqwest::Client,
    model: String,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String) -> Self {
        // Initialize Ollama client
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        // GET /api/tags
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        // POST /api/generate
    }

    pub async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        // POST /api/chat
    }
}

impl LLMProvider for OllamaProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        // Implement using Ollama API
    }

    async fn stream(&self, request: CompletionRequest) -> Result<impl Stream<Item = String>> {
        // Implement streaming
    }
}
```

**Configuration Example:**

```json
{
  "providers": {
    "ollama": {
      "base_url": "http://localhost:11434",
      "models": ["llama2", "mistral", "codellama"]
    }
  }
}
```

**Dependencies to Add:**

```toml
# No additional dependencies needed - uses standard reqwest
# Ollama API is REST-based and compatible with existing HTTP client
```

### 2.2 LM Studio Support - NOT in Specification

**What is LM Studio?**
- Desktop application for running local LLMs
- Provides OpenAI-compatible REST API
- Default port: 1234
- Easy model management with GUI

**Required Implementation:**

```rust
// src/llm/provider/lm_studio.rs - NEW FILE NEEDED
pub struct LMStudioProvider {
    base_url: String,          // Default: http://localhost:1234
    http_client: reqwest::Client,
    model: String,
}

impl LMStudioProvider {
    pub fn new(base_url: String, model: String) -> Self {
        // Initialize LM Studio client
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        // GET /v1/models (OpenAI compatible)
    }
}

impl LLMProvider for LMStudioProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        // Use OpenAI-compatible API
        // POST /v1/completions
    }

    async fn stream(&self, request: CompletionRequest) -> Result<impl Stream<Item = String>> {
        // POST /v1/chat/completions with stream=true
    }
}
```

**Configuration Example:**

```json
{
  "providers": {
    "lm-studio": {
      "base_url": "http://localhost:1234",
      "model": "local-model"
    }
  }
}
```

**Implementation Note:**
- LM Studio API is OpenAI-compatible
- Could potentially reuse OpenAI provider code with different base URL
- Minimal additional code needed

---

## 3. Catwalk Integration - REQUIRED by Specification

### 3.1 What is Catwalk?

**Catwalk** is a **community model registry** that provides:

1. **Centralized Provider Metadata**
   - Up-to-date list of LLM providers
   - Model information and capabilities
   - API endpoint information
   - Pricing and rate limit data

2. **Auto-Update Functionality**
   - Automatically fetch new provider configurations
   - Keep model lists up-to-date
   - Discover new LLM providers as they become available

3. **Community-Driven**
   - Community maintains provider definitions
   - Reduces need for manual configuration updates
   - Ensures compatibility with latest provider APIs

### 3.2 Why Catwalk is Needed

**Problem Without Catwalk:**
- Users must manually update provider configurations
- New LLM providers require code changes
- Model lists become outdated
- Provider API changes break compatibility

**Benefits With Catwalk:**
```
┌─────────────────────────────────────┐
│     Crustly Application             │
│                                     │
│  ┌──────────────────────────────┐  │
│  │   Local Config File           │  │
│  │   (crustly.json)             │  │
│  │   - API Keys                  │  │
│  │   - User Preferences         │  │
│  └──────────────────────────────┘  │
│              ↓                      │
│  ┌──────────────────────────────┐  │
│  │   Catwalk Client              │  │
│  │   (Auto-Update)              │  │
│  └──────────────────────────────┘  │
│              ↓                      │
└──────────────┼──────────────────────┘
               │
               ↓ HTTP Request
┌──────────────────────────────────────┐
│   Catwalk API Server                 │
│   (Community Registry)               │
│                                      │
│  /providers → List all providers     │
│  /providers/{id}/models → Models     │
│  /providers/{id}/config → Settings   │
└──────────────────────────────────────┘
```

### 3.3 Specification Details for Catwalk

**From CRUSTY_SPECIFICATION_FINAL.md (Lines 628-681):**

#### Architecture Integration:

```
Integration Layer:
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│ LLM      │  │ LSP      │  │ MCP      │  │ Catwalk  │
│Providers │  │ Client   │  │ Client   │  │ Client ✅│
└──────────┘  └──────────┘  └──────────┘  └──────────┘
```

#### Implementation Files Required:

1. **`src/config/catwalk.rs`** - Catwalk client integration
2. **`src/config/update.rs`** - Provider auto-update logic
3. **`src/cli/update_providers.rs`** - CLI command for manual updates

#### Core Implementation:

```rust
// src/config/catwalk.rs
pub struct CatwalkClient {
    http_client: reqwest::Client,
    base_url: String,  // e.g., "https://api.catwalk.dev"
}

impl CatwalkClient {
    pub async fn fetch_providers(&self) -> Result<Vec<ProviderConfig>> {
        let response = self.http_client
            .get(&format!("{}/providers", self.base_url))
            .send()
            .await?;

        let providers: Vec<ProviderConfig> = response.json().await?;
        Ok(providers)
    }

    pub async fn fetch_models(&self, provider: &str) -> Result<Vec<ModelInfo>> {
        let response = self.http_client
            .get(&format!("{}/providers/{}/models", self.base_url, provider))
            .send()
            .await?;

        let models: Vec<ModelInfo> = response.json().await?;
        Ok(models)
    }
}
```

```rust
// src/config/update.rs
pub async fn update_providers_from_catwalk(
    config: &mut Config,
) -> Result<UpdateSummary> {
    let catwalk = config.catwalk.as_ref()
        .ok_or_else(|| anyhow!("Catwalk client not initialized"))?;

    let providers = catwalk.fetch_providers().await?;

    let mut summary = UpdateSummary {
        added: Vec::new(),
        updated: Vec::new(),
        removed: Vec::new(),
    };

    for provider in providers {
        if !config.providers.contains_key(&provider.id) {
            summary.added.push(provider.name.clone());
        } else {
            summary.updated.push(provider.name.clone());
        }
        config.providers.insert(provider.id.clone(), provider);
    }

    Ok(summary)
}
```

#### Configuration Integration:

```rust
// src/config/mod.rs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub providers: HashMap<String, ProviderConfig>,
    pub models: HashMap<String, SelectedModel>,
    pub lsp: HashMap<String, LspConfig>,
    pub mcp: HashMap<String, McpConfig>,
    pub agent: AgentConfig,
    pub options: Options,
    pub permissions: Permissions,
    #[serde(skip)]
    pub catwalk: Option<CatwalkClient>, // ✅ NEW
}
```

```json
{
  "catwalk": {
    "enabled": true,
    "auto_update": true,
    "update_interval": 86400,
    "base_url": "https://api.catwalk.dev"
  }
}
```

### 3.4 Catwalk Priority & Timeline

**Priority:** CRITICAL (Priority 1, #4 in specification)

**Sprint:** Sprint 2 (Configuration System)
- Day 4: Implement Catwalk integration
- Day 4: Add provider auto-update
- Estimated effort: 3 days

**Files to Create:**
1. `src/config/catwalk.rs` (NEW)
2. `src/config/update.rs` (NEW)
3. `src/cli/update_providers.rs` (NEW)

---

## 4. Comparison: What's Missing vs What's Required

### Required by Spec (Documented):

| Feature | Priority | Sprint | Status |
|---------|----------|--------|--------|
| Anthropic Provider | Critical | 5 | ✅ Specified |
| OpenAI Provider | Critical | 5 | ✅ Specified |
| Gemini Provider | Critical | 5 | ✅ Specified |
| Bedrock Provider | Critical | 5 | ✅ Specified |
| Azure Provider | Critical | 5 | ✅ Specified |
| VertexAI Provider | Critical | 5 | ✅ Specified |
| **Catwalk Integration** | **Critical** | **2** | **✅ Specified** |

### NOT in Spec (User Requirements):

| Feature | Priority | Sprint | Status |
|---------|----------|--------|--------|
| Ollama Provider | ❓ New | ❓ TBD | ❌ NOT in spec |
| LM Studio Provider | ❓ New | ❓ TBD | ❌ NOT in spec |

---

## 5. Recommendations

### 5.1 Immediate Actions (Follow Specification):

1. **Implement Catwalk Integration (REQUIRED)**
   - Priority: CRITICAL
   - Timeline: Sprint 2
   - Files: 3 new files as specified
   - This is REQUIRED by the specification

### 5.2 Optional Enhancements (User Request):

2. **Add Ollama Support (OPTIONAL)**
   - Priority: Nice-to-have
   - Timeline: After Sprint 5 (after other providers)
   - Implementation effort: ~2 days
   - Files: 1 new file (`src/llm/provider/ollama.rs`)
   - Benefits: Local inference, privacy, offline use

3. **Add LM Studio Support (OPTIONAL)**
   - Priority: Nice-to-have
   - Timeline: After Sprint 5
   - Implementation effort: ~1 day (OpenAI-compatible)
   - Files: 1 new file or reuse OpenAI provider
   - Benefits: Easy local deployment

### 5.3 Proposed Implementation Order:

```
Phase 1 (Follow Spec): ✅ REQUIRED
├── Sprint 2: Catwalk Integration (3 days)
└── Sprint 5: Core 6 Providers (7 days)
    ├── Anthropic
    ├── OpenAI
    ├── Gemini
    ├── Bedrock
    ├── Azure
    └── VertexAI

Phase 2 (Extend Spec): ⭕ OPTIONAL
└── Sprint 5.5: Local LLM Support (3 days)
    ├── Ollama Provider (2 days)
    └── LM Studio Provider (1 day)
```

---

## 6. Implementation Guide for Local LLMs

### 6.1 Adding Ollama Provider

**Step 1:** Create provider file
```bash
touch src/llm/provider/ollama.rs
```

**Step 2:** Implement LLMProvider trait
```rust
// src/llm/provider/ollama.rs
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OllamaProvider {
    base_url: String,
    client: Client,
    model: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
            model,
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let ollama_req = OllamaRequest {
            model: self.model.clone(),
            prompt: request.prompt,
            stream: false,
        };

        let response = self.client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&ollama_req)
            .send()
            .await?;

        let ollama_resp: OllamaResponse = response.json().await?;

        Ok(CompletionResponse {
            text: ollama_resp.response,
            tokens_used: 0, // Ollama doesn't provide token count
            finish_reason: "complete".to_string(),
        })
    }

    async fn stream(
        &self,
        request: CompletionRequest
    ) -> Result<impl Stream<Item = Result<String>>> {
        let ollama_req = OllamaRequest {
            model: self.model.clone(),
            prompt: request.prompt,
            stream: true,
        };

        let response = self.client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&ollama_req)
            .send()
            .await?;

        // Parse streaming NDJSON response
        // Implementation details...
    }
}
```

**Step 3:** Register in provider factory
```rust
// src/llm/provider/factory.rs
pub fn create_provider(config: &ProviderConfig) -> Result<Box<dyn LLMProvider>> {
    match config.provider_type.as_str() {
        "anthropic" => Ok(Box::new(AnthropicProvider::new(config))),
        "openai" => Ok(Box::new(OpenAIProvider::new(config))),
        "ollama" => Ok(Box::new(OllamaProvider::new(
            config.base_url.clone().unwrap_or_else(|| "http://localhost:11434".to_string()),
            config.model.clone(),
        ))),
        // ... other providers
    }
}
```

**Step 4:** Add configuration support
```rust
// Example configuration
{
  "providers": {
    "ollama": {
      "type": "ollama",
      "base_url": "http://localhost:11434",
      "model": "llama2"
    }
  }
}
```

### 6.2 Adding LM Studio Provider

**Option A:** Reuse OpenAI provider
```rust
// LM Studio is OpenAI-compatible
// Just use OpenAI provider with different base URL
{
  "providers": {
    "lm-studio": {
      "type": "openai",
      "base_url": "http://localhost:1234/v1",
      "api_key": "not-needed",
      "model": "local-model"
    }
  }
}
```

**Option B:** Dedicated provider (if custom features needed)
```rust
// src/llm/provider/lm_studio.rs
// Similar to OpenAI but with LM Studio specifics
```

---

## 7. Catwalk API Specification (Hypothetical)

Since Catwalk is mentioned in the spec but no actual API exists, here's what it likely should provide:

### API Endpoints:

```
GET /api/v1/providers
Response: List of all available providers
[
  {
    "id": "anthropic",
    "name": "Anthropic",
    "type": "cloud",
    "api_base": "https://api.anthropic.com",
    "auth_type": "api_key",
    "models": ["claude-3-5-sonnet-20241022", "claude-3-opus", ...]
  },
  {
    "id": "ollama",
    "name": "Ollama",
    "type": "local",
    "api_base": "http://localhost:11434",
    "auth_type": "none",
    "models": ["llama2", "mistral", "codellama"]
  }
]

GET /api/v1/providers/{id}
Response: Detailed provider information

GET /api/v1/providers/{id}/models
Response: Available models for provider

GET /api/v1/providers/{id}/config
Response: Configuration template for provider
```

---

## 8. Action Items Summary

### Must Do (Following Specification):

- [ ] **Implement Catwalk Client** (`src/config/catwalk.rs`)
- [ ] **Implement Provider Auto-Update** (`src/config/update.rs`)
- [ ] **Add CLI Update Command** (`src/cli/update_providers.rs`)
- [ ] **Implement 6 Core Providers**:
  - [ ] Anthropic
  - [ ] OpenAI
  - [ ] Gemini
  - [ ] Bedrock
  - [ ] Azure
  - [ ] VertexAI

### Optional (User Enhancement):

- [ ] **Add Ollama Provider** (`src/llm/provider/ollama.rs`)
- [ ] **Add LM Studio Provider** (`src/llm/provider/lm_studio.rs`)
- [ ] **Update Documentation** for local LLM usage
- [ ] **Add Configuration Examples** for local LLMs

---

## 9. Conclusion

### Key Takeaways:

1. **Catwalk IS Required** - It's a critical (Priority 1) feature in the specification for provider auto-updates

2. **Local LLMs NOT in Spec** - Ollama and LM Studio are not mentioned in the original specification but are valuable additions

3. **Easy to Add** - Local LLM support is straightforward to implement using existing HTTP client infrastructure

4. **Recommendation**:
   - First, implement Catwalk as specified (REQUIRED)
   - Then, add local LLM support as an enhancement (OPTIONAL but valuable)

### Next Steps:

1. Review this analysis
2. Decide on implementation priority
3. Begin with Catwalk integration (follows spec)
4. Optionally add Ollama/LM Studio support after core providers

---

**Analysis completed by:** Claude (Crustly Development Assistant)
**Files analyzed:**
- `docs/CRUSTY_SPECIFICATION_FINAL.md`
- `docs/SPECIFICATION_REVIEW.md`
- `docs/IMPLEMENTATION_SUMMARY.md`
