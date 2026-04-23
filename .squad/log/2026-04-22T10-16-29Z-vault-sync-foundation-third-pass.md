# Session Log — Vault-Sync Foundation Third-Pass Review

**Session ID:** vault-sync-foundation-third-pass  
**Timestamp:** 2026-04-22T10:16:29Z  
**Agent:** Professor  
**Requester:** macro88

---

## Problem Statement

The vault-sync-engine foundation slice (tasks 1.1–2.6) was repaired by Leela for:
1. Schema coherence (181 test failures resolved)
2. Legacy write path compatibility (DEFAULT values and shim tables)
3. Quarantine filter wiring (vector search aligned with FTS5)
4. Task documentation (truthfulness updated)

Professor conducted third-pass review to validate repairs against foundational consistency standards before the slice can be merged.

---

## Approach

1. **Artifact cross-validation:** Re-read proposal/design text against actual code to check for overstated removals or misaligned descriptions
2. **Safety preflight audit:** Trace schema version checking in `db.rs` to verify gating happens before v5 mutations
3. **Coverage depth assessment:** Identify new branchy code seams and check for direct unit-test coverage
4. **Review feedback:** Issue formal repair decision with gate conditions and required changes

---

## Changes Validated

### 1. Schema Compatibility Repairs (`src/schema.sql`, `src/core/db.rs`)

- ✅ `pages.collection_id DEFAULT 1` with `ensure_default_collection()` called at every open
- ✅ `pages.uuid` changed to nullable with partial unique index
- ✅ `ingest_log` retained as compatibility shim
- ✅ All legacy INSERT statements work without modification

**Finding:** Compatibility shim is still in place; proposal/design text must be updated to reflect this.

### 2. Write Path Repairs (`ingest.rs`, `migrate.rs`)

- ✅ All `ON CONFLICT` clauses target `(collection_id, slug)`
- ✅ Legacy single-slug upserts updated to match new constraint
- ✅ All tests now pass (0 failures from prior 181)

**Finding:** Repairs are sound; legacy compatibility is maintained.

### 3. Search Path Repairs (`inference.rs`)

- ✅ `search_vec` now includes `AND p.quarantined_at IS NULL`
- ✅ Vector search aligned with FTS5 quarantine behavior
- ✅ Query logic correct

**Finding:** Implementation is correct; regression test coverage is missing.

### 4. Schema Version Gating (`db.rs`)

- ⚠️ `open_connection()` executes v5 schema before checking legacy version
- ⚠️ Pre-v5 databases can be partially mutated before re-init refusal

**Finding:** Preflight safety not yet implemented; reordering required.

### 5. Task Documentation (`tasks.md`)

- ✅ Task statuses updated to reflect actual completion state
- ✅ Deferred tasks explicitly marked (UUID lifecycle, watcher slice)

**Finding:** Documentation is truthful; proposal/design must align.

---

## Test Results

| Phase | Result |
|-------|--------|
| Before Leela repair | **181 failures** in test suite |
| After Leela repair | **0 failures** — `cargo test --quiet` passes |
| Gate validation | **3 conditions** for landing: artifact truthfulness, preflight safety, coverage depth |

---

## Review Findings

### Finding 1: Artifact Truthfulness Gap

**Issue:** Proposal and design documents describe `gbrain import` and `ingest_log` as removed, but implementation retains both as temporary compatibility shims.

**Impact:** Review cannot approve without artifact alignment.

**Resolution:** Two paths:
- Path A: Update proposal/design to explicitly state compatibility shim is intentional and temporary (recommended)
- Path B: Remove compatibility shims now and migrate commands

### Finding 2: Legacy-Open Safety

**Issue:** `open_with_model()` → `open_connection()` → v5 schema DDL execution → version check. Pre-v5 databases can be partially mutated before error is returned.

**Impact:** Unsafe state change for legacy databases.

**Resolution:** Reorder `open_connection()` to check version before ANY v5 DDL.

### Finding 3: Coverage Depth Gaps

**Issue:** Three new branchy seams lack direct regression tests:
1. `collections::parse_slug()` collection routing matrix
2. `search_vec()` quarantine filtering
3. `db::open_with_model()` schema refusal

**Impact:** New logic paths are untested; regression risk high.

**Resolution:** Add three focused test modules with direct coverage for each seam.

---

## Verification

- [x] Repairs resolve 181 prior test failures
- [x] Legacy write helpers work with new schema
- [x] Quarantine filtering is wired (FTS5 + vector search aligned)
- [x] Task documentation is truthful
- [x] Three gates identified for landing approval
- ⚠️ Proposal/design truthfulness — GATE 1
- ⚠️ Legacy-open safety reordering — GATE 2
- ⚠️ Coverage depth for new seams — GATE 3

---

## Cross-Agent Impact

- **Leela:** Led repair pass; now owns addressing review gates 1–3
- **Scruffy:** Test coverage now credible (181 failures gone); coverage depth gates clarify missing direct tests
- **Fry:** Foundation is unblocked after gates pass; can proceed with section B (command wiring)

---

## Decision Artifacts Merged

One decision issued to canonical `decisions.md`:

**Vault-Sync Foundation Review Gating — Three-Gate Policy**

Future vault-sync review passes must validate:
1. Proposal/design accurately describe implementation state (no overstated removals)
2. Schema version checks happen BEFORE any v5 DDL side effects (preflight safety)
3. New branchy code seams are directly tested (collection routing, quarantine filtering, schema refusal)

---

## Next Steps

- Leela addresses gates 1–3 (artifact alignment, preflight safety reordering, coverage depth)
- Fry stands by for section B command wiring after foundation lands
- Scruffy awaits coverage depth completion before coverage-based approval
- Professor conducts final review pass once repairs land

