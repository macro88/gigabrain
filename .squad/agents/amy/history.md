# Project Context

- **Owner:** macro88
- **Project:** GigaBrain — local-first Rust knowledge brain
- **Stack:** Rust, rusqlite, SQLite FTS5, sqlite-vec, candle + BGE-small-en-v1.5, clap, rmcp
- **Created:** 2026-04-13T14:22:20Z

## Learnings

- Docs must make a sophisticated local-first system feel approachable.
- OpenSpec proposals are part of the writing input, not just implementation input.
- The docs goal is excellent onboarding and reference quality.
- Always define and apply a single status/install matrix across README and all docs pages at once — drift between surfaces confuses users and is hard to catch later.
- "Deferred follow-on" language for npm/installer must be explicit in both README and docs, not buried in a single footnote. A table showing supported-now vs. deferred reads faster than prose.
- When README and docs roadmap disagree on phase status (e.g., "not started" vs. "in progress"), the roadmap docs are usually more current — resolve by reading both sources before writing.
- Phase 3 gate in roadmap.md said `v0.1.0` when it should have been `v1.0.0` — easy to miss without cross-checking version targets table.

## 2026-04-15 P2 Docs Audit

**Role:** Pre-ship docs audit for Phase 2 (intelligence layer)

**What happened:**
- Audited all repo docs impacted by Phase 2 against the proposal, design, tasks, and five feature specs.
- Made safe implementation-independent updates: README.md (roadmap row, usage note, MCP tools split, contributing section), docs/roadmap.md (status → In progress, branch/PR reference), docs/getting-started.md (Phase 2 command callouts, MCP note), docs/contributing.md (Phase 2 reviewers added to gates table).
- Produced a full post-ship update map in `.squad/decisions/inbox/amy-phase2-docs.md` — exact per-file, per-location change map for README, roadmap, getting-started, contributing, spec.md, and proposal frontmatter.
- Left a comment on issue #26 summarizing the work.
- Did NOT update docs/spec.md (already accurate for Phase 2 design; needs post-ship verification pass only).

**Outcome:** Pre-ship pass COMPLETE. Post-ship pass gated on PR #22 merge + v0.2.0 tag push. Update map ready.

**Learnings:**
- Split "available MCP tools" into Phase 1 (shipped) and Phase 2 (in progress) when tools are phased — a flat list implies all tools are live.
- Getting-started tutorials that show Phase 2 commands need explicit phase callouts; tutorials read as "this works now" by default.
- The post-ship update map should be written at audit time (while the specs are fresh) — not after code lands.



**Role:** Public documentation refresh, install/status matrix owner

**What happened:**
- Amy refreshed `README.md` and created three new docs: `docs/getting-started.md`, `docs/roadmap.md`, `docs/contributing.md` as part of P3 release scope.
- Scruffy's review (task 5.2) rejected because coverage guidance was missing from README/docs pages — no pointer to GitHub Actions coverage artifact or job summary.
- Amy added coverage guidance to all public docs pages stating coverage is informational, not gating, and pointing readers to the GitHub Actions surface.
- After fixes, task 5.2 passed Scruffy's re-review.

**Outcome:** P3 Release docs component **COMPLETE**. README/docs aligned on status, install, coverage, and phase/version messaging. All gates passed.

**Decision notes:** `.squad/decisions.md` (merged from inbox) — Amy's three-file decision (getting-started, roadmap, contributing split) + final doc fix decisions.
