## Mom — conversation-memory-foundations slice 2 revision

**Date:** 2026-05-04T07:22:12.881+08:00  
**Requested by:** macro88  
**Change:** conversation-memory-foundations

## Decision

Keep supersede-chain validation in two places on the put path: preflight it before any Unix vault rename machinery starts, and keep the existing transactional reconcile as the final race backstop.

## Why

- Preflight is what makes the non-head supersede refusal honest on the real write-through seam; otherwise the vault can mutate before the typed conflict returns.
- The transactional reconcile still has to guard the DB edge because another writer can change chain state after preflight and before commit.

## Evidence

- `src/commands/put.rs` now validates `supersedes` before sentinel/tempfile/rename work.
- The new Unix test proves rejected non-head supersedes leave vault bytes, active raw-import bytes, and recovery state unchanged while still surfacing `SupersedeConflictError`.
