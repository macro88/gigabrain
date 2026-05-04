# Scruffy — conversation-memory queue/core coverage decision

- **Timestamp:** 2026-05-04T07:22:12.881+08:00
- **Scope:** `conversation-memory-foundations` tasks 4.1-6.6 and 11.1-11.4
- **Decision:** Cover the slice at the core seams, not through premature MCP wiring: keep round-trip and parse failures in `src\core\conversation\format.rs`, append/path/layout proofs in `tests\conversation_turn_capture.rs`, and queue collapse/order/retry/lease proofs in `tests\extraction_queue.rs`.
- **Why:** This slice is plumbing. If the tests wait for `memory_add_turn` / `memory_close_session` tool wiring, coverage will lag the landed behavior and the branch will look under-tested for the wrong reason. The dedicated-collection path is therefore proved through the core root resolver and append path, with the current implementation creating a companion `*-memory` collection/root on first use.
- **Coverage note:** On this Windows lane, `cargo test -j 1` passes and `cargo llvm-cov --lib --tests --summary-only --no-clean -j 1` reports 90.02% total line coverage, so the slice clears the requested floor without pretending branch coverage was rerun.
