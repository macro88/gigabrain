updated_at: 2026-04-18T00:00:00Z
focus_area: simplified-install — v0.9.0 shell-first rollout
active_issues: [D.2-npm-platform-blocked, D.5-publish-workflow-static-review-only]
active_branch: simplified-install
---

# What We're Focused On

**Active change:** `simplified-install` — shell-first `v0.9.0` installer rollout.

Implementation is complete. Two verification items remain environment-blocked.

**Done:**
- **A** ✅ — `scripts/install.sh` (POSIX, platform detect, SHA-256 verify, `GBRAIN_DB` tip)
- **B** ✅ — `packages/gbrain-npm/` scaffolding + `postinstall.js` + `.github/workflows/publish-npm.yml`
- **C** ✅ — README, `website/…/install.md`, `docs/getting-started.md` updated (shell-first, npm staged)
- **D.1** ✅ — `install.sh` smoke-tested against `v0.9.0` release asset shape
- **D.3** ✅ — `npm pack --dry-run` confirms binary not packed
- **D.4** ✅ — error paths validated (bad version, bad checksum, no-internet postinstall exit-0)

**Blocked / static-review only:**
- **D.2** ⚠️ — npm postinstall live test blocked: Windows host hits `EBADPLATFORM`; WSL has no working Node runtime; `v0.9.0` is not a real GitHub Release yet
- **D.5** ⚠️ — `publish-npm.yml` token-guard reviewed statically only; no local Actions runner; `npm publish --dry-run` blocked by existing `gbrain@1.3.1` on public registry

**Next gate:** D.2 + D.5 can close once `v0.9.0` is tagged and a macOS/Linux runner is available. npm public publication stays gated behind `NPM_TOKEN` secret and confirmed shell-installer test.
