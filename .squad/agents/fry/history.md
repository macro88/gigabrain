# fry history

- [2026-04-29T07-04-07Z] History summarized and archived

## Learnings

- [2026-04-29T20:33:01.970+08:00] Batch 3 recon: `src\commands\collection.rs` still exposes deferred `write_memory_id` on `CollectionAddArgs`; `CollectionAction` has no `migrate-uuids` variant yet, so Batch 3 must add new CLI args/dispatch and retire the defer test.
- [2026-04-29T20:33:01.970+08:00] UUID/frontmatter naming is still `memory_id` across `src\core\page_uuid.rs`, `src\core\markdown.rs`, `src\core\reconciler.rs`, `src\core\vault_sync.rs`, `src\commands\put.rs`, and `tests\roundtrip_raw.rs`, while vault-sync OpenSpec Batch 3 language says `quaid_id`; this is the main contract seam to settle before write-back lands.
- [2026-04-29T20:33:01.970+08:00] Rename-before-commit production logic currently lives in `src\commands\put.rs::persist_with_vault_write`, while `src\core\vault_sync.rs` only has test-only writer crash-core helpers; Batch 3 should reuse/extract that path rather than duplicate raw_import/file_state rotation logic.
- [2026-04-29T20:33:01.970+08:00] Live-owner data already exists in `serve_sessions(pid, host, heartbeat_at)` plus `collection_owners`, but `VaultSyncError::ServeOwnsCollectionError` only carries `owner_session_id`; Batch 3 bulk-write guards will need an owner-detail lookup seam before CLI can report pid/host truthfully.
- [2026-04-29T20:33:01.970+08:00] Batch 3 landed by routing UUID write-back through `src\commands\put.rs::put_from_string`, so `collection add --write-quaid-id` and `collection migrate-uuids` reuse the production sentinel/tempfile/rename/file_state/raw_imports path instead of duplicating a weaker writer.
- [2026-04-29T20:33:01.970+08:00] `src\core\page_uuid.rs` now accepts legacy `memory_id` on read, but `src\core\markdown.rs::render_page` canonicalizes every persisted/exported write to `quaid_id`; migration commands intentionally rewrite files that still lack `quaid_id`. 

## 2026-04-29T13:57:48Z — Memory Cycle: Batch 3 Validation Gate FAIL

- Scruffy validation: **REJECTED** (Windows lane 90.52% line, 89.03% region; UUID write-back proof Unix-only; compile blockers at vault_sync.rs:1917 & :3772)
- Mom: Revision cycle RUNNING; Fry locked out pending completion
- Decisions merged: 1 inbox entry
- Archive: 22 entries moved to decisions-archive.md (file was 438KB)
