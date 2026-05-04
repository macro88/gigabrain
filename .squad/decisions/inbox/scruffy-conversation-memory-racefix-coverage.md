# Scruffy — conversation-memory race-fix coverage decision

- **Timestamp:** 2026-05-04T07:22:12.881+08:00
- **Scope:** `conversation-memory-foundations` tasks 2.2-2.5 after commit `d98e010`
- **Decision:** Treat the branch as still above the requested honest coverage floor based on the practical Windows lane (`cargo test -j 1` and `RUST_TEST_THREADS=1 cargo llvm-cov --lib --tests --summary-only -j 1`), but do not claim a full local branch-coverage rerun or a locally executed deterministic race regression from this environment.
- **Why:** The measured repo-wide line coverage is 90.18%, and `src\commands\put.rs` remains well-covered at 94.26% line coverage after the race fix. But `cargo llvm-cov --branch` on the available stable toolchain fails because branch coverage is nightly-only, and the new contender test in `src\commands\put.rs` is `#[cfg(unix)]`, so this Windows lane cannot honestly say it re-executed that exact race proof.
- **Test note:** No extra test was added in this lane because the missing proof is environmental, not a missing branch in the committed test suite.
