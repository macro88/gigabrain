updated_at: 2026-04-23T07:32:00+08:00
focus_area: vault-sync-engine Batch J — plain sync + restore/integrity proof closure
active_issues: []
active_branch: spec/vault-sync-engine
---

# What We're Focused On

**Active change (vault-sync-engine):**

1. `vault-sync-engine` — Batch J pre-gated / queued for implementation.
   Owner lane: Fry. Reviewers: Professor, Nibbler. Test lane: Scruffy.
   - Make plain `gbrain collection sync <name>` real without weakening the H/I restore/remap safety model
   - Close the deferred restore/integrity proof set that must be real for honest landing claims
   - Keep reopen fail-closed until RCRT and preserve singular ownership truth

**Completed in this branch:**
- Batch H — Phase 0-3 restore/remap safety helpers + fresh-connection full-hash activation
- Batch I — restore/remap orchestration + ownership recovery, including legacy write-gating and RCRT-only reopen

**Explicitly deferred into Batch J or later:**
- Plain `gbrain collection sync <name>` operator path
- End-to-end offline CLI-to-RCRT proof and remaining restore/integrity closure tasks

**Gate:** Batch J requires Professor pre-implementation gate and mandatory Nibbler adversarial review before implementation.
