# Orchestration Log: Scruffy — Model Resolution Tests

**Date:** 2026-04-18T12:32:00Z  
**Agent:** Scruffy  
**Change:** flexible-model-resolution (Issue #60) — unit test coverage  
**Commit:** 20ffdd6  

## Task

Added focused unit test coverage in `src/core/inference.rs`:
- Tests for new alias mappings: `medium → base`, `max → m3`.
- Normalization test: known full Hugging Face IDs resolve to canonical aliases with correct dimensions.
- Custom-model acceptance test: arbitrary `owner/repo` IDs stay as `custom` with `embedding_dim = 0`.

## Validation

- `cargo test --quiet`: all passed.
- Coverage completes OpenSpec task 5 (tests section).

## Outcome

**Status:** APPROVED  
**Decision:** Document as `scruffy-model-resolution-tests.md` in decisions inbox.
