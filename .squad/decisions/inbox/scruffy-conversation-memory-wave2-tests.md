---
agent: scruffy
date: 2026-05-04T07:22:12.881+08:00
change: conversation-memory-foundations
---

# Decision: namespace-isolated queue proofs use composite internal session keys

For Wave 2 conversation-memory coverage, I treated queue isolation as an internal storage concern rather than widening the public MCP contract. The proof lane assumes extraction rows are keyed internally as `<namespace>::<session_id>` while file paths remain `<namespace>/conversations/<date>/<session-id>.md`.

Why:
- the queue schema in this wave still stores a single `session_id` text field
- namespace isolation must prove "same session id, different namespace" does not collapse to one pending row
- keeping the composite key internal avoids inventing a new public session identifier format

Test impact:
- end-to-end namespace isolation checks should assert two pending rows, not one collapsed row
- close-session and add-turn queue assertions should use the effective internal key only when they are inspecting raw queue rows directly
