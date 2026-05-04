# Conversation Memory Core Coverage

Use this when a conversation-memory slice lands before full MCP wiring.

## Pattern

1. Put parse/render round-trip and malformed-format proofs next to `src\core\conversation\format.rs`.
2. Put append durability, day rollover, namespace nesting, and dedicated-root layout proofs in `tests\conversation_turn_capture.rs`.
3. Put queue collapse, dequeue ordering, retry/fail, lease-expiry, and restart-persistence proofs in `tests\extraction_queue.rs`.

## Why

These slices are storage plumbing first. Waiting for higher-level tool wiring leaves honest core behavior uncovered and drags unrelated seams into the test lane.

## Quaid fit

- Matches `conversation-memory-foundations` tasks `4.*`-`6.*` and `11.*`.
- Keeps coverage truthful on Windows where the core seams can be exercised locally before the full daemon/tool path is present.
