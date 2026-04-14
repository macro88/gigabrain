## 1. Branch Setup

- [x] 1.1 Create and checkout branch `phase1/p1-core-storage-cli` from `main`
- [x] 1.2 Verify `cargo check` passes on the stub codebase before touching any implementation

## 2. Core Types

- [x] 2.1 Implement `src/core/types.rs`: define `Page` struct (all v4 fields: slug, type, title, summary, compiled_truth, timeline, frontmatter, wing, room, version, timestamps)
- [x] 2.2 Implement `src/core/types.rs`: define `Link` struct (id, from_slug, to_slug, relationship, valid_from, valid_until, created_at)
- [x] 2.3 Implement `src/core/types.rs`: define `Tag`, `TimelineEntry`, `SearchResult`, `KnowledgeGap`, `IngestRecord` structs
- [x] 2.4 Implement `src/core/types.rs`: define `OccError` (Conflict { current_version }) and `DbError` thiserror enums
- [x] 2.5 Implement `src/core/types.rs`: define `SearchMergeStrategy` enum (SetUnion, Rrf)
- [x] 2.6 Run `cargo check` — zero errors before moving on

## 3. Database Layer

- [ ] 3.1 Implement `src/core/db.rs`: `open(path: &str) -> Result<Connection>` — rusqlite open, apply `include_str!("../schema.sql")` DDL, `PRAGMA journal_mode = WAL`, `PRAGMA foreign_keys = ON`, load sqlite-vec extension
- [ ] 3.2 Implement `src/core/db.rs`: `compact(conn: &Connection) -> Result<()>` — `PRAGMA wal_checkpoint(TRUNCATE)`
- [ ] 3.3 Implement `src/core/db.rs`: `set_version(conn: &Connection) -> Result<()>` — set `PRAGMA user_version = 4`
- [ ] 3.4 Write unit test: `db::open` on a temp path creates all expected tables
- [ ] 3.5 Run `cargo test db` — test passes

## 4. Markdown Parsing

- [ ] 4.1 Implement `src/core/markdown.rs`: `parse_frontmatter(raw: &str) -> (HashMap<String, String>, String)` — extract YAML frontmatter block; return empty map if none
- [ ] 4.2 Implement `src/core/markdown.rs`: `split_content(body: &str) -> (String, String)` — split at first lone `---` line into (compiled_truth, timeline)
- [ ] 4.3 Implement `src/core/markdown.rs`: `extract_summary(content: &str) -> String` — return first non-heading paragraph (max 200 chars)
- [ ] 4.4 Implement `src/core/markdown.rs`: `render_page(page: &Page) -> String` — reconstruct: YAML frontmatter + compiled_truth + `\n---\n` + timeline
- [ ] 4.5 Write unit tests: round-trip frontmatter parse → render is identical; split at boundary; no-frontmatter case; no-timeline case
- [ ] 4.6 Run `cargo test markdown` — all tests pass

## 5. Palace Metadata

- [ ] 5.1 Implement `src/core/palace.rs`: `derive_wing(slug: &str) -> String` — first path segment (e.g. `people/alice` → `people`); `"general"` for flat slugs
- [ ] 5.2 Implement `src/core/palace.rs`: `derive_room(content: &str) -> String` — most-frequent `##` heading in compiled_truth; `""` if none (room-level filtering deferred to Phase 2)
- [ ] 5.3 Implement `src/core/palace.rs`: `classify_intent(query: &str) -> Option<String>` — simple heuristic: if query contains a slug-like token return the wing segment, else None
- [ ] 5.4 Write unit tests: `derive_wing` for nested slug, flat slug, empty slug
- [ ] 5.5 Run `cargo test palace` — tests pass

## 6. CRUD Commands

- [ ] 6.1 Implement `src/commands/init.rs`: create brain.db at path, print success or "already exists" message
- [ ] 6.2 Implement `src/commands/get.rs`: read page by slug, render to stdout; exit 1 with stderr message if not found
- [ ] 6.3 Implement `src/commands/put.rs`: read stdin markdown, parse frontmatter, derive wing/room/summary, upsert with OCC (`--expected-version` flag); print resulting version on success
- [ ] 6.4 Implement `src/commands/list.rs`: list pages with `--wing`, `--type`, `--limit` filters; print `slug: summary` per line
- [ ] 6.5 Implement `src/commands/stats.rs`: print page count by type, link count, embedding count, file size
- [ ] 6.6 Implement `src/commands/tags.rs`: list tags; `--add` / `--remove` with OCC-safe page update
- [ ] 6.7 Implement `src/commands/link.rs`: insert into `links` table; `--relationship`, `--valid-from`, `--valid-until` flags; also handle closing a link (update `valid_until`)
- [ ] 6.8 Implement `src/commands/compact.rs`: call `db::compact`, print checkpoint result
- [ ] 6.9 Wire all commands into `src/main.rs` clap dispatch (replace `todo!()` stubs)
- [ ] 6.10 Run `cargo test` — no regressions; manually smoke-test `gbrain init`, `gbrain put`, `gbrain get`

## 7. FTS5 and Keyword Search

- [ ] 7.1 Implement `src/core/fts.rs`: `search_fts(query: &str, wing_filter: Option<&str>, conn: &Connection) -> Result<Vec<SearchResult>>` — BM25-ranked FTS5 query with optional wing filter
- [ ] 7.2 Implement `src/commands/search.rs`: call `search_fts`, print results; `--wing`, `--limit` flags
- [ ] 7.3 Write unit tests: FTS5 search returns correct results; wing filter restricts results; empty DB returns empty vec
- [ ] 7.4 Run `cargo test fts` — tests pass

## 8. Candle Embeddings

- [ ] 8.1 Implement `src/core/inference.rs`: `EmbeddingModel` struct wrapping candle model + tokenizer
- [ ] 8.2 Implement `src/core/inference.rs`: `ensure_model() -> &'static EmbeddingModel` — OnceLock lazy initialisation with BGE-small-en-v1.5 via `include_bytes!`
- [ ] 8.3 Implement `src/core/inference.rs`: `embed(text: &str) -> Result<Vec<f32>>` — tokenize, forward pass, L2-normalize output; return `InferenceError::EmptyInput` for empty string
- [ ] 8.4 Implement `src/core/inference.rs`: `search_vec(query: &str, k: usize, wing_filter: Option<&str>, conn: &Connection) -> Result<Vec<SearchResult>>` — embed query, query vec0 table, return top-k with wing filter
- [ ] 8.5 Implement `src/core/chunking.rs`: `chunk_page(page: &Page) -> Vec<Chunk>` — split compiled_truth at `##` boundaries + timeline entries individually; set content_hash (SHA-256), token_count, heading_path per chunk
- [ ] 8.6 Implement `src/commands/embed.rs`: embed single page; `--all` for all pages; `--stale` for changed chunks (compare content_hash)
- [ ] 8.7 Write unit tests: embed returns 384-dim L2-normalised vector; chunking splits correctly; empty string returns error
- [ ] 8.8 Run `cargo test inference chunking` — tests pass (note: first run is slow due to model init)

## 9. Hybrid Search

- [ ] 9.1 Implement `src/core/search.rs`: `hybrid_search(query: &str, wing_filter: Option<&str>, conn: &Connection) -> Result<Vec<SearchResult>>`
  - Stage 1: SMS check (exact slug or `[[slug]]` match → return immediately)
  - Stage 2: FTS5 fan-out + vec0 fan-out
  - Stage 3: set-union merge (default) or RRF (from config); deduplicate by slug; combine scores
- [ ] 9.2 Implement `src/core/search.rs`: `read_merge_strategy(conn: &Connection) -> SearchMergeStrategy` — read from `config` table, default SetUnion
- [ ] 9.3 Implement `src/commands/query.rs`: call `hybrid_search`, print results with slug + summary; `--wing`, `--limit`, `--depth` (Phase 1: depth ignored, full page returned), `--token-budget` (Phase 1: hard cap on output chars)
- [ ] 9.4 Write unit tests: set-union dedup; SMS short-circuit fires on exact slug; RRF switchable via config; wing filter restricts both sub-queries
- [ ] 9.5 Run `cargo test search` — tests pass

## 10. Ingest and Import/Export

- [ ] 10.1 Implement `src/core/links.rs`: `extract_links(content: &str) -> Vec<String>` — find `[[slug]]` patterns in markdown; `resolve_slug(raw: &str) -> String` — normalise to kebab-case path
- [ ] 10.2 Implement `src/core/migrate.rs`: `import_dir(path: &Path, conn: &mut Connection) -> Result<ImportSummary>` — recursive scan, parse, transaction insert, SHA-256 idempotency via `ingest_log`, derive wing/room/summary per page; embed after import
- [ ] 10.3 Implement `src/core/migrate.rs`: `export_dir(output: &Path, conn: &Connection) -> Result<()>` — for each page, reconstruct markdown, write to `<output>/<slug>.md` with parent dirs
- [ ] 10.4 Implement `src/commands/import.rs`: call `import_dir`; `--validate-only` flag (parse but no writes); print import summary
- [ ] 10.5 Implement `src/commands/export.rs`: call `export_dir`; print export summary
- [ ] 10.6 Implement `src/commands/ingest.rs`: single-file ingest with SHA-256 idempotency; `--force` flag
- [ ] 10.7 Implement `src/commands/timeline.rs`: print timeline entries for a slug in chronological order
- [ ] 10.8 Write `tests/roundtrip_semantic.rs`: import `tests/fixtures/`, export, re-import, assert page count and content hashes match
- [ ] 10.9 Write `tests/roundtrip_raw.rs`: import canonical fixture, export with `--raw --import-id`, byte-exact diff
- [ ] 10.10 Run `cargo test roundtrip` — both tests pass

## 11. MCP Server

- [ ] 11.1 Implement `src/mcp/server.rs`: register 5 tool definitions (`brain_get`, `brain_put`, `brain_query`, `brain_search`, `brain_list`) with JSON schema for each tool's input
- [ ] 11.2 Implement `src/mcp/server.rs`: `brain_get` handler — delegate to core get; return rendered markdown or JSON-RPC error `-32001`
- [ ] 11.3 Implement `src/mcp/server.rs`: `brain_put` handler — parse content + optional `expected_version`; delegate to core put with OCC; return `{"version": N}` or error `-32009` with current version
- [ ] 11.4 Implement `src/mcp/server.rs`: `brain_query` handler — delegate to `hybrid_search`; return JSON array of results
- [ ] 11.5 Implement `src/mcp/server.rs`: `brain_search` handler — delegate to `search_fts`; return JSON array of results
- [ ] 11.6 Implement `src/mcp/server.rs`: `brain_list` handler — delegate to list query; return JSON array
- [ ] 11.7 Implement `src/mcp/server.rs`: MCP error code mapping table (`OccError` → `-32009`, not found → `-32001`, parse error → `-32002`, db error → `-32003`)
- [ ] 11.8 Implement `src/commands/serve.rs`: open DB, start rmcp stdio server with `#[tokio::main]`, block until stdin close
- [ ] 11.9 Wire `serve` command in `src/main.rs`
- [ ] 11.10 Manual integration test: `gbrain serve` connects to Claude Code; send `tools/list` and verify 5 tools present; send `brain_get` for a known page

## 12. Finalization and Ship Gate

- [ ] 12.1 Implement `src/commands/config.rs`: `config get <KEY>`, `config set <KEY> <VALUE>`, `config list` — read/write `config` table
- [ ] 12.2 Implement `src/commands/version.rs`: print binary version from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`
- [ ] 12.3 Run full test suite: `cargo test` — all tests pass, zero warnings (clippy clean)
- [ ] 12.4 Build musl static binary: `cross build --release --target x86_64-unknown-linux-musl` — verify `ldd` output: "not a dynamic executable"
- [ ] 12.5 Run `gbrain import <real-corpus>` → `gbrain export` → re-import — confirm zero semantic diff
- [ ] 12.6 Establish BEIR nDCG@10 baseline: run `benchmarks/` harness on NQ subset and record score in `benchmarks/baseline.md`
- [ ] 12.7 Update `skills/ingest/SKILL.md` with source attribution format and filing disambiguation rules (Phase 1 embedded skills requirement)
- [ ] 12.8 Update `skills/query/SKILL.md` with Phase 1 query workflow (hybrid search, wing filter usage)
- [ ] 12.9 Open PR from `phase1/p1-core-storage-cli` → `main`, link to Phase 1 GitHub issue, request Nibbler adversarial review on MCP server
- [ ] 12.10 Address Nibbler + Professor review feedback; merge when all ship gate criteria pass
