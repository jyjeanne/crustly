# Testing Crustly with Ornith-1.0 9B (`ornith:9b`)

Step-by-step test plan for validating Crustly's native Ollama provider and
TUI against **Ornith-1.0-9B**, including the model-management features that
are new in `ollama-rs` 0.3.x — `create_model` / `create_model_stream`
(import a local GGUF, build custom variants, quantize) and `push_model` /
`push_model_stream` (upload a model to an Ollama registry namespace).

Companion documents:

- [`ollama-local-llm-test-plan.md`](./ollama-local-llm-test-plan.md) —
  general Ollama + TUI ergonomics test plan (PR #14/#15). This plan reuses
  its structure but focuses on one specific model and on the model
  **upload/import** path that the earlier plan did not cover.
- [`ollama-rs-integration-plan.md`](./ollama-rs-integration-plan.md) —
  design doc for the native provider.

## 1. The model under test

**Ornith-1.0-9B** (DeepReinforce, `deepreinforce-ai/Ornith-1.0-9B` on
Hugging Face, `ornith:9b` on the Ollama library):

| Property | Value |
|---|---|
| Family | Ornith-1.0 (9B-Dense, 31B-Dense, 35B-MoE, 397B-MoE) |
| Under test | 9B **dense**, the smallest member, sized for single-GPU use |
| Base | Post-trained (RL) on Qwen 3.5 — not a from-scratch pretrain |
| Specialty | Agentic **coding** (SWE-Bench / Terminal-Bench-class tasks) — a good match for Crustly's agent + tool-calling loop |
| Context window | 256K tokens (262,144) |
| License | MIT |
| Default Ollama tag | `ornith:9b` (explicit quant tags exist, e.g. `ornith:9b-q4_K_M`) |

Why this model is a useful test subject for Crustly specifically:

1. It is an *agentic coding* model, so it should exercise the tool-calling
   path (`read_file`, `edit_file`, `bash`) harder than a generic chat model.
2. Its Qwen 3.5 base means it may emit a `thinking` field and/or `<think>`
   tags — both of Crustly's reasoning-extraction paths (TC-OL-05/06 in the
   companion plan) become testable with one model.
3. 256K context makes `num_ctx` overrides and their memory cost observable.

> **Verify, don't trust:** the first action after pulling the model is
> `crustly ollama show ornith:9b` — confirm the reported **capabilities**
> (expect at least `completion`, `tools`; possibly `thinking`) and the
> context length before running the dependent test cases. Any case below
> that assumes a capability the model doesn't report is skipped, not failed.

## 2. Required hardware

Sizes below are planning estimates for a 9B dense Qwen-3.5-family model
(weights: ~5.5–6 GB at Q4_K_M, ~9.5 GB at Q8_0, ~18 GB at FP16; KV cache:
roughly 140–150 KiB/token at FP16 for this architecture class). Verify the
actual numbers on your machine with `ollama ps` (shows real VRAM/RAM of the
loaded model + context) and adjust.

| Tier | Hardware | What it supports |
|---|---|---|
| **Minimum (CPU-only)** | 8-core x86/ARM, **16 GB RAM**, 15 GB free disk | `ornith:9b` (Q4_K_M) at small context (≤8K). Expect single-digit tok/s — usable for functional correctness testing only, not for the full E2E session. |
| **Recommended** | GPU with **8 GB VRAM** (RTX 3060/4060, RX 7600 XT) or Apple Silicon with **16 GB unified memory**, 16 GB system RAM, 20 GB free disk | Q4_K_M fully offloaded at 8K–16K context, interactive speed (≳25 tok/s). This is the baseline tier this plan is written for. |
| **Comfortable** | **12–16 GB VRAM** (RTX 4070 Ti S/3090/4080, M-series 32 GB) | Q4_K_M at 32K context (the recommended `num_ctx` for agentic coding runs), or Q8_0 at 8K. |
| **Large-context / quantize tests** | **24 GB+ VRAM** (RTX 3090/4090, M-series 64 GB) | 64K+ context; FP16 weights for the §6.4 re-quantization test case. 256K context is a *stress* case: KV cache alone is estimated at ~35 GB FP16 — only attempt on ≥48 GB unified memory / multi-GPU, otherwise expect partial CPU offload and a large slowdown (that graceful degradation is itself a test case, TC-ORN-10). |

Disk note: §6 (import + custom variants + quantize) temporarily holds **two
to three copies** of the weights (source GGUF + blob store + quantized
output). Budget **≥ 25 GB free** for the full plan, ~15 GB if you skip §6.4.

## 3. Environment setup

### 3.1 Software prerequisites

```bash
# 1. Ollama daemon — use a recent build (>= 0.9; the /api/create "files"/
#    "from" JSON shape used by ollama-rs 0.3.x replaced the old raw-
#    Modelfile API in Ollama 0.5.5+, and Ornith needs a 2026-era runtime).
ollama --version
ollama serve   # if not already running as a service

# 2. Confirm the API is up at the default host Crustly expects
#    (DEFAULT_OLLAMA_HOST in src/llm/provider/ollama.rs).
curl -s http://127.0.0.1:11434/api/version

# 3. Rust toolchain per Cargo.toml (rust-version = "1.75")
rustc --version
```

### 3.2 Build matrix (regression gate — run before any manual step)

```bash
cargo fmt --check
cargo clippy --all-targets --features all-llm -- -D warnings
cargo test --no-default-features   # ollama feature truly optional
cargo test --features ollama       # primary build under test
cargo build --release --features ollama
```

All green before proceeding. The release build is what you'll drive
interactively (a debug TUI skews the tok/s readings in the Model Info panel).

### 3.3 Crustly config

Start **without** `[providers.ollama]` in `config.toml` — §4 validates the
download-then-use flow from a clean slate. After §4, configure:

```toml
[providers.ollama]
host = "http://127.0.0.1:11434"
model = "ornith:9b"
keep_alive = "10m"    # keep the 5.7 GB model resident between test cases
num_ctx = 32768       # agentic-coding sweet spot; see hardware tiers above
```

## 4. Getting the model — Path A: pull from the Ollama library (primary)

This is the path an end user takes, entirely through Crustly's own tooling.

| ID | Step | Command / keys | Expected |
|---|---|---|---|
| TC-ORN-01 | Baseline: no ornith installed | `crustly ollama list` | `ornith` absent (remove a stale copy first: `crustly ollama rm ornith:9b`). |
| TC-ORN-02 | CLI pull | `crustly ollama pull ornith:9b` | Streaming progress (`pulling manifest` → layer digests with growing % → `success`); exit 0; `crustly ollama list` now shows `ornith:9b` at ~5–6 GB. |
| TC-ORN-03 | Capability check | `crustly ollama show ornith:9b` | License (MIT), parameters, template, and capabilities print. **Record the capabilities list** — it gates TC-ORN-08/09. |
| TC-ORN-04 | TUI pull (alternate route) | `cargo run --release --features ollama`, then `Ctrl+D`, type `ornith:9b` | Free-text entry accepted (ornith is not in the curated suggestion list — typing an arbitrary `repo:tag` must work); live progress bar; dialog reports completion. Run after `crustly ollama rm ornith:9b` if you want a real download, otherwise it re-pulls instantly from cache (also fine — verifies the idempotent path). |
| TC-ORN-05 | Nonexistent tag | `crustly ollama pull ornith:999b` | Clear error, non-zero exit, no panic/hang. |

## 5. Getting the model — Path B: upload/import a local GGUF (ollama-rs `create_model`)

This path covers the **new ollama-rs 0.3.x features** and simulates the
air-gapped / custom-weights workflow: you have a GGUF file (e.g.
`Ornith-1.0-9B-Q4_K_M.gguf` downloaded from the Hugging Face repo on
another machine) and want it served by Ollama **without** `ollama pull`.

> **Known gap (by design of this plan):** Crustly's model-management wrapper
> (`src/llm/provider/ollama_models.rs`) and the `crustly ollama` subcommand
> (`src/cli/mod.rs`, `OllamaCommands`) currently expose only
> `list | pull | rm | show | embed`. `create`, `cp` and `push` exist in
> `ollama-rs` 0.3.5 but are **not yet wired into Crustly** — so this section
> is exercised through a small standalone harness crate (below). If these
> flows should become first-class (`crustly ollama create/cp/push`), that is
> a follow-up implementation task, listed in §9; the harness here doubles as
> its reference implementation.

### 5.1 Upload the GGUF blob into Ollama's store

`ollama-rs` has no wrapper for the blob endpoint, so this one step is raw
HTTP (`/api/blobs/sha256:<digest>`). The server verifies the digest.

```bash
GGUF=Ornith-1.0-9B-Q4_K_M.gguf
DIGEST=$(sha256sum "$GGUF" | cut -d' ' -f1)
curl -f -T "$GGUF" "http://127.0.0.1:11434/api/blobs/sha256:$DIGEST"
echo "sha256:$DIGEST"   # keep this for step 5.2
```

Expected: HTTP 201 (or 200 if the blob already exists). A corrupted upload
returns HTTP 400 `digest mismatch` — that negative case is TC-ORN-B3.

### 5.2 Test harness crate

```bash
cargo new --bin ornith-import-test && cd ornith-import-test
cargo add ollama-rs@0.3.5 --features stream
cargo add tokio --features full
cargo add tokio-stream
```

`src/main.rs`:

```rust
use std::collections::HashMap;

use ollama_rs::models::create::CreateModelRequest;
use ollama_rs::Ollama;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::try_new("http://127.0.0.1:11434")?;

    // args: <model-name> <blob-digest e.g. sha256:abc...>
    let mut args = std::env::args().skip(1);
    let name = args.next().expect("model name");
    let digest = args.next().expect("blob digest");

    let mut files = HashMap::new();
    files.insert("ornith-1.0-9b-q4_k_m.gguf".to_string(), digest);

    let request = CreateModelRequest::new(name).files(files);

    // New in ollama-rs 0.3.x: streaming create with per-step status.
    let mut stream = ollama.create_model_stream(request).await?;
    while let Some(status) = stream.next().await {
        println!("status: {}", status?.message);
    }
    Ok(())
}
```

### 5.3 Import test cases

| ID | Step | Expected |
|---|---|---|
| TC-ORN-B1 | Blob upload (§5.1) | 201/200; blob visible under Ollama's blob store. |
| TC-ORN-B2 | `cargo run -- ornith-local:9b sha256:<digest>` | Statuses stream (`using existing layer …` / `creating new layer …` → `writing manifest` → `success`); `crustly ollama list` shows `ornith-local:9b`. |
| TC-ORN-B3 | Re-run §5.1 with a wrong digest in the URL | HTTP 400, no partial blob left behind. |
| TC-ORN-B4 | Create referencing a digest never uploaded | `create_model_stream` yields an error status/`Err` (not a hang). |
| TC-ORN-B5 | Chat template sanity | Point `[providers.ollama] model = "ornith-local:9b"`, send one tool-using prompt in Crustly. **If tool calls come back malformed**, the GGUF's embedded chat template is missing/wrong — re-create with an explicit `.template(...)`/`.system(...)` on the `CreateModelRequest` (community re-packs of ornith exist precisely because of template issues; library `ornith:9b` from Path A is the reference behavior to compare against). |

## 6. Custom variants, quantization, copy, push (ollama-rs new features, continued)

All via the §5.2 harness (extend `main.rs` per snippet), against the Path A
model so results are comparable to a known-good baseline.

### 6.1 Derived model with Crustly-tuned defaults (`from` + `system` + `parameters`)

```rust
use ollama_rs::models::ModelOptions;

let request = CreateModelRequest::new("ornith-crustly:9b".to_string())
    .from_model("ornith:9b".to_string())
    .system("You are Crustly's coding agent. Prefer minimal diffs and \
             always explain shell commands before proposing them.".to_string())
    .parameters(ModelOptions::default().num_ctx(32768).temperature(0.6));
```

**TC-ORN-C1:** create succeeds; `crustly ollama show ornith-crustly:9b`
reflects the system prompt and parameters; a Crustly chat session against it
observably follows the baked-in system prompt without Crustly sending one.

### 6.2 Copy

```rust
ollama.copy_model("ornith-crustly:9b".into(), "ornith-backup:9b".into()).await?;
```

**TC-ORN-C2:** copy is instant (blobs shared, not duplicated — disk usage
must NOT grow by another ~6 GB); both names listed; `crustly ollama rm
ornith-backup:9b` removes the name without breaking the original.

### 6.3 Delete via Crustly (round-trip with harness-created models)

**TC-ORN-C3:** `crustly ollama rm ornith-local:9b` deletes a model that
Crustly itself never created — confirms Crustly's wrapper and the harness
operate on the same store with no bookkeeping mismatch.

### 6.4 Re-quantization (`quantize`, needs the FP16 source + 24 GB-tier hardware)

```rust
use ollama_rs::models::create::QuantizationType;

let request = CreateModelRequest::new("ornith:9b-q4km-local".to_string())
    .from_model("ornith-fp16:9b".to_string()) // FP16 import via §5 first
    .quantize(QuantizationType::Q4KM);
```

**TC-ORN-C4 (optional):** streaming statuses show quantization progress;
resulting model is ~⅓ the FP16 size; a short chat sanity-checks coherence.

### 6.5 Push (upload to a registry namespace)

Prerequisites: an ollama.com account; your machine's public key
(`~/.ollama/id_ed25519.pub`) added under account settings → Ollama keys.
Only push content whose license permits redistribution — Ornith is MIT, and
your §6.1 derivative is fine.

```rust
// Registry names must be <namespace>/<model>:<tag> — copy first:
ollama.copy_model("ornith-crustly:9b".into(),
                  "<your-namespace>/ornith-crustly:9b".into()).await?;

let mut stream = ollama
    .push_model_stream("<your-namespace>/ornith-crustly:9b".to_string(), false)
    .await?;
while let Some(status) = stream.next().await {
    let s = status?;
    println!("{} {:?}/{:?}", s.message, s.digest, s.total);
}
```

| ID | Step | Expected |
|---|---|---|
| TC-ORN-P1 | Push with key registered | Statuses stream (`retrieving manifest` → `pushing <digest>` with `total` bytes → `success`); model visible in your namespace on ollama.com. |
| TC-ORN-P2 | Push without registering the key | Clean streamed error (unauthorized), not a panic. |
| TC-ORN-P3 | Round-trip | From a second machine (or after `rm`): `crustly ollama pull <your-namespace>/ornith-crustly:9b` — Crustly pulls your uploaded model and chats with it. **This closes the loop: model uploaded with ollama-rs, consumed by Crustly.** |

## 7. Functional tests: Crustly + `ornith:9b`

Preconditions: §3.3 config active, `--features ollama` release build.
These specialize the companion plan's Section 4 to ornith's profile.

| ID | Objective | Steps | Expected |
|---|---|---|---|
| TC-ORN-06 | Basic chat + provider badge | Ask "What is 2+2?" in the TUI | Correct answer; header shows `ollama · ornith:9b`; token usage populated. |
| TC-ORN-07 | Streaming throughput | Ask for a ~300-word explanation | Tokens render incrementally; on the "Recommended" hardware tier expect ≳25 tok/s in the header/Model Info; clean `MessageStop`. |
| TC-ORN-08 | Tool calling (ornith's specialty) | "Read Cargo.toml and tell me which TUI crates this project uses" | Well-formed `read_file` tool call, result fed back, correct final answer naming ratatui/crossterm/tui-textarea. Gate: `tools` in TC-ORN-03 capabilities. |
| TC-ORN-09 | Reasoning extraction | Ask a multi-step planning question | If the model emits `thinking` (Qwen 3.5 heritage): distinct Thinking block from `message.thinking`. If it emits inline `<think>` tags: fallback splitter engages, no literal tags leak into the answer. Either path passing = pass; record which one fired. |
| TC-ORN-10 | Large context (`num_ctx`) | Raise `num_ctx` to 65536, restart, send a message; watch `ollama ps` | Model loads with larger KV allocation; on VRAM-constrained tiers, partial CPU offload degrades speed but nothing crashes; `Ctrl+O` Model Info shows the overridden context window. Then restore 32768. |
| TC-ORN-11 | Agentic multi-step task | In `AutoPlan` mode (`Shift+Tab`): "add a doc comment to `ollama_host()` in src/cli/mod.rs" | Read-only tools run unprompted, `edit_file` still prompts (AutoPlan guardrail); proposed diff is sane. This is the model class's home turf — quality of the plan/diff is worth noting in the test log. |
| TC-ORN-12 | Perf metrics & warm/cold | First message after `ollama stop ornith:9b`, then a second message; `Ctrl+O` after each | First: nonzero load duration (cold). Second: ~0 load (warm). tok/s, prefill and generation durations all plausible. |
| TC-ORN-13 | `keep_alive` | Set `keep_alive = "0"`, one message, then `ollama ps` | Model unloads immediately; with `"10m"` it stays resident. |
| TC-ORN-14 | Provider switch | Pull a second small model (`llama3.2:3b`), `Ctrl+W`, switch to it and back | Next message answered by the selected model; `Ctrl+O` confirms; no restart needed. |
| TC-ORN-15 | Daemon down | `pkill ollama` (or `systemctl stop ollama`), send a message, restart daemon, retry | Clear network error, no panic; recovers after restart. |
| TC-ORN-16 | Cost stays $0.00 | Check cost display after several exchanges | Always $0.00 for local inference. |

## 8. Combined end-to-end walkthrough (the "one sitting" script)

1. §3.2 build matrix green; `ollama serve` running; no ornith installed,
   no `[providers.ollama]` config.
2. `crustly ollama pull ornith:9b` (TC-ORN-02) → `crustly ollama show
   ornith:9b` and record capabilities (TC-ORN-03).
3. Add §3.3 config; launch `cargo run --release --features ollama`.
4. Chat smoke test + streaming (TC-ORN-06/07); `Ctrl+O` for metrics
   (TC-ORN-12).
5. Tool-calling and agentic tasks (TC-ORN-08/11); reasoning check
   (TC-ORN-09).
6. Blob upload + `create_model_stream` import via harness (§5, TC-ORN-B1..B5).
7. Derived model + copy + Crustly `rm` round-trip (§6.1–6.3, TC-ORN-C1..C3).
8. Optional: quantize (§6.4) and push + pull-back round-trip (§6.5,
   TC-ORN-P1..P3).
9. Failure drills: bad tag, daemon down, wrong digest (TC-ORN-05/15/B3/B4).
10. Cleanup: `crustly ollama rm` for every model created in 6–8;
    `crustly ollama list` back to the pre-test state; disk space reclaimed.

Pass criterion: every non-optional case passes; no panic, hang, stuck
terminal, or silent no-op anywhere; capabilities-gated cases either pass or
are recorded as skipped-with-reason.

## 9. Follow-up implementation candidates surfaced by this plan

Not required for sign-off — file as issues if the team wants them:

1. `crustly ollama create <name> --from <model> | --gguf <path>` wrapping
   `create_model_stream` (+ the raw `/api/blobs` upload, which ollama-rs
   does not wrap) with the same live progress UX as `pull`.
2. `crustly ollama cp <src> <dst>` and `crustly ollama push <name>` wrapping
   `copy_model` / `push_model_stream`.
3. Add `ornith:9b` to the TUI download dialog's curated suggestions
   (`ollama_download::CURATED_MODELS`) — it is a strong fit for Crustly's
   coding-agent use case.

## 10. Sign-off checklist

- [ ] §3.2 automated suite green (fmt, clippy, tests with and without the
      `ollama` feature).
- [ ] Path A acquisition (TC-ORN-01..05) passed via Crustly's own tooling.
- [ ] Path B GGUF import via `ollama-rs create_model_stream`
      (TC-ORN-B1..B5) passed.
- [ ] Custom variant / copy / delete (TC-ORN-C1..C3) passed; quantize
      (C4) passed or skipped-with-reason (hardware).
- [ ] Push round-trip (TC-ORN-P1..P3) passed or skipped-with-reason
      (no registry account).
- [ ] Functional suite (TC-ORN-06..16) passed; capability-gated cases
      annotated with the §4 capability record.
- [ ] E2E walkthrough (§8) completed in one sitting without failure.
- [ ] All deviations logged with repro steps and linked to follow-up issues.

## 11. References

- Model card: <https://huggingface.co/deepreinforce-ai/Ornith-1.0-9B>
- Ollama library page: <https://ollama.com/library/ornith>
- `ollama-rs` crate (0.3.5 pinned in `Cargo.lock`):
  <https://crates.io/crates/ollama-rs> /
  <https://github.com/pepperoni21/ollama-rs>
- Ollama HTTP API (`/api/create`, `/api/blobs`, `/api/push`):
  <https://github.com/ollama/ollama/blob/main/docs/api.md>
- Ollama model import guide: <https://docs.ollama.com/import>
