---
date: 2026-04-19
by: Bender (Tester)
scope: PR #46 — install profile flow
type: revision
---

# Bender re-re-revision of PR #46

## What was wrong

Three categories of defect remained after Fry, Leela, and Mom's revisions:

1. **T10–T13 tested a copied function body.** The `detect_profile()` function was pasted into the test file instead of re-sourcing `install.sh`. If production code changed, tests would silently pass against stale logic.

2. **No end-to-end `GBRAIN_NO_PROFILE=1` → `main()` coverage.** T16 verified the env-var-to-variable propagation, and T14 verified `--no-profile` flag through `main()`, but no test actually ran `main()` with the env var set and verified the profile was untouched. The documented pipe flow (`curl ... | GBRAIN_NO_PROFILE=1 sh`) was unproven.

3. **Env vars on the wrong side of `curl ... | sh`.** Five copy-paste examples and two recovery hints in `install.sh` placed `GBRAIN_VERSION`, `GBRAIN_CHANNEL`, or `GBRAIN_INSTALL_DIR` on the `curl` side of the pipe, where they apply to `curl` (which ignores them) rather than `sh` (which reads them). Users copying these examples would get default behavior silently.

## What was fixed

1. T10–T13 now re-source `install.sh` to restore production `detect_profile`.
2. New T17 re-sources `install.sh` with `GBRAIN_NO_PROFILE=1`, re-applies stubs, calls `main()`, and asserts the profile is empty.
3. All six `curl | sh` examples moved env vars to the `sh` side: `curl ... | VAR=val sh`. The two `install.sh` recovery hints were corrected the same way.
4. OpenSpec tasks.md A.2/A.3/A.4 aligned with actual `write_profile_line(profile, line)` signature.

## Verification

- 21/21 shell tests pass (was 20, +1 new T17)
- All cargo tests pass
- No remaining `GBRAIN_*` env vars on the `curl` side of any executable example

## Decision

This revision is scoped to test fidelity and doc correctness. No production logic was changed.
