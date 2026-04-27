# Search Surface Coverage

Use this when a search bug touches core ranking/query logic plus CLI or MCP wrappers.

## Pattern

1. **Pin the denominator first.** Treat the touched surface as the search core plus every wrapper
   that changes query semantics or output contracts.
2. **Test helpers before wrappers.** Any query rewriting, fallback planning, or merge policy needs
   a pure helper with direct unit tests.
3. **Prove SQL-assembly branches directly.** Canonical slug rendering, collection filters, and
   combined parameter ordering belong in source-level tests, not only subprocess tests.
4. **Use wrapper tests for contracts only.** CLI/MCP tests should prove JSON/text shape, canonical
   slugs, filter fencing, and expert/raw bypass behavior.
5. **Avoid model-quality dependence.** If hybrid/vector fallback is involved, expose a
   deterministic seam so coverage does not depend on live embedding behavior.

## Minimum matrix

- empty / punctuation-only query
- natural-language punctuation query
- 2-term query
- 3+ term compound query
- collection-filtered query
- canonical exact-slug ambiguity / not-found / matching-prefix / mismatched-prefix
- merge-strategy branch (`SetUnion` and `Rrf`)
