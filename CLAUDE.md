# Quaid

Personal AI memory. SQLite + FTS5 + local vector embeddings. One binary.

## Architecture

```
Consumers (Claude Code, any MCP client)
    ↓ stdio JSON-RPC 2.0
src/mcp/server.rs          — MCP tool definitions + handlers
    ↓
src/main.rs                — clap CLI dispatch
    ↓
src/commands/              — one file per command
    ↓
src/core/                  — library: DB, search, embeddings, parsing
    ↓
memory.db                  — SQLite: pages + FTS5 + vec0 + links + assertions
```

**Thin harness, fat skills.** The binary is plumbing. All agent workflows live in `skills/*/SKILL.md`.

## Key files

| File | Purpose |
|------|---------|
| `src/core/db.rs` | rusqlite connection, schema init, WAL, sqlite-vec load |
| `src/core/types.rs` | Page, Link, Tag, SearchResult, KnowledgeGap, etc. |
| `src/core/markdown.rs` | `parse_frontmatter()`, `split_content()`, `extract_summary()`, `render_page()` |
| `src/core/fts.rs` | FTS5 search: `search_fts(query, wing_filter, db)` → ranked results |
| `src/core/inference.rs` | candle init, `embed(text)`, `search_vec(query, k, wing_filter, db)` |
| `src/core/search.rs` | `hybrid_search(query, db)`: SMS + palace filter + FTS5 + vec + set-union |
| `src/core/progressive.rs` | `progressive_retrieve(results, budget, depth)`: token-budget expansion |
| `src/core/palace.rs` | `derive_wing(slug)`, `derive_room(content)`, `classify_intent(query)` |
| `src/core/novelty.rs` | `check_novelty(content, page, db)`: Jaccard + cosine dedup |
| `src/core/assertions.rs` | `check_assertions(slug, db)`: heuristic contradiction detection |
| `src/core/graph.rs` | `neighborhood_graph(slug, depth, db)`: N-hop BFS over links |
| `src/core/gaps.rs` | `log_gap()`, `list_gaps()`, `resolve_gap()` |
| `src/core/chunking.rs` | temporal sub-chunking: truth sections + individual timeline entries |
| `src/core/links.rs` | `extract_links()`, `resolve_slug()`, temporal validity |
| `src/core/migrate.rs` | `export_dir()` plus round-trip export helpers |
| `src/core/raw_imports.rs` | Active-source rotation, retention, and byte-exact restore support |
| `src/mcp/server.rs` | MCP stdio server with all tools |
| `src/schema.sql` | Current DDL — embedded via `include_str!()` |

## Build

```bash
# Debug
cargo build

# Release (airgapped default — embeds BGE-small-en-v1.5 for offline use)
cargo build --release

# Online release (downloads/caches the selected BGE model on first semantic use)
cargo build --release --no-default-features --features bundled,online-model

# Cross-compile
cargo install cross
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-apple-darwin
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl
```

## Test

```bash
cargo test
# Key: tests/roundtrip_semantic.rs (normalized export) + tests/roundtrip_raw.rs (byte-exact)
```

## Embedding model

Quaid defaults to `BAAI/bge-small-en-v1.5` (384 dimensions), but the `online-model`
build now accepts runtime model selection via `QUAID_MODEL` or `--model`.

- `small` → `BAAI/bge-small-en-v1.5` (384d, default)
- `base` → `BAAI/bge-base-en-v1.5` (768d)
- `large` → `BAAI/bge-large-en-v1.5` (1024d)
- `m3` → `BAAI/bge-m3` (1024d, multilingual)
- any other value is treated as a full Hugging Face model ID

Compile-time channels:
- default `embedded-model` build — airgapped channel, always uses embedded BGE-small and warns if another model is requested
- `online-model` build — downloads/caches the selected model on first semantic use

Model metadata is persisted in the `quaid_config` table at `quaid init` and validated on every subsequent open. If the requested model differs from the initialized model, the command errors before touching embeddings.

## Skills

Read `skills/` before doing brain operations. All workflow intelligence lives there.
Skills are embedded in the binary and extracted to `~/.quaid/skills/` on first run.
Drop a custom `SKILL.md` in your working directory to override any default.

## Database schema

See `src/schema.sql` for the current DDL. Key tables:
- `pages` — core content (compiled_truth + timeline markdown)
- `page_fts` — FTS5 virtual table (content-rowid, porter tokenizer)
- `quaid_config` — persisted `model_id`, `model_alias`, `embedding_dim`, `schema_version`
- `page_embeddings_vec_384` — vec0 virtual table for the default small model (additional vec tables are created for larger dimensions as needed)
- `page_embeddings` — chunk metadata + vec rowid join table
- `links` — typed temporal cross-references
- `assertions` — heuristic contradiction detection
- `knowledge_gaps` — queries the brain couldn't answer
- `raw_imports` — active source bytes plus bounded inactive history for byte-exact restore

## MCP tools

Core (Phase 1): `memory_get`, `memory_put`, `memory_query`, `memory_search`, `memory_list`

Full surface (Phase 2+): `memory_link`, `memory_link_close`, `memory_backlinks`, `memory_graph`,
`memory_timeline`, `memory_tags`, `memory_check`, `memory_gap`, `memory_gaps`, `memory_stats`, `memory_raw`

## Optimistic concurrency

`memory_put` accepts an optional `expected_version`. If the page's current version
doesn't match, the call returns `ConflictError`. Always re-fetch before writing.
