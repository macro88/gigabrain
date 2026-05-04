---
timestamp: 2026-05-04T07:22:12.881+08:00
author: Mom
change: conversation-memory-foundations
topic: file-edit supersede closure
---

- Preserve the manual-edit chain by inserting one archived predecessor row and rewiring any prior predecessor to point at that archive before updating the live head.
- Treat whitespace-only extracted edits as semantic no-ops: no page mutation, no raw-import rotation, no file-state refresh.
- Exclude `extracted/_history/**/*.md` from watcher dirty-path classification and reconciler ingestion so opt-in sidecars cannot become live pages or self-archive recursively.
