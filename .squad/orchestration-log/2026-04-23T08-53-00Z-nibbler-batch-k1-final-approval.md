# 2026-04-23T08:53:00Z — Nibbler K1 Final Approval

## Session: Vault-Sync Batch K1 Final Approval

**Author:** Nibbler  
**Date:** 2026-04-23  
**Verdict:** APPROVE  

---

## Why This Gate Clears

The adversarial seams named in pre-gate are now acceptably controlled for the narrowed K1 slice:

1. **Add-time lease ownership / cleanup**
   - `collection add` validates name, root, and `.gbrainignore` before inserting any `collections` row.
   - Fresh attach runs from `state='detached'` through `fresh_attach_collection()` under a short-lived `collection_owners` lease.
   - The lease/session cleanup path is RAII-backed, and the command deletes the newly inserted row if fresh attach fails, avoiding a stranded active-looking stub.

2. **Writable/read-only truth**
   - Capability probe only downgrades on permission/read-only class refusal and aborts on other probe failures.
   - Probe tempfiles are removed on both success and refusal/error paths.
   - `collection info` / `collection list` surface `writable` truthfully rather than implying health beyond persisted state.

3. **Shared refusal paths are honestly scoped**
   - Vault-byte writers (`gbrain put` and MCP `brain_put`) route through `ensure_collection_vault_write_allowed()` and now have direct refusal proof for persisted `writable=0`.
   - Slug-bound `brain_gap` still correctly remains a write-interlocked DB mutation (`ensure_collection_write_allowed()`), not a read-only-gated vault-byte writer.
   - The task ledger now explicitly says DB-only mutators are out of the `CollectionReadOnlyError` claim for K1, which closes the prior overclaim seam.

4. **Task honesty**
   - `tasks.md` keeps `9.2a` and `17.11` deferred and does not pretend K1 certifies offline restore integrity or CLI finalize closure.
   - The repair notes for `1.1b`, `9.2b`, and `17.5qq11` now match the actual code and proof surface.

---

## Required Caveat

This approval covers **only** the narrowed K1 attach/read-only slice: collection add/list truth, validation-before-row-creation, short-lived lease cleanup, truthful `writable=0`, vault-byte refusal for `gbrain put` / `brain_put`, and restoring-gated slug-bound `brain_gap`.

It does **not** certify offline restore integrity, RCRT/CLI finalize end-to-end closure, broader DB-only mutator read-only blocking, or any K2 destructive-path proof.

---

## Gate Status

✅ K1 APPROVED FOR LANDING (narrowed slice: `1.1b`, `1.1c`, `9.2`, `9.2b`, `9.3`, `17.5qq10`, `17.5qq11`)
