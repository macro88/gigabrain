updated_at: 2026-04-16T05:49:01Z
focus_area: Phase 3 — Skills, Benchmarks, and CLI Polish (p3-skills-benchmarks)
active_issues: []
active_branch: phase3/p3-skills-benchmarks
---

# What We're Focused On

**Phase 3 is now in execution.** Phase 2 shipped in v0.2.0. The team is now executing skills completion, benchmark harnesses, and CLI/MCP polish on branch `phase3/p3-skills-benchmarks`.

**Phase 3 scope:**
- Skill completion: `skills/briefing`, `alerts`, `research`, `upgrade`, `enrich`
- CLI completion: `validate`, `call`, `pipe`, `skills doctor`, and remaining `--json` coverage
- MCP Phase 3 surface: `brain_gap`, `brain_gaps`, `brain_stats`, `brain_raw`
- Benchmark harnesses: BEIR, corpus-reality, concurrency stress, embedding migration, LongMemEval, LoCoMo, Ragas
- CI integration for offline benchmark gates and release regression checks

**Team lanes:**
- **Amy**: Production-ready SKILL.md authoring for the five remaining agent skills
- **Fry**: Core CLI/MCP implementation, tests, and CI wiring
- **Kif**: Benchmark datasets, harnesses, and measurement methodology
- **Professor**: Validate/MCP correctness review once the first code wave lands
- **Nibbler**: Adversarial review of new MCP inputs and raw-data storage
- **Scruffy**: Reproducibility and regression verification for benchmark harnesses
- **Leela**: Skills executability review and scope control

**Quality target:** zero command stubs, 16 MCP tools fully registered, and offline benchmarks enforced in CI.

**Phase gate requirements:**
- `cargo test` all pass
- `cargo clippy -- -D warnings` clean
- `cargo fmt --check` clean
- Professor + Nibbler sign-off before merge
- Skill files reviewed for executability and clarity
- Benchmark baselines established and reproducible

**Phase sequence:**
- ✅ Sprint 0 → ✅ Phase 1 (v0.1.0) → ✅ Phase 2 (v0.2.0) → 🚀 **Phase 3 (now)**
