# MCP Tool Rename Spec

**Change:** All 17 MCP tools rename from `brain_*` prefix to `memory_*` prefix.

## Full rename table

| Old name | New name |
|----------|----------|
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

## Invariants

1. The `#[tool(name = "...")]` annotation in `src/mcp/server.rs` for each tool must use the `memory_*` name exactly as listed above.
2. The corresponding Rust method name should match the tool name for clarity (e.g., method `memory_get` for tool `memory_get`).
3. No `brain_*` name appears in any `#[tool]` annotation in the final implementation.
4. Tool descriptions and input schema are unchanged — only the tool name changes.
5. Error messages that refer to a tool by name (e.g., "use `brain_put` to write") must be updated to the new name.

## Validation

- `rg 'brain_get\|brain_put\|brain_query\|brain_search\|brain_list\|brain_link\|brain_backlinks\|brain_graph\|brain_timeline\|brain_tags\|brain_check\|brain_gap\|brain_gaps\|brain_stats\|brain_raw\|brain_collections' src/mcp/server.rs` → zero matches.
- MCP `tools/list` response from `quaid serve` must contain only `memory_*` tool names.
