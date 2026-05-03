## ADDED Requirements

### Requirement: Additive cross-reference boost from co-cited top-ranked candidates

After dedup and before the confidence floor, for each candidate `c` the system SHALL compute a cross-reference boost equal to:

```
boost(c) = cross_ref_boost_weight · Σ_{s ∈ top_n_initial, (s → c) ∈ links} edge_weight(s → c)
```

where `top_n_initial` is the post-dedup candidate set for the current query and `edge_weight(s → c)` is read from the `links` table populated by Epic 1 (`knowledge-graph-layer`). The system SHALL add `boost(c)` to `c`'s `fused_score` to produce the post-boost score used by the confidence floor and MMR.

#### Scenario: Co-cited candidate receives boost
- **WHEN** the initial candidate set contains pages `alice`, `brex`, and `yc-w17`, the `links` table contains active rows `(alice → brex, edge_weight=1.0)` and `(yc-w17 → brex, edge_weight=0.5)`, and `cross_ref_boost_weight = 0.05`
- **THEN** `brex` receives a boost of `0.05 × (1.0 + 0.5) = 0.075` added to its fused score

#### Scenario: Candidate with no incoming edges from the working set receives no boost
- **WHEN** a candidate `c` has no `links` row from any other member of `top_n_initial`
- **THEN** `boost(c) = 0` and its `fused_score` is unchanged

#### Scenario: Only currently active edges contribute
- **WHEN** an edge `(alice → brex)` has `valid_until = 2020-01-01` in the past and the query runs today
- **THEN** the expired edge does NOT contribute to `boost(brex)`

### Requirement: Cap on per-candidate boost magnitude

The total `boost(c)` SHALL be capped at `cross_ref_boost_cap` (default `0.15`). Hub pages with many incoming edges from the working set SHALL NOT receive unbounded boosts.

#### Scenario: Hub page boost saturates at the cap
- **WHEN** a candidate `hub` is the target of 10 active `links` rows from members of `top_n_initial`, each with `edge_weight = 1.0`, `cross_ref_boost_weight = 0.05` (uncapped boost = 0.50), and `cross_ref_boost_cap = 0.15`
- **THEN** `boost(hub)` is `0.15`, not `0.50`

#### Scenario: Below-cap boost passes through unchanged
- **WHEN** the uncapped boost is `0.07` and `cross_ref_boost_cap = 0.15`
- **THEN** the applied boost equals `0.07`

### Requirement: Graceful no-op when the graph is empty

When the `links` table contains no rows reachable from the working set, the system SHALL apply zero boost to every candidate without raising an error or logging a warning. Cross-reference scoring SHALL NOT depend on Epic 1 having landed; absence of graph data SHALL produce identical results to disabling the feature.

#### Scenario: Fresh database with no graph edges
- **WHEN** a query runs against a database whose `links` table is empty
- **THEN** every candidate's `cross_ref_boost` is `0` and `fused_score` is unchanged

#### Scenario: Candidate set has no co-citations
- **WHEN** all candidates have edges only to pages outside the working set
- **THEN** every `cross_ref_boost` is `0`

### Requirement: Configurable weight and cap with disable contract

The system SHALL read `cross_ref_boost_weight` (default `0.05`) and `cross_ref_boost_cap` (default `0.15`) from the `config` table. Both SHALL be in `[0.0, 1.0]`. Setting `cross_ref_boost_weight = 0.0` SHALL disable the pass entirely (no `links` lookups performed).

#### Scenario: Weight set to zero short-circuits the lookup
- **WHEN** `cross_ref_boost_weight = 0.0` and a query runs
- **THEN** the system SHALL NOT execute `links`-table queries for cross-reference scoring and every candidate's `cross_ref_boost` SHALL be `0`

#### Scenario: Default config values populated at init
- **WHEN** `quaid init` runs against a fresh database
- **THEN** the `config` table contains `cross_ref_boost_weight = 0.05` and `cross_ref_boost_cap = 0.15`

#### Scenario: Out-of-range values rejected at write time
- **WHEN** a caller attempts to set `cross_ref_boost_weight = 1.5`
- **THEN** the write is rejected with a validation error

### Requirement: Boost magnitude surfaced on returned rows

Each returned `SearchResult` row SHALL include a `cross_ref_boost` field equal to the applied boost (post-cap). When no boost was applied, the field SHALL be `0.0`.

#### Scenario: Boosted row reports the boost
- **WHEN** a candidate received a boost of `0.075`
- **THEN** the returned row has `cross_ref_boost = 0.075`

#### Scenario: Un-boosted row reports zero
- **WHEN** a candidate received no boost
- **THEN** the returned row has `cross_ref_boost = 0.0`
