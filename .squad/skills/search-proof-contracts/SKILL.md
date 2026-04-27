# Search Proof Contracts

Use this skill when reviewing or writing tests for CLI/MCP search behavior.

## Rules

1. If the surface calls `search_fts_canonical*` or `hybrid_search_canonical`, assert canonical `<collection>::<slug>` output, not bare slugs.
2. For AND-first / OR-fallback logic, require one direct proof that AND results short-circuit widening and one direct proof that OR fallback fires only after AND returns empty.
3. Do not accept wrapper-only `is_ok()` tests as proof for changed search behavior; assert returned rows, slugs, and the boundary between raw/expert and natural-language paths.
4. When hybrid search changes, add a deterministic test that isolates the FTS-arm behavior instead of letting vector recall mask the branch under review.
