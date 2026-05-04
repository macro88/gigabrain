---
recorded_at: 2026-05-04T07:22:12.881+08:00
author: Fry
change: conversation-memory-foundations
topic: session-tool-contract
---

# Decision

Wave 2 session tooling should persist `closed_at` in conversation frontmatter and store namespace-qualified queue session keys internally whenever the public `session_id` is only namespace-local.

# Why

- `memory_close_session` must return the original close timestamp on idempotent re-close, which is not recoverable truthfully from file mtime or queue state alone.
- The current `extraction_queue` schema has only `session_id`, so raw namespace-local ids would collapse unrelated `alpha/main` and `beta/main` sessions onto one pending row.
- Keeping the qualification internal preserves the public MCP contract (`session_id` stays namespace-local) while protecting queue semantics and future worker routing.

# Consequence

- Conversation files remain the source of truth for session lifecycle because `closed_at` lives with the session frontmatter.
- Queue producers and future workers must treat `extraction_queue.session_id` as an internal routing key, not blindly as the public caller-facing session id.
