## Context

Quaid's retrieval pipeline today is `src/core/search.rs::hybrid_search` — an SMS exact-match short-circuit, palace-wing filter, FTS5 + vector candidate set, and a set-union merge — followed by `src/core/progressive.rs::progressive_retrieve`, which expands and gates candidates against a token budget. There is no diversity step, no relevance floor, no per-page collapsing, and no use of the link graph at ranking time. The benchmark signal is unambiguous: nine consecutive DAB releases (v0.9.1 → v0.9.10) score 19–31/50 on §4 Semantic / Hybrid, while every other section is at or near full marks. Beta-tester writeups (#56, #57, #63, #71, #82, #85, #87, #96, #108) repeatedly describe two failure shapes — repeated chunks of the same page, and noisy low-confidence hits filling slots to honor `k`.

Constraints shaping this change:

- **Airgapped binary rule.** No new runtime dependencies, no LLM calls, no network. All reranking is local and deterministic.
- **Epic 1 (`knowledge-graph-layer`) is the upstream feeder.** Cross-reference scoring reads `links.edge_weight` and the partial unique index Epic 1 introduces. This change must degrade gracefully when the graph is sparse.
- **Pre-release.** No back-compat obligations on the `SearchResult` shape; new optional fields can be added freely.
- **Latency budget.** Retrieval p95 today sits inside the existing skill latency target. The four ranking signals together must add ≤ 15 ms p95.
- **Benchmark reproducibility.** DAB and MSMARCO must score deterministically, so all reranking must produce the same ordering for the same input.

## Goals / Non-Goals

**Goals:**

- Eliminate per-page chunk repetition in default top-k results.
- Diversify the top-k via MMR so semantically near-duplicate hits don't crowd out other relevant pages.
- Drop low-confidence noise even at the cost of returning fewer than `k` results.
- Use Epic 1's link graph to boost candidates co-cited by other top-ranked candidates within the same query.
- Plumb all four signals through `progressive_retrieve` so token-budget expansion sees the same filtered set.
- Land an opt-in extractive reranker pass that improves snippet quality without leaving the binary.
- DAB §4 ≥ 35/50 sustained over two consecutive releases. MSMARCO P@5 non-regressing vs Epic 1 baseline; expected lift ≥ 3 points.

**Non-Goals:**

- LLM-assisted reranking, REFRAG-style chunk compression, or any model larger than the current bge-small default. (REFRAG remains deferred per roadmap Question 3; the embedding-model matrix question is Epic 6's lane.)
- Contradiction detection changes. The semantic gating fix is already specced under `contradiction-semantic-gate`.
- Modifying the FTS5 or vector candidate generation layers. The four signals are post-fusion only.
- Cross-collection ranking. Collection-local routing remains the contract.
- New persistent storage. All knobs live in the existing mutable `config` table; no new tables, no schema bump on this change unless Epic 1's bump hasn't landed.

## Decisions

### Decision 1 — Apply the four signals as ordered post-fusion passes, not a single rewrite.

**Why:** Each signal has a distinct measurable effect, and the team needs to be able to A/B them on DAB without entangling the diff. Order matters for benchmark stability: dedup → confidence floor → cross-ref boost → MMR. Dedup runs first so MMR doesn't waste its diversity budget penalizing chunks of the same page; the confidence floor runs before the boost so the boost can't drag a noise candidate above the floor; MMR runs last because its λ depends on the score distribution after boosting.

**Alternatives considered:** A single fused scoring formula `score = α·sim + β·boost + γ·diversity − δ·near_dup`. Rejected because it makes per-signal regression analysis impossible and forces every benchmark run to re-tune four hyperparameters jointly.

### Decision 2 — Intra-document dedup collapses chunks to the strongest, with a `collapsed_count` indicator.

**Why:** The `page_embeddings` table chunks pages for retrieval, but the user-perceived unit of recall is the page. Returning three chunks of the same page wastes top-k slots and creates the exact repetition tester reports flag. Collapse rule: among chunks sharing `page_id`, retain the one with the highest fused score; sum the remaining chunks into a `dedup_collapsed_count` field on the surviving row so callers and benchmarks can see the collapse magnitude.

**Alternatives considered:**
- *Sum chunk scores into a page-level score.* Rejected: amplifies pages with many low-relevance chunks over pages with one perfect chunk.
- *Page-level retrieval only.* Rejected: chunk-level retrieval is what makes long-page recall work; this is a presentation/ranking choice, not a storage choice.

**Knob:** `--max-chunks-per-doc N` CLI flag and `max_chunks_per_doc_default` config key. `N=1` is the default for skill flows; allow `N>1` for callers that need granularity (e.g., browse UIs). Document references for collapsed chunks include their offsets so callers can re-fetch on demand.

### Decision 3 — MMR with cosine on the existing query/page vectors; λ = 0.7 default.

**Why:** Maximal Marginal Relevance is the cheapest, most-validated diversity reranker in the literature, and it composes with any score function. Using cosine on the embeddings already in `page_embeddings_vec_*` means no new vector math and zero extra storage. λ = 0.7 weights relevance over diversity — the right default for a memory tool where the user is asking a specific question, not browsing.

**Algorithm shape:** Greedy selection from the post-dedup, post-floor candidate set:

```text
selected = []
remaining = candidates  # sorted by fused score desc
while remaining and len(selected) < k:
    best = argmax_{c in remaining} (
        λ * c.fused_score
        - (1 - λ) * max_{s in selected} cosine(c.vec, s.vec)
    )
    selected.append(best); remaining.remove(best)
```

**Determinism:** Tie-break on stable `(fused_score desc, page_id asc, chunk_id asc)` so two runs with identical inputs produce identical orderings — required for benchmark gating.

**Alternatives considered:**
- *DPP (Determinantal Point Process).* More principled, much more expensive, and the lift over MMR on retrieval-scale k is small.
- *Per-cluster sampling on `page_embeddings_vec_*`.* Requires a clustering pass; not worth the cost for k ≤ 50.

**Knob:** `mmr_lambda` config key (default 0.7), `--mmr-lambda` CLI flag, `mmr_lambda=1.0` disables MMR entirely (relevance-only ordering, useful for A/B baselines).

### Decision 4 — Confidence floor is a hard filter, not a soft penalty, and it can return fewer than `k`.

**Why:** Soft penalties still let noise into the top-k when the candidate pool is thin; users describe this as "Quaid making something up to fill the slot." A hard floor with an explicit "fewer-than-k" contract aligns the API to its honest behavior. Skill prompts already handle empty results — they don't need padding.

**Score normalization:** The fused score from `hybrid_search` is in `[0, 1]` after the existing min-max normalization step. The default floor of `0.3` is a conservative starting point; benchmark-tunable. The floor compares against the post-boost fused score so cross-references can rescue an otherwise marginal candidate.

**Knob:** `relevance_floor` config key (default 0.3), `--relevance-floor` CLI flag, `0.0` disables filtering. MCP `memory_search` and `memory_query` accept the parameter directly.

### Decision 5 — Cross-reference boost is additive, capped, and depth-1 only.

**Why:** This is the graph-lite shortcut that gives Epic 2 some of Epic 1's value before deeper graph traversal lands at the ranking layer. A candidate that is the target of a `links` edge from another top-ranked candidate (within the current query's working set) gets an additive bonus, weighted by `links.edge_weight` from Epic 1. Depth-1 keeps the cost to one indexed lookup per candidate and keeps the boost interpretable.

**Formula:** For each candidate `c`, compute `boost(c) = cross_ref_boost_weight * sum_{s in top_n_initial, s→c in links} edge_weight(s→c)`, capped at `cross_ref_boost_cap` (default 0.15). Add to fused score. `top_n_initial` is the post-floor, pre-MMR candidate set so the boost reflects the working pool, not a self-referential top-k.

**Graceful degradation:** When `links` is empty (Epic 1 not landed, or fresh DB), every boost is 0; the path no-ops without conditional code.

**Alternatives considered:**
- *Multiplicative boost.* Easier to overshoot; harder to bound.
- *Full multi-hop graph traversal at ranking time.* Better belongs to a future "graph-aware-retrieval" capability stacking on this work; out of scope here to keep the diff measurable.

**Knob:** `cross_ref_boost_weight` config key (default 0.05), `cross_ref_boost_cap` (default 0.15). Disabled by setting weight to 0.

### Decision 6 — Extractive reranker is opt-in, time-budgeted, and lives in a new module.

**Why:** Per-chunk extractive sentence selection improves snippet quality and downstream LLM grounding, but it adds work to every retrieval and is not strictly necessary for DAB §4 ≥ 35/50. Ship the four core signals first, gate this behind `rerank_extractive` (default false), and let benchmarks decide whether to flip the default.

**Algorithm:** For each candidate chunk, score sentences by max cosine similarity to the query embedding, return top-3 contiguous sentence span (configurable). No model, no LLM. Implementation lives in a new `src/core/rerank.rs` so it can be feature-gated cleanly and tested in isolation.

**Time budget:** ≤ 10 ms per chunk wall-clock. Over-budget chunks skip extractive rerank and return the original chunk text, with a debug log. Never blocks the candidate from appearing in results.

**Alternatives considered:**
- *Cross-encoder reranker (e.g., bge-reranker-base).* Quality is higher; adds a 130MB model and 50–200ms latency. Reconsider when the embedding-model matrix question (Open question 1 in roadmap) is settled, not before.

### Decision 7 — Plumb all four filters through `progressive_retrieve` identically.

**Why:** `progressive_retrieve` is the path that actually expands a result set under a token budget; if it bypasses the filters, callers see one quality bar from `memory_search` and a different one from skill flows. Apply dedup, floor, boost, and MMR on the candidate set before the budget walk; on each expansion step, re-apply dedup and floor (MMR/boost are top-level only — they don't make sense on per-step expansions).

**Consequence:** A page that fails the floor is never expanded, even if its outbound links would otherwise fit the budget. This is the right behavior — expanding a noise hit pollutes downstream context.

### Decision 8 — All knobs live in the existing `config` table; no new tables, no schema migration.

**Why:** None of these signals need persistent state beyond their tuning constants. The mutable `config` table already holds runtime knobs. If Epic 1 lands first, this change inherits its v7 schema bump; if not, no bump is required.

**Keys added:** `relevance_floor`, `mmr_lambda`, `max_chunks_per_doc_default`, `cross_ref_boost_weight`, `cross_ref_boost_cap`, `rerank_extractive`, `rerank_extractive_top_n` (default 3), `rerank_extractive_budget_ms` (default 10).

## Risks / Trade-offs

- **Risk: dedup hides legitimately distinct content from long pages.** A page with two genuinely different sections becomes one row. → Mitigation: surfaces `dedup_collapsed_count` so callers can re-query with `--max-chunks-per-doc 3` for browse-style UIs; document the trade-off in the search skill.
- **Risk: MMR ordering becomes unstable across runs if cosine ties.** → Mitigation: stable tie-break on `(fused_score desc, page_id asc, chunk_id asc)` is part of the spec; benchmark harness verifies determinism on every run.
- **Risk: confidence floor returns empty results on legitimately hard queries.** → Mitigation: `--relevance-floor 0.0` flag is documented as the escape hatch; skill prompts treat empty results as "I don't know" rather than retry. The 0.3 default is benchmark-tunable.
- **Risk: cross-reference boost rewards densely-linked hub pages disproportionately.** → Mitigation: `cross_ref_boost_cap` (default 0.15) caps the bonus; benchmarks include hub-heavy queries to detect skew.
- **Risk: extractive reranker latency exceeds budget on long chunks.** → Mitigation: 10 ms per-chunk wall-clock budget with skip-on-timeout; chunks fall through to the original text. Off by default.
- **Risk: ordering of passes regresses some queries.** → Mitigation: each signal has a disable flag (set its weight/lambda to 0 / floor to 0 / max-chunks to ∞); CI runs DAB with each signal individually disabled to attribute lift.
- **Trade-off: signal weights are global, not query-class-aware.** A "find the latest decision about X" query might want different λ than "show me everything related to X". This change keeps tuning global; a future change can introduce intent-class routing if benchmarks justify it.

## Migration Plan

No data migration. The change is additive at the API/code level:

1. Land all four signals behind their disable flags (set to identity values: `mmr_lambda=1.0`, `relevance_floor=0.0`, `cross_ref_boost_weight=0.0`, `max_chunks_per_doc=∞`). This is a no-op release that proves the plumbing works without changing rankings.
2. Flip defaults on a follow-up commit: `mmr_lambda=0.7`, `relevance_floor=0.3`, `cross_ref_boost_weight=0.05`, `max_chunks_per_doc_default=1`. Ship as the actual quality release.
3. Run DAB §4 + MSMARCO with each signal individually disabled to attribute lift.
4. If §4 ≥ 35/50 sustained across two releases, the change ships green. If it regresses any subsection, revert defaults to identity and investigate before flipping again.

Rollback: revert the defaults to identity values via config; no code revert needed for emergency mitigation.

## Open Questions

- **Question 1 — Should `--max-chunks-per-doc` default to 1 or 2?** 1 is cleaner and what testers ask for; 2 preserves long-page recall on chunked corpora. Resolve with DAB §4 + a long-page-recall sub-benchmark.
- **Question 2 — Floor as absolute or as a fraction of the top result's score?** Absolute is simpler and benchmark-comparable across queries; fractional adapts to query difficulty. Default to absolute; revisit if hard-query empty-result rates are high in tester feedback.
- **Question 3 — Cross-ref boost should it weight by edge `relationship` type?** A `founded` link probably outranks a `related` link. Defer to per-relationship weight tuning, which is also Epic 1's open question 1 — resolve jointly.
- **Question 4 — Should MMR run before or after the budget walk in `progressive_retrieve`?** Currently specified to run before. If budget-walk expansions reintroduce near-duplicates, a second MMR pass at the budget tail may be needed. Measure first.
