# Professor — Vault Sync Batch F Review

**Date:** 2026-04-22  
**Status:** Approved  
**Role:** Primary reviewer  

## Verdict

**APPROVE** — Batch F is ready to land as the apply-pipeline slice of `vault-sync-engine`.

## Key Findings

### 1. Shared raw_imports Rotation (✅)
- `core::raw_imports::rotate_active_raw_import()` now sits behind a single entry point
- Used by all in-scope content-changing paths: `ingest`, `import_dir`, reconciler apply
- Page/file_state mutation, raw_imports rotation, and embedding enqueue in one SQLite transaction ✅

### 2. Active-Row Invariant (✅)
- Fails explicitly on corrupt history (zero active rows with historical rows present)
- No silent repair — `InvariantViolationError` on first encounter
- Protects restore/remap/audit operations from unrecoverable states

### 3. Quarantine vs Hard-Delete (✅)
- Apply re-evaluates `has_db_only_state()` via fresh DB queries over all five categories
- Execution does not trust stale classification snapshot
- Protects pages that gain DB-only state after initial classification

### 4. Batch Commit Discipline (✅)
- Explicit 500-action transactions
- Regression coverage for partial progress on later-chunk failure
- Raw_imports rotation runs per-file within each batch transaction

### 5. Honest Deferred Work (✅)
- Restore/full-hash zero-active enforcement named as deferred, not hidden
- `brain_put` write-through surfaces named as deferred
- tasks.md documents when later implementations must unignore locked seams

## Deferred Surfaces (OK — Documented)

- `full_hash_reconcile` ← only needed by restore, remap, audit (none in Batch F)
- Restore caller hookup ← depends on 4.4
- UUID write-back / `brain_put` write-through ← depends on rename-before-commit landing first
- Daily background sweep ← requires serve infrastructure

## Test Surface

- ✅ All 439 tests pass
- ✅ Locked seams (4 tests) explicitly ignored with task references
- ✅ Live coverage on idempotency, OCC immutability, classifier freshness, DB-only-state re-eval
- ✅ Default and online-model validation green

## Mergeable

This slice is coherent and does not carry hidden risk. The raw_imports invariant is enforced, quarantine decisions are re-evaluated at apply time, and deferred work is named rather than smuggled in.
