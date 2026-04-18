# Decision: v0.9.3 routing — DAB benchmark triage to v0.9.4

**Author:** Leela (Lead)
**Date:** 2026-04-19
**Context:** Doug's DAB v1.0 benchmark run (issue #56) on GigaBrain v0.9.1 scored 133/200.
Issues #52, #53, #54, #55, #38 were filed. This note maps each issue to a proposal lane,
captures cross-check against current repo state (v0.9.2 main), and defines v0.9.4 ship gates.

---

## Issue → Proposal Lane Routing

| Issue | Description | Status | Lane | Owner | Reviewers |
|-------|-------------|--------|------|-------|-----------|
| #38 | check --all false-positive cascade (prose extraction) | 🔴 Not fixed | `assertion-extraction-tightening` | Professor | Leela, Nibbler |
| #52 | FTS5 crash on `?`, `'`, `%` in `gbrain search` | 🔴 Not fixed (only query path sanitized) | `fts5-search-robustness` | Fry | Leela, Professor |
| #53 | JSON parse error on `gpt-5.4` style queries | 🔴 Not fixed (same root cause as #52) | `fts5-search-robustness` | Fry | Leela, Professor |
| #54 | PARA directory type inference | ✅ Fixed in v0.9.2 (PR #48) | `import-type-inference` (archived) | — | Close issue |
| #55 | Contradiction detection high false-positive rate | 🟡 Conditional | `assertion-extraction-tightening` Phase E | Professor | Leela, Nibbler |

---

## Lane Decisions

### Lane 1: `fts5-search-robustness` (new — covers #52, #53)
- **Status:** apply-ready (all 4 artifacts complete)
- **Root cause confirmed:** PR #43 applied `sanitize_fts_query` only in the `hybrid_search`
  path (`gbrain query`). The `gbrain search` command calls `search_fts` raw. The MCP
  `brain_search` tool also calls `search_fts` raw. Both paths still crash on `?`, `'`, `%`.
- **Fix:** Apply sanitizer in `src/commands/search.rs` (default on, `--raw` flag to bypass);
  apply sanitizer in `brain_search` MCP handler; emit `{"error":...}` JSON on raw errors.
- **Routing:** Fry implements on `release/v0.9.3`. Professor reviews. Nibbler to verify MCP
  tool doesn't expose new attack surface.
- **Gate:** All Phase D/E tasks green + `gbrain search "what is CLARITY?"` and
  `gbrain search --json "gpt-5.4 codex model"` must pass without error.

### Lane 2: `assertion-extraction-tightening` (existing — covers #38, conditional #55)
- **Status:** apply-ready (4/4 artifacts complete). 0/13 tasks implemented. All tasks unchecked.
- **Root cause confirmed (issue #38):** `extract_from_content` in `src/core/assertions.rs`
  runs regex patterns across entire `compiled_truth` body text. Any prose sentence matching
  `is_a`, `works_at`, or `founded` patterns becomes a contradiction participant.
- **Fix:** Scope extraction to `## Assertions` section + frontmatter fields only (Phase A);
  add min object-length guard (Phase A); frontmatter tier-1 extraction (Phase A).
- **Phase E (semantic gate for #55) is CONDITIONAL:** Rerun benchmark corpus after Phase A
  lands. Only implement Phase E if false-positive rate remains material. Kif confirms this
  sequencing in `.squad/decisions/inbox/kif-v0.9.4-benchmark-triage.md`.
- **Routing:** Professor implements. Nibbler adversarial review required before merge (high
  risk — changes runtime extraction behavior for all vaults). Professor owns the Phase E
  rerun decision after Phase A lands.

### #54 — Closed, no action needed
`import-type-inference` is fully implemented in v0.9.2 (PR #48, commit 9a5515f).
PARA type inference works: `1. Projects/ → project`, `Areas/ → area`, etc. Close issue #54.

---

## Near-complete lanes — include in v0.9.4

| Lane | Tasks remaining | Action |
|------|-----------------|--------|
| `configurable-embedding-model` | 2/29 | Complete final 2 tasks; PR already partially reviewed |
| `bge-small-dual-release-channels` | 2/14 | Complete final 2 tasks (D.1 validation, D.2 PR) |
| `simplified-install` | 1/18 | Complete final task |

These should be merged before v0.9.4 tag if not already on the release branch.

---

## v0.9.4 Ship Gates

1. `gbrain search "what is CLARITY?"` → exits 0, returns results or empty list.
2. `gbrain search --json "gpt-5.4 codex model"` → exits 0, stdout is valid JSON.
3. `gbrain check --all` on a 350+ page PARA vault → zero contradiction floods from prose-only pages.
4. `gbrain import` on a PARA vault → type distribution reflects folder structure (not 99% concept).
5. Full `cargo test` suite green on `release/v0.9.3` branch.
6. `configurable-embedding-model`, `bge-small-dual-release-channels`, `simplified-install`
   lanes complete or confirmed already-merged.

---

## Branch Strategy

- **`release/v0.9.3`** — Implementation branch. All v0.9.4 bug fixes and completions land here.
  Created from `origin/main` (v0.9.2). OpenSpec proposals committed.
- **`release/v0.9.4`** — Will be tagged from `release/v0.9.3` after all gates pass.
  Or: merge `release/v0.9.3` → `main` and tag `v0.9.4` from main per existing convention.

---

## §4 Semantic/Hybrid quality (not a v0.9.4 gate)

Doug's crypto/finance paraphrase misses (§4, -24 points) are a model quality issue.
`configurable-embedding-model` (merged in v0.9.2) lets users switch to `bge-base` or
`bge-m3` for higher recall. A future benchmark lane (`kif-model-comparison`) should run
DAB against `small` vs `base` vs `m3` to establish per-model baselines before making
quality promises. Do not gate v0.9.4 on §4 improvement.

---

Leela, 2026-04-19
