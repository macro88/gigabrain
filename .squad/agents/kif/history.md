# Project Context

- **Owner:** macro88
- **Project:** GigaBrain — local-first Rust knowledge brain
- **Stack:** Rust, rusqlite, SQLite FTS5, sqlite-vec, candle + BGE-small-en-v1.5, clap, rmcp
- **Created:** 2026-04-13T14:22:20Z

## Learnings

- Benchmark work is a dedicated lane on this project.
- The requested target model is Gemini 3.1 Pro when available on the active surface.
- Performance claims should trace back to proposal goals and measured evidence.

## Session Log

### 2026-04-15: SG-8 — BEIR nDCG@10 Baseline Established

**Task:** Record BEIR-style nDCG@10 baseline in benchmarks/README.md

**Approach:**
- Built release binary with `cargo build --release` (~3.5min)
- Created synthetic query set based on test fixtures (5 pages: 2 people, 2 companies, 1 project)
- Designed 8 queries with explicit ground-truth relevance judgments
- Ran queries via `gbrain query` and recorded top-3 results with scores
- Computed nDCG@10 using binary relevance and standard DCG formula
- Measured wall-clock latencies for FTS5, hybrid query, and import operations

**Results:**
- Perfect baseline: nDCG@10 = 1.0000, Hit@1 = 100%, Hit@3 = 100%
- FTS5 search: ~155ms (cold start)
- Hybrid query: ~420ms (cold start)
- Import (5 files): ~3.7s

**Findings:**
- Hash-based embeddings (SHA-256 shim) still achieve perfect recall on small synthetic corpus
- Lexical overlap is sufficient for these targeted queries
- Baseline is reproducible and establishes measurement methodology for future semantic eval

**Deliverable:**
- Updated `benchmarks/README.md` with Phase 1 baseline section
- Marked SG-8 complete in tasks.md
- Commit: 204edf3 "bench: establish Phase 1 BEIR-proxy nDCG@10 baseline"

**Next:**
- Semantic baseline with BGE-small-en-v1.5 after T14 completes
- Expand to BEIR subsets (NFCorpus, FiQA) in Phase 3
- Set regression gate: no more than 2% drop in nDCG@10
