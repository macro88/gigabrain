### 2026-04-28: Professor Batch 1 watcher-reliability pre-gate — REJECT current closure

**By:** Professor  
**What:** Rejected Batch 1 watcher-reliability closure plan as written due to three blocking contradictions.  
**Why:** Overflow recovery authorization contract must reuse existing `ActiveLease`, not bypass it; `memory_collections` frozen 13-field schema cannot widen without explicit 13.6 reopen; `WatcherMode` semantics contradictory with unreachable `"inactive"` variant.

**Decisions:**
- **D-B1-1:** Overflow recovery operation mode (`OverflowRecovery`) is acceptable as a `FullHashReconcileMode` label, but authorization must remain `FullHashReconcileAuthorization::ActiveLease { lease_session_id }`. No new authorization variant exists.
- **D-B1-2:** `memory_collections` 13.6 frozen 13-field schema must not widen under Batch 1. Watcher health can expand CLI `quaid collection info` only. MCP widening deferred pending explicit 13.6 reopen with design + test updates.
- **D-B1-3:** `WatcherMode` must be truthfully defined: either `Native | Poll | Crashed` only with `null` for non-active/Windows, or `"inactive"` is a real surfaced state with precise definition. No ambiguous mixed contract accepted.

**Verdict:** REJECT Batch 1 closure. Awaiting scope repair. Batch 1 not honestly closable; v0.10.0 not shippable until resolved.

**Result:** Rejection recorded. Leela repair in progress.

---

### 2026-04-29T21:29:11.071+08:00: User directive
**By:** macro88 (via Copilot)
**What:** Start implementation branches from main/origin-main, not from an existing release or dirty branch.
**Why:** User request — captured for team memory


# Fry Batch 3 implementation

- Date: 2026-04-29T20:33:01.970+08:00
- Decision: Keep `page_uuid` dual-read (`quaid_id` first, legacy `memory_id` fallback) but canonicalize every render/write/export to `quaid_id`.
- Why: Batch 3 needs an explicit migration target for files missing `quaid_id`, while existing vault content and fixtures can still be ingested safely during the transition. Reusing `put_from_string` for write-back preserves the rename-before-commit and raw_import invariants without maintaining a second file writer.


# Fry Batch 3 Recon

- **Timestamp:** 2026-04-29T20:33:01.970+08:00
- **Change:** `vault-sync-engine`
- **Scope:** Batch 3 reconnaissance and execution order for `v0.12.0`

## Decision

Implement Batch 3 in this order:

1. Settle the UUID/frontmatter contract seam first (`memory_id` vs `quaid_id`) and thread that decision through the shared UUID helpers.
2. Extract or reuse the existing rename-before-commit writer path so UUID write-back uses the same sentinel + dedup + post-rename stat + single-tx `file_state`/`raw_imports` rotation as `memory_put`.
3. Add bulk-write guard helpers for WriteAdmin/offline-live-owner checks before exposing new CLI entrypoints.
4. Only then wire `collection migrate-uuids` and `collection add --write-quaid-id`, followed immediately by task-aligned tests and OpenSpec checkbox updates.

## Why

The current tree has three coupled seams: the frontmatter key name is still `memory_id`, the only production rename-before-commit implementation lives in `src\commands\put.rs`, and live-owner refusal currently reports only `owner_session_id` even though pid/host data already exists in `serve_sessions`. Landing CLI surface changes before those seams are resolved would either duplicate the write path, produce dishonest task closures, or force follow-up churn across tests and error text.

This ordering also keeps task checkboxes honest. `5a.5`/`17.5ww*` become true when the shared writer path is real, `17.5ii9` becomes true when the live-owner helper is wired, and `5a.5a`/`9.2a` only close once the CLI commands are actually surfaced on top of those lower-level guarantees.

## Notes

- Existing write-gate semantics already live in `src\core\vault_sync.rs::ensure_collection_write_allowed`; Batch 3 should preserve the restoring/`needs_full_sync` fail-closed behavior for WriteAdmin flows.
- The reconciler preflight already points operators at `quaid collection migrate-uuids`, so the new CLI should be added in the same lane as the real write-back implementation, not earlier.


# Leela Batch 3 lane

- **Timestamp:** 2026-04-29T20:33:01.970+08:00
- **Change:** `vault-sync-engine`
- **Scope:** Safe execution lane + release sequencing for Batch 3 / `v0.12.0`

## Decision

Use a separate clean worktree for Batch 3 implementation and release prep staging.

- **Keep untouched:** `D:\repos\quaid` on `release/v0.11.0` (dirty `.squad/` state, ahead of origin by 1 commit)
- **Implementation lane:** `D:\repos\quaid-vault-sync-batch3-v0120`
- **Branch:** `spec/vault-sync-engine-batch3-v0120`
- **Base:** `origin/main` at `fdc20a0` (`Cargo.toml` = `0.11.6`, latest published release = `v0.11.6`)

## Why

The current checkout is not safe for Batch 3 or release work: it is on an old release branch, has uncommitted `.squad/` changes, and diverges from `origin/main`. A sibling worktree isolates implementation from that state and lines Batch 3 up with the real release base.

## Release sequencing constraints

1. Batch 3 is still open in OpenSpec (`5a.5`, `5a.5a`, `9.2a`, `5a.7`, `17.5ww`, `17.5ww2`, `17.5ww3`, `17.5ii9`), so `v0.12.0` cannot ship yet.
2. The next release must not start from `release/v0.11.0`; it must start from the clean main-based lane and merge back to `main` first, per the user instruction.
3. `Cargo.toml` on main is still `0.11.6`; a `v0.12.0` tag would require a version bump before tagging.
4. Release automation is tag-driven (`.github\workflows\release.yml`) and fails if the tag/version mismatch or the 17-file release manifest is incomplete.
5. Coverage over 90% must be checked explicitly via `cargo llvm-cov`; CI reports coverage but does not fail the build for dipping below the requested threshold.

## Routing

- **Fry first:** implement Batch 3 in the clean worktree using the existing recon order (UUID/frontmatter seam → shared write path → live-owner guard → CLI surfaces/tests/OpenSpec checkboxes).
- **Professor second:** review `src\core\vault_sync.rs`, `src\core\page_uuid.rs`, and any shared writer extraction before merge.
- **Scruffy third:** run targeted test/coverage passes and verify the coverage report stays above 90%.
- **Nibbler fourth:** adversarial review of serve-live refusal, write-gate enforcement, and rename-before-commit safety.
- **Zapp last:** only after PR review is complete, comments are resolved, CI is green, and the branch is merged to `main`; then run the release checklist and tag/release `v0.12.0` from the merged state.


# Leela Batch 3 branch ancestry

- **Timestamp:** 2026-04-29T21:29:11.071+08:00
- **Change:** `vault-sync-engine`
- **Scope:** Batch 3 branch ancestry and conflict-risk only

## Decision

No branch-base recovery is needed.

The Batch 3 worktree branch `spec/vault-sync-engine-batch3-v0120` was created from `origin/main` at `fdc20a0` and then received one Batch 3 commit (`4401ed7`). It was not started from `origin/release/v0.11.0`.

## Why

- The branch reflog records the branch creation point as `origin/main`.
- `HEAD` is a single-child commit on top of `fdc20a0`, which is `origin/main` / `v0.11.6`.
- Relative to `origin/release/v0.11.0`, the branch is three commits ahead because it includes the newer main-line commits plus the Batch 3 commit; that is expected ancestry, not evidence of a wrong starting branch.

## Recovery action

Do not rebase, cherry-pick, or rebuild the branch for base-branch reasons. If merge conflicts appear later, treat them as normal forward-integration conflicts from subsequent changes, not as fallout from starting Batch 3 on the wrong base.


# Nibbler Batch 3 review

- **Date:** 2026-04-29T20:33:01.970+08:00
- **Requested by:** macro88
- **Verdict:** REJECT

## Decision

Batch 3 safety is not acceptable to ship.

## Blocking findings

1. `collection add --write-quaid-id` does not truly refuse live serve ownership for the same vault root. The guard is keyed to the newly created `collection_id`, while `collections.root_path` is not unique and `add()` only rejects duplicate names. A second collection row can point at the same canonical root and run bulk UUID rewrites while serve still owns the original row.
2. The bulk UUID rewrite path does not hold an offline owner lease for the duration of the batch. `run_uuid_write_back()` only performs a one-time `ensure_no_live_serve_owner()` preflight, and `collection add --write-quaid-id` drops the fresh-attach short-lived lease before starting the rewrite. A serve session can claim ownership after preflight and race the rewrite mid-batch.
3. The completion claims overstate proof. The landed tests cover `migrate-uuids` live-owner refusal for an existing collection, dry-run, and permission skip, but they do not prove `collection add --write-quaid-id` refusal, the same-root alias case, or the missing lease/race seam.

## Rejected artifacts

- `D:\repos\quaid-vault-sync-batch3-v0120\src\commands\collection.rs`
- `D:\repos\quaid-vault-sync-batch3-v0120\src\core\vault_sync.rs`
- `D:\repos\quaid-vault-sync-batch3-v0120\tests\collection_cli_truth.rs`
- `D:\repos\quaid-vault-sync-batch3-v0120\openspec\changes\vault-sync-engine\tasks.md` (the checked closure claims for `5a.5a`, `9.2a`, `12.6b`, `17.5ii9`)


# Professor Batch 3 Review

**Date:** 2026-04-29T20:33:01.970+08:00
**Reviewer:** Professor
**Verdict:** REJECT

## Blocking findings

1. `src\commands\collection.rs` / `src\core\vault_sync.rs`
   - Batch 3 closes `12.6b` and `17.5ii9` in `openspec\changes\vault-sync-engine\tasks.md`, and the implementation plan says the refusal must name pid/host **and instruct the operator to stop serve first**.
   - The landed `ServeOwnsCollectionError` now includes pid/host, but neither the error text nor the CLI handler adds the required operator guidance. Current tests only assert the tag plus pid/host, so the claimed task closure is not truthful.

## Non-blocking notes

- The shared rename-before-commit seam reuse is honest: `write_quaid_id_to_file(...)` delegates to `put::put_from_string(...)`, so UUID write-back rides the existing sentinel/tempfile/rename/fsync/post-rename-stat/single-tx path instead of introducing a parallel writer.
- The frozen `brain_collections` MCP contract stays closed: `failing_jobs` remains skipped from serialization and the exact-key test still enforces the existing field set.


## 2026-04-29T20:33:01.970+08:00 — Batch 3 coverage lane split

- Keep the Batch 3 proof on the real seams:
  - `src/core/vault_sync.rs` owns atomic UUID write-back, read-only skip, `file_state`/`raw_imports` rotation, and live-owner refusal helpers.
  - `src/commands/collection.rs` owns `collection add --write-quaid-id` / `collection migrate-uuids --dry-run` routing, restoring-state/write-gate checks, and summary shaping.
  - `tests/collection_cli_truth.rs` owns subprocess truth: exit codes, JSON summary, plain-text operator guidance, and serve-live refusal wording.
- Treat `tests/command_surface_coverage.rs` as a last-mile dispatch smoke only; do not spend Batch 3 effort there until the real helper and CLI truth seams are locked.
- Windows iteration should stay cheap: targeted tests first, then `cargo llvm-cov --lib --tests --summary-only --no-clean -j 1`, then `cargo llvm-cov report --json --output-path target\llvm-cov-report.json` for missed-line movement.



# Mom Batch 3 Revision

## Mom Batch 3 Revision

- **Date:** 2026-04-29T21:29:11.071+08:00
- **Decision:** Treat bulk UUID rewrite ownership as a canonical-root seam, not a single-row seam.

### Why

`collection_owners` is keyed by `collection_id`, but `collections.root_path` is not unique. That means `collection add --write-quaid-id` can create an alias row that points at the same vault root while serve still owns a different row, and a row-scoped preflight/lease is not enough to keep the watcher out.

### Applied rule

1. Before `collection add --write-quaid-id` inserts the alias row, preflight the canonical root and fail closed if any same-root row has a live serve owner.
2. For non-dry-run bulk UUID rewrites, acquire one short-lived offline session across **all** collection rows sharing the canonical root and hold it for the entire rewrite loop.
3. Keep the operator-facing refusal honest: tell them to stop serve first, run the bulk rewrite offline, then restart serve.

### Scope

This is intentionally narrow to bulk UUID rewrites (`migrate-uuids` and `collection add --write-quaid-id`). It does not widen generic duplicate-root policy or imply that all collection commands are now root-unique.



# Nibbler Batch 3 rereview

# Nibbler Batch 3 rereview

- **Timestamp:** 2026-04-29T21:29:11.071+08:00
- **Worktree:** `D:\repos\quaid-vault-sync-batch3-v0120`
- **Branch/Commit:** `spec/vault-sync-engine-batch3-v0120` @ `67f4091`
- **Verdict:** **APPROVE**

## Why

1. The same-root alias bypass is closed in both directions:
   - `collection add --write-quaid-id` now refuses before inserting a second row when any same-root alias is live-owned.
   - Bulk UUID rewrite refusal now resolves live ownership by canonical root, not only the target collection row.
2. The offline race is closed at the right seam:
   - non-dry-run UUID write-back acquires a short-lived owner lease covering every same-root collection row before the rewrite loop starts;
   - helper coverage proves the root-scoped lease claims aliases together and cleans up after drop.
3. The operator-facing story is now honest:
   - refusal text explicitly tells operators to stop serve first, rerun offline, then restart serve;
   - task closure notes were narrowed to the actual proof: same-root alias refusal plus a root-scoped lease/source-invariant seam, not a broader claim.

## Residual non-blocking risks

- The end-to-end refusal tests remain Unix-gated, so on a Windows host the rerun evidence comes from helper/unit proof rather than executing the CLI path directly. That matches the current Unix-only command surface, but it is still narrower evidence than a native Unix validation lane.



# Professor Batch 3 re-review

# Professor Batch 3 re-review

- **Date:** 2026-04-29T21:29:11.071+08:00
- **Requested by:** macro88
- **Verdict:** APPROVE
- **Revision reviewed:** 67f4091 on spec/vault-sync-engine-batch3-v0120

## Decision

The revised Batch 3 implementation now honestly closes the prior rejection findings.

## What changed enough to pass

1. collection add --write-quaid-id now refuses before inserting an alias row when any same-root collection is live-owned. The refusal is root-scoped rather than keyed only to the newly created row.
2. Non-dry-run bulk UUID rewrite now acquires a short-lived owner lease across every collection row sharing the canonical root before the rewrite loop begins, so serve cannot claim an alias mid-batch.
3. Operator-facing refusal text now includes the required stop serve first guidance, and the tests/proof seam were updated to cover that wording and the root-scoped lease ordering.
4. openspec/changes/vault-sync-engine/tasks.md no longer overclaims the repaired seam: the revised notes explicitly tie closure to same-root alias refusal, root-scoped lease coverage, and the stop-serve guidance.

## Non-blocking follow-ups

- None.






---
---
timestamp: 2026-04-29T21:29:11.071+08:00
requested_by: macro88
worktree: D:\repos\quaid-v0.12.0-release
branch: release/v0.12.0
head: 90f888ab48fd7e36869b84757a04c5abecffa8ef
topic: v0.12.0 docs/release truth review
---

# Decision: APPROVE `release/v0.12.0` docs truth

## Verdict

APPROVE

## Why

1. `Cargo.toml` is bumped to `0.12.0`, and the public install surfaces now treat `v0.12.0` as branch-prep state rather than pretending the tag is already published.
2. `README.md`, `docs/getting-started.md`, `docs/roadmap.md`, and `website/src/content/docs/tutorials/install.mdx` now truthfully describe the shipped Batch 3 UUID slice: opt-in `quaid collection add --write-quaid-id`, offline `quaid collection migrate-uuids [--dry-run]`, UUID-migration preflight before restore/remap, and `memory_put` preserving `quaid_id`.
3. The docs match the implementation boundary: bulk UUID rewrites are Unix-only and offline, while preserved-UUID behavior is covered on the write/read path.

## Blocking findings

None.

## Non-blocking polish

- Optional: mirror the getting-started page's explicit "Unix-only bulk rewrite" caveat into the README Batch 3 mention so every top-level surface carries the same constraint wording.

---
# Leela decision — v0.12.0 merge lane

- **Timestamp:** 2026-04-29T21:29:11.071+08:00
- **Requested by:** macro88
- **PR:** `#123`
- **Scope:** `release/v0.12.0` final merge lane

## Decision

Clear only the real merge blockers inside the release branch, then merge normally. That meant fixing the flaky env-var test race, adding coverage for the env-guard restore path so `codecov/patch` cleared, accepting the docs correction raised in review, resolving the review threads, and explicitly avoiding an admin merge.

## Why

- The branch itself was already the intended release-prep lane and was only blocked by merge policy.
- The failing `Test` / `codecov/patch` gate and the unresolved review conversations were all scoped to the branch and could be repaired surgically without reopening release scope.
- Admin merge would have hidden a real quality gate failure and violated the no-bypass rule already established for merge-lane work.

## Outcome

- PR `#123` merged cleanly into `main`.
- The exact `main` SHA to tag for `v0.12.0` is `5a8bdf068bf54be52f9b2bc661af34056473221a`.




# Fry Batch 4 gap audit

- **Date:** 2026-04-30T06:37:20.531+08:00
- **Change:** `vault-sync-engine`
- **Scope:** Read-only Batch 4 audit for tasks `12.1`, `12.6`, `12.6a`, `12.6b`, `12.7`

## Decision

Do **not** start Batch 4 implementation on this branch state yet. The rename-before-commit core is close, but `12.6b` is blocked by missing Batch 3 UUID-write surfaces, and the remaining `12.1` gap is still a real source-seam issue rather than a checkbox cleanup.

## Guardrails

1. Keep the Unix platform gate narrow; do **not** widen Windows vault-write support as part of this slice.
2. Keep `memory_collections` on the frozen 13-field MCP schema; no Batch 4 work should add fields there.

## Task 12.1 — full 13-step rename-before-commit

### Already implemented

- Shared writer core exists and is used by both CLI and MCP through `src\commands\put.rs::put_from_string(...)` and `persist_with_vault_write(...)` (`src\commands\put.rs:100-191`, `342-623`).
- The current writer already covers most of the design sequence:
  - step 1 CAS / write gate: `resolve_slug_for_op`, `ensure_collection_vault_write_allowed`, `check_update_expected_version` (`src\commands\put.rs:109-117`, `376-381`; `src\core\vault_sync.rs:556-577`)
  - step 3 precondition: `check_fs_precondition_before_sentinel(...)` (`src\commands\put.rs:382-387`; `src\core\vault_sync.rs:667-674`)
  - step 4 sha256: `prepared.sha256` (`src\commands\put.rs:166`, `372-375`)
  - steps 5-6 sentinel + tempfile fsync: `create_recovery_sentinel(...)`, `create_tempfile(...)` (`src\commands\put.rs:390`, `424-438`, `652-719`)
  - step 7 symlink guard: `stat_at_nofollow(...)` check before rename (`src\commands\put.rs:439-451`)
  - step 8 dedup insert: `insert_write_dedup(...)` + `remember_self_write_path(...)` (`src\commands\put.rs:467-489`)
  - steps 9-11 rename, parent fsync, post-rename stat/inode/hash guard (`src\commands\put.rs:506-595`)
  - steps 12-13 single SQLite tx + sentinel unlink (`src\commands\put.rs:597-623`)

### Partially implemented

- The filesystem precondition logic itself is good and tested (`src\core\vault_sync.rs:581-700`, `5259-5402`), but it is still wired as a separate helper that reopens the root / parent rather than operating on the final trusted parent fd that the writer later uses.
- Post-rename abort handling is already fail-closed and sentinel-backed (`src\commands\put.rs:750-778`), so the recovery model is mostly correct even before the last seam is repaired.

### Still missing

- **Step 2 is not design-complete.** `walk_to_parent(...)` has no `create_dirs=true` mode (`src\core\fs_safety.rs:58-132`), and the writer still falls back to path-based `fs::create_dir_all(parent)` before reopening the parent fd (`src\commands\put.rs:392-412`). That is the main remaining `12.1` gap.
- The actual step ordering is still split: precondition runs through `check_fs_precondition_before_sentinel(...)` before the final parent fd is opened for writing (`src\commands\put.rs:382-387` vs `399-412`), instead of one exact fd-relative sequence.
- The symlink refusal path still returns a generic I/O error string (`"target path is a symlink"`) rather than a dedicated typed write error (`src\commands\put.rs:439-449`).
- The implementation-plan pointer is stale: it says audit `put_from_string` in `vault_sync.rs`, but the production writer lives in `src\commands\put.rs`.

### Tests that already exist

- Precondition/OCC before sentinel: `unix_update_without_expected_version_conflicts_before_sentinel_creation`, `unix_stale_expected_version_conflicts_before_sentinel_creation`, `unix_external_delete_conflicts_before_sentinel_creation`, `unix_external_create_conflicts_before_sentinel_creation`, `unix_fresh_create_succeeds_without_existing_file_state` (`src\commands\put.rs:1221-1347`)
- Failure matrix and recovery: sentinel failure, pre-rename failure, rename failure, parent fsync failure, foreign rename, commit busy recovery, foreign-rename + startup recovery (`src\commands\put.rs:1462-1754`)
- Filesystem-precondition behavior: fast path, ctime self-heal, hash mismatch, same-size external rewrite (`src\core\vault_sync.rs:5259-5402`)

### Tests still missing

- Explicit tempfile `fsync` failure coverage (today there is no dedicated hook for the tempfile fsync branch)
- Explicit post-rename `stat` failure coverage
- Explicit dedup-insert collision / duplicate-entry failure coverage
- Typed symlink-escape coverage (today only the raw error string is present)

## Task 12.6 — mandatory `expected_version` everywhere

### Already implemented

- MCP enforces the contract up front:
  - existing page + missing `expected_version` → conflict (`src\mcp\server.rs:589-615`, tests at `1651-1673`, `1677-1707`)
  - stale `expected_version` → conflict (`src\mcp\server.rs:589-615`, tests at `1711-1740`)
  - create with unexpected `expected_version` → conflict (`src\mcp\server.rs:597-604`, tests at `1814-1828`)
- The Unix CLI/write-through core also enforces missing/stale update versions before sentinel creation (`src\commands\put.rs:376-381`, tests at `1221-1280`).
- CLI help text already documents the intended rule: `--expected-version` required for Unix updates, optional for creates (`src\main.rs:41-46`).

### Partially implemented

- The real OCC rule is already present for the shipped MCP and direct Unix CLI path, so this task is mostly a truth-closure task rather than a missing-core-logic task.

### Still missing

- The contract is not yet closed through the deferred live-routing path from `12.6a`; `quaid put` still writes directly regardless of serve ownership.
- There is still a non-Unix fallback path and test that allow unconditional update semantics (`src\commands\put.rs:323-339`, `1780-1792`). Do **not** widen platform support to “fix” this; instead keep the Unix gate truthful and keep Batch 4 scoped to vault-write surfaces only.

### Tests that already exist

- MCP OCC tests: `src\mcp\server.rs:1651-1828`
- Unix CLI-core OCC tests: `src\commands\put.rs:1221-1280`

### Tests still missing

- A serve-owned CLI-routing test proving the same OCC contract still holds once `12.6a` is implemented

## Task 12.6a — `quaid put` live-owner/offline routing

### Already implemented

- Core owner-lease infrastructure exists:
  - `acquire_owner_lease(...)` / `owner_session_id(...)` (`src\core\vault_sync.rs:1865-1910`)
  - tests for refusing a live foreign owner and reclaiming stale residue (`src\core\vault_sync.rs:6422-6492`)

### Partially implemented

- `ServeOwnsCollectionError` exists, but it only carries `owner_session_id`, not the `pid/host` detail required by the Batch 4 wording (`src\core\vault_sync.rs:307-310`).

### Still missing

- `quaid put` is still direct-dispatch only:
  - `main.rs` sends `Commands::Put` straight to `commands::put::run(...)` (`src\main.rs:301-305`)
  - `commands::put::run(...)` only applies the Unix gate, reads stdin, and calls `put_from_string(...)` (`src\commands\put.rs:90-97`)
  - there is **no** live-owner detection, no refusal instructing “use MCP or stop serve”, no offline temporary lease/heartbeat wrapper, and no IPC path
- This task must stay in the refuse-or-offline shape only; do not reopen Batch 5 IPC work here.

### Tests that already exist

- Only lower-level lease helper tests in `vault_sync.rs` (`6422-6492`)

### Tests still missing

- `quaid put` refuses while a live serve owner exists
- `quaid put` acquires/releases an offline owner lease when no live owner exists
- refusal message includes pid/host once the error surface is repaired

## Task 12.6b — bulk rewrite routing

### Already implemented

- Nothing user-facing for this task is actually implemented yet.

### Partially implemented

- The branch has prerequisite clues only:
  - restore/reconcile status text already tells operators to run `migrate-uuids work` in the trivial-content halt case (`src\commands\collection.rs:3000-3005`)
  - Batch 3 tasks remain open in `tasks.md` (`openspec\changes\vault-sync-engine\tasks.md:116-121`, `174`, `236`, `373`, `418-419`)

### Still missing

- `CollectionAction` still has **no** `MigrateUuids` variant (`src\commands\collection.rs:19-55`)
- `CollectionAddArgs` still uses the old `write_memory_id` field name, and `add(...)` explicitly rejects it as deferred (`src\commands\collection.rs:58-67`, `234-237`)
- There is a direct defer-test proving the flag is still blocked (`src\commands\collection.rs:1790-1812`)
- No live-owner refusal exists for bulk UUID rewrites because the bulk UUID rewrite commands themselves do not exist yet
- Even if they did exist, the current `ServeOwnsCollectionError` cannot yet name pid/host

### Batch 3 stale/incomplete callout

- `tasks.md` is honest that Batch 3 remains open (`5a.5`, `5a.5a`, `9.2a`, `17.5ii9`, `17.5ww`, `17.5ww2` are still unchecked), but the current `implementation_plan.md` is stale where it says Batch 3 bulk-write routing “already implements” the `12.6b` refusal (`openspec\changes\vault-sync-engine\implementation_plan.md:221`).
- That stale assumption is contradicted by the live code in `src\commands\collection.rs`, which still rejects `--write-quaid-id` and exposes no `migrate-uuids` command.

### Tests that already exist

- Only the defer test: `add_rejects_write_memory_id_before_creating_collection_row` (`src\commands\collection.rs:1790-1812`)

### Tests still missing

- `migrate-uuids` offline success
- `migrate-uuids --dry-run` no-op
- `collection add --write-quaid-id` live-owner refusal
- bulk refusal message naming pid/host and stop-serve guidance

## Task 12.7 — tests

### What already exists

- Strong direct coverage already exists for:
  - OCC-before-sentinel and filesystem-precondition cases (`src\commands\put.rs:1221-1347`)
  - per-slug mutex behavior (`src\commands\put.rs:1351-1458`)
  - sentinel/pre-rename/rename cleanup (`src\commands\put.rs:1462-1538`)
  - parent-fsync failure (`src\commands\put.rs:1578-1615`)
  - foreign rename / concurrent rename (`src\commands\put.rs:1619-1653`)
  - commit failure and sentinel-driven startup recovery (`src\commands\put.rs:1657-1754`)
  - MCP-side OCC / no-vault-mutation assertions (`src\mcp\server.rs:1651-1828`)

### What is still missing

- explicit tempfile fsync failure
- explicit post-rename stat failure
- explicit dedup-entry collision
- CLI live-owner routing tests (`12.6a`)
- bulk UUID rewrite routing tests (`12.6b`, blocked by missing Batch 3 commands)

## Concrete implementation checklist once branch state is corrected

1. **Do not touch platform scope or MCP schema.**
   - Keep the Unix gate closed.
   - Keep `memory_collections` frozen at 13 fields.
2. **Repair Batch 3 first; Batch 4 depends on it.**
   - Add `CollectionAction::MigrateUuids { name, dry_run }`
   - Rename `write_memory_id` to the truthful `write_quaid_id`
   - Implement the actual bulk UUID writer by reusing the production writer path, not a second file rewrite path
   - Add the live-owner refusal for those bulk commands, with pid/host detail
   - Mark Batch 3 tasks immediately as each one is truly done
3. **Finish the real `12.1` seam.**
   - Replace the path-based `fs::create_dir_all(...)` fallback with an fd-relative parent-directory creation/walk flow
   - Unify the write sequence so the precondition and rename operate on the same trusted parent-fd path
   - Add a typed symlink-escape error instead of a generic I/O string
4. **Implement `12.6a` in the narrowed Batch 4 shape only.**
   - Before direct `quaid put`, detect a live owner from `collection_owners` + `serve_sessions`
   - If live owner exists, refuse and instruct the operator to use MCP or stop serve
   - If no live owner exists, acquire a temporary offline lease + heartbeat around the direct write, then release it
5. **Close `12.7` with the missing failure tests.**
   - tempfile fsync failure
   - post-rename stat failure
   - dedup collision
   - CLI live-owner refusal / offline lease flow
   - bulk UUID rewrite routing once Batch 3 surfaces exist
6. **Protect the >90% coverage bar during the implementation lane.**
   - keep new tests inline with the touched modules
   - rerun the existing coverage command after Batch 3 + Batch 4 land together


---
created_at: 2026-04-30T06:37:20.531+08:00
author: Leela
type: routing-decision
subject: Batch 4 execution lane — recovery path from stale checkout
---

# Decision: Batch 4 Branch Routing and Recovery Path

## Context

The current working directory (`D:\repos\quaid`) is parked on `release/v0.11.0`, which is
12 commits ahead of `origin/release/v0.11.0` (all Scribe log commits) and is **not on main**.
`origin/main` is at `v0.12.0` (SHA `5a8bdf0`). The local tasks.md shows Batch 3 items as
open only because the stale branch predates the Batch 3 merge — all Batch 3 closures
(`5a.5`, `5a.5a`, `9.2a`, `5a.7`, `17.5ww`, `17.5ww2`, `17.5ww3`, `17.5ii9`, `12.6b`, `17.5www`)
are confirmed closed on `origin/main`. No `v0.13.0` tag or `release/v0.13.0` branch exists.
There are 2 modified `.squad/` files and 1 untracked `.squad/` health report in the working tree.

## Decision

**Batch 4 work begins in a sibling worktree created from `origin/main`.**

The `D:\repos\quaid` checkout is NOT touched for Batch 4 code work. The stale
`release/v0.11.0` working tree's dirty files are low-risk (`.squad/` only) and do not
conflict with a sibling worktree's object store.

### Worktree setup

```powershell
cd D:\repos\quaid
git fetch origin main --tags
git worktree add ..\quaid-vault-sync-batch4-v0130 -b spec/vault-sync-engine-batch4-v0130 origin/main
```

Starting SHA: `5a8bdf0` (tagged `v0.12.0`, confirmed clean).

### Batch 4 task scope

Open tasks on `origin/main`:
- `12.1` — complete the 13-step rename-before-commit sequence (audit `put_from_string` against all 13 steps; wire steps 2 `walk_to_parent`, 3 `check_fs_precondition`, 7 symlink defense-in-depth, and 8 dedup insert timing on ALL vault-byte write paths)
- `12.6` — mandatory `expected_version` enforcement audit across MCP + CLI (no blind-update escape hatch)
- `12.6a` — CLI write routing for `quaid put` single-file (refuse with `ServeOwnsCollectionError` when live owner exists; offline lease path when no live owner)
- `12.6b` — **ALREADY CLOSED** on main (Batch 3 Mom revision). Verify guard in place; no re-implementation needed.
- `12.7` — tests covering every rename-before-commit failure mode (tempfile fsync error, parent fsync error, commit error, foreign rename in window, concurrent dedup entries, external write mid-precondition)

### Agent assignments

| Agent | Task |
|-------|------|
| Fry | Implements 12.1, 12.6, 12.6a, 12.7 in the sibling worktree |
| Scruffy | Monitors unit test coverage ≥ 90% throughout |
| Professor | Code peer review of 12.1 (security-adjacent) and 12.6 (contract enforcement) |
| Nibbler | Adversarial review of 12.6a (CLI write routing, live-owner detection) |
| Bender | End-to-end validation pass after Fry signals implementation complete |
| Amy | Documentation review for any new error types or CLI changes |
| Zapp | Release lane: `release/v0.13.0` → PR → merge to main → tag `v0.13.0` after all gates clear |

### Gate sequence before code begins

1. ✅ No active reviewer gate (all prior Batch 3 gates cleared at v0.12.0 merge)
2. ✅ No v0.13.0 tag collision
3. ✅ `origin/main` is clean at `5a8bdf0`
4. ✅ Batch 3 closures verified on `origin/main` — no re-closure needed
5. **Required before first commit:** Fry creates the worktree as specified above

### Gate sequence before release

1. `cargo test` green in the worktree
2. Coverage ≥ 90% confirmed by Scruffy (CI publishes coverage evidence; Scruffy must confirm manually)
3. Professor and Nibbler approve (no admin-merge around reviewer gates — lesson from v0.12.0)
4. All review threads resolved
5. `release/v0.13.0` branch PR opened against `main`
6. PR merged cleanly
7. Zapp creates annotated tag `v0.13.0` from merge SHA and pushes it

### Constraints

- **Do NOT merge Batch 4 into or from `release/v0.11.0`** — that branch is dead.
- **Do NOT touch the 3 dirty files in `D:\repos\quaid`** during Batch 4 — they are Scribe artifacts and should be committed or pruned separately by Scribe.
- Tasks `12.6c`–`12.6g` (IPC socket) are **Batch 5 scope** — do not pull them into Batch 4.
- `12.6b` is already closed; Batch 4 only needs to verify the guard is present, not re-implement it.

## Risk flags

- `12.1` is security-adjacent (rename-before-commit discipline). Professor must review before merge, not after.
- The coverage threshold is not CI-enforced — human confirmation required before Zapp starts release lane.
- `now.md` is stale (updated 2026-04-25). The active branch field says `spec/vault-sync-engine` but actual work branch is a sibling worktree. No action needed for Batch 4 execution, but Scribe should update `now.md` after Batch 4 lands.


---
created_at: 2026-04-30T06:37:20.531+08:00
author: Scruffy
type: testing-decision
subject: Batch 4 coverage baseline and closure guard
---

# Decision: Batch 4 coverage baseline and truthful closure gate

## Context

A read-only Batch 4 assessment on `D:\repos\quaid` found that the current repo-wide Rust
coverage baseline is **89.47%** from
`cargo llvm-cov --lib --tests --summary-only --no-clean -j 1`.

The Batch 4 lane is uneven:

- `src\core\vault_sync.rs` — 83.22% line coverage
- `src\commands\put.rs` — 95.70%
- `src\commands\collection.rs` — 91.70%
- `src\mcp\server.rs` — 96.90%

The same assessment also confirmed that Batch 4 routing tasks are still genuinely open:
`quaid put` does not yet perform live-owner routing, `ServeOwnsCollectionError` still lacks
the pid/host detail required by the spec, `--write-quaid-id` is still explicitly deferred,
and there is no `migrate-uuids` collection subcommand in the current command surface.

## Decision

**Do not claim Batch 4 is above 90% or closure-complete unless validation includes both:**

1. a fresh `cargo llvm-cov --lib --tests --summary-only --no-clean -j 1` run, and
2. a refreshed `target\llvm-cov-report.json` via
   `cargo llvm-cov report --json --output-path target\llvm-cov-report.json`.

**Do not close `12.6`, `12.6a`, `12.6b`, or `12.7` on the current surface.**

## Rationale

- The repo is already below the stated 90% bar before any Batch 4 code lands.
- `vault_sync.rs` is the dominant coverage risk, so touching it without direct backfill is
  likely to worsen both patch and project coverage.
- The current codebase has good low-level OCC and rename-failure proof, but it still lacks the
  live-owner routing and bulk UUID rewrite surfaces needed for truthful closure of the open
  Batch 4 tasks.

## Lean validation path

For Batch 4 implementation work, the lean honest path is:

1. targeted Rust tests for `src\commands\put.rs` and `src\core\vault_sync.rs`
2. any new CLI truth tests needed for live-owner refusal / offline lease flow
3. final coverage rerun with the two-command llvm-cov loop above

This keeps scope tight while still proving the real Batch 4 contract.

# Bender — conversation memory baseline

- **Date:** 2026-05-04T07:22:12.881+08:00
- **Decision:** Do not call the conversation-memory branch release-ready yet, even though the current baseline clears the requested line-coverage bar.
- **Why now:** The measured baseline is good enough on code health (`cargo llvm-cov report` = 92.11% TOTAL line coverage; default coverage run, online-feature tests, clippy, cargo check, release-asset parity, and install-release seam all passed), but the release lane still has two hard gates: `Cargo.toml` is still `0.17.0`, so the tag-driven `release.yml` would reject `v0.18.0`, and the >90% coverage requirement still depends on explicit human confirmation because CI only reports coverage. Local `tests/install_profile.sh` failures are permission-semantics noise from the Windows bash / NTFS environment, not evidence that the Linux/macOS release asset contract is broken.
- **Next gate:** Let implementation continue, but do not open or merge a release-bound PR until the version bump is in the actual release candidate commit and someone reruns `cargo llvm-cov report` on the final tree to re-confirm the line-coverage floor.

# Fry — conversation-memory-foundations schema slice

**Date:** 2026-05-04T07:22:12.881+08:00  
**Requested by:** macro88  
**Change:** conversation-memory-foundations

## Decision

Implement the first conversation-memory schema slice as a strict v8 foundation patch on top of the existing `pages.type` model, not by renaming the column to `kind` or introducing a migration lane. The new session-expression index must guard `json_extract(...)` with `json_valid(frontmatter)` so malformed-frontmatter rows remain tolerated while the new v8 artefacts are present.

## Why

The repo already ships `SCHEMA_VERSION = 8`, so the honest minimal slice is to add the new `superseded_by`/`extraction_queue` artefacts, strengthen tests, and keep v7 databases on the existing schema-mismatch/re-init path. A raw `json_extract(frontmatter, '$.session_id')` expression index broke existing malformed-frontmatter tolerance in unit tests, so the guarded form is the safe way to land the session lookup seam without widening this slice into frontmatter-cleanup or migration work.

# Fry — Batch 7 PR opening gate

**Date:** 2026-05-02T21:49:40.366+08:00  
**Requested by:** macro88  
**Change:** vault-sync-engine

## Decision

Open the Batch 7 product PR from `sync-engine/batch-7` to `main` after committing and pushing the non-`.squad` branch work. Merge remains blocked until review feedback exists and is fully resolved in a later pass.

## Why

This records the explicit review gate for the Batch 7 lane and keeps the release handoff truthful: `v0.17.0` is still deferred until the PR lands and post-merge validation is rerun on `main`.

# Leela — conversation-memory-foundations batching gate

**Date:** 2026-05-04T07:22:12.881+08:00  
**Requested by:** macro88  
**Change:** conversation-memory-foundations

## Decision

Do not fan implementation past the already-started schema edits until the OpenSpec artifacts are truth-repaired and routing is reset. Treat the schema work as a v8 → v9 change until proven otherwise, resolve the `pages.type` versus `pages.kind` DDL mismatch in the artifacts before more Section 1 work, and require Nibbler pre-gate on the watcher/file-edit slice before Fry starts task 10. Open the draft PR after the corrected preflight slice plus the first stable implementation slices land (`1.1–2.5` and `11.1–11.2`), not at the end of the 70-task change.

## Why

The repo already advertises schema version 8 in code and schema, while the change artifacts still describe a v7 → v8 reset. The current tasks also specify `idx_pages_supersede_head ON pages(kind, superseded_by)` even though the live table stores that field as `type`, so leaving the artifacts unchanged would make the first batch lie about what is actually shipping. The branch is already dirty with partial work on this change, so the safe routing move is to pause widening, repair the truth in the specs/tasks, then continue under explicit reviewer and coverage gates.

# Leela — conversation-memory-foundations truth repair

**Date:** 2026-05-04T07:22:12.881+08:00  
**Requested by:** macro88  
**Change:** conversation-memory-foundations

## Decision

Truth-repair this change so it explicitly treats schema v8 plus the landed first plumbing slice (`pages.superseded_by`, the head/session indexes, `extraction_queue`, config defaults, and `Page.superseded_by`) as the current baseline. Rewrite stale `pages.kind` references to `pages.type`, and keep tasks `1.1`–`1.8` / `2.1` checked by rephrasing them as already-landed baseline work. Remaining implementation scope starts at `2.2`; no additional schema bump is in scope.

## Why

The live repo already ships the first slice, so leaving the artifacts on a planned `v7 → v8` bump and `pages(kind, superseded_by)` would make reviewers and implementers work against a false baseline. Reframing the checked tasks keeps scope unchanged while making OpenSpec honest about what is already landed versus what remains.

# Professor — conversation-memory-foundations slice 1 review

**Date:** 2026-05-04T07:22:12.881+08:00  
**Requested by:** macro88  
**Change:** conversation-memory-foundations  
**Commit:** a1ceae8

## Decision

Reject Fry's first slice for tasks 1.1-1.8 and 2.1. The code lands the narrow `pages.type` + guarded-`json_valid(frontmatter)` variant, but the OpenSpec artifacts still mark done against the older `pages(kind, ...)` and raw `json_extract(frontmatter, ...)` wording, so the shipped contract and the checked task text are out of sync.

## Highest-priority issue

**Spec/task truth mismatch:** `proposal.md` and `tasks.md` still describe the wrong schema contract for the checked items. This slice is only reviewable after those artifacts are rewritten to match what actually shipped.

## Gate outcome

- **Professor:** REJECT
- **Reason:** schema truth / task honesty failure, not a code correctness failure
- **Lockout:** Fry may not author the next revision of this rejected artifact

## Evidence

- `src/schema.sql` ships `idx_pages_supersede_head ON pages(type, superseded_by)` and guards the session index with `json_valid(frontmatter)`.
- `openspec/changes/conversation-memory-foundations/proposal.md` and `tasks.md` still describe `pages(kind, superseded_by)` and an unguarded `json_extract(frontmatter, '$.session_id')`.
- `cargo test --quiet -j 1` passed during review, so the rejection is about contract truth, not failing tests.

## 2026-05-04T07:22:12.881+08:00 — Conversation-memory slice 1 test gate

- `src\core\db.rs` already carries the high-value slice-1 proofs: schema v8 artefacts/defaults, `superseded_by` foreign-key enforcement, `extraction_queue` CHECK failures, and v7 rejection on open/init.
- The practical seam to keep green while Fry widens the slice is every hand-built `Page` fixture. When `Page` gains a field, update those fixtures in the same commit and add one serde-backcompat test proving legacy payloads still deserialize with the new field defaulted.
- Coordinator gate nuance: run `cargo test --quiet -j 1` with `RUST_TEST_THREADS=1` before `cargo llvm-cov --lib --tests --summary-only --no-clean -j 1` (also with `RUST_TEST_THREADS=1`). The plain serial test pass flushes fixture drift and the `commands::embed` ordering flake early; otherwise the coverage lane fails late on compile-only or order-sensitive targets and muddies the real coverage signal.

# Zapp — conversation memory release lane

- **Date:** 2026-05-04T07:22:12.881+08:00
- **Decision:** Do not open the draft PR for `feat/slm-conversation-mem` yet.
- **Why now:** The branch has no remote tracking ref or PR, the working tree mixes uncommitted implementation work with unrelated doc moves, and the public release surfaces are still stale (`v0.15.0` language, 17-tool copy, `roadmap.md` references, and `MIGRATION.md` links if that move lands).
- **Earliest safe moment:** After the branch is pushed with a coherent commit set, the draft body can truthfully describe the landed slice, and the public docs/release references are repaired. `Cargo.toml` should only move to `0.18.0` on the actual release-bound commit that will be tagged.
