# Session Log: Sprint 0 PR-Opening and Decision Merge

**Timestamp:** 2026-04-13T14:32:00Z  
**Requestor:** macro88  
**Work:** Log Sprint 0 PR session and merge decision inbox items  

## What Happened

This session completed the Sprint 0 artifact remediation cycle initiated by Nibbler's adversarial review. All blockers were systematically addressed:

### Artifacts Revised

1. **Cargo.toml** — Added `env` feature to `clap` so `#[arg(env)]` compiles in `src/main.rs`
2. **src/main.rs** — Replaced literal `~/brain.db` default with `default_db_path()` function; platform-safe for Windows, macOS, Linux
3. **CI workflow (.github/workflows/ci.yml)** — Removed musl/static-link gates; now matches proposal: `cargo fmt` + `cargo clippy` + `cargo check` + `cargo test`
4. **Release workflow (.github/workflows/release.yml)** — Fixed tag trigger glob pattern from `v[0-9]+.[0-9]+.[0-9]+` to `v[0-9]*.[0-9]*.[0-9]*`; pinned `cross` to v0.2.5
5. **Phase 1 proposal** — Added explicit OCC (Optimistic Concurrency Control) semantics: compare-and-swap, version bump, MCP contract definition, structured conflict format
6. **Schema (src/schema.sql)** — Replaced `knowledge_gaps.query_text NOT NULL` with `query_hash NOT NULL` + `query_text DEFAULT NULL`; added CHECK constraint gating raw-query retention on approval audit data

### Review Gate Outcomes

- **Nibbler:** Blockers identified and systematically cleared. Scribe verification: all 6 blocker categories now resolved.
- **Professor:** Final gate approval lifted. `knowledge_gaps` schema now matches spec prose; no design-integrity drift remains.

### Decision Inbox Items Processed

All team decisions created during this cycle are now being merged from `.squad/decisions/inbox/` into `.squad/decisions.md`:

1. **2026-04-13T14:32:08Z: User directive** — Branch + PR workflow governance
2. **2026-04-13: Fry Sprint 0 revision** — Artifact remediation record
3. **2026-04-13: Professor final gate** — knowledge_gaps blocker resolution

The broader phase-gate and model-preference decisions (from Leela and previous team setup) are already recorded in decisions.md.

## Decision Merge Outcomes

All inbox items have been merged into the canonical `.squad/decisions.md` ledger under **Active Decisions**. The decision record now reflects:

- All four phase gates and routing assignments (Leela)
- All three artifact remediation categories (Fry)
- User workflow directive (macro88 via Copilot)
- Model preferences and team cast (from team setup)

## Next Steps

Sprint 0 is now ready for PR opening:

```
git checkout -b sprint-0/scaffold
git add .
git commit -m "Sprint 0: repository scaffold, CI/CD, OpenSpec proposals

- Cargo.toml with full dependency declarations
- src/ module stubs (commands, core, mcp)
- src/schema.sql — full v4 DDL
- skills/ stubs for all 8 skills
- tests/fixtures, benchmarks/README.md
- CLAUDE.md, AGENTS.md
- .github/workflows/ci.yml and release.yml
- openspec/changes/ proposals for all 4 phases

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"

git push origin sprint-0/scaffold
# Open PR to main, link to Sprint 0 issue
```

After merge, Phase 1 work begins under Fry ownership.

## Governance Applied

- All remediated artifacts follow OpenSpec proposal-first model
- No decisions made in isolation; all team feedback recorded in inbox before merge
- Branch + PR workflow enforced per user directive
- All co-authorship credits applied
