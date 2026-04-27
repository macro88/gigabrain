# Compound-Term Tiered FTS

Use this when a full-text search surface fails on multi-word natural-language queries because the
underlying engine applies implicit AND semantics.

## Pattern

1. Keep the expert/raw query function unchanged.
2. Sanitize natural-language input separately.
3. Run the precise AND pass first.
4. Only if AND returns zero and the sanitized query has 2+ tokens, widen to an explicit OR chain.
5. Put the fallback helper in the core search library, then wire thin CLI/query wrappers to it.

## Proof seams

- helper test: multi-token input expands to `term1 OR term2 ...`
- helper test: empty and single-token inputs stay unchanged
- FTS test: raw AND path still returns empty on the compound miss
- tiered FTS test: same query returns recall via OR fallback
- precision test: when AND finds a full match, tiered results equal AND results
- wrapper test: default CLI path widens, raw CLI path does not
