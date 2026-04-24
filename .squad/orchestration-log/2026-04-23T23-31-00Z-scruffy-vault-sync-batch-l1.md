# Orchestration Log: Scruffy — Vault-Sync Batch L1 Proof Lane

**Session:** 2026-04-23T23:31:00Z  
**Agent:** Scruffy  
**Change:** vault-sync-engine  
**Lane:** Testing / Proof  

## Summary

Scruffy completed proof lane for Batch L1 (restore-orphan startup recovery narrowed slice).

### Proof Coverage

- **11.1a** — startup-order evidence: serve startup acquires ownership, runs orphan recovery, leaves no supervisor-ack residue
- **17.5ll** — direct tests for shared 15s heartbeat gate, stale-orphan exact-once startup finalize, fresh-heartbeat defer, collection_owners beating stale/foreign serve_sessions
- **17.13** — real `start_serve_runtime()` crash-between-rename-and-Tx-B recovery path (not fixture shortcut)

### Test Scope

- Shared 15-second stale-heartbeat threshold
- Stale-orphan exact-once startup finalize behavior
- Fresh-heartbeat defer (does not finalize, leaves collection blocked)
- `collection_owners` ownership truth vs stale/foreign `serve_sessions`
- Real crash-recovery path with actual state transitions

### Scope Guardrail

Proof lane is intentionally scoped to restore-owned pending-finalize state **only**. These tests do NOT support:
- Generic `needs_full_sync` recovery claims
- Remap startup attach recovery
- Sentinel recovery
- Broader "serve startup heals dirty collections" narrative

## Status

✅ Proof lane complete. All tests pass. Guardrails documented.

## Orchestration Notes

- 11 mandatory proofs from Nibbler pre-gate decision all addressed
- CLI-only surface approved; MCP deferred per Fry decision
- Honest scope boundary maintained
