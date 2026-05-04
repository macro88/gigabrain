---
name: "file-edit-history-integrity-gate"
description: "Guard watcher-driven archive-on-edit features from chain forks and sidecar recursion"
domain: "review, integrity"
confidence: "high"
source: "earned"
---

## Use when

- A watcher turns manual file edits into archived history rows
- The same feature can optionally write history sidecars back into the watched tree
- The product claims ADD-only truth preservation instead of in-place overwrite

## Required invariants

1. **Archive before overwrite, inside one atomic path**
   - Snapshot the live head before any in-place overwrite path destroys the old bytes.
   - The archived predecessor, live-head update, and raw/file-state updates must fail closed together.

2. **Keep one linear chain**
   - Test the real hard case: editing a head that already supersedes something.
   - Safe result is `A -> archived-B -> live-B`, not two predecessors pointing at the same head.

3. **Whitespace-only saves are full no-ops**
   - No archive row
   - No version bump
   - No `raw_imports` rotation
   - No `file_state` churn
   - Also prove the diff/full-hash classifier suppresses the same path on the next pass, or the "no-op" will reappear as endless modified noise.

4. **Gate by both path and page type**
   - Restrict the handler to the intended extracted subtree
   - Restrict it to the intended page kinds
   - Explicitly bypass conversations, ordinary notes, and other non-extracted surfaces

5. **Disk history sidecars must not re-enter the watcher**
   - If sidecars are written under the watched tree, they must be excluded or self-write-suppressed.
   - Otherwise the watcher can ingest the sidecar as a live page or recursively archive the archive.

6. **Linear-chain repair can reuse the live row**
   - Safe closure does not require allocating a brand-new head id every time.
   - If the current live row stays the head, insert one archived predecessor row and rewire any older predecessor onto that archive before updating the live row.

## Minimum test shape

- Public-chain proof for `A -> B -> C`
- Manual-edit proof for an already-chained head
- Whitespace-only no-op proof
- `history_on_disk=true` proof that the sidecar exists but does not become a live ingest event

## Smells

- "We archive after reingest" — prior truth is already gone
- Tests only cover the first edit of a singleton page
- The handler matches `extracted/**/*.md` but forgets to exclude `_history/`
- The proof only checks the new head exists, not that there is still exactly one predecessor path
