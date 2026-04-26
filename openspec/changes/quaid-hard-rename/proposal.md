## Why

GigaBrain has grown into a production-quality tool but ships under a name that is descriptive-but-generic and carries the Garry Tan heritage branding. The canonical repository is already at `quaid-app/quaid`. The rename completes the identity migration: product is **Quaid**, CLI binary is `quaid`, the conceptual layer is **memory**, and all MCP tools become `memory_*`. This is a hard rename — no legacy aliases, no shims, no compatibility wrappers.

## What Changes

### Surface-level (user-visible)

| Before | After |
|--------|-------|
| Binary name | `gbrain` | `quaid` |
| Product name | GigaBrain | Quaid |
| Crate name (`Cargo.toml`) | `gbrain` | `quaid` |
| Default DB filename | `brain.db` | `memory.db` |
| Default DB directory | `~/.gbrain/` | `~/.quaid/` |
| Default DB full path | `~/.gbrain/brain.db` | `~/.quaid/memory.db` |
| Env var: DB path | `GBRAIN_DB` | `QUAID_DB` |
| Env var: model selection | `GBRAIN_MODEL` | `QUAID_MODEL` |
| Env var: channel | `GBRAIN_CHANNEL` | `QUAID_CHANNEL` |
| Env var: install dir | `GBRAIN_INSTALL_DIR` | `QUAID_INSTALL_DIR` |
| Env var: version | `GBRAIN_VERSION` | `QUAID_VERSION` |
| Env var: no-profile | `GBRAIN_NO_PROFILE` | `QUAID_NO_PROFILE` |
| Env var: release API URL | `GBRAIN_RELEASE_API_URL` | `QUAID_RELEASE_API_URL` |
| Env var: release base URL | `GBRAIN_RELEASE_BASE_URL` | `QUAID_RELEASE_BASE_URL` |
| GitHub repo slug | `macro88/gigabrain` | `quaid-app/quaid` |
| MCP tool prefix | `brain_*` | `memory_*` |

### MCP tool renames (exhaustive)

| Before | After |
|--------|-------|
| `brain_get` | `memory_get` |
| `brain_put` | `memory_put` |
| `brain_query` | `memory_query` |
| `brain_search` | `memory_search` |
| `brain_list` | `memory_list` |
| `brain_link` | `memory_link` |
| `brain_link_close` | `memory_link_close` |
| `brain_backlinks` | `memory_backlinks` |
| `brain_graph` | `memory_graph` |
| `brain_timeline` | `memory_timeline` |
| `brain_tags` | `memory_tags` |
| `brain_check` | `memory_check` |
| `brain_gap` | `memory_gap` |
| `brain_gaps` | `memory_gaps` |
| `brain_stats` | `memory_stats` |
| `brain_raw` | `memory_raw` |
| `brain_collections` | `memory_collections` |

### Schema changes

The internal table `brain_config` (stores model metadata) is renamed to `quaid_config`. All other table names (`pages`, `page_fts`, `page_embeddings`, `page_embeddings_vec_*`, `links`, `assertions`, `knowledge_gaps`, `ingest_log`) are unchanged — they name domain concepts, not the product.

This constitutes a **breaking schema change** requiring a `SCHEMA_VERSION` bump. Old `brain.db` files are incompatible with the renamed binary. Migration path: export with the old `gbrain` binary, re-init with `quaid init`, re-import. No automatic migration is provided (consistent with "no legacy support" directive).

### Code-level

- `Cargo.toml`: `name = "quaid"`, `[[bin]] name = "quaid"`, `repository` updated to `quaid-app/quaid`
- `src/main.rs`: clap `name = "quaid"`, env var references updated
- `src/mcp/server.rs`: all `#[tool(name = "brain_*")]` annotations and method names updated to `memory_*`
- `src/core/db.rs`: `brain_config` → `quaid_config`, `SCHEMA_VERSION` bumped, `~/.gbrain` → `~/.quaid` default path
- `src/schema.sql`: `brain_config` DDL renamed to `quaid_config`, any remaining GigaBrain comments updated
- `build.rs`: any GigaBrain references updated
- `scripts/install.sh`: all `GBRAIN_*` env vars, `macro88/gigabrain` repo slug, profile-injection text
- `README.md`, `CLAUDE.md`, `docs/spec.md`, `docs/getting-started.md`, `docs/contributing.md`, `docs/roadmap.md`: all product-name occurrences
- `website/`: all site content, metadata, package.json (if product name appears)
- `.github/workflows/`: release artifact names (`gbrain-*` → `quaid-*`), workflow titles
- `skills/*/SKILL.md`: all `gbrain` CLI examples, `GBRAIN_*` env var examples, `brain_*` MCP tool examples
- `openspec/`: existing change proposals updated in their prose to reflect new names (non-structural)
- `tests/`: test fixture strings, DB path helpers, MCP tool name assertions

## Capabilities

### Modified Capabilities
- `cli-binary`: Binary name changes from `gbrain` to `quaid`. Shell completions, PATH entries, MCP config blocks all need updating by users.
- `mcp-server`: All 17 tool names change from `brain_*` to `memory_*`. Any MCP client config (Claude Code `.mcp.json`, etc.) must be updated.
- `default-db-path`: Default DB location changes from `~/.gbrain/brain.db` to `~/.quaid/memory.db`. Explicit `QUAID_DB` env or `--db` flag still overrides.
- `env-vars`: All `GBRAIN_*` env vars renamed to `QUAID_*`.

### Removed Capabilities
- No `gbrain` binary shim or alias is provided.
- No `brain_*` MCP tool aliases are provided.
- No `GBRAIN_*` → `QUAID_*` env var forwarding.
- No automatic migration of `brain_config` → `quaid_config` in existing DBs.

## Non-Goals

- Re-architecture of any runtime behaviour (search, embeddings, storage model, skill system).
- Version bump of any user-facing functionality beyond the rename.
- Providing a migration tool from old to new DB format (manual export/re-init is the documented path).
- Renaming internal non-surface tables (`pages`, `links`, `assertions`, etc.).
- Updating `.squad/` agent history files (historical record; should not be retroactively rewritten).

## Impact

- **230 files** contain at least one `gbrain`, `GigaBrain`, `gigabrain`, `GBRAIN`, or `brain.db` reference (scanned 2026-04-25).
- **Breaking schema change** (`brain_config` → `quaid_config`) requires `SCHEMA_VERSION` bump. Existing DBs cannot be opened without re-init.
- **Active branch conflict**: `vault-sync-engine` is in-flight on `spec/vault-sync-engine` and touches many of the same files. This rename should land on `main` first; vault-sync owners (Fry, Professor, Nibbler) must rebase. Coordinate before any rename PR is raised.
- **MCP client configs**: Any live Claude Code / other MCP client `mcp.json` referencing `gbrain` binary path or `brain_*` tool names will break silently until updated.
- **CI release artifacts**: Release workflow produces artifacts like `gbrain-darwin-arm64-airgapped`. These names change to `quaid-*`. Any downstream install scripts pinned to the old asset naming pattern will break.

## Risks

1. **vault-sync rebase complexity** — the active branch is large; a conflict-heavy rebase post-rename could introduce bugs. Mitigation: freeze vault-sync slice selection until rename is merged, or merge vault-sync first.
2. **External MCP client configs** — users with live `~/.config/claude/claude_desktop_config.json` or similar will see all tools disappear until they update `command: gbrain` → `command: quaid` and all tool names. Mitigation: prominent migration note in README and release notes.
3. **Schema version mismatch on existing DBs** — users upgrading from any v0.x `gbrain` binary will be unable to open their DB with `quaid`. Mitigation: document the `gbrain export` → `quaid init` → `quaid import` path in the migration guide before the release tag is published.
