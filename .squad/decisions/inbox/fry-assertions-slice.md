---
date: 2026-04-15
author: fry
status: implemented
---

# Assertions/Check Slice Complete (Tasks 3.1-4.5)

## What shipped

- **src/core/assertions.rs**: Full implementation of triple extraction (`extract_assertions`) and contradiction detection (`check_assertions`). Three regex patterns (works_at, is_a, founded) with OnceLock-cached compilation. Temporal overlap checking with canonical pair ordering prevents duplicate contradictions. Resolved contradictions are never re-inserted.
- **src/commands/check.rs**: CLI with `--all` / slug modes, human-readable and JSON output, type filtering. Reads unresolved contradictions from the DB after running extract+check passes.
- **tests/assertions.rs**: 8 integration tests covering round-trip conflict detection, clean page checks, missing page errors, and CLI modes.
- **14 unit tests** in assertions.rs covering extraction, re-indexing (preserves manual assertions), same-page and cross-page conflicts, resolved dedup, non-overlapping validity windows.

## Design decisions

1. **Agent-only deletion on re-index**: `extract_assertions` deletes only `asserted_by = 'agent'` rows, preserving manual assertions across re-indexing. This was an improvement over the spec's "DELETE all assertions for page" approach.
2. **OnceLock for regex caching**: Regex patterns are compiled once per process via `OnceLock<Regex>`, avoiding per-call recompilation.
3. **Canonical pair ordering**: Contradiction pairs are sorted by `(page_id, page_slug, object)` to ensure deterministic insertion order and prevent duplicate detection from both directions.
4. **Contradiction dedup includes resolved**: Existing contradictions (whether resolved or unresolved) block re-insertion. This prevents noise when a human has already triaged a conflict.

## Test coverage

All 193 tests pass (up from 185). Clippy clean. Fmt clean. Phase 1 roundtrip tests unaffected.
