# Session Log — Vault-Sync Foundation Repair Pass

**Session ID:** vault-sync-foundation-repair  
**Timestamp:** 2026-04-22T17:37:51Z  
**Agent:** Leela  
**Requester:** macro88  

---

## Problem Statement

The vault-sync-engine foundation slice (tasks 1.1–2.6) was rejected by Professor for:

1. Schema coherence — 181 failing tests due to `NOT NULL` constraints on `collection_id` and `uuid` without updating legacy INSERT helpers
2. Incomplete schema — `serve_sessions` and `collection_owners` marked complete but not defined
3. Quarantine filtering gaps — vector search missing `quarantined_at IS NULL` filter
4. Task truthfulness — status checkmarks don't match implementation reality

---

## Approach

1. **Schema v5 integration:** Restore backward compatibility by adding DEFAULT values and compatibility shims
2. **Legacy write paths:** Update all INSERT sites to match new unique constraint
3. **Quarantine filtering:** Wire `quarantined_at` filter through all search surfaces
4. **Test validation:** Verify all 181 failures are resolved and full test suite passes
5. **Task documentation:** Update `tasks.md` to reflect actual completion state

---

## Changes Made

### 1. Schema Compatibility (`src/schema.sql`, `src/core/db.rs`)

- `pages.collection_id` now has `DEFAULT 1`
- `pages.uuid` changed from `NOT NULL` to `DEFAULT NULL` with partial unique index
- `ingest_log` retained as compatibility shim
- New `ensure_default_collection()` function auto-creates default collection

### 2. Write Paths (`ingest.rs`, `migrate.rs`)

- Updated all `ON CONFLICT` clauses to target `(collection_id, slug)`
- Verified all legacy INSERT statements work with default collection FK

### 3. Search Paths (`inference.rs`)

- Added `AND p.quarantined_at IS NULL` to `search_vec` query
- Aligned vector search quarantine handling with FTS5 behavior

### 4. Schema Version Gating (`db.rs`)

- Fixed `open_connection()` to validate legacy version before v5 schema execution
- Pre-`brain_config` databases now properly rejected

### 5. Task Documentation (`tasks.md`)

- Updated completion status to reflect actual code state
- Honest marking of 1.1, 1.6, 2.6 as pending related wiring

---

## Test Results

| Phase | Result |
|-------|--------|
| Before repair | **181 failures** in `commands::check`, `core::fts`, `core::inference` |
| After repair | **0 failures** — `cargo test --quiet` passes full suite |

---

## Verification

- [x] All legacy write helpers work with new schema
- [x] Default collection auto-created on every open
- [x] Vector search now filters quarantined pages
- [x] Full test suite passes
- [x] Schema version gating protects legacy databases
- [x] Task documentation updated for truthfulness

---

## Decision Artifacts Merged

Five decisions merged into canonical `decisions.md`:

1. `pages.collection_id DEFAULT 1` + auto-created default collection
2. `pages.uuid` becomes nullable until UUID lifecycle tasks
3. `ingest_log` retained as compatibility shim
4. `ON CONFLICT(collection_id, slug)` for unique constraint
5. `search_vec` gains quarantine filter

---

## Cross-Agent Impact

- **Professor:** Foundation review validated; schema gating now safe for legacy databases
- **Scruffy:** Test coverage now credible; no longer blocked by 181 failures
- **Fry:** Watcher/reconciler implementation can now proceed on solid foundation

---

## Next Steps

- Follow-on implementation batches unblocked
- Watcher slice can reference updated foundation
- UUID lifecycle tasks can be scheduled for later phase
- Collection ownership completion deferred to section 5a

