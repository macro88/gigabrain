## Leela — conversation-memory foundations Wave 1 truth repair

**Date:** 2026-05-04T07:22:12.881+08:00  
**Requested by:** macro88  
**Change:** conversation-memory-foundations

## Decision

Truth-repair the Wave 1 OpenSpec artifacts to describe the shipped queue lease recovery as a fixed 300-second window and the shipped `memory.location` routing/tests as conversation-root-only.

## Why

- `src/core/conversation/queue.rs` hardcodes `DEFAULT_LEASE_EXPIRY_SECONDS = 300`; there is no lease-expiry config key or runtime config read.
- `src/core/conversation/turn_writer.rs` and `tests/conversation_turn_capture.rs` only resolve and prove conversation-file placement under `memory.location`.
- Leaving broader wording in checked tasks/spec text keeps the Wave 1 closure dishonest even though the underlying code is correct for the narrower shipped slice.

## Scope preserved

- No product code changes are part of this repair.
- Future extracted-root routing remains with the later extracted-fact/file-edit work; this repair only narrows wording to the shipped Wave 1 surface.
