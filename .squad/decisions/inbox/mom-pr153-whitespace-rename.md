---
timestamp: 2026-05-04T07:22:12.881+08:00
author: Mom
change: conversation-memory-foundations
topic: whitespace-noop rename tracking
---

- Treat rename-only extracted whitespace no-ops as tracked-path moves, not deletions.
- Preserve the existing page/raw-import state, but move the `file_state` row onto the new relative path so future reconciles still see the file as tracked.
- Prove the seam with an `apply_reingest` test that renames an extracted preference without changing bytes, then asserts the new path is still classified as `unchanged`.
