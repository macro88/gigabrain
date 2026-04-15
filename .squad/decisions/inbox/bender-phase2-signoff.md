# Phase 2 Destructive Validation & Sign-off Plan

**By:** Bender (Tester)
**Date:** 2026-04-16
**Scope:** Task 10.9 — Bender sign-off for Phase 2 Intelligence Layer
**PR:** #22 | **Issue:** #25

---

## Pre-Sign-off Blocker: Schema Gap Found

**`knowledge_gaps.query_hash` has NO UNIQUE constraint.** Task 8.1 specifies
`INSERT OR IGNORE` for idempotency on duplicate queries, but `INSERT OR IGNORE`
only triggers on a UNIQUE violation. Without it, every low-confidence query logs
a new gap row — the idempotency contract is broken.

**Resolution required before Bender can validate Group 8:**

1. Add `UNIQUE(query_hash)` to the `knowledge_gaps` table (preferred — simplest),
   OR create the index at init time in `db.rs` (avoids editing `schema.sql`),
   OR use `SELECT EXISTS` guard before INSERT (race-prone, not recommended).

The design says "no DDL changes" but the behavior requires a constraint that doesn't
exist. Fry must decide; Bender will validate whichever path is chosen.

---

## Validation Scenarios for Task 10.9

All scenarios below are destructive: they attempt to break the implementation by
exercising edge cases, boundary conditions, and adversarial inputs. Each has a
pass/fail criterion and the evidence needed for sign-off.

### S1 — Contradiction Round-trip (Ingest → Check)

**What:** Ingest two pages with conflicting assertions, then verify `gbrain check --all`
detects the contradiction.

**Steps:**
1. `gbrain init test.db`
2. Ingest page A with compiled_truth: "Alice works at Acme Corp"
3. Ingest page B with compiled_truth: "Alice works at Beta Corp"
4. `gbrain check --all --json`

**Pass:** JSON output contains a contradiction with `type = 'assertion_conflict'`,
referencing both page slugs. Exit code 0.

**Fail:** No contradiction detected, or contradiction references wrong pages.

### S2 — Same-Page Contradiction

**What:** A single page contains two conflicting assertions.

**Steps:**
1. Put a page with: "Alice works at Acme Corp. Alice works at Beta Corp."
2. `gbrain check --slug people/alice`

**Pass:** Contradiction detected within the same page.

**Fail:** Only cross-page conflicts are caught.

### S3 — Resolved Contradiction Non-Duplication

**What:** A previously resolved contradiction should not be re-inserted.

**Steps:**
1. Create a contradiction via S1.
2. Manually UPDATE the contradiction to `resolved_at = datetime('now')`.
3. Re-run `gbrain check --all`.

**Pass:** No new contradiction row inserted for the same (subject, predicate) pair.

**Fail:** Duplicate contradiction row created.

### S4 — Novelty-Skip Ingest

**What:** Near-duplicate content (≥ 85% Jaccard) is rejected; distinct content passes.

**Steps:**
1. Ingest `note.md` with body: "Alice works at Acme Corp in San Francisco."
2. Re-ingest with body: "Alice works at Acme Corp in San Francisco." (identical)
3. Capture stderr.

**Pass:** Second ingest prints "Skipping ingest: content not novel (slug: <slug>)"
to stderr. No DB write. Exit 0.

**Fail:** Content is written again, or no warning message.

### S5 — Novelty --force Bypass

**What:** `--force` overrides the novelty check.

**Steps:**
1. After S4, re-ingest same content with `--force`.

**Pass:** Write proceeds. No "Skipping" warning.

**Fail:** --force doesn't bypass, or novelty check is still called.

### S6 — First-Time Ingest Skips Novelty

**What:** Brand new page has no prior content to compare against.

**Steps:**
1. Ingest a new file targeting a slug that doesn't exist.

**Pass:** Page created normally. No novelty check invoked. No stderr warning.

**Fail:** Novelty check errors on missing page, or page not created.

### S7 — Graph: Cyclic Graph Terminates

**What:** Two pages link to each other; BFS must not loop.

**Steps:**
1. Create pages A, B.
2. Link A → B, B → A.
3. `gbrain graph a --depth 10 --json`

**Pass:** Returns within reasonable time. Both A and B appear exactly once in nodes.
Edge list has exactly 2 edges. No panic or timeout.

**Fail:** Hang, panic, duplicate nodes, or missing edges.

### S8 — Graph: Zero-Hop Returns Root Only

**Steps:**
1. `gbrain graph people/alice --depth 0 --json`

**Pass:** Exactly one node (alice). Empty edges array.

**Fail:** Extra nodes or non-empty edges.

### S9 — Graph: Temporal Filter Excludes Closed Links

**Steps:**
1. Create link alice → acme with `valid_until = '2020-01-01'`.
2. `gbrain graph people/alice --depth 1 --json` (default active filter)

**Pass:** acme NOT in nodes.

**Fail:** Closed link leaks into active view.

### S10 — Graph: --all Temporal Filter Includes Closed Links

**Steps:**
1. Same setup as S9.
2. `gbrain graph people/alice --depth 1 --temporal all --json`

**Pass:** acme IS in nodes.

**Fail:** acme still excluded.

### S11 — Graph: Non-Existent Slug Errors

**Steps:**
1. `gbrain graph nobody/ghost --depth 1`

**Pass:** stderr: "page not found: nobody/ghost". Non-zero exit code.

**Fail:** Empty output with exit 0, or panic.

### S12 — Graph: Depth > 10 Capped

**Steps:**
1. `gbrain graph people/alice --depth 999 --json`

**Pass:** Depth is internally capped to 10. No stack overflow. Returns valid JSON.

**Fail:** Panic, excessive traversal, or error on depth > 10.

### S13 — Progressive Retrieval: Budget Exhaustion

**Steps:**
1. Create pages with known compiled_truth sizes.
2. `gbrain query "test" --depth auto --token-budget 100`

**Pass:** Expansion stops when cumulative tokens ≥ budget. Result set is bounded.

**Fail:** Expansion exceeds budget, or runs to depth cap despite budget being hit.

### S14 — Progressive Retrieval: Depth Cap

**Steps:**
1. Set very large budget (100000).
2. `gbrain query "test" --depth auto`

**Pass:** Expansion stops at depth 3 regardless of remaining budget.

**Fail:** Expansion beyond 3 hops.

### S15 — Progressive Retrieval: Deduplication

**Steps:**
1. Two initial results both link to page C.

**Pass:** Page C appears exactly once in expanded results.

**Fail:** Duplicate entries in result set.

### S16 — Knowledge Gap Auto-Logging

**Steps:**
1. `gbrain query "xyzzy quantum socks"` (no results expected)

**Pass:** stderr: "Knowledge gap logged." Row inserted into `knowledge_gaps` with
correct query_hash. query_text IS NULL. sensitivity = 'internal'.

**Fail:** No gap logged, or query_text leaked into DB.

### S17 — Knowledge Gap Idempotency

**Steps:**
1. Run same low-result query twice.

**Pass:** Only one row in `knowledge_gaps` for that query_hash.

**Fail:** Two rows (this is the blocker — see schema gap above).

### S18 — Knowledge Gap Resolution

**Steps:**
1. Log a gap.
2. `gbrain gaps` → shows the gap.
3. Resolve the gap via code/API.
4. `gbrain gaps` → no longer shows it.
5. `gbrain gaps --resolved` → shows it.

**Pass:** Resolution lifecycle works. resolved_at and resolved_by_slug populated.

**Fail:** Gap not removed from default view, or missing from --resolved view.

### S19 — MCP: brain_link Unknown Slug → -32001

**Steps:**
1. Call brain_link with `from_slug = "people/ghost"`.

**Pass:** Error response with code -32001, message contains "page not found".

**Fail:** Silent success or wrong error code.

### S20 — MCP: brain_graph Returns Valid Schema

**Steps:**
1. Call brain_graph with valid slug.

**Pass:** JSON has exactly `nodes` (array) and `edges` (array) top-level keys.
Each node has `slug`, `type`, `title`. Each edge has `from`, `to`, `relationship`.

**Fail:** Missing keys, wrong types, or extra unexpected fields.

### S21 — MCP: brain_check Clean Page → Empty Array

**Steps:**
1. Call brain_check on a page with no conflicting assertions.

**Pass:** Returns `[]`.

**Fail:** Returns error or non-empty array.

### S22 — MCP: brain_tags Round-trip

**Steps:**
1. List tags (expect []).
2. Add tag "investor".
3. List tags (expect ["investor"]).
4. Remove tag "investor".
5. List tags (expect []).

**Pass:** Each step returns expected tag list.

**Fail:** Tag not persisted, not removed, or list stale.

### S23 — Phase 1 Round-trip Regression

**Steps:**
1. `cargo test --test roundtrip_raw --test roundtrip_semantic`

**Pass:** All tests pass. SHA-256 hashes unchanged.

**Fail:** Any Phase 1 test failure = automatic Phase 2 rejection.

### S24 — Full Test Suite Gate

**Steps:**
1. `cargo test`
2. `cargo clippy -- -D warnings`
3. `cargo fmt --check`

**Pass:** All tests pass (≥ 200). Zero clippy warnings. Fmt clean.

**Fail:** Any failure = ship gate blocked.

---

## Evidence Required Before Sign-off

Bender will NOT sign off task 10.9 until ALL of the following exist:

| # | Evidence | Source |
|---|----------|--------|
| E1 | S1–S6 pass (contradictions + novelty) | Integration test output or manual demo |
| E2 | S7–S12 pass (graph traversal) | Integration test output |
| E3 | S13–S15 pass (progressive retrieval) | Unit test output |
| E4 | S16–S18 pass (knowledge gaps) | Integration test output |
| E5 | S19–S22 pass (MCP Phase 2 tools) | Unit/integration test output |
| E6 | S23 pass (Phase 1 round-trip regression) | `cargo test` output |
| E7 | S24 pass (full suite + clippy + fmt) | CI or local run output |
| E8 | Schema gap resolved (query_hash UNIQUE) | Code diff showing constraint |
| E9 | `#![allow(dead_code)]` removed from novelty.rs | Code diff |
| E10 | `derive_room` returns non-empty for headed content | Unit test output |

---

## Coordination Notes

- **Fry:** Bender needs the schema gap (E8) resolved before Group 8 validation can begin.
  All other groups can be validated independently.
- **Scruffy:** Unit tests for S7–S12, S13–S15 should already exist from task groups 1, 5.
  Bender will independently verify they pass but won't duplicate them.
- **Professor:** Graph BFS correctness (S7, S8, S12) overlaps with your 10.6 review.
  Bender's focus is on behavioral edge cases, not algorithmic proof.
- **Nibbler:** MCP scenarios S19–S22 overlap with your 10.7 adversarial review.
  Bender checks happy-path + error-path contracts; Nibbler checks abuse vectors.
