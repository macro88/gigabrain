---
title: Quaid Roadmap — Beta Feedback Edition
type: reference
date: 2026-04-27
---

# Quaid Roadmap — Beta Feedback Edition

> Nine consecutive DAB releases. Eight other sections at or near full marks. **§4 Semantic / Hybrid never breaks 30/50.** That's the signal that drives this roadmap.

Last updated: 2026-04-27 · Source: 23 open issues from beta testers ([github.com/quaid-app/quaid/issues](https://github.com/quaid-app/quaid/issues))

This roadmap synthesises what beta testers are actually telling us. For the release-phase plan and shipped milestones, see [`docs/roadmap_v3.md`](docs/roadmap_v3.md).

## How to read this roadmap

What does the data say? Across nine DAB runs (v0.9.1 → v0.9.10), retrieval quality is the single thing standing between Quaid and a 🟢 Excellent grade. Six of the nine epics below trace back to that signal. The other three address performance friction, release hygiene, and the agent-loop integration that turns every session into real-world testing.

Each epic gives you the supporting issues, a frequency / severity read, the proposed direction (with concrete examples), and the rationale for where it sits in the sequence. **NOW** epics ship in parallel. **NEXT** epics queue behind them.

---

## Epic 1 — Knowledge Graph Layer (NOW)

> Foundation epic. Unblocks Epic 2. (1/3 in the NOW track.)

### Signal

| Issue | Title | Status |
| --- | --- | --- |
| [#72](https://github.com/quaid-app/quaid/issues/72) | Self-wiring knowledge graph: YAML frontmatter as first-class graph edges | Open, enhancement |
| [#74](https://github.com/quaid-app/quaid/issues/74) | Graph search: multi-hop traversal with relationship scoring | Open, enhancement |
| [#107](https://github.com/quaid-app/quaid/issues/107) | Zero-LLM entity extraction at write time (graph foundation) | Open, enhancement |

### Why this matters

Quaid already has typed links and the `memory_graph` / `memory_backlinks` MCP tools. So why is the graph empty in practice? Because edges have to be created by hand, and nobody does. Doug's note in #107 is blunt: *"nobody creates them manually."* GBrain v0.12 ships a self-wiring graph and reports +5% precision, +11% recall, +28% on graph queries, and −53% noisy results against the same kind of corpus. That's a surprising amount of lift from one architectural change.

### Direction (3 layers, in shipping order)

**Step 1 of 3 — Auto-wire from frontmatter (#72).** On ingest, parse `links:`, `related:`, `parent:`, `children:`, `tags:` and create typed edges automatically. Wikilinks in body content become soft edges.

Example frontmatter you should be able to drop into any page:

```yaml
---
links:
  - target: companies/brex
    type: founded
    valid_from: 2017-01-01
tags: [fintech, yc-w17]
---
```

**Step 2 of 3 — Zero-LLM entity extraction (#107).** Regex-based pattern matching at write time for `works_at`, `founded`, `invested_in`, `acquired`, `leads`. Patterns configurable via `~/.quaid/entity-patterns.yaml`. Runs async. Crucial constraint: under 5 ms per page, no LLM calls.

Example pattern set:

```rust
let patterns = [
  (r"(?i)(\w+)\s+(?:founded|co-founded)\s+(\w+)", "founded"),
  (r"(?i)(\w+)\s+(?:works at|works for|employed by)\s+(\w+)", "works_at"),
  (r"(?i)(\w+)\s+(?:invested in|funded)\s+(\w+)", "invested_in"),
  (r"(?i)(\w+)\s+(?:acquired|bought)\s+(\w+)", "acquired"),
  (r"(?i)(\w+)\s+(?:is CEO of|leads|runs)\s+(\w+)", "leads"),
];
```

**Step 3 of 3 — Multi-hop traversal in retrieval (#74).** Score = (semantic similarity) × (edge weight) × (distance penalty). Configurable depth (1–3 hops). Path explanations for navigational queries.

### Why first?

Every retrieval improvement compounds with a populated graph. Shipping noise reduction (Epic 2) before the graph means we tune dedup heuristics against a signal that is missing its strongest input. Auto-wiring ships first because the data plumbing has to exist before the search layer can use it.

### Acceptance signals

Once this epic lands, you should see:

- After ingesting the DAB corpus, `quaid graph <slug>` returns non-empty neighbourhoods on the majority of pages.
- DAB §4 Semantic improves by ≥ 8 points (current ceiling 27/50 → target 35/50).
- MSMARCO P@5 improves by at least 5 points versus the bge-small baseline.

---

## Epic 2 — Retrieval Quality: Noise, Reranking, Contradictions (NOW)

> The §4 problem. Highest user-perceived pain. (2/3 in the NOW track.)

Building on Epic 1's graph, this epic targets the chronic top-line drag on overall grade.

### Signal

| Issue | Title | Status |
| --- | --- | --- |
| [#75](https://github.com/quaid-app/quaid/issues/75) | Noise reduction: result deduplication and relevance filtering | Open, enhancement |
| [#76](https://github.com/quaid-app/quaid/issues/76) | Context compression: REFRAG-style chunk compression before LLM decoding | Open, enhancement |
| [#56](https://github.com/quaid-app/quaid/issues/56), [#57](https://github.com/quaid-app/quaid/issues/57), [#63](https://github.com/quaid-app/quaid/issues/63), [#71](https://github.com/quaid-app/quaid/issues/71), [#82](https://github.com/quaid-app/quaid/issues/82), [#85](https://github.com/quaid-app/quaid/issues/85), [#87](https://github.com/quaid-app/quaid/issues/87), [#96](https://github.com/quaid-app/quaid/issues/96), [#108](https://github.com/quaid-app/quaid/issues/108) | DAB results — every release flags §4 as the bottleneck | Open, feedback |

### Frequency / severity

How bad is it really? Here are the DAB §4 Semantic / Hybrid scores across nine releases:

```
v0.9.1: 26/50    v0.9.4: 31/50    v0.9.7: 10/50    v0.9.9:  26/50
v0.9.2: 31/50    v0.9.5: 30/50    v0.9.8: 22/50    v0.9.10: 27/50
v0.9.6: 19/50
```

Volatile, never above 31/50, and the chronic top-line drag on overall grade. Every other section consistently scores at or near full marks. Doug's read in #56: *"bge-small-en-v1.5 is weak on crypto/finance domain paraphrasing."*

### Direction (4 ship-able changes + 2 follow-ons)

All four changes land in `src/core/search.rs` and `src/core/progressive.rs`, in this shipping order:

**Step 1 of 4 — Intra-document deduplication.** When the top-k contains multiple chunks from the same page, collapse to the strongest chunk plus a document reference. New flag: `--max-chunks-per-doc N`.

**Step 2 of 4 — MMR (Maximal Marginal Relevance).** Penalise candidates that are semantically too close to already-selected results. This single change typically delivers the biggest perceived-quality lift, and it's corpus-agnostic.

**Step 3 of 4 — Confidence threshold.** Drop results below a configurable relevance score (default ~0.3) even if they would have made top-k. Stops Quaid from filling slots with noise just to honour `k`.

**Step 4 of 4 — Cross-reference scoring.** A document referenced by other top-ranked documents earns a boost. This is a graph-lite shortcut that gives you some of Epic 1's value before the full graph lands.

Two follow-on items, sequenced after the four above:

**Follow-on 1 — Reranker pass (extension of #76).** Lightweight extractive summarisation per chunk before final ranking. Lower complexity than full REFRAG; viable on the airgapped binary.

**Follow-on 2 — Contradiction detection: semantic gating.** §7 has been stuck at 7–8/10 since v0.9.1, with 0 recall on real conflicts (#82, #85, #87, #96, #108). Doug's #56 diagnosis: *"the feature works mechanically — it just needs a semantic similarity gate before comparing assertions."* Tracked here because the fix sits in the same retrieval layer; the existing roadmap explicitly defers LLM-assisted detection to skill-side, which remains the right call.

### Why second?

Most of these changes are local improvements to `hybrid_search` and `progressive_retrieve`. They land independently of the graph, but they produce far more lift once the graph is populated (cross-reference scoring in particular). Shipping in this order means we get measurable wins from Epic 1, then stack Epic 2 on top.

### Acceptance signals

You'll know this epic worked when:

- DAB §4 Semantic / Hybrid ≥ 35/50 sustained across two consecutive releases.
- DAB §7 Contradiction Detection ≥ 9/10, with non-zero recall on real-corpus conflicts.
- Subjective tester report: results feel less repetitive on broad queries.

---

## Epic 3 — Conversation Memory and the Agent Loop (NOW)

> Strategic gate. Unlocks compounding real-world feedback. (3/3 in the NOW track.)

Now that we've covered retrieval, what about the input side? Today, your agent starts every session blank.

### Signal

| Issue | Title | Status |
| --- | --- | --- |
| [#105](https://github.com/quaid-app/quaid/issues/105) | Conversation turn ingestion with fact extraction (`memory_add_turn`) | Open, enhancement |
| [#106](https://github.com/quaid-app/quaid/issues/106) | Native Quaid MCP support as OpenClaw memory backend | Open, enhancement |

### Why this matters

Quaid stores markdown documents. Agents need to also store and retrieve **conversation turns** — what was said, what was decided, what the user asked for. Doug's framing in #105: *"Without this, agents start every session blank. With it, agents compound knowledge across sessions."*

LoCoMo (multi-session conversational recall benchmark) score sits near zero today because there's no ingestion path. Mem0 v3 reports 91.6% on LoCoMo using exactly this pattern. #106 is the integration gate: until OpenClaw can run `memory.backend: "quaid"`, every benchmark still has to run against a synthetic corpus rather than real sessions.

### Direction (2 parallel workstreams)

**Workstream A — `memory_add_turn` MCP tool + `quaid ingest-session` (#105).** Pipeline: receive turn → extract facts (decisions, preferences, learned facts, action items) → deduplicate via existing novelty check → store as typed page (`decision`, `preference`, `action_item`) with `session_id` frontmatter. Start with a regex/rule extraction pass (zero API cost), with a hybrid LLM fallback gated behind config for ambiguous content.

Example MCP call:

```json
{
  "tool": "memory_add_turn",
  "input": {
    "session_id": "2026-04-27-main",
    "role": "user",
    "content": "We decided to use MSMARCO as the benchmark corpus instead of Wikipedia because it has real ground-truth relevance labels.",
    "timestamp": "2026-04-27T10:30:00Z"
  }
}
```

**Workstream B — OpenClaw integration (#106).** Wire `memory_search` and `memory_query` to OpenClaw's memory router. Define the contract for graceful fallback when `quaid serve` isn't running.

Example OpenClaw config:

```json
{
  "memory": { "backend": "quaid" },
  "mcp": {
    "servers": {
      "quaid": {
        "command": "quaid",
        "args": ["serve"],
        "env": { "QUAID_DB": "/Users/doug/.quaid/memory.db" }
      }
    }
  }
}
```

### Why now (in parallel with Epic 1 and Epic 2)?

Independent code path — doesn't block on graph or retrieval changes. The strategic value is essential: once OpenClaw uses Quaid as its primary memory backend, every session becomes a real-world benchmark. We move from synthetic DAB queries to actual usage signal, which is the only way to find the quality gaps benchmarks miss.

### Acceptance signals

You're done when:

- Setting `memory.backend: "quaid"` in `openclaw.json` works end-to-end.
- LoCoMo score climbs above 40% (from current ~0%).
- `quaid query "what did we decide about X last week"` returns relevant session memory.

---

## Epic 4 — Performance: Import Speed and Async Operations (NEXT)

> Daily friction. Important, but not gated.

With the strategic moves staked out, let's move to operational quality.

### Signal

| Issue | Title | Status |
| --- | --- | --- |
| [#59](https://github.com/quaid-app/quaid/issues/59) | Import speed regression v0.9.2: 429s vs 159s for 350-page corpus | Open, no label |
| [#73](https://github.com/quaid-app/quaid/issues/73) | Minion queue: async job system to bypass subagent timeouts | Open, enhancement |

### Frequency / severity

Why is #59 the longest-running open performance bug? Look at import time across releases:

```
v0.9.1: 159s    v0.9.5: 428s    v0.9.8: 240s
v0.9.2: 429s    v0.9.6: 428s    v0.9.9: 427s
v0.9.4: 427s    v0.9.7: 180s    v0.9.10: 428s
```

The v0.9.7 dip suggests the regression isn't structural — it can be unwound with profiling. Every DAB results issue from v0.9.2 onward calls this out (#57, #63, #71, #82, #85, #87, #96, #108). Tester hypothesis in #59: PARA type inference (#54 fix) added a per-page folder-path scan that's O(n²) against the folder tree, or triggers extra DB writes per page.

#73 is the long-tail companion: any operation > 5 minutes (large imports, bulk re-embedding when the model changes, full contradiction scans) hits OpenClaw's subagent timeout and fails. You then have to re-run manually.

### Direction (4 steps)

**Step 1 of 4 — Profile the import path.** Confirm whether type inference is the bottleneck. Easy diagnostic: bypass type inference and measure delta.

**Step 2 of 4 — Restore parity with v0.9.1** (sub-180s for 350 pages). Optimise PARA inference to a single pass with cached folder-prefix lookup.

**Step 3 of 4 — SQLite-backed job queue.** Reuse the existing daemon infrastructure rather than introducing BullMQ-style external dependencies — this keeps the "single binary, no cloud" promise. Jobs: enqueue, status poll, cancel. Runs to completion regardless of caller timeout.

Example CLI surface:

```bash
quaid job submit import ~/repos/large-vault
# → job-id: j-2026-04-27-001

quaid job status j-2026-04-27-001
# → running (page 142/2400, eta 8m)
```

**Step 4 of 4 — Wire long-running CLI operations through the queue.** `import`, bulk re-embed, `check --all` on large corpora.

### Why next, not now?

Epics 1–3 are the strategic moves. Epic 4 is daily friction — important, but not gated. Profiling #59 is a 1–2 day investigation any squad member can take in parallel.

### Acceptance signals

- 350-page DAB corpus imports in < 180s (target: < 60s for full DAB §5 marks).
- `quaid job submit import <large-corpus>` returns immediately with a job ID; status polling works; the job survives caller disconnection.

---

## Epic 5 — Release Hygiene and Stability (NEXT)

> Cheap to ship. Prevents one issue per release.

So far we've focused on building. With that covered, let's harden the release process itself.

### Signal

| Issue | Title | Status |
| --- | --- | --- |
| [#68](https://github.com/quaid-app/quaid/issues/68), [#70](https://github.com/quaid-app/quaid/issues/70) | `gbrain contradictions` subcommand removed without changelog entry (duplicate) | Open, documentation |
| Embedded in [#85](https://github.com/quaid-app/quaid/issues/85) | v0.9.7: MCP serve crashes on empty `root_path` | Reported, fix referenced in commit `dd05af6` |
| Embedded in [#87](https://github.com/quaid-app/quaid/issues/87) | v0.9.8: import process hangs after completion (watcher not torn down) | Reported, fixed in v0.9.8+ |
| Embedded in [#96](https://github.com/quaid-app/quaid/issues/96) | v0.9.9: binary reports `quaid 0.9.8` — version string not bumped | Reported |

### The pattern

One avoidable bug per release. Why does this keep happening? The DAB harness catches each one within a day, but each one costs a release cycle. The bugs aren't architectural — they're pre-flight checks that didn't run.

### Direction (4 steps)

**Step 1 of 4 — Pre-release smoke test in CI.** Five gates before a release tag is accepted:

```bash
# Pre-release smoke test
quaid --version | grep -q "$TAG"           # version string matches tag
quaid init /tmp/smoke.db                    # init exits clean
quaid serve --db /tmp/smoke.db &            # serve starts on empty collections
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | nc localhost 0
kill %1                                     # clean shutdown on SIGTERM
```

**Step 2 of 4 — Changelog gate.** PRs that add or remove a CLI subcommand or MCP tool must update `CHANGELOG.md`. Enforced in CI.

**Step 3 of 4 — Deprecation aliases for one release cycle.** When you rename a subcommand (e.g. `contradictions` → `check --all`), keep the old name as an alias that prints a deprecation warning. This is the specific ask from #68 and #70.

**Step 4 of 4 — Close the duplicate.** #68 and #70 are the same issue — close one, reference the other.

### Why next?

Cheap to ship, prevents one issue per release. Not strategic, but exceptional return on engineering hours.

### Acceptance signals

- Three consecutive releases with zero release-quality issues filed against them.
- `gbrain contradictions` (or any deprecated alias) prints a deprecation warning and routes you to the new command.

---

## Epic 6 — Benchmark Evolution (NEXT)

> Already running. Extend as Epics 1–3 land.

### Signal

| Issue | Title | Status |
| --- | --- | --- |
| [#50](https://github.com/quaid-app/quaid/issues/50) | Beta Test Plan v1 — structured testing framework for each release | Open, feedback (operational) |
| [#51](https://github.com/quaid-app/quaid/issues/51) | DAB v1.0 — stable scoring framework for releases and embedding models | Open, feedback (operational) |

### Context

DAB and the Beta Test Plan are already operational and producing the feedback that drives every other epic on this roadmap. Of the 23 open issues, 9 are DAB results issues. The infrastructure works; this epic is about extending it to cover the gaps real usage will surface.

### Direction (4 steps)

**Step 1 of 4 — LoCoMo adapter** (covered partly under Epic 3). Multi-session conversational recall, gated to run after #105 lands.

**Step 2 of 4 — MSMARCO P@5 / R@5** as a regression gate on Epic 1 and Epic 2 changes. Real ground-truth labels, not the synthetic anchor facts in DAB.

**Step 3 of 4 — Embedding model matrix.** DAB's design intent — head-to-head BGE-small / BGE-base / BGE-large / BGE-m3 — hasn't yet shipped. The README claims 193/215 (90%) on a newer DAB version; align the public scoreboard with what actually runs in CI.

**Step 4 of 4 — Public leaderboard.** Track scores over time at `quaid-evals` so regressions are visible in the open.

### Why now?

Already running. The work is to extend it as Epics 1–3 land so we can prove the lift, not assert it.

### Acceptance signals

- LoCoMo, MSMARCO, and DAB all run in CI on every release.
- Public leaderboard updated automatically.
- Model-matrix table populated for at least three model variants.

---

## Open questions

A handful of things the issues don't settle. Worth a tester poll before committing.

**Question 1 — Embedding model default.** §4 Semantic ceiling is 27/50 with bge-small. Does upgrading the airgapped default to bge-base (768d) move the needle enough to justify the binary-size cost? Imagine doubling the embedding-table footprint just to see what we already suspect: that domain paraphrasing is bottlenecked by model capacity. DAB §4 should answer this once Epic 6's model matrix runs.

**Question 2 — LLM-assisted contradiction detection.** Currently deferred per `docs/roadmap_v3.md` — *"the binary stays dumb."* Do we hold that line, or is the +3 points on §7 worth a config-gated opt-in?

**Question 3 — REFRAG-style compression** (#76). Worth pursuing only if latency becomes a tester complaint. Nobody is currently reporting it as pain. Defer until signalled.

---

## Appendix — Issues not represented above

- [#50](https://github.com/quaid-app/quaid/issues/50), [#51](https://github.com/quaid-app/quaid/issues/51) — operational, covered by Epic 6.
- [#54](https://github.com/quaid-app/quaid/issues/54) — already shipped in v0.9.2, but referenced as suspected cause of #59.
- [#56](https://github.com/quaid-app/quaid/issues/56), [#57](https://github.com/quaid-app/quaid/issues/57), [#63](https://github.com/quaid-app/quaid/issues/63), [#71](https://github.com/quaid-app/quaid/issues/71), [#82](https://github.com/quaid-app/quaid/issues/82), [#85](https://github.com/quaid-app/quaid/issues/85), [#87](https://github.com/quaid-app/quaid/issues/87), [#96](https://github.com/quaid-app/quaid/issues/96), [#108](https://github.com/quaid-app/quaid/issues/108) — DAB results; their findings fold into Epics 1, 2, 4, 5.

---

## Next steps — sequencing summary

Now that you've read the epics, here's how they sequence. Three concurrent NOW workstreams, three queued NEXT:

```
NOW (concurrent — start this week)
├── Epic 1: Knowledge Graph Layer       (foundation)
├── Epic 2: Retrieval Quality            (highest user pain — MMR/dedup land independent of graph; cross-ref scoring stacks)
└── Epic 3: Conversation Memory          (strategic gate, independent code path)

NEXT (queued behind NOW)
├── Epic 4: Performance                  (#59 profiling can start in parallel)
├── Epic 5: Release Hygiene              (cheap, immediate)
└── Epic 6: Benchmark Evolution          (extends as 1–3 land)
```

Now you have a sequence. Pick up Epic 1 and let's start wiring frontmatter edges.
