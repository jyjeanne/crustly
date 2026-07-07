# 0003. Crabrace for provider discovery (replaces Catwalk)

Status: Accepted

## Context

Crustly needs to discover which LLM providers/models are available (cloud and
local), with up-to-date capabilities, pricing, and context-window
information, without hand-maintaining that list in `crustly` itself. Catwalk
was the originally planned integration for this.

## Decision

We use Crabrace, a Rust port of Catwalk, as the provider registry
(`src/config/crabrace.rs`, `CrabraceIntegration`/`CrabraceConfig`). It talks
to a Crabrace server (self-hosted or Docker) over HTTP for provider/model
discovery and health checks, and works with both cloud APIs and local runtimes
(Ollama, LM Studio). See `docs/guides/CRABRACE_INTEGRATION.md` for the wire
protocol and setup.

## Consequences

Provider/model metadata stays out of crustly's own release cycle - a new
model or pricing change doesn't need a crustly upgrade, just a Crabrace
registry refresh. The cost: crustly now depends on a separate service being
reachable for auto-discovery (with local/cloud fallback behavior handled in
`CrabraceIntegration`), and Catwalk is no longer relevant to this codebase -
any doc or comment still referencing Catwalk as the active integration is
stale.
