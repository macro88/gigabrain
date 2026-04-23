updated_at: 2026-04-23T09:12:00+08:00
focus_area: vault-sync-engine Batch K1 — collection add scaffolding + shared read-only write gate
active_issues: []
active_branch: spec/vault-sync-engine
---

# What We're Focused On

**Active change (vault-sync-engine):**

1. `vault-sync-engine` — Batch K1 pre-gated / queued for implementation.
   Owner lane: Fry. Reviewers: Professor, Nibbler. Test lane: Scruffy.
   - Make `gbrain collection add` real with fresh-attach + short-lived lease discipline
   - Land the shared `CollectionReadOnlyError` write gate and keep add read-only by default
   - Preserve the H/I/J fail-closed restore/remap model and truthful CLI/operator surfaces without claiming offline restore integrity closure yet

**Completed in this branch:**
- Batch H — Phase 0-3 restore/remap safety helpers + fresh-connection full-hash activation
- Batch I — restore/remap orchestration + ownership recovery, including legacy write-gating and RCRT-only reopen
- Batch J — plain sync active-root reconcile path + CLI finalize truth fix

**Explicitly deferred into K2 or later:**
- Offline restore integrity matrix (`17.5kk3`, `17.5ll3-5`, `17.5mm`, `17.11`) until restore originator identity and reset/finalize honesty gaps are fixed
- Online restore handshake, IPC socket work, broader MCP widening, and remaining destructive-path closure beyond K2

**Gate:** Batch K1 requires Professor pre-implementation gate and mandatory Nibbler adversarial review before implementation.
