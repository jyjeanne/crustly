# Crustly Performance Improvement Plan

**Status:** Proposed
**Focus:** Overall runtime performance, with emphasis on local LLM usage (Ollama and similar local runtimes)
**Companion reading:** `docs/ANALYSIS_LOCAL_LLM_AND_CATWALK.md` (historical, pre-implementation), `docs/architecture/decisions/0003-crabrace-provider-registry.md`, `docs/models/gemma-4-26b-a4b/` (model-specific tuning notes)

## Why local LLM performance gets special weight

Cloud providers hide their inference latency behind a network round-trip Crustly can't influence.
Ollama (and LM Studio-style local runtimes) are different: Crustly is talking to a process on the
same machine, sharing the same CPU/GPU/RAM as everything else the user is running. Every
inefficiency Crustly adds on top of local inference — buffering instead of streaming, re-tokenizing
history from scratch, blocking the async runtime — is fully attributable to Crustly, not the network.
Local models are also frequently the *slower* generator per-token (7B–32B general-purpose or
reasoning models on consumer GPUs/CPUs, running the same DeepSeek-R1/QwQ-style `<think>` blocks
called out in CLAUDE.md), which makes perceived-latency work (real streaming, incremental render)
proportionally more valuable there than for a fast cloud endpoint.

This plan is organized into four phases, ordered by risk/effort vs. impact. Each item lists the
concrete file(s) to change so it can be picked up independently.

---

## Phase 0 — Low-risk, high-confidence fixes (target: 1 sprint)

These are localized, don't change public APIs, and each is independently shippable.

| # | Fix | Files | Why it matters for local LLM |
|---|-----|-------|-------------------------------|
| 0.1 | Give `OllamaProvider` an explicit `reqwest`-level timeout and idle-pool tuning, matching `openai.rs`/`anthropic.rs`/`gemini.rs` (`DEFAULT_TIMEOUT`, `pool_idle_timeout`, `pool_max_idle_per_host`) | `src/llm/provider/ollama.rs:92-112` | Ollama is currently the *only* provider with no client-side timeout. A stalled/overloaded local daemon (e.g. loading a large model, or fighting the OS for VRAM) hangs the whole turn indefinitely instead of failing fast. |
| 0.2 | Classify Ollama connection errors as retryable so `retry::retry_with_backoff` and `FailoverProvider` can act on them, instead of mapping everything to `ProviderError::ApiError{status:0}` | `src/llm/provider/ollama.rs:13-25`, `src/llm/provider/retry.rs`, `src/llm/provider/factory.rs:36-43` | A single dropped local connection currently fails the whole turn with zero retry. Local daemons restart/reload models far more often than cloud APIs stay down. |
| 0.3 | Add `PRAGMA synchronous = NORMAL` alongside the existing `PRAGMA journal_mode = WAL` | `src/db/mod.rs:56-86` | WAL + `synchronous=NORMAL` is the documented safe pairing and avoids an `fsync` on every commit. Every assistant turn writes 4 times (see 1.3), so this is a direct per-turn latency win, independent of provider. |
| 0.4 | Move `std::fs::canonicalize` calls in `PermissionPolicy::evaluate()` into `tokio::task::spawn_blocking`, matching the pattern already used in `grep.rs`/`glob.rs` | `src/llm/tools/sandbox.rs:144,151,448,463` | Every path-checked tool call currently makes a blocking syscall directly on the async worker thread. Under a local-model session where many small tool calls interleave with generation, this steals scheduler time from the streaming/render loop. |
| 0.5 | Scope tool-cache invalidation to the mutated path instead of invalidating all `ReadFiles`-capable cache entries on any write | `src/llm/tools/cache.rs`, `src/llm/agent/service.rs:1310-1316` | Coarse invalidation is correct but wastes cache hits in tool-heavy sessions (agentic coding loops), which are exactly the sessions where local-model latency per tool round-trip matters most. |

**Verification:** existing unit tests plus a manual session against a local Ollama instance
(`cargo run -- run "..."` with a small model) checking that response time doesn't regress and that
a killed Ollama process now surfaces a bounded-time error instead of hanging.

---

## Phase 1 — Ollama-specific configuration and correctness (target: 1 sprint)

### 1.1 Expose the local-inference knobs that actually move throughput

`OllamaProviderConfig` (`src/config/mod.rs:443-476`) currently exposes `keep_alive`, `num_ctx`,
`temperature`, `top_p`, `top_k`. It does **not** expose `num_gpu` (GPU layer offload — the single
biggest lever for GPU-accelerated throughput), `num_thread` (CPU thread count), or `num_batch`
(prompt batch size), even though `to_ollama_request()` (`src/llm/provider/ollama.rs:219-243`)
already builds on `ollama-rs`'s `ModelOptions`, which supports all three.

**Action:** add `num_gpu: Option<u32>`, `num_thread: Option<u32>`, `num_batch: Option<u32>` to
`OllamaProviderConfig`, wire them through `with_num_gpu`/`with_num_thread`/`with_num_batch` builder
methods on `OllamaProvider`, and set them in `to_ollama_request()`. Document sensible starting
points in `config.toml` comments (e.g. `num_gpu` for full offload vs. partial, `num_thread` defaulting
to physical core count). Cross-reference the hardware-sizing notes already in
`docs/models/gemma-4-26b-a4b/` (VRAM/KV-cache/quantization tradeoffs) so the new config fields and
existing model docs point at each other.

### 1.2 Reconcile Crabrace's documented role with what it actually does

ADR 0003 and CLAUDE.md describe Crabrace as active provider/model discovery that "works with
local runtimes (Ollama, LM Studio)." In the current code, `create_provider()`
(`src/llm/provider/factory.rs`) never calls into `CrabraceIntegration`/`ProviderUpdater`
(`src/config/crabrace.rs`, `src/config/update.rs`) — nothing starts the auto-update loop, so
Crabrace has no effect on how Ollama is configured or selected today.

**Action (pick one, don't leave it ambiguous):**
- **Wire it up:** call `ProviderUpdater` at startup (or behind a config flag) so Ollama model
  metadata (available models, context windows) is discovered rather than hardcoded, OR
- **Document reality:** update CLAUDE.md/ADR 0003 to state Crabrace is a discovery mechanism that
  is implemented but not yet wired into the runtime path, so this doesn't mislead future
  contributors (including future planning work) into thinking it affects current behavior.

This plan recommends wiring it up for Ollama specifically first (model list + context-window
metadata), since that directly removes the `DEFAULT_NUM_CTX = 8_192` hardcoded guess
(`src/llm/provider/ollama.rs:61`) in favor of the model's real trained context length where
Crabrace/Ollama's `/api/show` can report it.

### 1.3 Tokenizer-accuracy check for compaction timing

`token_count()` (`src/llm/agent/context.rs:195-209`) uses `tiktoken_rs::cl100k_base()` (OpenAI's
BPE vocab) as an approximation for every provider, including Ollama. For Ollama, this is layered on
top of `DEFAULT_NUM_CTX = 8_192`, meaning compaction timing (the 80% threshold) is calibrated
against a tokenizer that doesn't match Llama/Qwen/Gemma/Mistral vocabularies running locally.

**Action:** short investigation spike, not a full fix — measure actual divergence (cl100k_base vs.
each local model family's real tokenizer, if a fast local tokenizer crate is available e.g. via
`tokenizers` from Hugging Face) on representative Crustly transcripts. If divergence exceeds ~10%,
consider a per-provider correction factor (cheap) before attempting a full pluggable-tokenizer
architecture (expensive). Document the finding either way so the 8,192 default and compaction
threshold are known-good or flagged.

---

## Phase 2 — Streaming and context reuse (target: 2 sprints, larger surface area)

These touch `LLMRequest`/`AgentContext`, both flagged as top-10 "god nodes" in
`docs/graph/GRAPH_REPORT.md`, so budget extra review time and add tests before refactoring.

### 2.1 Stop deep-cloning the full message history on every tool-loop iteration

`send_message_with_tools_inner` rebuilds `LLMRequest::new(model_name.clone(), context.messages.clone())`
on **every iteration** of the tool loop (`src/llm/agent/service.rs:743, 1049, 1493`), not once per
user turn — up to `max_tool_iterations` (default 10) full-history clones per turn, including full
text of every prior tool result (e.g. whole file contents from earlier `read_file` calls).

**Action:** change `LLMRequest` to hold `Arc<[Message]>` (or `Arc<Vec<Message>>`) instead of an
owned `Vec<Message>`, so appending one new message per tool-loop iteration is an `Arc::clone` +
small append, not a full deep clone. This is the single highest-leverage fix in this plan for
agentic/tool-heavy sessions against any provider, and disproportionately helps local models where
each tool-loop round-trip already costs more wall-clock time.

### 2.2 Real incremental streaming for Ollama instead of buffer-then-replay

`OllamaProvider::stream()` currently drains Ollama's network stream fully into a `Vec<StreamEvent>`
before wrapping it in `futures::stream::iter` (`src/llm/provider/ollama.rs:420, 570-571`). The first
event isn't emitted to `drain_stream_to_response` until the *entire* Ollama response has arrived —
functionally, Crustly buffers the whole turn and replays it as a synthetic completed stream, rather
than truly streaming.

**Action:** restructure `OllamaProvider::stream()` to forward each `StreamEvent` to the caller as
soon as it's derived from the corresponding Ollama chunk (e.g. via an `async_stream`/channel-backed
`Stream` instead of building a `Vec` first), matching the architecture the module doc implies. Keep
the existing tool-call-recovery buffering (`ollama.rs:454-503, 660-663`) for the specific case where
text might be a JSON tool call — that's a deliberate correctness tradeoff — but everything else
should flow through immediately. This is the most visible perceived-latency win for local models,
which are exactly the case where users watch tokens arrive in real time.

### 2.3 Avoid re-parsing the entire response as markdown on every redraw

`render_streaming_response` (`src/tui/render.rs:372-386`) calls `parse_markdown()` on the full
accumulated response string on every terminal redraw, giving roughly O(n²) total parsing work over
the life of one streamed response (n = final length). This is worse for local reasoning models that
emit large `<think>` blocks and longer outputs.

**Action:** cache the parsed markdown AST/rendered lines and only re-parse the delta (or the last
incomplete block) on each redraw, falling back to a full re-parse only when a block boundary changes
retroactively (e.g. a fenced code block that was ambiguous mid-stream). If a fully incremental
parser is too invasive, a cheaper first step is to throttle redraw-triggered re-parses (e.g. re-parse
at most every N chunks or M milliseconds) rather than on every batch drained by the runner loop
(`src/tui/runner.rs:101-140`).

### 2.4 Stop reloading and re-tokenizing full session history every turn

`AgentContext::from_db_messages` is called fresh on every `send_message*` call
(`src/llm/agent/service.rs:657-658, 1465-1466`), re-running `list_messages_for_session` (full table
scan by session) and full BPE re-tokenization of the entire history, even though nothing about
already-processed history changed since the last turn.

**Action:** keep `AgentContext` (or its token-count bookkeeping) alive on `AgentService` across
turns within a session, appending only the new message(s) instead of reloading from SQLite and
re-tokenizing from scratch each time. This is a bigger structural change than 2.1 (touches session
lifecycle, not just request construction) — sequence it after 2.1 lands and its tests establish the
pattern for `Arc`-shared message state.

---

## Phase 3 — Measurement (do this in parallel with Phases 0-2, not after)

Current benchmarks (`benches/database.rs`, `benches/parallel_tool_dispatch.rs`) don't cover any of
the above. Without benchmarks, "faster" claims in this plan can't be verified or protected from
regression.

**New benchmarks to add:**

1. **`benches/ollama_provider.rs`** — round-trip cost of `to_ollama_request`/`from_ollama_response`
   conversion, and (with a mocked local HTTP server, e.g. `wiremock`) end-to-end `stream()` latency
   to first byte vs. total — this is what will prove Phase 2.2 actually fixed the buffer-then-replay
   issue.
2. **`benches/context_compaction.rs`** — `token_count()`/BPE-encode cost at realistic transcript
   sizes (10, 50, 200 turns), and `compact()` cost — proves/disproves the Phase 1.3 and 2.4 findings
   with numbers instead of code-reading.
3. **`benches/markdown_render.rs`** — `parse_markdown()` cost as a function of accumulated response
   length, called repeatedly to simulate redraw-per-chunk — quantifies the O(n²) claim in 2.3 and
   verifies the fix.
4. Fix the measurement bug in `benches/database.rs` where `setup_test_db()` (new runtime + tempdir +
   migrations) runs *inside* `b.iter()` for every sample (`benches/database.rs:16-24,31-34`) — move
   setup outside the timed closure so the existing benchmarks measure what they claim to.

**Also add:** a short manual test protocol for local-model sessions specifically — e.g. a documented
`cargo run -- run "..."` script against a small (7B, CPU-feasible) and a larger (30B+, GPU) Ollama
model, capturing `PerfMetrics` (`ollama.rs:781-792`, already surfaces cold/warm start, eval duration)
before and after each phase, since that field already exists and is Ollama-specific — no new
instrumentation needed to start collecting a baseline.

---

## Suggested sequencing

```
Week 1:      Phase 0 (all 5 items, independently shippable)
Week 2:      Phase 1.1 (config knobs) + start Phase 3 benchmarks in parallel
Week 3-4:    Phase 2.1 (Arc<[Message]>) — do this before 2.4, establishes the shared-state pattern
Week 4-5:    Phase 2.2 (real streaming) — highest user-visible impact for local models
Week 5-6:    Phase 2.3 (incremental markdown) + Phase 2.4 (persistent context across turns)
Ongoing:     Phase 1.2 (Crabrace decision) and 1.3 (tokenizer spike) — low effort, can slot in
             wherever convenient; don't block the rest of the plan on them.
```

## Non-goals

- This plan does not propose adding new local-runtime backends (e.g. llama.cpp direct bindings,
  vLLM). It focuses on making the existing Ollama integration as fast as Crustly's own code allows.
- This plan does not change compaction *quality* (summarization is intentionally cheap/non-LLM per
  `compaction.rs` — see research notes; that's a good tradeoff for local models where an extra
  summarization round-trip would itself be slow, and is out of scope here).
- Cloud-provider-specific optimizations are out of scope except where a fix (e.g. Phase 2.1, 0.3)
  benefits every provider incidentally.
