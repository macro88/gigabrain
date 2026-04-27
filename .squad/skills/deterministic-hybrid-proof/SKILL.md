---
name: deterministic-hybrid-proof
description: Prove hybrid-search behavior without letting vector quality hide the real seam under test.
---

# Deterministic Hybrid Proof

Use this when a hybrid search/query path changed but the regression claim belongs to one arm, not the merge in general.

## Pattern

1. **Force the target arm to matter**
   - Build a corpus where the pre-change branch would miss (for example, FTS implicit AND returns zero on a compound query).

2. **Zero the competing arm**
   - For vector-search competition, do not write embeddings.
   - Assert `search_vec*` is empty before calling the hybrid entry point.

3. **Prove the precondition explicitly**
   - Assert the narrow helper still misses (`search_fts` empty, exact path empty, etc.).
   - Do not infer the miss from the later hybrid result.

4. **Assert the hybrid result set exactly**
   - Check the concrete slugs, not just `!is_empty()`.
   - For CLI/canonical paths, assert `<collection>::<slug>` output.

## Guardrails

- Do not rely on live model quality for deterministic regressions.
- If the surface canonicalizes page references, the proof must canonicalize its expectation too.
- Remove or downgrade smoke tests that only prove `Ok` when a reviewer is asking for behavioral proof.
