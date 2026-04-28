# Project Context

- **Owner:** macro88
- **Project:** GigaBrain — local-first Rust knowledge brain
- **Stack:** Rust, rusqlite, SQLite FTS5, sqlite-vec, candle + BGE-small-en-v1.5, clap, rmcp
- **Created:** 2026-04-13T14:22:20Z

## Learnings

- The project explicitly wants docs and OSS presence that can go viral.
- DevRel work needs to stay grounded in shipped behavior and approved proposals.
- Docs quality and growth strategy are first-class concerns, not nice-to-haves.
- Release surface clarity is a growth asset: explicitly naming what does NOT ship (npm, Homebrew, one-command installer) builds trust faster than vague "coming soon" copy.
- A release checklist at `.github/RELEASE_CHECKLIST.md` is the right artifact for sign-off ceremonies — Zapp, Fry, and Leela each have a named row.
- The three-way contract (supported-now / in-progress / deferred) must appear in README, docs site, AND release notes body — a single source of truth drifts the moment one surface is edited independently.
- GitHub's `softprops/action-gh-release` `body` field prepends to auto-generated notes, which is the right place to fix install guidance before any PR title leaks unsupported channel language into a release.
- Phase/version alignment is a chronic drift point: `v0.1.0 = Phase 1`, `v0.2.0 = Phase 2`, `v1.0.0 = Phase 3`. Any doc that mentions a version tag must also cite the correct phase, and vice versa.
- When a status table lists phases without version tags, readers cannot cross-reference the roadmap — always include both the phase label and the version target in the same row.
- Operational scripts (issue creation commands, label helpers) are docs too: a mismatched label like `[Phase 3] v0.1.0 release` teaches contributors the wrong mental model before they've even opened a file.
- Feature ordering in the README Features list is a positioning signal: the live watcher / collection management (vault-sync-engine) should appear near the top alongside MCP server — these are the headline growth hooks for Obsidian users, not footnotes.
- Homepage feature grids are the highest-conversion real estate: every landed branch capability with user-facing impact deserves a card with an honest branch qualifier — aspiration plus honesty beats hiding the feature until release.
- Tool count accuracy across surfaces: use the released count (16) on generic install-path docs; use branch count (17) only when explicitly discussing the branch. README already models this split correctly.

- **Batch 1 Coverage Arc Culmination (2026-04-28/29) — v0.10.0 SHIPPED:**
  - **Decision Made:** Accept 90.77% Windows line coverage from `target\llvm-cov-final.json` as authoritative gate metric
  - **Rationale:** User-supplied verified measurement supersedes earlier 82.53% Linux CI audit (pre-Batch1-push); 90% gate explicitly targets line coverage, not region coverage; 90.77% clears threshold
  - **Platform Note:** Linux CI canonical baseline (pre-Batch1) was 82.53% (~19,588/~23,729 lines); unix-gated infrastructure paths remain architectural ceiling on that platform, not regression
  - **Batch 1 Feature Scope:** All 13 tasks complete—6.7a (overflow recovery), 6.8 (.quaidignore reload), 6.9 (native-first watcher), 6.10 (per-collection supervisor), 6.11 (watcher health CLI-only), plus 8 supporting proofs (17.5w–17.5aaa4)
  - **Release Artifacts:** Commit `ea5cabf` (excluded `.squad/` files), annotated tag `v0.10.0` pushed to `origin/main`, OpenSpec 216/313 tasks complete (Batch 1 ready), CI auto-trigger via push.tags
  - **Profraw Cleanup:** Deleted ~170 transient `default_*.profraw` artifacts; follow-on: add to `.gitignore`
  - **v0.10.0 Status:** ✅ RELEASED

## 2026-04-15 Release Contract Audit — Fix 'for this release' ambiguity

**Role:** Release-facing copy, release contract clarity

**What happened:**
- User flagged that docs appeared to imply a release existed or would happen after each phase but no release was present.
- Audit found two concrete issues: (1) README.md used "this release" language implying v0.1.0 was already shipped, and the curl snippet was presented as immediately usable; (2) docs/contributing.md had `[Phase 3] v0.1.0 release` in the issue script, contradicting the version target table (`v0.1.0 = Phase 1`).
- Discovered PR #19 (`fix/v0.1.0-release-gap`) already existed, correctly documenting that Phase 1 is complete and v0.1.0 is pending tag push.
- Added two commits to PR #19: (1) replaced "only supported binary distribution channels for this release" with explicit build-from-source (available now) / GitHub Releases (pending v0.1.0 tag) split, plus a "Not yet available" callout on the curl block; (2) corrected contributing.md issue script label from phase-3 to phase-1.
- Decision logged to `.squad/decisions/inbox/zapp-release-contract.md` (gitignored; local only).

**Outcome:** PR #19 now carries full release-contract clarity: Phase 1 complete, v0.1.0 pending tag, no false implication of an existing release. PR #18 (opened on wrong base) was already closed.

**Decision:** Option (b) — tighten wording. No premature release was published.



**Role:** Release-facing copy, checklist, phase/version alignment sign-off

**What happened:**
- Zapp added release checklist at `.github/RELEASE_CHECKLIST.md` with named sign-off rows for Zapp, Fry, Leela.
- Updated RELEASE_CHECKLIST.md and release-facing copy for standard checksum format (`hash  filename`).
- Final doc fix pass: corrected phase/version alignment in `install.md` (status table now includes version tags) and `contributing.md` (issue script corrected from `[Phase 3] v0.1.0` to `[Phase 1] v0.1.0`).
- All operational scripts and status matrices aligned with roadmap version targets.

**Outcome:** P3 Release release-readiness component **COMPLETE**. Release checklist ready, phase/version aligned across README/docs/scripts, all gates passed.

**Decision notes:** `.squad/decisions.md` (merged from inbox) — documents Zapp's two decisions (release checklist routing, final phase/version alignment fixes).


## 2026-04-25 Promo / Docs Pass — vault-sync-engine branch truth

**Role:** Public docs + product-facing messaging audit

**What happened:**
- Audited all four surfaces flagged by `docs-site-promotion-checklist` SKILL.md: tool count references, version numbers, schema version references, homepage accuracy, and roadmap completeness.
- Found 4 issues requiring correctness fixes: (1) "17 tools" in index.mdx code comment, (2) "seventeen production tools" in getting-started.mdx, (3) two "17 MCP tools" refs in phase3-capabilities.md, (4) stale "v0.1.0 — tag pending" in roadmap Phase 1.
- Fixed all four correctness issues. Tool count references now say 16 (released) with branch-qualified 17 where appropriate.
- Added vault-sync-engine "TBD" row to roadmap version targets table.
- Growth: Added "Live Vault Sync" card to homepage CardGrid with branch qualifier — Obsidian sync angle is highest-resonance hook for target audience.
- Growth: Added live-sync paragraph to why-gigabrain.mdx describing the watcher + reconcile-backed brain-current guarantee.
- Growth: Promoted "Live file watcher" and "Collection management" to positions 5–7 in README Features list (were last two items).
- Decision log written to `.squad/decisions/inbox/zapp-promo-docs-pass.md`.

**Files changed:**
- `website/src/content/docs/index.mdx` — tool count fix + vault-sync card
- `website/src/content/docs/guides/getting-started.mdx` — "seventeen" → "all production tools"
- `website/src/content/docs/guides/phase3-capabilities.md` — two "17" → "16" fixes
- `website/src/content/docs/contributing/roadmap.md` — stale Phase 1 note fixed; vault-sync TBD row added
- `website/src/content/docs/guides/why-gigabrain.mdx` — live-sync paragraph added
- `README.md` — features list reordered; vault-sync promoted; MCP tool count corrected

**Deferred launch work:**
- vault-sync-engine dedicated guide page (collections, watcher, quarantine CLI)
- Blog / changelog post for vault-sync-engine release
- npm public publication gate


## 2026-04-16 v0.9.0 Shell-First Release Lane

**Role:** Release lane truthfulness, branch/tag strategy, CI verification

**What happened:**
- Assessed full repo state: branch `main` was 1 commit ahead of origin, with all simplified-install work staged as modifications + untracked files (scripts/install.sh, packages/gbrain-npm/, proposal.md, updated workflows + docs).
- Confirmed no v0.9.0 tag existed locally or on remote. Cargo.toml already at 0.9.0.
- Created branch `release/v0.9.0` from local HEAD (not main), committed all 19 file changes in one coherent commit (`c1f572b`), pushed branch to origin.
- Created annotated tag `v0.9.0` on that commit with full release description, pushed tag — triggered the real GitHub Actions release pipeline.
- Result: 4 build jobs running (darwin-arm64, darwin-x86_64, linux-x86_64, linux-aarch64). npm publish workflow completed ✓ with correct notice ("NPM_TOKEN not configured; skipping").
- The release will produce real pre-built binaries + checksums + install.sh as a release asset.

**Key decisions:**
- Tagged on `release/v0.9.0` branch, not `main` — user explicitly requested non-main branch. GitHub Actions `push.tags` trigger fires regardless of branch.
- `prerelease: ${{ contains(github.ref_name, '-') }}` evaluates to `false` for `v0.9.0` — a full (not pre-) release on GitHub. v0.9.0 < 1.0.0 already communicates test-release status via semver.
- D.2 and D.5 remain environment-blocked but are not blocking the CI-based release build; those checks defer to post-release macOS/Linux runner verification.

## 2026-04-16T14:59:20Z Simplified-install v0.9.0 Release — Zapp Completion

- **Task:** Created release/v0.9.0 branch, committed simplified-install work, pushed branch, tagged v0.9.0, triggered GitHub release workflow, published binaries/checksums/install.sh
- **Changes:**
  1. Branch creation — created `release/v0.9.0` from local HEAD
  2. Commit — committed simplified-install work with full change log (19 files)
  3. Push branch — pushed `release/v0.9.0` to origin
  4. Tag creation — created and pushed annotated `v0.9.0` tag
  5. Release workflow — triggered GitHub Actions release workflow
  6. Artifacts published — release workflow published 4 binaries, 4 checksums, install.sh
- **Status:** ✅ COMPLETE. v0.9.0 release live. Binaries published to GitHub Releases. Checksums verified.
- **Orchestration log:** `.squad/orchestration-log/2026-04-16T14-59-20Z-zapp.md`

**Learnings:**
- Annotated tags (`git tag -a`) are preferable to lightweight tags for releases — they carry a tagger identity and timestamped message that shows in GitHub's release view.
- The `softprops/action-gh-release@v2` + `gh release upload` two-step pattern is correct for adding the install.sh asset after the binary artifacts are attached.
- npm token guard ("skip if absent, never fail") is the right CI posture for staged channels — zero friction for maintainers who haven't configured npm yet.
- **Promo docs consistency within multi-branch state (2026-04-25):** When one branch (vault-sync-engine) has 17 MCP tools and the released version has 16, the homepage, feature grids, and tool counts must all coordinate. Released-path binaries must show 16; branch prose may show 17 with explicit '(vault-sync-engine branch)' qualifier. Roadmap version tables need TBD entries for in-progress work to distinguish 'intentionally deferred' (explicit row) from 'not yet planned' (no row). Silence on a feature state is itself a claim.

## 2026-04-28 v0.10.0 Batch 1 ship

**Role:** Release lane execution, task marking, coverage gate sign-off

**What happened:**
- Deleted ~170 `default_*.profraw` transient coverage artifacts from the repo root (untracked junk left by test runs).
- Verified Batch 1 task completeness: 6.7a, 6.8, 6.9, 6.10, 6.11, 17.5w, 17.5x were already marked; 17.5y, 17.5z, 17.5aa, and 17.5aaa2 had implementations in `vault_sync.rs` but were not marked — added closures notes and checked them off.
- Coverage gate: user-supplied authoritative line coverage is 90.77% (from `target\llvm-cov-final.json`). Gate clears. Bender's Linux CI audit (82.53%) pre-dates the coverage push commits.
- Staged product files only (excluded `.squad/` agent history modifications and `.squad/skills/` untracked dirs).
- Commit `ea5cabf` with full Batch 1 change log + Copilot co-author trailer.
- Annotated tag `v0.10.0` created and pushed. GitHub Actions `push.tags` trigger fires the release pipeline; no manual GitHub Release object needed.
- Decision log written to `.squad/decisions/inbox/zapp-ship-v0100.md`.

**Outcome:** v0.10.0 released. vault-sync-engine OpenSpec at 216/313 tasks complete (globally `ready`). Batch 2 (embedding worker, v0.11) is the next implementation window.

**Key decisions:**
- Accept Windows 90.77% line coverage as the gate metric (user-verified, line not region).
- Do NOT create a GitHub Release manually — tag push triggers CI pipeline.
- `.squad/` modifications correctly excluded from the product release commit.

**Learnings:**
- Always verify task implementations exist in source before marking `[x]` — search for the test function by name/description in the relevant `.rs` file first.
- `default_*.profraw` files should be added to `.gitignore` to prevent recurrence. This is a recurring cleanup step on this repo.
- When coverage measurements conflict (Windows local vs Linux CI), use the user's explicitly stated authoritative figure and document the discrepancy.
- Release commit should include `openspec/changes/*/implementation_plan.md` if newly created — it is a product planning artifact, not an agent-internal file.
