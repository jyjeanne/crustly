# Crustly - Priority Matrix

Quick reference for improvement priorities. See `IMPROVEMENT_ROADMAP.md` for detailed information.

---

## 🎯 Sprint 10 - Critical (2 weeks)

### Must Have - Production Blockers

| # | Feature | Impact | Effort | Status |
|---|---------|--------|--------|--------|
| 1 | **Multi-Provider Support** | 🔴 CRITICAL | 3-5 days | ❌ Only Anthropic |
| 2 | **Error Recovery** | 🔴 CRITICAL | 2-3 days | ⚠️ Limited |
| 3 | **Security Hardening** | 🔴 CRITICAL | 2-3 days | ⚠️ Basic |

**Why Sprint 10:**
- Users expect multi-provider (OpenAI, Gemini, etc.) - advertised but not implemented
- App crashes on network issues - unacceptable for production
- API keys in plaintext - security risk

---

## 🚀 Sprint 11-12 - High Priority (3 weeks)

### Should Have - Feature Completeness

| # | Feature | Impact | Effort | Status |
|---|---------|--------|--------|--------|
| 4 | **Enhanced Tool System** | 🟡 HIGH | 4-5 days | ⚠️ Only 3 tools |
| 5 | **Approval System v2** | 🟡 HIGH | 3-4 days | ⚠️ v1 limited |
| 6 | **Local LLM Integration** | 🟡 HIGH | 2-3 days | ⚠️ Untested |
| 7 | **Performance & Benchmarks** | 🟡 HIGH | 3-4 days | ❌ No data |

**Why Sprint 11-12:**
- 3 tools (read, write, bash) insufficient for real coding
- Approval system v1 works but needs enhancements (diff preview, history)
- Local LLM is key differentiator but untested
- No performance data or benchmarks - flying blind

---

## 🔧 Sprint 13-15 - Medium Priority (4 weeks)

### Nice to Have - Advanced Features

| # | Feature | Impact | Effort | Status |
|---|---------|--------|--------|--------|
| 8 | **MCP Protocol** | 🟢 MEDIUM | 5-7 days | ❌ TODO stub |
| 9 | **LSP Integration** | 🟢 MEDIUM | 4-5 days | ❌ TODO stub |
| 10 | **Enhanced Sessions** | 🟢 MEDIUM | 2-3 days | ⚠️ Basic |
| 11 | **Git Integration** | 🟢 MEDIUM | 3-4 days | ⚠️ Basic git2 |
| 12 | **Configuration UI** | 🟢 MEDIUM | 2-3 days | ⚠️ CLI only |
| 13 | **Testing Infrastructure** | 🟢 MEDIUM | 3-4 days | ⚠️ 145 unit tests |

**Why Sprint 13-15:**
- MCP enables ecosystem but not critical
- LSP adds intelligence but complex
- Git/Session/Config improvements enhance workflow
- Tests are good but need integration/E2E coverage

---

## 💎 Sprint 16+ - Low Priority (Ongoing)

### Could Have - Polish & Extensions

| # | Feature | Impact | Effort | Status |
|---|---------|--------|--------|--------|
| 14 | **Advanced TUI** | 🔵 LOW | 4-5 days | ⚠️ Basic functional |
| 15 | **Telemetry** | 🔵 LOW | 2-3 days | ❌ Not implemented |
| 16 | **Plugin System** | 🔵 LOW | 7-10 days | ❌ Not planned |
| 17 | **Documentation** | 🔵 LOW | 3-4 days | ⚠️ Good README |
| 18 | **CI/CD** | 🔵 LOW | 2-3 days | ❌ Manual |
| 19 | **Examples** | 🔵 LOW | 2-3 days | ❌ Empty dir |
| 20 | **Cross-Platform** | 🔵 LOW | 2-3 days | ⚠️ Windows only |

**Why Sprint 16+:**
- Nice polish features but not essential
- Can be done incrementally
- Community contributions welcome

---

## ⚡ Quick Wins (Do Immediately)

**High Value, Low Effort - Do Today:**

1. ✅ **Fix Hard-Coded Model** (30 min)
   - Location: `src/tui/render.rs:413`
   - Problem: Model name hard-coded instead of from state

2. ✅ **Add Config Path Support** (1 hour)
   - Location: `src/cli/mod.rs:125`
   - Problem: TODO comment, feature half-implemented

3. ✅ **Add Connection Timeout** (1 hour)
   - Location: `src/llm/provider/anthropic.rs`
   - Problem: Can hang indefinitely on network issues

4. ✅ **Implement Approval Timeout** (2 hours)
   - Location: `src/tui/app.rs`
   - Problem: Approval requests never expire

5. ✅ **Create First Benchmark** (2 hours)
   - Location: `benches/database.rs` (create)
   - Problem: No performance baseline

**Total:** ~6 hours of work, huge impact

---

## 🚨 Blocking Issues for 1.0 Release

### Show-Stoppers

- ❌ **Only Anthropic works** - Claims multi-provider but only 1/6 implemented
- ❌ **Crashes on network errors** - No retry logic, no graceful degradation
- ❌ **API keys in plaintext** - Major security issue
- ❌ **No local LLM validation** - Feature advertised but not tested
- ❌ **Hard-coded model** - Bug in render.rs

### Major Issues

- ⚠️ **Only 3 tools** - Insufficient for real use
- ⚠️ **No performance data** - Can't claim "high-performance" without benchmarks
- ⚠️ **Limited error handling** - Silent failures, poor error messages

---

## 📈 Effort vs Impact Chart

```
High Impact │ 1.Multi-Provider ║ 2.Error Recovery
            │ 3.Security       ║
            │─────────────────────────────────────
            │ 4.Tools          │ 8.MCP
            │ 5.Approval v2    │ 9.LSP
            │ 6.Local LLM      │ 14.Advanced TUI
Medium      │ 7.Performance    │
Impact      │─────────────────────────────────────
            │ 10.Sessions      │ 16.Plugins
            │ 11.Git           │
            │ 12.Config UI     │
Low Impact  │ 13.Testing       │ 17.Docs
            │ 15.Telemetry     │ 18.CI/CD
            │ 19.Examples      │ 20.Cross-Platform
            └─────────────────────────────────────
             Low Effort (1-3d)   High Effort (4-10d)
```

**Priority Quadrants:**
- **Top Left (Do First):** Items 1-3, 5-7 - High impact, reasonable effort
- **Top Right (Plan Carefully):** Items 4, 8-9 - High impact but expensive
- **Bottom Left (Quick Wins):** Items 10-15, 17-19 - Easy improvements
- **Bottom Right (Defer):** Item 16 - Major effort, lower priority

---

## 🎯 Version Milestones

### v0.3.0 (Current - Sprint 9 Complete)
✅ Core TUI with markdown & syntax highlighting
✅ Interactive approval system
✅ 145 tests passing
✅ Anthropic provider working

### v0.5.0 (Beta - After Sprint 10-12)
- ✅ Multi-provider support (OpenAI, Gemini, Bedrock, Azure)
- ✅ Error recovery and resilience
- ✅ Security hardened (keyring storage)
- ✅ 8+ tools available
- ✅ Local LLM validated
- ✅ Performance benchmarks established
- ✅ Approval system v2

### v1.0.0 (Stable - After Sprint 13-15)
- ✅ All high priority features
- ✅ MCP protocol support
- ✅ LSP integration
- ✅ Enhanced git/session management
- ✅ Config UI
- ✅ Integration tests
- ✅ External security audit passed
- ✅ Production-ready documentation

### v2.0.0 (Future - Sprint 16+)
- Plugin system
- Advanced TUI features
- Telemetry
- Full CI/CD
- Community contributions

---

## 📊 Current Status Summary

| Category | Status | Grade |
|----------|--------|-------|
| **Core Functionality** | ⚠️ Works but limited | C+ |
| **Provider Support** | ❌ Only 1 of 6 | D |
| **Error Handling** | ⚠️ Basic, crashes | C- |
| **Security** | ⚠️ Plaintext keys | D+ |
| **Tools** | ⚠️ Only 3 | C |
| **TUI/UX** | ✅ Good | B+ |
| **Testing** | ✅ 145 unit tests | B |
| **Documentation** | ✅ Good README | B+ |
| **Performance** | ❓ Unknown | ? |
| **Stability** | ⚠️ Needs work | C |

**Overall Grade:** C+ (Functional prototype, not production-ready)

---

## 🎬 Action Plan - Next 30 Days

### Week 1: Quick Wins + Multi-Provider Start
- [ ] Day 1-2: Fix 5 quick wins (6 hours total)
- [ ] Day 3-5: Implement OpenAI provider
- [ ] Weekend: Test OpenAI integration

### Week 2: Complete Multi-Provider
- [ ] Day 1-3: Implement Gemini + Bedrock providers
- [ ] Day 4-5: Add provider switching UI
- [ ] Weekend: Documentation and testing

### Week 3: Error Recovery + Security
- [ ] Day 1-2: Implement retry logic and timeouts
- [ ] Day 3-4: Add keyring-based secret storage
- [ ] Day 5: Bash command sanitization
- [ ] Weekend: Security audit

### Week 4: Tool System Expansion
- [ ] Day 1: ListTool + SearchTool
- [ ] Day 2: GitTool
- [ ] Day 3: DiffTool + WebFetchTool
- [ ] Day 4-5: Tool testing and documentation
- [ ] Weekend: Sprint 10 retrospective

**Deliverable:** v0.5.0-beta with multi-provider, error handling, security, and 8+ tools

---

## 📞 Stakeholder Communication

### For Users
**Priority:** Multi-provider support, more tools, stability
**Timeline:** 4-6 weeks for beta
**Benefits:** Work with any LLM, more coding capabilities, fewer crashes

### For Contributors
**Priority:** Good first issues in tools, examples, docs
**Timeline:** Ongoing, easier items available now
**Benefits:** Clear roadmap, well-defined tasks

### For Enterprise
**Priority:** Security audit, SSO, compliance features
**Timeline:** Post-1.0 (3-4 months)
**Benefits:** Production-ready, audited, enterprise features

---

**Generated:** 2025-10-28
**Next Review:** After Sprint 10 completion (2 weeks)
**Maintained By:** Core team
