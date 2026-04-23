# Orchestration: Professor — Vault Sync Batch D Gate (Scope Review)

**Agent:** professor | reviewer  
**Status:** rejected_then_approved  
**Date:** 2026-04-22T15:23:03Z  

## Summary

Initial rejection on `tasks.md` truthfulness grounds (3 stale/false claims). Re-gate approval after Leela's narrow documentation repair. No code changes required.

## Initial Gate Review (Rejected)

**Scope:** Code, tests, design, schema.sql, tasks.md  
**Verdict:** REJECT (documentation blocker, not code quality)

### Clearing Evidence

1. **Root-bounded reconciler walk is real** — `reconcile()` opens `root_fd` first; `walk_root()` re-walks every candidate via `walk_to_parent()` + `stat_at_nofollow()`. Tests cover root refusal + entry-skip.
2. **stat_diff is no longer a stub** — Performs real filesystem walk; loads `file_state`; classifies `unchanged`/`modified`/`new`/`missing` via `stat_differs()`.
3. **has_db_only_state is implemented truthfully** — Five SQL branches explicit; direct unit coverage for each true case + all-clear false case.
4. **Provenance audit in code** — `brain_link` writes `source_kind='programmatic'`; `check_assertions()` writes `asserted_by='import'`. Full test + clippy green.

### Blocking Issue

**tasks.md is not truthful enough for landing:**

- **4.3** still claimed DB-file "missing" stub + 5.2 deferred; now false (stat_diff is real and wired)
- **5.1 repair note** still claimed `has_db_only_state()` returns `Err` + `walk_collection` unwired; now false
- **5.4a** claimed `extract_links()` sets `wiki_link`; never true (function only returns slug strings)

**Required:** Narrow documentation repair by non-original author (Leela).

## Re-gate Review (Approved)

**Date:** 2026-04-22  
**Scope:** Narrow re-gate on `tasks.md` truthfulness only  
**Verdict:** APPROVE FOR LANDING

### Clearing Evidence (Repair Validation)

1. **Task 4.3 now matches code** — `stat_diff()` real walk via `walk_root()/ignore::WalkBuilder`, re-stats entries with fd-relative nofollow, classifies via `stat_diff_from_walk()`. Covered by test.
2. **Task 5.1 now describes scaffold accurately** — `walk_collection()` and `has_db_only_state()` are real; `full_hash_reconcile()` still explicit not-yet-implemented error.
3. **Task 5.4a no longer overclaims** — `commands/link.rs` writes `source_kind='programmatic'`; `core/links.rs::extract_links()` returns slugs only; no wiki-link writer yet.

---

**Status:** APPROVED. No new code reviewed in re-gate pass.
