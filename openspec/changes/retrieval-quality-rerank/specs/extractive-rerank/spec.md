## ADDED Requirements

### Requirement: Opt-in extractive sentence selection per candidate chunk

When `config.rerank_extractive` is `true`, the system SHALL select the most query-relevant contiguous span of `rerank_extractive_top_n` sentences (default `3`) from each candidate chunk's text and SHALL replace the chunk's `snippet` field with this span before returning results. Sentence ranking SHALL use cosine similarity between each sentence's embedding and the query embedding. No LLM call SHALL be made and no external service SHALL be contacted.

#### Scenario: Extractive rerank returns top-3 contiguous sentences
- **WHEN** a candidate chunk has 12 sentences, `rerank_extractive = true`, `rerank_extractive_top_n = 3`, and sentences 5–7 form the highest-scoring contiguous span
- **THEN** the returned row's `snippet` contains sentences 5–7 joined and the original chunk text is preserved on the page record

#### Scenario: Disabled by default
- **WHEN** `rerank_extractive = false` (the default)
- **THEN** the system returns the chunk text unchanged and does not invoke the extractive pass

#### Scenario: Configurable span size
- **WHEN** `rerank_extractive = true` and `rerank_extractive_top_n = 1`
- **THEN** the returned `snippet` is the single highest-scoring sentence

### Requirement: Per-chunk wall-clock time budget with fallback

The extractive pass SHALL enforce a per-chunk budget of `rerank_extractive_budget_ms` milliseconds (default `10`). When a chunk exceeds the budget, the system SHALL skip extractive selection for that chunk, return the original chunk text as the snippet, and emit a debug-level log entry. The candidate SHALL remain in the result set with its original score and ordering.

#### Scenario: Over-budget chunk falls through to original text
- **WHEN** a candidate chunk's extractive pass exceeds `rerank_extractive_budget_ms = 10`
- **THEN** the returned row's `snippet` equals the original chunk text and a debug log is emitted

#### Scenario: Skip does not remove the candidate
- **WHEN** the extractive pass times out for a candidate
- **THEN** the candidate's score, ordering, and presence in the result set are unchanged

#### Scenario: Budget configurable via config
- **WHEN** `rerank_extractive_budget_ms = 25` and a chunk completes extractive rerank in `18 ms`
- **THEN** the extractive snippet is returned (no skip)

### Requirement: Candidates without embeddings or short chunks pass through unchanged

The system SHALL NOT attempt extractive rerank for chunks containing fewer than `rerank_extractive_top_n + 1` sentences or for chunks lacking a stored embedding. Such chunks SHALL pass through with their original snippet.

#### Scenario: Short chunk skipped
- **WHEN** a chunk contains 2 sentences and `rerank_extractive_top_n = 3`
- **THEN** the chunk passes through with its original text as snippet

#### Scenario: Chunk without embedding skipped without error
- **WHEN** a candidate chunk lacks a stored embedding
- **THEN** the system returns the original chunk text and emits no error

### Requirement: No new runtime dependencies

The extractive reranker SHALL be implemented using existing in-tree primitives — sentence segmentation via the existing tokenizer or a deterministic punctuation-based splitter, and cosine similarity reusing the candle/embedding code path. The change SHALL NOT introduce new crates, model files, or binary-size growth beyond the new module's source code.

#### Scenario: No new crate added to `Cargo.toml`
- **WHEN** the change lands
- **THEN** `Cargo.toml` runtime dependencies are unchanged

#### Scenario: Airgapped binary remains airgapped
- **WHEN** the extractive reranker runs with `rerank_extractive = true`
- **THEN** no network call is made and the binary continues to satisfy the airgapped-build contract

### Requirement: Feature lives in a dedicated module and is testable in isolation

The extractive reranker SHALL be implemented in a new module (`src/core/rerank.rs`) with a public function callable from `hybrid_search` and `progressive_retrieve`. The module SHALL be unit-testable independently of the SQLite database.

#### Scenario: Unit test exercises rerank without a database
- **WHEN** a unit test constructs a `Vec<CandidateChunk>` with synthetic sentence vectors and calls the rerank entry point directly
- **THEN** the test asserts the selected span without opening a `Connection`
