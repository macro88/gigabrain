# Vault-Sync Engine Foundation Slice A — Session Log

**Date:** 2026-04-22  
**Agent:** Fry  
**OpenSpec:** `openspec/changes/vault-sync-engine/`  
**Status:** Delivered  

## Overview

Fry implemented the first coherent foundation slice of the vault-sync-engine OpenSpec change, establishing the v5 schema architecture and collections abstraction module. This slice provides the structural foundation for multi-collection support without yet wiring into commands or the reconciler.

## Scope: Schema v5 Foundation (1.1–1.6)

### Completed Tasks

- **1.1** Implement v5 schema DDL (collections, file_state, embedding_jobs tables)
- **1.1a** Create unique index on pages(uuid)
- **1.2** Add index on pages.quarantined_at for efficient filtering
- **1.3** Update brain_config to write schema_version = 5 on init
- **1.4** Implement db::open version detection with v4→v5 migration rejection
- **1.5** Update FTS5 triggers to exclude quarantined pages
- **1.6** Verify vector search paths align with new schema (no code change needed)

### Design Decisions

**Breaking Change by Design:** v5 rejects v4 databases with actionable error:
```
Database schema version 4 is older than required version 5. GigaBrain v5 requires re-initialization.
Backup your data, then run: rm {path} && gbrain init
```

**Why:** Zero users, clean redesign opportunity. Schema changes include:
- `pages`: added `collection_id`, `uuid`, `quarantined_at`
- `links`: added `source_kind` for programmatic vs wiki-link provenance tracking
- `contradictions`: `other_page_id` now `ON DELETE CASCADE` (was `SET NULL`)
- `knowledge_gaps`: new `page_id` column for slug-bound gap tracking
- Removed `ingest_log` (replaced by `file_state` + collection sync model)

## Scope: Collections Foundation (2.1–2.6)

### Completed Tasks

- **2.1** Create `src/core/collections.rs`
- **2.2** Implement validators: `validate_collection_name()`, `validate_relative_path()`
- **2.3** Implement CRUD: `get_by_name()`, `get_write_target()`, `parse_slug()`
- **2.4** Define `OpKind` enum (Read, WriteCreate, WriteUpdate, WriteAdmin)
- **2.5** Implement path traversal protection (reject `..`, absolute paths, NUL bytes, empty segments)
- **2.6** Implement slug resolution pipeline with `Resolved | NotFound | Ambiguous` returns

### Design Decisions

**Slug Resolution by OpKind:** Prevents silent wrong-collection writes by classifying the intent:
- **Read:** Exactly-one match or Ambiguous
- **WriteCreate:** Zero owners → write-target; one owner AND is write-target → that collection; else Ambiguous
- **WriteUpdate/WriteAdmin:** Exactly-one match or Ambiguous/NotFound

**Ambiguity Error Structure:** `SlugResolution::Ambiguous` carries `Vec<AmbiguityCandidate>` with:
```rust
pub struct AmbiguityCandidate {
    pub collection_name: String,
    pub full_address: String,  // e.g., "work::notes/meeting"
}
```
Enables MCP clients and CLI to surface structured resolution hints.

**Path Traversal Protection:** Validators reject:
- `..` segments (directory traversal)
- Absolute paths (Unix `/`, Windows `C:\`)
- Empty segments (consecutive slashes)
- NUL bytes

## Testing

**Schema Tests:** Updated 19 existing db tests to expect v5 schema. All pass.

**Collections Unit Tests:** 8 new tests:
- Name validation (valid names, reserved keywords, length bounds)
- Path validation (traversal rejection, absolute path rejection, empty segment rejection)
- Slug resolution under each OpKind

**All Gates:** `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` all pass.

## Deferred Items

Deferred to later slices per scope charter:
- Platform-specific fd-safety primitives (`rustix`/`nix`) — needs `#[cfg(unix)]` gating
- `knowledge_gaps.page_id` wiring — requires `gaps.rs` integration
- Command wiring (init, serve, get, put) — requires reconciler + watcher

## Files Changed

| File | Change |
|------|--------|
| `src/schema.sql` | v5 DDL: new tables + modified columns + indexes |
| `src/core/db.rs` | Version detection + v4 rejection logic |
| `src/core/mod.rs` | Register collections module |
| `src/core/collections.rs` | NEW: validators, CRUD, slug resolution |
| `openspec/changes/vault-sync-engine/tasks.md` | Checkboxes 1.1–1.6, 2.1–2.6 marked complete |

## Next Phase

**Slice B:** Wire collections into commands:
1. Update `init` to create default collection
2. Make `get`, `put`, `query`, `search` collection-aware
3. Update MCP tool signatures to accept collection context

## Context

This slice is part of the vault-sync-engine OpenSpec change, which establishes a foundation for:
- Multi-collection organization
- Reconciliation-based sync model
- File-state tracking for incremental ingest
- Platform-gated fs-safety (Unix + Windows paths)

The slice is intentionally schema + foundation types only, kept coherent and testable independently.
