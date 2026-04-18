updated_at: 2026-04-19T00:00:00Z
focus_area: v0.9.3 implementation — fts5-search-robustness + assertion-extraction-tightening
active_issues: [52, 53, 38, 55]
active_branch: release/v0.9.3
---

# What We're Focused On

**Active changes (v0.9.4 target):**

1. `fts5-search-robustness` — apply-ready (4/4 artifacts). Covers #52 + #53.
   Owner: Fry. Reviewers: Leela, Professor.
   - Apply `sanitize_fts_query` in `gbrain search` (default on, `--raw` to bypass)
   - Apply sanitizer in MCP `brain_search` handler
   - Emit `{"error":...}` JSON on raw FTS5 errors

2. `assertion-extraction-tightening` — apply-ready (4/4 artifacts). Covers #38, conditional #55.
   Owner: Professor. Reviewers: Leela, Nibbler.
   - Scope extraction to `## Assertions` sections + frontmatter only (Phase A–D)
   - Phase E (semantic similarity gate for #55) is CONDITIONAL on post-Phase-A benchmark rerun

**Near-complete lanes to finish before v0.9.4:**
- `configurable-embedding-model` (27/29 tasks)
- `bge-small-dual-release-channels` (12/14 tasks)
- `simplified-install` (17/18 tasks)

**Already fixed in v0.9.2:**
- #54 PARA type inference → close issue

**Gate:** All v0.9.4 ship gates documented in `.squad/decisions/inbox/leela-v093-routing.md`.
