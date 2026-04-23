# Nibbler — Vault Sync Batch F Adversarial Review

**Date:** 2026-04-22  
**Status:** Approved  
**Role:** Adversarial reviewer  

## Verdict

**APPROVE** — No in-scope path can commit zero active `raw_imports` rows through split transactions, and no apply-time delete path trusts stale DB-only-state classification.

## Controlled Seams

### 1. Raw_imports Rotation Atomicity (✅)
- In-scope raw-import writers (`ingest`, `import_dir`, reconciler apply) call shared rotation helper from same SQLite transaction that mutates `pages` / `file_state`
- Active-row flip is never left stranded outside commit boundaries
- Verified: no split-transaction paths in reviewed code

### 2. Zero-Active Invariant Enforcement (✅)
- Rotation helper refuses to run when a page already has historical `raw_imports` rows but zero active rows
- Fails closed: `InvariantViolationError` instead of silently healing
- Verified: corrupt history detection gates further mutation

### 3. Quarantine vs Hard-Delete Routing (✅)
- Reconciler hard-delete vs quarantine verdict re-evaluated inside apply through fresh DB-only-state query
- A page that gains DB-only state after classification is quarantined, not hard-deleted
- Verified: apply path calls `has_db_only_state()` at apply time, not from stale snapshot

## Edge Cases Tested

- (a) Re-ingest of modified file produces exactly one active row ✅
- (b) GC cap enforcement never deletes newly-inserted active row ✅
- (c) `KEEP_ALL=1` override bypasses GC without touching active row ✅
- (d) Simulated tx rollback after `is_active=0` update leaves prior active row intact ✅

## Deferred Seams Kept Honest

- Restore / `full_hash_reconcile` zero-active handling: still deferred, but code and tasks keep it error-shaped and explicitly unimplemented rather than pretending success ✅
- Later UUID writeback / `brain_put` write-through: deferred and named as such in tasks, not smuggled into this approval ✅

## Remaining Risk Surface

Risk sits in later restore/remap/full-hash and UUID writeback work, and that risk is documented as future work rather than hidden inside Batch F.

## Safe to Merge

No data-loss vector found in in-scope paths. Atomicity seam is explicitly stress-tested and holds.
