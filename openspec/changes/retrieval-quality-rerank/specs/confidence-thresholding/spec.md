## ADDED Requirements

### Requirement: Hard relevance floor on returned candidates

After dedup and cross-reference scoring, and before MMR, the system SHALL drop any candidate whose post-boost `fused_score` is strictly less than `config.relevance_floor`. Dropped candidates SHALL NOT appear in the returned result set even when fewer than `k` results remain.

#### Scenario: Candidate below floor is excluded
- **WHEN** `relevance_floor = 0.3` and a candidate has `fused_score = 0.27`
- **THEN** the candidate is excluded from the result set

#### Scenario: Candidate equal to floor passes
- **WHEN** `relevance_floor = 0.3` and a candidate has `fused_score = 0.30`
- **THEN** the candidate is retained

#### Scenario: Floor compares post-boost score
- **WHEN** a candidate has pre-boost score `0.28`, receives a cross-reference boost of `+0.05` to `0.33`, and `relevance_floor = 0.30`
- **THEN** the candidate is retained

### Requirement: Returning fewer than `k` is the contract, not an error

The system SHALL return only candidates that pass the floor, even if the resulting count is less than the caller's requested `k`. The system SHALL NOT pad results, retry with a relaxed floor, or error on under-fill. CLI and MCP responses SHALL include the actual returned count and SHALL NOT mark short results as failures.

#### Scenario: Empty result set returned without error
- **WHEN** every candidate falls below `relevance_floor` for a query
- **THEN** the system returns an empty result set with a successful exit/return status; no error is raised

#### Scenario: Two of `k = 5` candidates pass the floor
- **WHEN** a query requests `k = 5` and only two candidates score above the floor
- **THEN** the result set contains those two candidates and the response indicates `count = 2` without padding

### Requirement: Configurable floor with disable contract

The system SHALL read `relevance_floor` from the `config` table (default `0.3`) and SHALL accept a `--relevance-floor` CLI flag on `quaid search` and `quaid query` and a `relevance_floor` parameter on `memory_search` and `memory_query` MCP tools. Values SHALL fall in `[0.0, 1.0]`. A value of `0.0` SHALL disable filtering, returning the full post-boost candidate set.

#### Scenario: Default floor applied when no override is set
- **WHEN** `relevance_floor = 0.3` and a query runs without flag/parameter
- **THEN** filtering applies at `0.3`

#### Scenario: `--relevance-floor 0.0` disables filtering
- **WHEN** the user invokes `quaid query "obscure topic" --relevance-floor 0.0`
- **THEN** every candidate (after dedup and boost) is returned regardless of score

#### Scenario: MCP parameter overrides config
- **WHEN** an MCP `memory_search` call passes `"relevance_floor": 0.5` and config sets `0.3`
- **THEN** the call filters at `0.5` for this invocation

#### Scenario: Out-of-range value is rejected
- **WHEN** the user invokes `quaid query "x" --relevance-floor 1.5`
- **THEN** the CLI returns a validation error and no query is executed

### Requirement: Floor applies inside `progressive_retrieve` on initial and expansion-step candidates

The confidence floor SHALL be applied both to the initial candidate set and to candidates introduced by each expansion step within `progressive_retrieve`. Below-floor candidates SHALL NOT be expanded.

#### Scenario: Below-floor candidate is not expanded
- **WHEN** a candidate's `fused_score = 0.20` falls below `relevance_floor = 0.3` and `progressive_retrieve` would otherwise expand its outbound links
- **THEN** the candidate is dropped and its outbound links are not walked

#### Scenario: Above-floor expansion candidate enters the result set
- **WHEN** an expansion-step candidate has `fused_score = 0.45` and `relevance_floor = 0.3`
- **THEN** the candidate is retained and counted toward the token budget
