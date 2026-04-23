updated_at: 2026-04-24T13:08:00Z
focus_area: vault-sync-engine post-M1a next-slice selection
active_issues: []
active_branch: spec/vault-sync-engine
---

# What We're Focused On

**Active change (vault-sync-engine):**

1. `vault-sync-engine` — Batch M1a closed; next slice not yet selected.
   Owner lane: Fry. Reviewers: Professor, Nibbler. Test lane: Scruffy.
   - M1a landed only the writer-side sentinel crash core: `12.1a`, `12.4aa-d`, `17.5t`, `17.5u`, `17.5u2`, `17.5v`
   - Keep full write-through closure, mutex/CAS/precondition work, live routing, and generic startup healing claims deferred
   - Pick the next truthful slice before widening beyond the proved crash-core seam

**Completed in this branch:**
- Batch H — Phase 0-3 restore/remap safety helpers + fresh-connection full-hash activation
- Batch I — restore/remap orchestration + ownership recovery, including legacy write-gating and RCRT-only reopen
- Batch J — plain sync active-root reconcile path + CLI finalize truth fix
- Batch K1 — collection add/list plus truthful read-only gate
- Batch K2 — offline restore integrity closure with CLI finalize path
- Batch L1 — registry-startup scaffolding + restore-orphan startup recovery
- Batch L2 — startup-only sentinel recovery
- Batch M1a — writer-side sentinel crash core

**Explicitly deferred after M1a:**
- Online restore handshake, IPC socket work, and the `17.5pp` / `17.5qq*` series that depend on IPC security design
- Broader MCP widening and remaining post-Tx-B / destructive restore surfaces beyond startup recovery
- Full `12.1`, `12.2`, `12.3`, full `12.4` mutex/CAS proof, `12.5`, `12.6*`, `12.7`, and any full happy-path write-through closure claim
- Live/background recovery worker, IPC/live routing, and any claim that generic startup healing or remap reopen is already complete
- Post-landing coverage/docs/release/cleanup/issues agenda remains queued until the vault-sync branch reaches an appropriate stop point

**Gate:** No next vault-sync slice is active yet; require a fresh scoped gate before implementation resumes.
