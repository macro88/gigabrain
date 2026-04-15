### Fry SG-6 Fixes — 2026-04-15

**Verdict:** IMPLEMENTED (pending Nibbler re-review)

Addressed all 5 categories from Nibbler's SG-6 rejection of `src/mcp/server.rs`:

1. **OCC bypass closed.** `brain_put` now rejects updates to existing pages when `expected_version` is `None`. Returns `-32009` with `current_version` in error data so the client knows what to send. New page creation (INSERT path) still allows `None`.

2. **Slug + content validation added.** `validate_slug()` enforces `[a-z0-9/_-]` charset and 512-char max. `validate_content()` caps at 1 MB. Both return `-32602` (invalid params). Applied at top of `brain_get` and `brain_put`.

3. **Error code consistency.** Centralized `map_db_error(rusqlite::Error)` correctly routes SQLITE_CONSTRAINT_UNIQUE → `-32009`, FTS5 parse errors → `-32602`, all others → `-32003`. `map_search_error(SearchError)` delegates to `map_db_error` for SQLite variants. No more generic `-32003` leaking for distinguishable error classes.

4. **Resource exhaustion capped.** `brain_list`, `brain_query`, `brain_search` all clamp `limit` to `MAX_LIMIT = 1000`. Added `limit` field to `BrainQueryInput` and `BrainSearchInput` (previously missing vs spec). Results are truncated after retrieval.

5. **Mutex poisoning recovery.** All `self.db.lock()` calls now use `unwrap_or_else(|e| e.into_inner())` which recovers the underlying connection from a poisoned mutex. Safe for SQLite connections — they aren't corrupted by a handler panic.

**Tests:** 304 pass (8 new: OCC bypass rejection, invalid slug, oversized content, empty slug, plus existing tests updated). `cargo clippy -- -D warnings` clean.

**Commit:** `5886ec2` on `phase1/p1-core-storage-cli`.
