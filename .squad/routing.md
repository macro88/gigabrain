# Work Routing

How to decide who handles what in GigaBrain.

## Routing Table

| Work Type | Route To | Examples |
|-----------|----------|----------|
| Scope, architecture, sequencing | Leela | Decompose `docs\spec.md`, approve system boundaries, resolve trade-offs |
| Core implementation | Fry | Rust CLI, ingest pipeline, retrieval, storage, MCP server |
| End-to-end testing | Bender | Validation passes, regressions, round-trip checks, failure hunts |
| Technical docs | Amy | Getting started, tutorials, reference docs, examples |
| Docs website | Hermes | Docs IA, website structure, search UX, examples, theming |
| DevRel and launch | Zapp | OSS messaging, launch assets, growth loops, adoption content |
| Code peer review | Professor | Code review, maintainability, architecture sanity checks |
| Adversarial review | Nibbler | Security, abuse cases, hidden risk, design breakpoints |
| Unit test depth | Scruffy | Coverage, deterministic unit tests, hard-to-hit paths |
| Benchmarks | Kif | Performance harnesses, throughput, latency baselines |
| Edge cases | Mom | Pathological inputs, migration weirdness, failure-mode exploration |
| Session logging | Scribe | Automatic — never needs routing |
| Work monitoring | Ralph | Backlog scans, issue pickup, PR/CI follow-through |

## Proposal Routing

| Work Type | Route To | Examples |
|-----------|----------|----------|
| OpenSpec change proposal | Leela + relevant owner | Any meaningful change to code, docs, benchmarks, or docs-site work |
| Proposal for implementation work | Fry / Hermes / Amy / Zapp + Leela | New feature, docs-system change, launch initiative |
| Proposal for testing/perf work | Bender / Scruffy / Kif / Mom + Leela | New coverage plan, benchmark suite, edge-case campaign |

## Issue Routing

| Label | Action | Who |
|-------|--------|-----|
| `squad` | Triage: analyze issue, assign `squad:{member}` label | Leela |
| `squad:leela` | Pick up lead / architecture work | Leela |
| `squad:fry` | Pick up implementation work | Fry |
| `squad:bender` | Pick up testing work | Bender |
| `squad:amy` | Pick up documentation writing | Amy |
| `squad:hermes` | Pick up docs-site work | Hermes |
| `squad:zapp` | Pick up DevRel / growth work | Zapp |
| `squad:professor` | Pick up peer review work | Professor |
| `squad:nibbler` | Pick up adversarial review work | Nibbler |
| `squad:scruffy` | Pick up unit-test coverage work | Scruffy |
| `squad:kif` | Pick up benchmarking work | Kif |
| `squad:mom` | Pick up edge-case work | Mom |

### How Issue Assignment Works

1. When a GitHub issue gets the `squad` label, **Leela** triages it, assigns the right `squad:{member}` label, and comments with the routing note.
2. Before implementation starts, the assigned owner writes or updates an OpenSpec change proposal when the work is meaningful.
3. When a `squad:{member}` label is applied, that member picks up the issue in their next session.
4. The `squad` label is the untriaged inbox. Member labels are execution lanes.

## Rules

1. **OpenSpec first** — meaningful changes require a change proposal in `openspec\` before implementation begins.
2. **Eager by default** — spawn all agents who could usefully start work, including tests, docs, review, and benchmark work that can begin from the proposal.
3. **Scribe always runs** after substantial work, always as `mode: "background"`. Scribe logs outcomes; Scribe does not replace proposal authoring.
4. **Quick facts → coordinator answers directly.**
5. **When two agents could handle it**, pick the one whose domain is the primary concern and attach the reviewer most likely to challenge it.
6. **"Team, ..." → fan-out.** Spawn all relevant agents in parallel as `mode: "background"`.
