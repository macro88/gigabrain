# Project Context

- **Owner:** macro88
- **Project:** GigaBrain — local-first Rust knowledge brain
- **Stack:** Rust, rusqlite, SQLite FTS5, sqlite-vec, candle + BGE-small-en-v1.5, clap, rmcp
- **Created:** 2026-04-13T14:22:20Z

## Learnings

- The core implementation target is a Rust CLI plus MCP server.
- The system is intentionally local-first and zero-network for embeddings.
- Every meaningful implementation starts with an OpenSpec proposal.
- Always run `cargo fmt --all` before committing Rust code — CI enforces `cargo fmt --check` as the first gate and will skip all subsequent steps (clippy, check, test) if formatting fails.
- Local Windows dev environment lacks MSVC SDK libs (`msvcrt.lib`), so clippy/build/test cannot run locally. Use CI (Linux) for full validation. Only `cargo fmt` works locally.
- CI runs `cargo clippy -- -D warnings` which treats all warnings as errors. Stub functions with `todo!()` bodies must prefix all params with `_` to avoid unused variable errors.
- `version.rs` was removed — it was dead code never referenced from `mod.rs` or `main.rs`.
- PR #9 review (2026-04-14): Copilot automated reviewer caught 9 issues. 7 applied in Sprint 0 scope (CLI contract alignment, schema CHECK, docs fixes, Cargo.lock, hygiene). CI clippy `-D warnings` left as-is since stubs already fixed. Repo hygiene pass removed `gh_diagnostic.py`, one-time session artifacts.
- CLI contract must match `docs/spec.md` exactly — the spec defines the scaffold's surface. `default_db_path()` must resolve to `./brain.db`, not `$HOME/brain.db`.
- `init` and `version` commands don't require a database connection; dispatch them before `db::open()` in main.
- Reviewed and proposed adoption of `rust-best-practices` skill (Apollo GraphQL handbook, 9 chapters) at `.agents/skills/rust-best-practices/`. Decision note at `.squad/decisions/inbox/fry-rust-skill-adoption.md`. Key caveats: `#[expect]` needs MSRV ≥1.81, `rustfmt` import grouping needs nightly, snapshot testing (`insta`) deferred to Phase 1 test work.
- Error handling split already matches skill guidance: `thiserror` for `src/core/`, `anyhow` for `src/commands/` and `main.rs`.

## 2026-04-14 Update

- Rust skill adoption recommendation delivered and accepted by team. Fry's work product captured in `.squad/orchestration-log/2026-04-14T01-53-00Z-fry.md` and merged into team decisions ledger.
- Decision now stands: adopt `rust-best-practices` skill as standing guidance for all Rust implementation and review. Key caveats documented for future reference: MSRV ≥1.81 for `#[expect]`, nightly-only for `rustfmt` import grouping, snapshot testing deferred to Phase 1.
- Team coordination: orchestration logs written, session log recorded, inbox decisions merged and deleted, cross-agent updates applied. Ready for git commit.
