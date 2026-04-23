---
date: 2026-04-16T16:15:06Z
session: model-release-review
attendees: [leela, fry, bender]
---

# Session Log — Model Release Review

## Outcome

**Decision:** Approve `dual-release-distribution` change for BGE-small only. Defer global `--model small|base|large` UX to post-1.0 roadmap.

## Branches

- Proposal: `proposal/dual-release-distribution`
- Implementation: `feat/dual-release-distribution`

## Scope

- Variant 1: Airgapped (current) — full weights embedded, ~180MB binary, offline
- Variant 2: Slim/Online — no weights, ~90MB binary, init-time HF download
- Single model family: BGE-small-en-v1.5 only (no base/large variants)

## Key Constraints Honored

1. **Schema stability** — Single embedding dimension (384) across both variants
2. **Release sanity** — Two artifacts per platform, not six
3. **Test burden** — 4 validation scenarios, not 12+
4. **Doc maintenance** — Platform-specific guidance (HF vs. airgapped), not model UX
5. **Migration safety** — No re-embedding required for v0.9.0→future-1.0 path

## Next Steps

1. Leela: Finalize proposal branch and OpenSpec entry
2. Fry: Implement dual-distribution in feat/ branch
3. Bender: Define Phase 3.1 rollout test plan
4. Scribe: Merge this decision to decisions.md

## Deferred

- Global `--model` flag (Phase 4+)
- Re-embedding/export infrastructure (Phase 4+)
- Model-family lifecycle choices (Phase 4+)
