## ADDED Requirements

### Requirement: Intra-document chunk collapse in retrieval results

After fusing FTS5 and vector candidates, the system SHALL collapse multiple chunks that share the same `page_id` to a single representative row before further reranking. The representative SHALL be the chunk with the highest fused score for that page. Collapsed (non-representative) chunks SHALL NOT appear in the returned result set.

#### Scenario: Three chunks of the same page collapse to one
- **WHEN** the candidate set contains three chunks `(page=alice, score=0.81)`, `(page=alice, score=0.74)`, `(page=alice, score=0.62)` and a single chunk `(page=brex, score=0.55)`
- **THEN** the dedup pass returns two rows: the alice chunk with score 0.81 and the brex chunk with score 0.55

#### Scenario: Single-chunk pages pass through unchanged
- **WHEN** every candidate has a unique `page_id`
- **THEN** the dedup pass returns the candidate set with no rows removed

### Requirement: Surface collapsed-chunk count to callers

The retained representative row SHALL carry a `dedup_collapsed_count` field equal to the number of additional chunks of the same page that were collapsed into it. When no collapse occurred for that row, the field SHALL be `0`.

#### Scenario: Field reflects the collapse magnitude
- **WHEN** four chunks of `alice` collapse to one representative
- **THEN** the representative row has `dedup_collapsed_count = 3`

#### Scenario: Field is zero for un-collapsed rows
- **WHEN** a candidate is the only chunk of its page in the result set
- **THEN** the row has `dedup_collapsed_count = 0`

### Requirement: `--max-chunks-per-doc` knob controls collapse aggressiveness

The system SHALL expose `max_chunks_per_doc_default` in the `config` table (default `1`) and a `--max-chunks-per-doc N` CLI flag on `quaid search` and `quaid query` that overrides it for a single invocation. When `N > 1`, up to `N` highest-scoring chunks per page SHALL pass through; remaining chunks SHALL be collapsed and counted toward `dedup_collapsed_count` of the lowest-scoring retained chunk for that page.

#### Scenario: `--max-chunks-per-doc 2` retains two chunks per page
- **WHEN** five chunks of `alice` rank in the candidate set and the user invokes `quaid search "founders" --max-chunks-per-doc 2`
- **THEN** the result set contains the top-2 alice chunks; the lowest-retained one carries `dedup_collapsed_count = 3`

#### Scenario: Default value applied when no flag is set
- **WHEN** `max_chunks_per_doc_default = 1` and a query runs without the CLI flag
- **THEN** at most one chunk per page appears in the result set

#### Scenario: `N = ∞` (or unset) disables collapsing
- **WHEN** the CLI flag is given as `--max-chunks-per-doc 0` or omitted with `max_chunks_per_doc_default = 0`
- **THEN** no chunks are collapsed; the candidate set passes through unchanged and every row has `dedup_collapsed_count = 0`

### Requirement: Dedup runs before MMR, confidence floor, and cross-reference scoring

The dedup pass SHALL be the first ordered post-fusion pass in `hybrid_search`, executing before the confidence floor, cross-reference boost, and MMR reranker. `progressive_retrieve` SHALL apply the same dedup pass on its initial candidate set and on every expansion step.

#### Scenario: MMR sees the deduplicated candidate set
- **WHEN** dedup collapses three alice chunks to one and MMR is enabled
- **THEN** MMR's diversity penalty is computed against the single retained alice chunk, not against the collapsed duplicates

#### Scenario: Progressive expansion does not reintroduce same-page chunks past the limit
- **WHEN** `progressive_retrieve` expands a candidate's outbound links and the expansion would reintroduce a chunk of an already-represented page beyond `max_chunks_per_doc`
- **THEN** the expansion drops the duplicate chunk
