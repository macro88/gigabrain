# Project Context

- **Owner:** macro88
- **Project:** GigaBrain — local-first Rust knowledge brain
- **Stack:** Rust, rusqlite, SQLite FTS5, sqlite-vec, candle + BGE-small-en-v1.5, clap, rmcp
- **Created:** 2026-04-13T14:22:20Z

## Learnings

- Vault-sync-engine Batch J (2026-04-23): **RECONFIRMED NARROWED SLICE AFTER RESCOPE**. Nibbler's original pre-gate approved narrowed batch only as combined slice with all 18 proof items attached. When Professor proposed rescoping to plain sync + 7-ID closure only, Nibbler reconfirmed: the narrowed split is safe **if** implementation keeps plain sync strictly on active-root reconcile lane and does NOT use it as recovery multiplexer. Current code shape supports it: bare sync still hard-errors, fail-closed gates already exist, destructive paths separate, ownership/lease primitives center on `collection_owners`, restore/remap stays behind restoring + needs_full_sync + RCRT attach. Adversarial non-negotiables reaffirmed: active-root reconcile only, blocked states blocked and truthful, CLI ownership singular, reconcile halts terminal, operator surfaces honest. Fry implementation complete; Scruffy proof lane complete; all decisions merged. Next: implementation gate confirmation before landing.
- Narrowed Batch J is safe to implement next only if bare `collection sync` stays an active-root reconcile entrypoint, never a recovery multiplexer: no implicit finalize, remap, or write-reopen, and no new MCP surface just to report blocked states.
- Batch J is safe as one slice only if plain `collection sync` lands inseparably with lease/ack/finalize/integrity proof closure; enabling the default sync surface before those proofs turns normal operator traffic into a success-shaped reopen path.
- Once plain sync works, `sync`, `sync --finalize-pending`, `restore-reset`, `reconcile-reset`, and `brain_collections` become one operator trust surface: every command must preserve fail-closed state truth and never imply writes reopened before RCRT or reset preconditions are actually satisfied.
- Batch I re-gates cleanly only when legacy ingest/import honor the same global `state='restoring' OR needs_full_sync=1` interlock, offline restore/remap stop before attach, and the task ledger openly keeps plain sync plus offline CLI end-to-end recovery in deferred territory.
- Phase 3 review confirmed that raw-data and gap endpoints are only acceptable once payload shape checks, overwrite intent, and transport-size caps are all closed together; one missing seam keeps the whole surface soft.
- Adversarial review begins at the proposal, not only at the code diff.
- This project values hidden-risk discovery and reviewer lockout discipline.
- Local-first systems still need security and misuse thinking.
- Privacy-safe fields are not enough if adjacent free-form fields can still carry the same secret.
- Line-oriented shell protocols need explicit payload caps or raw-data endpoints become an easy memory-pressure path.
- For vault-bound walks, `WalkBuilder` output is only a candidate list; root-bounded `open_root_fd` + `walk_to_parent` + `stat_at_nofollow` must be the only authority for classification if symlink escapes are to stay closed.
- Hash-based rename guards are not safe if they use whole-file byte counts plus a non-empty-body check; conservative pairing needs post-frontmatter body significance, or template notes can inherit the wrong page identity.
- Batch E re-gate closed the hash-rename seam once both sides measured trimmed post-frontmatter body bytes, not whole-file size, and regression coverage pinned both refusal and success boundaries.
- Batch F is gateable when raw-import rotation fails closed on zero-active history inside the same write transaction and delete/quarantine decisions re-query DB-only state at apply time rather than replaying classification snapshots.
- Deferred restore/full-hash and UUID writeback seams are acceptable only when tasks and code comments keep them explicit and error-shaped; success-shaped stubs would make the same slice rejectable.
- Reconcile apply must distinguish true creates from existing-page updates before raw_import rotation; if an existing page reaches apply with zero total raw_import history, silent bootstrap is identity corruption, not healing.
- Zero-total `raw_imports` is a different seam from zero-active history: the shared rotation helper may still allow first-write bootstrap, so existing-page apply paths need their own pre-mutation guard while truly new pages remain the only narrow row-count-zero bootstrap case.
- Destructive bypass modes are not identity-scoped just because the API carries a string; if code only checks a non-empty `restore_command_id` or lease/session token without comparing it to persisted ownership state, any caller can forge the bypass.
- Batch H re-gates cleanly once restore/remap full-hash authorization compares the caller token to persisted collection owner fields and fails closed on missing or mismatched owners; mode shape plus any non-empty string is no longer enough.
- Batch I is only gateable as one slice if ownership, finalize, reattach, and write-gate land together; splitting them would leave a success-shaped destructive path with no trustworthy owner or reopen barrier.
- `collection_owners` must stay the sole ownership truth; any fallback to `serve_sessions`, `supervisor_handles`, or restore-command residue for live-owner resolution reopens spoofed-release and split-brain restore.
- The `(session_id, reload_generation)` ack is safe only if commands also fail closed on owner change, serve death, stale ack residue, and fresh-serve impersonation; matching one field is not enough.
- `run_tx_b` and RCRT are separate authority boundaries: finalize may happen through the canonical helper, but reattach/open-writes must stay exclusive to the RCRT attach-completion path under single-flight.
- Batch I credibility needs explicit tests for the OR write-gate and RCRT skip-on-halt behavior, even if those tests were not in the initial batch list; otherwise restore/remap can quietly reopen writes or bulldoze integrity blocks.
- Batch I still fails gate if any offline or command path calls `complete_attach` directly; even with `run_tx_b` canonicalized, bypassing RCRT turns `needs_full_sync` into a transient bit instead of the promised reopen barrier.

## 2026-04-15 In Progress

- Conducting final adversarial re-review of Phase 2 graph slice (tasks 1.1–2.5) after Scruffy cycle/self-loop suppression fix (commit `acd03ac`).
- Cross-team status: Professor completed parent-aware tree rendering (commit `44ad720`). Both commits now validated against graph specs. Awaiting Nibbler re-review completion before Phase 2 sign-off.


---

## 2026-04-16: Phase 3 Core Review — Rejection (task 8.2)

**Scope:** brain_gap, brain_gaps, brain_stats, brain_raw, call/pipe failure modes  
**Status:** Completed with REJECTION  

**Blocked artifacts:**
1. `src/mcp/server.rs` — brain_raw contract violation, no size limit, silent overwrites, gap privacy leak
2. `src/commands/pipe.rs` — oversized line handling

**Blocking findings:**
- brain_raw accepts non-object payloads (spec violation)
- No payload size limit (abuse vector)
- Silent replace semantics (data-loss risk)
- brain_gap context unbounded (privacy bypass seam)

**Decision:** nibbler-phase3-core-review.md merged to decisions.md  
**Task 8.2:** Not marked complete; different revision author required (reviewer lockout).

### 2026-04-22 17:02:27 - Vault-Sync Batch E Adversarial Review (Initial Rejection → Approval)

**Initial verdict:** REJECT

**Why initially blocked:**

The conservative hash-rename guard in src/core/reconciler.rs was optimistic for trivial/template notes with large frontmatter and tiny body. A template note with 200+ bytes of frontmatter and a trivially small body (e.g., 'Hi\n') could pass the ≥64-byte size check and be incorrectly paired as a rename, carrying the old page_id onto unrelated content once the apply pipeline lands.

**The exploit:** hash_refusal_reason() checked total file size instead of post-frontmatter body size. Large frontmatter satisfied the byte threshold while the actual human-authored body remained trivial.

**Repair delivered by Leela:**

1. MissingPageIdentity.body_size_bytes = compiled_truth.trim().len() + timeline.trim().len()
2. NewTreeIdentity.body_size_bytes = body.trim().len() (post-frontmatter)
3. hash_refusal_reason() gates on body_size_bytes < 64, not whole-file size
4. Refusal reason strings renamed for clarity

**Re-verdict:** APPROVE

**Why this is sufficient:**

- Note can no longer satisfy 64-byte threshold by stuffing bytes into frontmatter
- Refusal path tested directly at helper boundary
- Classification path tested end-to-end: whole-file-large / body-tiny note → hash_renamed = 0, quarantine
- Surrounding scope remains honest: tasks.md says native pairing is interface-only, apply/hash pipeline deferred

**Key learning for future batches:**

The 64-byte threshold in content-hash identity guards ALWAYS refers to body content after frontmatter delimiter. Whole-file size MUST NOT be used as a proxy. This is consistent with spec language in tasks 5.8a0 and 5.8e.

**Next adversarial focus:**
- Batch F apply pipeline: ensure quarantine semantics are not silently bypassed
- Batch F rename inference: test ambiguous cases stay quarantined (don't flip to false positives)
