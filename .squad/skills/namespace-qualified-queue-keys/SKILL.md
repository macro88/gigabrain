---
name: "namespace-qualified-queue-keys"
description: "Protect namespace-local identifiers when a shared queue schema only has one key column"
domain: "stateful storage, queueing, namespaces"
confidence: "medium"
source: "earned"
---

## Context

Use this when callers see identifiers as namespace-local (`session_id`, `slug`, etc.) but the backing queue or job table stores only a single key column. Without an internal qualification step, identical ids from different namespaces collapse onto the same pending row and silently corrupt queue semantics.

## Patterns

1. **Keep the public id stable.** Do not widen the external MCP/CLI contract if the caller should still think in namespace-local ids.
2. **Qualify only at the storage boundary.** Build an internal queue key like `<namespace>::<id>` right before enqueue/dequeue bookkeeping.
3. **Preserve the natural path alongside the key.** Store the file/path payload unchanged so workers can still open the right artifact without reverse-engineering the internal key.
4. **Test both file isolation and queue isolation.** It is not enough to prove two namespaces write different files; assert they also produce distinct pending jobs.

## Anti-patterns

- Reusing raw namespace-local ids in a shared queue table
- Widening the public API just to patch an internal queue collision
- Assuming file-path isolation automatically prevents queue collapse
