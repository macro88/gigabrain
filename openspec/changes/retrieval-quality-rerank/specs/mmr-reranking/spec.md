## ADDED Requirements

### Requirement: MMR-based diversity reranking on the candidate set

After dedup, confidence floor, and cross-reference scoring, the system SHALL apply Maximal Marginal Relevance (MMR) as a greedy reranking pass before returning results. For each remaining candidate `c`, the MMR score SHALL be:

```
mmr(c) = λ · fused_score(c) − (1 − λ) · max_{s ∈ selected} cosine(vec(c), vec(s))
```

where `vec(c)` is the candidate's stored embedding and `selected` is the set of already-chosen results. The system SHALL select candidates greedily by maximum MMR score until `k` results are chosen or candidates are exhausted.

#### Scenario: Diversity penalty downranks near-duplicates
- **WHEN** two candidates `c1` and `c2` have nearly identical fused scores (0.80, 0.79) and `cosine(vec(c1), vec(c2)) = 0.95`, and `λ = 0.7`
- **THEN** after `c1` is selected, `c2`'s MMR score becomes `0.7 × 0.79 − 0.3 × 0.95 = 0.268`, and a candidate `c3` with `score = 0.60` and `cosine ≤ 0.3` to `c1` (MMR ≈ `0.42 − 0.09 = 0.33`) is selected before `c2`

#### Scenario: First selection equals top fused-score candidate
- **WHEN** `selected` is empty
- **THEN** the candidate with the highest `fused_score` is selected first regardless of `λ`

#### Scenario: Candidates without stored vectors fall through with zero diversity penalty
- **WHEN** a candidate has no embedding (e.g., a chunk that failed to embed)
- **THEN** its MMR score equals `λ × fused_score(c)` and the system continues without erroring

### Requirement: Configurable `mmr_lambda` with disable contract

The system SHALL read `mmr_lambda` from the `config` table (default `0.7`) and SHALL accept a `--mmr-lambda` CLI flag on `quaid search` and `quaid query` to override it for a single invocation. Values SHALL fall in the closed interval `[0.0, 1.0]`. A value of `1.0` SHALL disable diversity penalization, producing pure relevance ordering equivalent to sorting by `fused_score` desc. A value of `0.0` SHALL produce pure diversity selection.

#### Scenario: `mmr_lambda = 1.0` reproduces relevance ordering
- **WHEN** `mmr_lambda = 1.0` and a query runs
- **THEN** the result ordering matches `fused_score desc` (with the deterministic tie-break) before MMR was added

#### Scenario: CLI flag overrides config value
- **WHEN** `mmr_lambda = 0.7` in config and the user invokes `quaid query "founders" --mmr-lambda 0.5`
- **THEN** MMR runs with `λ = 0.5` for this invocation only

#### Scenario: Out-of-range value is rejected
- **WHEN** the user invokes `quaid query "founders" --mmr-lambda 1.5`
- **THEN** the CLI returns a validation error and no query is executed

### Requirement: Deterministic ordering across runs

The MMR pass SHALL produce identical orderings on identical inputs. When two candidates tie on MMR score, the system SHALL break ties by `(fused_score desc, page_id asc, chunk_id asc)`. Implementations SHALL NOT rely on hash-map iteration order for selection.

#### Scenario: Two consecutive runs return identical orderings
- **WHEN** the same query is executed twice against an unchanged database with the same config
- **THEN** the returned `SearchResult` lists are equal element-for-element including `mmr_score` values

#### Scenario: Tied MMR scores break by `page_id` ascending
- **WHEN** candidates `(page_id=42, mmr=0.55)` and `(page_id=17, mmr=0.55)` tie
- **THEN** `page_id=17` is selected first

### Requirement: MMR score surfaced on returned rows

Each returned `SearchResult` row SHALL include an `mmr_score` field equal to the MMR score at the moment of selection. When MMR is disabled (`mmr_lambda = 1.0`), the field SHALL equal the row's `fused_score`.

#### Scenario: Field populated under default config
- **WHEN** a query runs with `mmr_lambda = 0.7`
- **THEN** every returned row has a non-null `mmr_score` field

#### Scenario: Field equals fused_score when MMR disabled
- **WHEN** a query runs with `mmr_lambda = 1.0`
- **THEN** every returned row has `mmr_score == fused_score`

### Requirement: MMR runs in `progressive_retrieve` only on the top-level candidate set

The system SHALL apply MMR once to the initial post-floor, post-boost candidate set inside `progressive_retrieve`. MMR SHALL NOT be re-applied on each per-step expansion within the budget walk. Dedup and confidence floor SHALL still apply to expansion-step candidates as specified by their respective requirements.

#### Scenario: Single MMR pass per query
- **WHEN** `progressive_retrieve` runs with depth 2 and produces three rounds of expansion
- **THEN** MMR scoring is computed exactly once over the initial selected set; expansion rounds use the existing budget-walk ordering
