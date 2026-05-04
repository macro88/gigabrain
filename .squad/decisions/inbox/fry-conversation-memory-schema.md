# Fry — conversation-memory-foundations schema slice

**Date:** 2026-05-04T07:22:12.881+08:00  
**Requested by:** macro88  
**Change:** conversation-memory-foundations

## Decision

Implement the first conversation-memory schema slice as a strict v8 foundation patch on top of the existing `pages.type` model, not by renaming the column to `kind` or introducing a migration lane. The new session-expression index must guard `json_extract(...)` with `json_valid(frontmatter)` so malformed-frontmatter rows remain tolerated while the new v8 artefacts are present.

## Why

The repo already ships `SCHEMA_VERSION = 8`, so the honest minimal slice is to add the new `superseded_by`/`extraction_queue` artefacts, strengthen tests, and keep v7 databases on the existing schema-mismatch/re-init path. A raw `json_extract(frontmatter, '$.session_id')` expression index broke existing malformed-frontmatter tolerance in unit tests, so the guarded form is the safe way to land the session lookup seam without widening this slice into frontmatter-cleanup or migration work.
