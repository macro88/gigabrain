# 2026-04-23T08:52:00Z — Professor K1 Final Approval

## Session: Vault-Sync Batch K1 Final Approval

**Author:** Professor  
**Date:** 2026-04-23  
**Verdict:** APPROVE  

---

## Why This Slice Clears

K1 stays inside the approved boundary. `collection add` validates root/name/ignore state before row creation, persists a detached row, routes fresh attach through the `FreshAttach` + `AttachCommand` seam, and clears the short-lived lease/session residue on success, failure, and panic-tested unwind. `collection list` and `collection info` surface the promised K1 truth, and the capability probe downgrades permission-denied roots to `writable=0` without leaving probe residue.

The read-only gate is now honestly scoped. `CollectionReadOnlyError` is shared only for K1 vault-byte writers (`gbrain put` / MCP `brain_put`), while slug-bound `brain_gap` and other DB-only mutators still use the restoring / `needs_full_sync` interlock instead of falsely claiming full read-only coverage. `brain_gap` now returns `page_id` in the MCP response, so `1.1b`, `1.1c`, `9.2`, `9.2b`, `9.3`, `17.5qq10`, and `17.5qq11` are supportable from code and tests in-tree.

## Required Caveat

Keep K1 described as **default attach + list/info truth + vault-byte refusal only**. `--write-gbrain-id`, broader collection-root mutators, and offline restore-integrity closure remain deferred to later batches, and the Windows `online-model` lane is still blocked by the known pre-existing dependency compilation crash rather than K1 behavior.

---

## Gate Status

✅ K1 APPROVED FOR LANDING (narrowed slice: `1.1b`, `1.1c`, `9.2`, `9.2b`, `9.3`, `17.5qq10`, `17.5qq11`)
