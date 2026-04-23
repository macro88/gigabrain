# Session Log — Vault-Sync-Engine Planning

**Date:** 2026-04-22T08:47:14Z  
**Topic:** vault-sync-engine exploration and execution breakdown  
**Requested by:** macro88  
**Agents:** Leela, Scruffy  

## Outcome

**Leela** completed decomposition of the complete vault-sync-engine OpenSpec (370+ tasks, 18 groups, v4→v5 breaking schema change) into 9 waves + 3 gated PRs. Identified critical path: Schema → Collections → Reconciler → Watcher+brain_put → Commands/Serve → MCP. Highest risks: two-phase restore/remap defense, brain_put crash-safety, IPC security, watcher overflow real-time constraint. Recommendation: Keep as one OpenSpec, implement in Foundation / Live Engine / Full Surface PRs starting with ~1-week Wave 1 batch.

**Scruffy** assessed current CI/coverage surface: `src/**` ≈ 88.71% line coverage, no enforced threshold. Flagged ambiguity in user's >90% overall requirement (denominator undefined: `src` only? all Rust? which features? which platforms?). Recommended two-tier approach: Tier 1 (vault-sync seams ≥90%), Tier 2 (repo-wide informational until scope locked). Added `cargo llvm-cov --fail-under-lines 90` hard gate recommended for PR A.

## Key Decisions Recorded

1. **One OpenSpec, three gated PRs:** Foundation (Waves 1–2) → Live Engine (Waves 3–5) → Full Surface (Waves 6–7, 9).
2. **Resolve v0.9.3/v0.9.4 work before starting vault-sync-engine** to avoid schema merge conflicts.
3. **Nibbler pre-implementation adversarial review** of IPC security (tasks 12.6c–g) before Wave 5.
4. **Bender + Scruffy track 90%+ coverage** with every PR.
5. **Coverage denominator must be explicitly scoped** before hard gate is enforced repo-wide.

## Open Actions

- Resolve 10 open questions (branch strategy, in-flight work, Windows CI, Cargo.toml deps, import removal lint, macOS CI, raw_imports audit, user v4 migration messaging, coverage scope).
- Clarify coverage requirement scope with macro88 (denominator, features, OS coverage).
- Nibbler to review IPC security design before implementation begins.

## Files Modified

- `.squad/orchestration-log/2026-04-22T08-47-14Z-leela.md` — Leela work summary
- `.squad/orchestration-log/2026-04-22T08-47-14Z-scruffy.md` — Scruffy work summary
- `.squad/decisions/inbox/leela-vault-sync-breakdown.md` — Architectural decomposition (305 lines)
- `.squad/decisions/inbox/scruffy-vault-sync-coverage.md` — Coverage assessment (24 lines)
- `.squad/decisions/inbox/copilot-directive-2026-04-22T16-10-56.md` — User directive to implement vault-sync-engine with >90% coverage
