# Project Context

- **Owner:** macro88
- **Project:** GigaBrain — local-first Rust knowledge brain
- **Stack:** Rust, rusqlite, SQLite FTS5, sqlite-vec, candle + BGE-small-en-v1.5, clap, rmcp
- **Created:** 2026-04-13T14:22:20Z

## Learnings

- Edge-case work is an explicit part of this squad, not an afterthought.
- The requested target model is Gemini 3.1 Pro when available on the active surface.
- Proposal-first work makes it easier to identify which assumptions deserve stress.

## 2026-04-15 Graph Temporal Gate Fix Resolution

- **Mom's edge-case note** on future-dated links was identified as part of initial graph slice review (directionality contract blockers).
- **Temporal gate gap:** The original graph query only checked `valid_until >= today` but did not gate `valid_from <= today`, which allowed future-dated links to appear in the "active" graph.
- **Resolution:** Leela's graph slice revision (tasks 1.1–2.5) incorporated the fix into decision D2. Active temporal filter now enforces:
  ```sql
  (l.valid_from IS NULL OR l.valid_from <= date('now'))
  AND (l.valid_until IS NULL OR l.valid_until >= date('now'))
  ```
- **Status:** INCORPORATED. Graph slice approved for landing on `phase2/p2-intelligence-layer` 2026-04-15T23:15:50Z.
- **Lessons:** Edge-case work is most effective when it surfaces during contract-review blockers, not during post-landing firefighting. Mom's temporal concern directly influenced the final graph design.
