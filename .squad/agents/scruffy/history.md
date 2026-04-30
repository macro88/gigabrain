# scruffy history

## 20260429T173541Z — Team sync

**Scribe update:** Decisions merged (inbox → decisions.md), orchestration logs written, Batch 3 merge lane BLOCKED by codecov/patch.

## 20260429T132911Z — Session: PR #122 Blocker Fix

**Status:** COMPLETE
**Commit:** c5b2b0c
**PR:** #122 (spec/vault-sync-engine-batch3-v0120)

**Summary:** Cleared Copilot threads, added coverage backfill, pushed c5b2b0c. PR #122 now waiting only on rerun completion.

**Details:**
- Addressed codecov/patch coverage deficit
- Cleaned up review thread backlog
- Branch ready for policy merge

**Next gate:** GitHub CI/workflow rerun

## Learnings

- 2026-04-30T06:37:20.531+08:00 — Read-only Batch 4 assessment: the honest local coverage loop is `cargo llvm-cov --lib --tests --summary-only --no-clean -j 1` followed by `cargo llvm-cov report --json --output-path target\llvm-cov-report.json`; current baseline is 89.47% total, with `src\core\vault_sync.rs` (83.22% lines) as the main Batch 4 coverage risk.
- 2026-04-30T06:37:20.531+08:00 — Batch 4 closure truth guard: keep `12.6`, `12.6a`, `12.6b`, and `12.7` open until `quaid put` has real live-owner routing tests, and until bulk UUID write surfaces actually exist; current code still rejects `--write-quaid-id` as deferred and has no `migrate-uuids` subcommand.
- [2026-04-30T06:37:20Z] Batch 4 coverage baseline decision merged to team ledger. Fresh llvm-cov run required after implementation lands.
- 2026-04-30T12:07:19.084+08:00 — Batch 5 preflight: the current repo gate is `cargo llvm-cov --lib --tests --summary-only --no-clean -j 1` at 90.95% total (24,789 / 27,257), but `src\core\vault_sync.rs` is still only 83.22% (3,193 / 3,837), so new IPC work must ship with near-complete branch coverage in `vault_sync.rs` and `put.rs` or the lane will fall back under 90%.
- 2026-04-30T12:07:19.084+08:00 — Best Batch 5 test split: keep socket placement, stale-socket cleanup, bind-audit, and server-side peer-refusal proofs as `src\core\vault_sync.rs` unit tests close to `start_serve_runtime`; keep the client-side spoofed-peer / refused-write proof as a subprocess CLI test in `tests\collection_cli_truth.rs`, then re-run llvm-cov summary plus JSON refresh for the truthful gate.

## 2026-04-30T00:30:31Z
- **Action:** Approved Batch 4 completion and v0.13.0 release status
- **Status:** APPROVED

