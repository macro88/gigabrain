# Decisions Archive

### 2026-04-23T18:01:18+08:00: User directive
**By:** macro88 (via Copilot)
**What:** Once the current work is done and pushed remote, start the next session to (1) achieve 90%+ code coverage and 100% pass, (2) update project docs completely, (3) update the public docs to be best in class, (4) get a PR in and merged, (5) release v0.9.6, (6) after merge to main, clean up unused files such as leftover root .md files on a feature branch and open a cleanup PR, and (7) check all issues and close what is fixed or no longer relevant/actionable.
**Why:** User request — captured for team memory


---
leela_id: next-gate-2026-04-25
issue_date: 2026-04-25T03:45:00Z
branch_state: after-commit-43d2117
closed_scope: |
  13.5, 13.6, 9.10/9.11, 17.5aa5, watcher core (6.1-6.4, 7.1-7.4, 7.6)
deferred_scope: |
  watcher overflow/supervision/health/live ignore reload,
  remaining dedup 7.5,
  broader watcher choreography,
  IPC/live routing, online restore handshake,
  destructive restore/remap surfaces (Phase 4, Tx-A),
  bulk UUID mutations (ServeOwnsCollection gates only, no proxy),
  embedding queue full suite,
  legacy ingest removal,
  documentation,
  follow-up stubs (daemon-install, openclaw-skill)
---

# Next Truthful Slice: Dedup Completion + Quarantine Delete Classifier

## Exactly One Next Slice

**Task IDs to close now:**
- `7.5` — Failure handlers: dedup removal after rename failure or post-rename abort
- `17.5g7` — Quarantine export: dump all five DB-only-state categories as JSON
- `17.5h` — Auto-sweep TTL: discard clean quarantined pages, preserve DB-only-state
- `17.5i` — Quarantine discard `--force`: require exported JSON for DB-only-state
- `17.5j` — Quarantine restore: re-ingest and reactivate file_state
- `9.8` — `gbrain collection quarantine {list,restore,discard,export,audit}`
- `9.9` — Auto-sweep TTL: configurable retention, preserve DB-only-state pages
- `9.9b` — `gbrain collection info` surfaces quarantine count

**Test tasks to close:**
- `17.4` — `.gbrainignore` atomic parse unit tests
- `17.5g7` — quarantine export JSON proof
- `17.5h` — auto-sweep TTL discard/preserve proof
- `17.5i` — discard --force with DB-only-state proof
- `17.5j` — quarantine restore re-ingest proof
- `17.5z` — gbrainignore parse failure preserves mirror
- `17.5y` — gbrainignore valid edit triggers reconcile
- `17.5aa` — gbrainignore absent-file semantics

## Files Likely Involved

**Core implementation:**
- `src/core/vault_sync.rs` — dedup remove-on-failure helpers
- `src/core/reconciler.rs` — update failure unlink path
- `src/commands/put.rs` — add sentinel unlink and dedup cleanup on failure
- `src/core/quarantine.rs` — **new file** — export, classify, TTL sweep
- `src/commands/collection.rs` — add `quarantine` subcommand + `info` quarantine count
- `src/mcp/server.rs` — expose quarantine list + stats

**Testing:**
- `tests/quarantine_lifecycle.rs` — **new file** — export/discard/restore round-trip
- `tests/dedup_failure_cleanup.rs` — **new file** — dedup removal after sentinel failure
- `tests/ignore_atomic_parse.rs` — existing, add negative cases

## What Remains Deferred

**Strictly out of scope to keep this honest:**
- Watcher overflow recovery (`needs_full_sync` set, recovery task polling) — `6.7a`
- Watcher health/supervision/restart logic — `6.10`, `6.11`
- Live `.gbrainignore` reload during serve — `6.8`
- Full dedup echo suppression beyond `17.5bb-dd` narrow proof — `7.5` full failure suite
- IPC socket, write routing, proxy mode — `12.6a`-`12.6g`
- Online restore handshake `(session_id, reload_generation)` ack — `17.5pp`-`17.5qq`
- Remap Phase 4 bijection verification — `17.5ii4`-`17.5ii5`
- Embedding job queue + worker — §8 full, `17.5ee`-`17.5gg`
- `migrate-uuids` + `--write-gbrain-id` bulk UUID rewrite — `9.2a`, `12.6b`, `17.5ii9`
- Legacy `gbrain import` removal — `15.1`-`15.4`
- Documentation refresh — §16

## Why This Slice Is Next

1. **Completes the dedup contract** — Watcher core lands dedup on happy path (`17.5bb-dd`), but failure handling (`7.5`) is unfinished. This slice closes the vault-byte write dedup story end-to-end before any broader watcher mutation or recovery machinery runs.

2. **Closes quarantine lifecycle** — Reconciler and restore-recovery both produce quarantined pages; we have no way to export, inspect, discard, or restore them yet. CLI and MCP surfaces are bare. Auto-sweep would silently delete recoverable pages. This slice is the operator-facing quarantine resolution that validates the five-category preserve logic downstream.

3. **Validates has_db_only_state predicate** — Export walks all five categories; discard-force refuses without exported JSON; auto-sweep preserves DB-only-state. This is the first real exercise of the quarantine delete-vs-preserve classifier (`has_db_only_state`) in anger. Bugs found here are cheaper than discovering them during a production restore when data is already at risk.

4. **Low risk, high signal** — No platform gates, no concurrency, no IPC, no new system integration. Pure CLI + SQL + JSON export. Reviewers can reason about it without watcher supervision, overflow recovery, or online handshake machinery in flight.

5. **Truthful stopping point** — Stops before watcher overflow (`6.7a`), live ignore reload (`6.8`), and broader watcher choreography. None of those can ship until dedup and quarantine are proven; this slice makes that boundary explicit.

## Reviewer-Friendly Story

The slice is: _Operator gets to inspect, export, and recover quarantined pages without data loss. Auto-sweep respects the five DB-only-state categories so recovery-worthy pages never disappear on TTL. Dedup failure path cleans up its own state so no phantom entries block reconciliation._

## GitHub Issues

None directly cited in the spec, but this enables:
- Safe operator recovery from partial restore failures
- Validation of the quarantine preserve logic that protects recovery-linked data
- Honest dedup failure semantics instead of leaving orphaned entries

## Architecture Alignment

- **Fry's lane** (implementation): dedup cleanup + quarantine export/discard/restore + auto-sweep TTL
- **Professor's lane** (review): five-category delete-vs-quarantine logic, operator guidance docs
- **Nibbler's lane** (test): quarantine lifecycle round-trip + failure cleanup edge cases
- **Scruffy's lane** (test): auto-sweep TTL with DB-only-state preservation + missing-page dedup cleanup

This slice lands in exactly the sequence Fry can execute after watcher core without waiting on online handshake or IPC machinery.



# Decision: next slice after watcher core (commit 43d2117): brain_put rename-before-commit seam

**Author:** Leela  
**Date:** 2026-04-25T03:45:00Z  
**Status:** Recommendation — requires fresh Professor + Nibbler pre-gate before implementation

---

## Current state (post-watcher-core)

- **Completed:** Watcher core slice (tasks `6.1–6.4`, `7.1–7.4`, `7.6`) landed as commit 43d2117.
  - Per-collection notify watcher + bounded queue + 1.5s debounce
  - Reconcile-backed flushes + path+hash dedup set with 5s TTL
