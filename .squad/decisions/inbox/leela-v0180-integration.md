---
recorded_at: 2026-05-04T07:22:12.881+08:00
author: Leela
change: release-v0.18.0
topic: remote-head-reintegration
---

# Decision

Integrate the `v0.18.0` release-prep side-lane commits onto `feat/slm-conversation-mem` from a clean sibling worktree rooted at `origin/feat/slm-conversation-mem`, then update PR #153 so it states that conversation-memory foundations are complete and only review, CI, and release-lane completion remain.

# Why

- The parked `D:\repos\quaid` checkout is dirty and stale, so it is not a trustworthy place to merge or push release-bound work.
- Fry's manifest/release-lane prep and Amy's doc-truth pass were stacked off an older branch point; cherry-picking onto the current remote PR head preserves later fmt/clippy fixes already on `feat/slm-conversation-mem`.
- With all 70/70 OpenSpec tasks closed, the PR body must stop implying any product seam is still in flight; the only honest remaining work is reviewer sign-off, CI, and the eventual release cut.

# Consequence

- `feat/slm-conversation-mem` remains the single truthful integration branch for `v0.18.0`, but no tag or GitHub Release should be created until review and CI clear.
- Future release-lane reintegration should treat the remote PR head, not a parked local checkout, as the source of truth whenever side-lane commits need to be folded back in.
