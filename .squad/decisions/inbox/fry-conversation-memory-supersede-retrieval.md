---
recorded_at: 2026-05-04T07:22:12.881+08:00
author: Fry
change: conversation-memory-foundations
topic: supersede-retrieval-surface
---

# Decision

`memory_get` should return structured JSON for the supersede-chain slice instead of rendered markdown so the caller can reliably read `superseded_by` and `supersedes` pointers without reparsing frontmatter text.

# Why

- The OpenSpec requirement for task 3.5 is about machine-readable chain traversal metadata, not presentation.
- MCP callers need a stable successor pointer surface; embedding it only in rendered markdown would force brittle text parsing.
- The CLI `get` surface remains markdown-oriented, so this narrows the structured change to MCP where it is needed.

# Consequence

- MCP consumers now get canonical slugs plus explicit `superseded_by` / `supersedes` fields.
- Future chain-aware tooling can build on `memory_get` without another response-shape change.
