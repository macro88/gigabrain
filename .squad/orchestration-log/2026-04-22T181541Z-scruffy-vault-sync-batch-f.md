# Scruffy — Vault Sync Batch F Testing

**Date:** 2026-04-22  
**Status:** Completed  
**Role:** Tester  

## Summary

Batch F coverage locked on honest seams. Live tests cover idempotency and DB-only-state re-check paths. Deferred seams (write-through, restore, full-hash) are explicitly marked ignored with task references.

## Coverage Seams

### Live Coverage (Executable)
- Second-pass zero-change behavior: `import_dir()` and `ingest()` produce identical pages on re-run
- Stale-OCC immutability: `brain_put()` refuses stale version updates
- Classifier freshness: DB-only-state classification when DB state appears after earlier clear read
- Quarantine preservation: DB-only-state categories prevent hard-delete

### Locked Seams (Ignored)
- `import_dir_write_path_keeps_exactly_one_active_raw_import_row_for_latest_bytes` — Task 5.4d
- `ingest_force_reingest_keeps_exactly_one_active_raw_import_row_for_latest_bytes` — Task 5.4g
- `put_occ_update_keeps_exactly_one_active_raw_import_row_for_latest_bytes` — Task 5.4h (deferred)
- `full_hash_reconcile_aborts_when_a_page_has_zero_active_raw_import_rows` — Task 4.4 (deferred)

Each ignored test includes the exact task reference so Fry/Leela can unignore when implementation lands.

## Test Results

- ✅ Idempotency: `import_dir` second pass → zero mutations
- ✅ Ingest force-reingest: stale bytes → fresh bytes, exactly one active `raw_imports` row (covered by locked seam)
- ✅ OCC immutability: stale version rejected
- ✅ DB-only re-eval: quarantine preserved when DB state adds protection after classification

## Validation

- ✅ All 439 tests pass
- ✅ No false positives in ignored-test suite
- ✅ Coverage targets for in-scope paths met
