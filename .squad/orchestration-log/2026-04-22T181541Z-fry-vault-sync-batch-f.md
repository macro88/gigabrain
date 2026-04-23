# Fry — Vault Sync Batch F Implementation

**Date:** 2026-04-22  
**Status:** Completed  
**Role:** Implementation author  

## Summary

Implemented Batch F apply pipeline: shared `core::raw_imports` rotation helper, inline GC, and full apply wiring. All in-scope paths (single-file ingest, directory import, reconciler apply) now call the shared rotation primitive inside the same SQLite transaction as page/file_state mutation.

## Key Deliverables

- ✅ `core::raw_imports::rotate_active_raw_import()` — atomic rotation helper with inline GC (KEEP cap + TTL)
- ✅ Idempotency seam: second import pass produces zero mutations
- ✅ Apply pipeline: stat-diff → rename resolution → classifier → apply (ingest, hard-delete, quarantine, hash-renames) → enqueue
- ✅ Batch commit: explicit 500-action transactions
- ✅ Per-phase log line: walked, unchanged, modified, new, missing, renamed (native/hash), quarantined (ambiguous/db_state), hard_deleted
- ✅ Quarantine protection: DB-only-state re-eval at apply time (not stale snapshot)
- ✅ Zero-active invariant: fails fast with `InvariantViolationError`

## Tests Locked

- `import_dir_write_path_keeps_exactly_one_active_raw_import_row_for_latest_bytes` — ignored, task 5.4d
- `ingest_force_reingest_keeps_exactly_one_active_raw_import_row_for_latest_bytes` — ignored, task 5.4g
- `put_occ_update_keeps_exactly_one_active_raw_import_row_for_latest_bytes` — ignored, task 5.4h (deferred to write-through)
- `full_hash_reconcile_aborts_when_a_page_has_zero_active_raw_import_rows` — ignored, task 4.4 (deferred)

## Deferred to Batch G+

- `brain_put` / UUID write-back raw_imports hookup
- `restore` / `full_hash_reconcile` zero-active enforcement
- Daily background sweep (requires serve infrastructure)

## Validation

- ✅ `cargo test --quiet` — all tests pass; locked seams explicitly ignored
- ✅ `cargo clippy -- -D warnings` — clean
- ✅ Default model validation
- ✅ Online-model validation

## Decision Link

Fry Decision Note — Vault Sync Batch F (shared raw_imports rotation contract).
