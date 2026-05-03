## Why

DAB §4 Semantic / Hybrid is the chronic top-line drag on Quaid's overall grade: across nine consecutive releases (v0.9.1 → v0.9.10) the score has oscillated 19–31/50 and never broken 30 consistently, while every other DAB section sits at or near full marks. Beta testers consistently flag two failure modes that don't depend on the embedding model: (1) the top-k is dominated by repeated chunks of the same source page, and (2) low-confidence noise fills slots just to honor `k`. Epic 1 (`knowledge-graph-layer`) populates the link graph; this change reshapes the search layer so that improvement actually surfaces in results, and adds the dedup, diversity, and confidence machinery testers have been asking for. Contradiction recall (DAB §7) has the same root cause family but is already specced under `contradiction-semantic-gate`; that work is referenced here, not duplicated.

## What Changes

- Add **intra-document deduplication** to `hybrid_search`: when the candidate set contains multiple chunks from the same page, collapse to the strongest chunk plus a document-level reference. Configurable via a new CLI flag `--max-chunks-per-doc N` (default 1 in skill flows; query-tunable).
- Add **MMR (Maximal Marginal Relevance)** as a post-fusion reranking pass on the merged FTS5 + vector candidate set, penalizing candidates that are semantically too close to already-selected results. λ tunable via config; default 0.7 (relevance-weighted).
- Add a **confidence threshold filter**: drop candidates whose normalized relevance score falls below a configurable floor (`relevance_floor`, default `0.3`) even if doing so returns fewer than `k` results. Skill-side callers can override per-query.
- Add **cross-reference scoring**: a graph-lite boost where a candidate page referenced by other top-ranked candidates earns an additive score increment. Reads the existing `links` table populated by Epic 1; degrades gracefully (no boost) when the graph is sparse.
- Add an optional **extractive reranker pass** that selects the most query-relevant 1–3 sentences from each candidate chunk before final ordering. Stays inside the airgapped binary — no LLM call, no external dependency. Off by default; opt-in via `rerank_extractive` config flag.
- Plumb all four ranking signals through `progressive_retrieve` so token-budget gating sees the same quality-filtered candidate set as the top-k API.
- **BREAKING (pre-release)**: `hybrid_search` and `progressive_retrieve` return values gain new optional fields (`mmr_score`, `cross_ref_boost`, `dedup_collapsed_count`). Existing fields are unchanged.
- **Out of scope (already specced or deferred)**: contradiction semantic gating is tracked under the existing `contradiction-semantic-gate` spec and is referenced, not re-proposed. REFRAG-style chunk compression (#76) remains deferred per the roadmap's Question 3 — revisit only if latency becomes a tester complaint.

## Capabilities

### New Capabilities

- `result-deduplication`: Intra-document collapse of multi-chunk hits in retrieval results, with a `--max-chunks-per-doc` knob and a `dedup_collapsed_count` indicator on returned rows.
- `mmr-reranking`: MMR-based diversity reranking applied to the fused candidate set, configurable λ, deterministic ordering for benchmark reproducibility.
- `confidence-thresholding`: Configurable relevance floor that filters low-confidence candidates from final results, with an explicit "fewer-than-k" contract for callers.
- `cross-reference-scoring`: Graph-lite boost for candidates co-cited by other top-ranked results within the same query, reading the `links` table populated by Epic 1.
- `extractive-rerank`: Optional per-chunk extractive sentence selection layered before final scoring, gated behind `rerank_extractive` and time-budgeted (≤ 10 ms per chunk, no LLM).

### Modified Capabilities

- None. The base `hybrid_search` and `progressive_retrieve` behaviors are not yet formally specced (they predate this OpenSpec workflow); the new capabilities above define their post-change surface explicitly. If a `hybrid-search` or `progressive-retrieval` spec lands ahead of this change, the four ranking signals fold into it as deltas instead.

## Impact

- **Code**: `src/core/search.rs` (dedup pass, MMR rerank, confidence filter, cross-ref boost integration), `src/core/progressive.rs` (apply same filters before token-budget expansion), `src/core/types.rs` (new optional `SearchResult` fields), `src/commands/search.rs` and `src/commands/query.rs` (`--max-chunks-per-doc`, `--relevance-floor`, `--mmr-lambda` flags), `src/mcp/server.rs` (`memory_search` / `memory_query` parameter pass-through and result schema extension), optionally `src/core/rerank.rs` (new file for extractive reranker; feature-flagged).
- **Schema**: No DDL changes. New keys in the existing mutable `config` table: `relevance_floor`, `mmr_lambda`, `max_chunks_per_doc_default`, `cross_ref_boost_weight`, `rerank_extractive`. Reads `links.edge_weight` from Epic 1; if absent (older databases without Epic 1), cross-reference scoring no-ops.
- **Config**: New keys above. Defaults chosen to preserve current top-k composition for unparameterized calls except where dedup activates (most user-visible win).
- **Migration**: None. Bump `SCHEMA_VERSION` only if Epic 1's bump hasn't landed; otherwise inherit v7.
- **Tests**: New `tests/search_dedup.rs`, `tests/search_mmr.rs`, `tests/search_confidence.rs`, `tests/search_cross_ref.rs`. Extend `tests/progressive_retrieve.rs` to cover all four filters. Add `tests/rerank_extractive.rs` behind the feature flag.
- **Benchmarks**: DAB §4 Semantic / Hybrid ≥ 35/50 sustained across two consecutive releases (current ceiling 31/50). MSMARCO P@5 must not regress versus the Epic 1 baseline; expected lift ≥ 3 points from MMR + dedup. DAB §7 Contradiction Detection is tracked separately under `contradiction-semantic-gate` and is not a gate for this change.
- **Dependencies**: No new runtime dependencies. Cosine similarity for MMR reuses the existing vector code path.
- **Performance**: Reranking adds one bounded O(k²) MMR pass per query (k ≤ 50 typical). Cross-reference scoring is one indexed `links` lookup per candidate. End-to-end retrieval latency budget: +15 ms p95 over the Epic 1 baseline. Extractive reranker, when enabled, adds ≤ 10 ms per candidate chunk and is gated off by default.
