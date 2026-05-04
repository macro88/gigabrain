---
recorded_at: 2026-05-04T07:22:12.881+08:00
author: Zapp
change: conversation-memory-foundations
topic: pr-153-last-product-slice
---

# Decision

Refresh draft PR #153 so it says `memory_close_action` is approved and the only remaining product scope is the file-edit/history-preservation slice, which is the active landing seam under Nibbler's pre-gated constraints rather than a shipped claim.

# Why

- Professor approved the `memory_close_action` slice at commit `ecd5513`, and Scruffy's focused coverage confirms the narrow MCP/OCC contract, so keeping that seam in "in flight" copy would now be stale.
- The remaining open tasks are the file-edit/history seam (`10.x`, `12.4`, `12.5`), and Nibbler already defined the non-negotiable landing constraints: archive-before-overwrite in one fail-closed path, linear-chain preservation on edited heads, whitespace-only total no-ops, extracted/type gating, and no `_history` watcher recursion.
- A fresh merge simulation against current `main` still reproduces six OpenSpec add/add conflicts, so the draft should stay draft and report that exact count without implying the final slice is merge-ready.
