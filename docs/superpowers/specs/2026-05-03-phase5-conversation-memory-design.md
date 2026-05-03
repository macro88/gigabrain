---
title: "Phase 5 — Conversation Memory + SLM Extraction"
date: 2026-05-03
status: draft
roadmap: docs/roadmap_v3.md
issues: ["#105", "#135", "#137"]
---

# Phase 5 — Conversation Memory + SLM Extraction

## Summary

Phase 5 turns Quaid from a document-memory tool into a conversational-memory tool. It adds the ability to ingest agent conversation turns, extract durable facts from those turns using a local airgapped SLM (Phi-3.5 Mini by default), and resolve those facts against existing memory using an ADD-only supersede chain. It also adds a bounded correction surface so wrong facts can be repaired without corrupting the supersede history.

The headline outcome is closing the LoCoMo / LongMemEval benchmark gap (currently 0.1% / 0.0%) where Mem0 v3 sits near 90%. The single biggest blocker is that Quaid stores raw documents but does not extract facts from conversation. This phase fixes that, while keeping the "single static binary, fully airgapped" promise intact.

## Motivation

Per the v3 roadmap, conversation memory + SLM extraction is the highest-impact single feature on the roadmap and the gating dependency for two further benchmarks (BEAM 100K joins LoCoMo and LongMemEval as gaps the current architecture cannot close). Today, asking Quaid "what degree did I graduate with?" cannot recover "Business Administration, spent 4 years at it" buried in casual conversation, because nothing extracts that fact at write time.

The bet is: extracting structured facts at write time using a small local model, stored as ordinary Quaid pages with a supersede chain for change-over-time, raises LoCoMo from 0.1% to ≥ 40% and LongMemEval to ≥ 40%, without breaking airgap or single-binary properties.

## Goals

1. Accept conversation turns from any caller via a new `memory_add_turn` MCP tool. Caller-agnostic — OpenClaw, Claude Code wrappers, custom agents, future integrations all use the same surface.
2. Extract durable facts (`decision`, `preference`, `fact`, `action_item`) from windowed turns using a local SLM. Extraction is background, debounced, and survives daemon restart.
3. Store extracted facts as ordinary Quaid pages, with type-specific structured frontmatter and a prose body for retrieval.
4. Model contradiction-over-time as an ADD-only supersede chain — never destroy history.
5. Provide a bounded `memory_correct` MCP tool so wrong facts can be repaired through dialogue, and make user edits to extracted-fact files in Obsidian preserve history rather than overwrite it.
6. Hit ≥ 40% on LoCoMo and ≥ 40% on LongMemEval. Maintain `memory_add_turn` p95 < 50 ms.
7. Stay fully airgapped after model download. Default `extraction.enabled = false` so docs-only users pay nothing.

## Non-goals

| Non-goal | Reason |
|---|---|
| Entity extraction (people / companies / concepts) | Phase 6 (`#107`). Shares some infrastructure but ships separately. |
| Active memory enrichment | Phase 7 (`#136`). Requires entity graph in place. |
| Cross-namespace fact recall | Namespace isolation is a hard product boundary (`#137`). |
| Synchronous extraction on the request path | Deliberate design choice; callers can force flush via `memory_close_session`. |
| LLM-assisted contradiction detection (DAB §7) | Tracked under the existing `contradiction-semantic-gate` spec. Phase 5's supersede is heuristic. |
| Fact-to-fact graph edges | Phase 6. |
| Web UI for browsing facts | Vault + Obsidian satisfy this for now. |
| Multi-language extraction validation | English-first; non-English is best-effort with the multilingual SLM. |
| Voice / audio turn ingest | Caller transcribes before calling `memory_add_turn`. |

## Architecture overview

Phase 5 is two cooperating pipelines, both downstream of the existing Phase 4 vault sync engine.

```
┌──────────────────────────────────────────────────────────────────┐
│  CALLER (any harness — agent, IDE wrapper, custom integration)   │
└────────────────────────────────┬─────────────────────────────────┘
                                 │ MCP: memory_add_turn
                                 │   {session_id, role, content, timestamp}
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│  quaid serve  (daemon)                                           │
│                                                                  │
│   ┌─────────────────┐     ┌──────────────────────────────────┐   │
│   │ turn writer     │────▶│  vault file: conversations/      │   │
│   │ (append + fsync)│     │   <YYYY-MM-DD>/<session>.md      │   │
│   └─────────────────┘     └──────────────────────────────────┘   │
│            │                              │                      │
│            │ returns immediately          │ (Phase 4) watcher    │
│            ▼                              ▼                      │
│   ┌─────────────────┐     ┌──────────────────────────────────┐   │
│   │ extraction      │◀────│  vault sync ingest               │   │
│   │ queue           │     │  (page row, FTS5, vec0)          │   │
│   │ (SQLite-backed) │     └──────────────────────────────────┘   │
│   └────────┬────────┘                                            │
│            │ 5s debounce + session-close trigger                 │
│            ▼                                                     │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │ SLM runner (Phi-3.5, in-process, lazy-loaded)           │    │
│   │  reads window → emits typed facts → resolve dedup/      │    │
│   │  supersede → write fact pages to vault                  │    │
│   └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│                              ▼                                   │
│                  extracted/<type>/<slug>.md                      │
└──────────────────────────────────────────────────────────────────┘
```

**Invariants:**

1. **Vault is source of truth.** Every turn and every extracted fact is a markdown file. The DB is a derived index. `rm -rf memory.db && quaid sync` rebuilds everything (including re-extraction).
2. **`memory_add_turn` returns synchronously and fast.** It does two things: append to the session file with fsync, enqueue a debounced extraction job. No SLM call on the request path. Target p95 < 50 ms.
3. **Extraction is fully decoupled.** SQLite-backed queue (reuses existing daemon infrastructure, satisfies the "single binary" promise). Failures don't affect the request path. Re-extraction is a first-class operation.
4. **The Phase 4 watcher does double duty.** No new file watcher is introduced. The existing watcher gains a second consumer that triggers extraction on `kind: conversation` page writes, and a small handler that detects user edits to `extracted/**` files.
5. **Caller-agnostic.** The MCP tool surface accepts turns. Auto-capture vs curated turn submission is each caller's choice.

## Storage layout

### Vault paths

Defaults to the user's main vault, in two sequestered top-level subdirectories. Configurable to a dedicated agent-memory collection via `memory.location`.

```
<vault-root>/
├── conversations/
│   └── 2026-05-03/
│       └── openclaw-main.md           # one session, appended over time
└── extracted/
    ├── decisions/
    │   └── 2026-05-03-benchmark-corpus.md
    ├── preferences/
    │   └── matt-prefers-rust.md
    ├── facts/
    │   └── matt-cs-business-systems-degree.md
    └── action-items/
        └── ship-phase5-2026-06-01.md
```

When namespace isolation is in use (`#137`), all paths above are nested under the namespace directory: `<vault-root>/<namespace>/conversations/...`.

### Conversation file format

Frontmatter + a series of turn blocks:

```markdown
---
kind: conversation
session_id: openclaw-main
date: 2026-05-03
started_at: 2026-05-03T09:14:22Z
status: open                          # 'open' or 'closed'
last_extracted_at: 2026-05-03T10:30:18Z
last_extracted_turn: 47               # idempotency cursor
---

## Turn 1 · user · 2026-05-03T09:14:22Z

Let's pick a benchmark corpus for the retrieval work.

---

## Turn 2 · assistant · 2026-05-03T09:14:31Z

Top candidates: MSMARCO, Wikipedia, the Reddit dump...
```

The cursor (`last_extracted_turn`) makes re-extraction idempotent. Crash recovery resumes from the cursor; SLM upgrades reset cursor to 0 to re-extract.

**Multi-day sessions.** A session that spans midnight produces a new file under the next day's directory (`conversations/2026-05-04/<session-id>.md`), with turn ordinals continuing from where the prior file left off (turn 48, 49, ... not restart at 1). Each file maintains its own cursor — extraction is file-scoped. The session-level cursor for "what's the latest extracted turn across the whole session" is derived at query time as `MAX(last_extracted_turn)` across the session's files. This keeps cursor management local to one file and accepts that the lookback window doesn't cross file (day) boundaries — a known limitation that is rarely a problem in practice and that Phase 6 entity extraction can compensate for once it lands.

### Extracted fact file format

Hybrid: type-specific structured frontmatter + prose body.

```markdown
---
kind: preference
about: programming-language
strength: high
session_id: openclaw-main
source_turns: [openclaw-main:14, openclaw-main:15, openclaw-main:18]
extracted_at: 2026-05-03T10:30:18Z
extracted_by: phi-3.5-mini
supersedes:                           # null = head of chain
corrected_via:                        # null | 'explicit' | 'file_edit'
---

Matt prefers Rust over Go for systems programming. Mentioned in the
context of choosing the language for the new memory backend, citing
performance, type safety, and the existing candle dependency.
```

**Type-specific required fields:**

- `decision`: `chose`, `rationale?`
- `preference`: `about`, `strength?` (`low` | `medium` | `high`)
- `fact`: `about`
- `action_item`: `who?`, `what`, `status` (`open` | `done` | `cancelled`), `due?`

The structured key (`about` / `chose` / `what`) is what supersede and dedup pivot off of. Prose body is what FTS5 and vec0 retrieve against.

### Schema additions (v8)

No new "facts" table — extracted facts are ordinary pages with new `kind` values. Three small extensions:

```sql
ALTER TABLE pages ADD COLUMN superseded_by INTEGER REFERENCES pages(id);

CREATE INDEX idx_pages_supersede_head
  ON pages(kind, superseded_by) WHERE superseded_by IS NULL;

CREATE INDEX idx_pages_session
  ON pages(json_extract(frontmatter, '$.session_id'))
  WHERE json_extract(frontmatter, '$.session_id') IS NOT NULL;

CREATE TABLE extraction_queue (
  id INTEGER PRIMARY KEY,
  session_id TEXT NOT NULL,
  conversation_path TEXT NOT NULL,    -- vault-relative
  trigger_kind TEXT NOT NULL,         -- 'debounce' | 'session_close' | 'manual'
  enqueued_at TEXT NOT NULL,
  scheduled_for TEXT NOT NULL,
  attempts INTEGER NOT NULL DEFAULT 0,
  last_error TEXT,
  status TEXT NOT NULL DEFAULT 'pending'  -- pending | running | done | failed
);
CREATE INDEX idx_extraction_queue_pending
  ON extraction_queue(status, scheduled_for) WHERE status = 'pending';

CREATE TABLE correction_sessions (
  correction_id TEXT PRIMARY KEY,
  fact_slug TEXT NOT NULL,
  exchange_log TEXT NOT NULL,           -- JSON array of {role, content}
  turns_used INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL,                 -- 'open' | 'committed' | 'abandoned' | 'expired'
  created_at TEXT NOT NULL,
  expires_at TEXT NOT NULL
);
CREATE INDEX idx_correction_open
  ON correction_sessions(status, expires_at) WHERE status = 'open';
```

The `superseded_by IS NULL` partial index makes "head only" filtering free at query time. Type-specific keys live in JSON frontmatter and are queried via `json_extract`.

The extraction queue UPSERTs on `(session_id, status='pending')` so 20 turns in 10 seconds collapses to one job whose `scheduled_for` keeps getting pushed forward.

## Components

New code is isolated in `src/core/conversation/` and `src/commands/extraction*`. Existing files get small surgical additions.

### New modules

| File | Purpose | Approx size |
|------|---------|-------------|
| `src/core/conversation/mod.rs` | Module root, public API surface | <50 lines |
| `src/core/conversation/turn_writer.rs` | Append turn to session `.md`, fsync, enqueue extraction | ~150 lines |
| `src/core/conversation/queue.rs` | Extraction queue: enqueue (UPSERT-collapse), dequeue, retry, mark-done | ~200 lines |
| `src/core/conversation/extractor.rs` | Extraction loop: pull job, read file, slice window, call SLM, parse, resolve, write facts | ~300 lines |
| `src/core/conversation/slm.rs` | Phi-3.5 wrapper over candle: lazy-load, prompt builder, output parser, panic boundary | ~250 lines |
| `src/core/conversation/supersede.rs` | Resolve dedup vs supersede vs coexist for each extracted fact | ~150 lines |
| `src/core/conversation/format.rs` | Conversation file parser/writer (frontmatter + turn blocks), cursor management | ~200 lines |
| `src/core/conversation/correction.rs` | `memory_correct` orchestration: SLM call, continuation tracking | ~200 lines |
| `src/core/conversation/file_edit.rs` | Vault-watcher hook: detect edits to `extracted/**`, write archived page, mark new head | ~120 lines |

Total new code: ~1,600–1,800 lines. Three reasonable-sized sub-pieces (turn capture, extraction pipeline, correction surface) each fit in one head.

### New CLI commands

| Command | Purpose |
|---------|---------|
| `quaid extraction enable` / `disable` | Flip config + trigger model download eagerly |
| `quaid extraction status` | Queue depth, model status, last extraction per session |
| `quaid model pull <alias>` | Manual model download (fallback / power-user path) |
| `quaid extract <session-id> [--force]` | Re-run extraction on a session file (`--force` resets cursor) |
| `quaid extract --all [--since <date>]` | Backfill / re-extract corpus |

### Modified files

| File | Change |
|------|--------|
| `src/core/db.rs` | Schema bump (v8); `superseded_by` column + indices; `extraction_queue` and `correction_sessions` tables |
| `src/core/types.rs` | New `Turn`, `ConversationFile`, `ExtractedFact` types; extend `Page` with `superseded_by` |
| `src/core/search.rs` | Add `superseded_by IS NULL` predicate to `hybrid_search`; new `--include-superseded` flag |
| `src/core/progressive.rs` | Same head-only filter applied before token-budget expansion |
| `src/core/migrate.rs` | `import_dir` recognizes new `kind` values for namespace classification |
| `src/mcp/server.rs` | Register `memory_add_turn`, `memory_close_session`, `memory_close_action`, `memory_correct`, `memory_correct_continue`; pass `--include-superseded` through `memory_search` and `memory_query` |
| `src/schema.sql` | Embedded DDL updated to v8 |
| `Cargo.toml` | Enable Phi-3 features in candle-transformers (already in dep tree) |

### Configuration keys

| Key | Default | Notes |
|-----|---------|-------|
| `extraction.enabled` | `false` | Master gate. `quaid extraction enable` flips this and triggers model fetch. |
| `extraction.model_alias` | `phi-3.5-mini` | Configurable: `gemma-3-1b`, `gemma-3-4b`, or HF model ID |
| `extraction.window_turns` | `5` | Sliding context window size |
| `extraction.debounce_ms` | `5000` | Debounce on incoming turn writes |
| `extraction.idle_close_ms` | `60000` | Idle timeout that triggers session-close flush |
| `extraction.max_retries` | `3` | Per-job retry cap |
| `memory.location` | `vault-subdir` | `vault-subdir` (default) or `dedicated-collection` |
| `corrections.history_on_disk` | `false` | Whether file-edit-archived pages are also written to `extracted/_history/` |

## SLM extraction pipeline

### Trigger

Three things put a job in the queue:

1. `memory_add_turn` enqueues `trigger_kind='debounce'`, `scheduled_for = now + 5s`. UPSERTs on `(session_id, status='pending')`.
2. `memory_close_session` enqueues `trigger_kind='session_close'`, `scheduled_for = now`. Bypasses debounce.
3. Idle timer (60s of no writes) auto-fires session_close.

A single worker tick polls `extraction_queue WHERE status='pending' AND scheduled_for <= now()` and processes one job at a time. Single worker keeps Phi-3.5 single-instance and avoids RAM contention. Multi-worker is a future option if throughput becomes a problem.

### Window selection

Given a job for `session_id`:

1. Read the conversation file. Parse frontmatter for `last_extracted_turn` cursor `C`.
2. New turns are `[C+1 .. last]`. If `last - C >= window_turns`, slice as non-overlapping windows of size `window_turns`. If less, slice as one window with up to `window_turns - new_count` prior turns as **lookback context** (shown to SLM but not extracted from).
3. For session_close jobs, force a final pass even if `new_count == 0`.

The lookback rule is what makes "yes, that works" extractable — the SLM sees what `that` refers to, but doesn't re-extract from already-extracted territory.

### Prompt structure

Single system prompt + windowed turns:

```
SYSTEM:
You extract durable facts from conversations. Output JSON only — no prose,
no markdown fences. Each fact is one of four kinds:

  decision     — a choice made between alternatives
  preference   — a stable inclination ("X likes/wants/prefers Y")
  fact         — a claim about the world or a person ("X is/has/works-at Y")
  action_item  — a commitment to do something with a clear actor

Skip ephemeral content (greetings, clarifications, transient task state).
Skip facts you already extracted in prior windows.
Facts must be supported by the windowed turns; do not infer beyond what was said.

Schema (one fact per object):
  decision     { kind, chose, rationale?, summary }
  preference   { kind, about, strength, summary }       # strength: low|medium|high
  fact         { kind, about, summary }
  action_item  { kind, who?, what, status, due?, summary }   # status: open

Required: kind, summary, plus the type-specific structured field(s).
Return: {"facts": [...]}.  Empty array if nothing durable.

USER:
Session: <session_id>
New turns to extract from (turns N..M):
  [turn N, role, ...]
  ...
Lookback context (do not extract from these — for reference only):
  [turn K, ...]
  ...
```

### Output parsing

Phi-3.5 outputs JSON. We parse strictly:

- Strip leading/trailing whitespace and accidental ```json fences.
- `serde_json::from_str` into `ExtractionResponse { facts: Vec<RawFact> }`.
- Validate each `RawFact` against the type-specific schema.
- Failures count toward `attempts`. Three strikes → mark job `failed`, surface in `quaid extraction status`. Daemon never blocks.

### Per-fact resolution

For each parsed fact F:

1. **Dedup.** Look up heads where `kind = F.kind AND <type_key> = F.<type_key>`. If found, compare prose embeddings to F. Cosine > 0.92 → drop F as duplicate.
2. **Supersede.** Same key match, prose embedding cosine in `[0.4, 0.92]` → write F as new page with `supersedes: <existing_head.id>`. Old head's `superseded_by = F.id`.
3. **Coexist (key match, low similarity).** Same key match, cosine < 0.4 → write F as a fresh head. The shared key is incidental — the SLM has extracted a fact about a different aspect (e.g. both `kind=preference, about=programming-language` but one is "prefers Rust for systems work" and the other is "comfortable in JavaScript for scripting").
4. **Coexist (no match).** No head shares the key → write F as a fresh head.
5. **Multi-match disambiguation.** Multiple heads match the key → pick the one with highest prose-embedding cosine to F, then apply rules 1–3 against that head only. The other heads are unchanged.

### Write step

Each accepted fact becomes a markdown file under `<vault>/extracted/<type>/<slug>.md`. Slug = `kind` + type-key + 4-char hash for collision avoidance (e.g. `matt-prefers-rust-a3f1.md`). The Phase 4 vault watcher picks up the new file and ingests as a page — **no direct DB write from the extraction path**. One write path keeps the architecture clean.

### After writing

Update conversation file frontmatter cursor: `last_extracted_turn = last_turn_in_window`, `last_extracted_at = now`. Mark queue job `done`.

### Failure modes

| Failure | Handling |
|---------|----------|
| Phi-3.5 panic (rare, candle) | `catch_unwind` boundary; mark job failed; flip `extraction.enabled = false` in-memory only (config row unchanged); log loudly. Daemon continues. Manual restart re-enables. |
| Garbage SLM output (3 strikes) | Mark job failed; cursor not advanced; surface in `quaid extraction status`. User re-triggers via `quaid extract <session>`. |
| Vault file write fails (disk full, perms) | Mark job failed; preserve queue row; log. The originating `memory_add_turn` already succeeded — we don't roll back the turn. |
| Model file missing | Daemon logs error at first job, marks `extraction.enabled` runtime-disabled, does not retry. Re-enable via `quaid extraction enable`. |
| Concurrent edits to a conversation file (Obsidian) | Vault sync is conflict-aware (Phase 4); we re-read cursor after sync. Parse errors on broken format are logged; extraction skips that window. |

## Correction surface

Two cooperating mechanisms.

### `memory_correct` and `memory_correct_continue` — bounded dialogue MCP tools

Two tools form the bounded dialogue: `memory_correct` initiates, `memory_correct_continue` advances or abandons. Capped at 3 SLM exchanges before forced-abandon. Backed by the `correction_sessions` table.

```json
// memory_correct (initiate) — input
{
  "fact_slug": "matt-business-administration-degree",
  "correction": "I didn't study business administration, I studied CS with a major in Business Systems"
}

// output (one of two shapes)
// (a) committed in one shot
{
  "status": "committed",
  "new_fact_slug": "matt-cs-business-systems-degree",
  "supersedes": "matt-business-administration-degree"
}
// (b) needs clarification
{
  "status": "needs_clarification",
  "correction_id": "corr_2026-05-03_a3f1",
  "question": "Should I replace the existing fact, or keep both — for example as 'initially pursued Business Administration' and 'graduated with CS, Business Systems major'?",
  "turns_remaining": 2
}
```

```json
// memory_correct_continue (continue) — input
{
  "correction_id": "corr_2026-05-03_a3f1",
  "response": "Replace it. I never actually studied Business Admin."
}
// output: same shape (committed | needs_clarification | abandoned)

// memory_correct_continue (abandon) — input
{ "correction_id": "corr_2026-05-03_a3f1", "abandon": true }
// output
{ "status": "abandoned", "reason": "user_requested" }
```

The SLM prompt for correction mode is constrained to one of three outcomes per turn: commit / clarify / abandon-with-reason. It is **not** a chat partner — its job is to determine the corrected fact and write it. Hard cap at 3 turns. A small janitor in `quaid serve` purges expired open sessions hourly.

The corrected fact's frontmatter records `corrected_via: explicit` so explicit corrections are distinguishable from organic supersedes.

### File-edit-aware supersede

The Phase 4 vault watcher already fires on file changes. We add a small handler for changes to `extracted/**/*.md`:

1. Compute hash of the file's prior content (already in `pages.content_hash`).
2. If hash differs and `kind ∈ {decision, preference, fact, action_item}`:
   - Write the **prior** version as a new page with `superseded_by=<new_id>`, slug `<original-slug>--archived-<timestamp>`. By default this archive page lives only in the DB (not in the vault) to keep `extracted/` clean. Power users can opt into disk-level history via `corrections.history_on_disk = true`.
   - The edited file becomes a new head with `supersedes: <archived_id>`, `corrected_via: file_edit`.
3. If hash differs but content is logically identical (whitespace-only edits, etc.), no-op.

Without this, every Obsidian user who fixes a typo in an extracted fact silently destroys history. With it, the supersede chain remains the canonical truth regardless of how the correction arrived.

## Public API contracts

### MCP tools — new

#### `memory_add_turn`

```json
// input
{
  "session_id": "string",          // required; caller-chosen, opaque to Quaid
  "role": "user|assistant|system|tool",
  "content": "string",             // required; raw turn text
  "timestamp": "ISO-8601",         // optional; defaults to server now
  "metadata": { ... }              // optional; opaque; preserved in turn block
}

// output (synchronous, p95 < 50 ms)
{
  "turn_id": "<session_id>:<ordinal>",
  "conversation_path": "conversations/2026-05-03/openclaw-main.md",
  "extraction_scheduled_at": "ISO-8601"  // null if extraction disabled
}

// errors
- ConflictError       // session was closed; caller must rotate session_id
- ConfigError         // memory.location collection isn't writable
```

#### `memory_close_session`

```json
// input
{ "session_id": "string" }

// output
{
  "closed_at": "ISO-8601",
  "extraction_triggered": true,    // false if no new turns since last extract
  "queue_position": 1              // estimated jobs ahead; null if disabled
}

// errors
- NotFoundError       // unknown session_id
- AlreadyClosedError  // idempotent re-close returns original close timestamp
```

#### `memory_close_action`

```json
// input
{
  "slug": "ship-phase5-2026-06-01",
  "status": "done|cancelled",
  "note": "string"                 // optional; appended to action_item body
}

// output
{ "updated_at": "ISO-8601", "version": 3 }

// errors
- NotFoundError       // unknown slug
- KindError           // slug isn't kind=action_item
- ConflictError       // optimistic concurrency clash
```

#### `memory_correct` and `memory_correct_continue`

See "Correction surface" above for shapes. Both tools share the same response shape (`committed` | `needs_clarification` | `abandoned`); `memory_correct` is the entry point that creates a `correction_id`, and `memory_correct_continue` advances or abandons an existing `correction_id`.

### MCP tools — modified

- `memory_search`, `memory_query`: new optional `include_superseded: bool` (default `false`). Default behavior is head-only.
- `memory_get`: returns the page as-is regardless of supersede status. New optional response field `superseded_by: "<slug>"`.
- `memory_graph`: exposes supersede chains as a new `superseded_by` edge type so the chain is navigable.

### CLI surface

```bash
# Lifecycle
quaid extraction enable                    # eager model fetch + flip flag
quaid extraction disable
quaid extraction status                    # queue, model, last-extract per session
quaid model pull <alias>                   # manual download path

# Operations
quaid extract <session-id> [--force]       # re-run extraction (--force resets cursor)
quaid extract --all [--since <date>]       # backfill

# Existing search / query gain a flag
quaid search "..." --include-superseded
quaid query "..." --include-superseded
```

### Namespace interaction (`#137`)

- `session_id` is namespace-local. Two agents in different namespaces can use the same `session_id` without collision; the conversation file path includes the namespace.
- Default vault path becomes `<namespace>/conversations/...` and `<namespace>/extracted/...`.
- Extraction runs per-namespace; the SLM is shared (one process, one model) but jobs are tagged with namespace and supersede lookups are namespace-scoped.
- Cross-namespace fact recall is explicitly **not** supported.

## Acceptance gates

| Gate | Target | Test |
|------|--------|------|
| LoCoMo benchmark | ≥ 40% (from 0.1% baseline) | benchmark harness CI |
| LongMemEval | ≥ 40% (from 0.0% baseline) | benchmark harness CI |
| `memory_add_turn` p95 latency | < 50 ms | `tests/turn_latency.rs` |
| Extraction p95 (per window) | < 3 s on M1/M2 Mac, < 8 s on x86_64 Linux | `benches/extraction.rs` |
| Extraction lag (turn arrival → fact queryable) | < 30 s typical cadence | end-to-end test |
| Idempotency | `quaid extract <session> --force` produces stable supersede chain | `tests/extraction_idempotency.rs` |
| Airgap | Zero network calls after `quaid extraction enable` succeeds | `tests/airgap_extraction.rs` (network-namespace-isolated) |
| Supersede correctness | "Latest in chain" is unambiguous; `--include-superseded` exposes history | `tests/supersede_chain.rs` |
| File-edit supersede | User edit produces archived breadcrumb + new head; doesn't lose history | `tests/file_edit_supersede.rs` |
| Correction tool | Bounded dialogue commits in ≤ 3 turns; abandons cleanly; expires after 1h | `tests/memory_correct.rs` |

## Observability

`quaid extraction status` returns a human-readable summary:

```
Model:               phi-3.5-mini (loaded, 1842 MB resident)
Extraction enabled:  yes
Queue:               2 pending, 0 running, 1 failed (last 24h)
Active sessions:     3 (openclaw-main idle 12s, claude-code-shell idle 47s, debugging-loop idle 2m)
Last extraction:     2026-05-03T10:31:04Z (session: openclaw-main, 4 facts written)
Failed jobs (24h):   1 — session: stale-test, attempts: 3, last_error: "JSON parse failure at offset 247"
```

Structured logs via existing `tracing` infrastructure:

- `extraction.job.started` / `.completed` / `.failed` — fields: `session_id`, `window_size`, `duration_ms`, `facts_written`
- `extraction.dedup` — fields: `kind`, `type_key`, `decision` (`drop`/`supersede`/`coexist`), `cosine`
- `extraction.parse_failure` — fields: `session_id`, `raw_output` (truncated to 500 chars), `attempt`
- `correction.session.opened` / `.committed` / `.abandoned` / `.expired`
- `file_edit.supersede` — fields: `slug`, `old_content_hash`, `new_content_hash`

DAB harness extension: a §8 Conversation Memory section scoring multi-session recall against the LoCoMo adapter, landing alongside Phase 5 so we can track regression across releases the way §4 currently is.

## Risks

1. **Phi-3.5 hallucinates facts that weren't in the conversation.** Mitigation: prompt explicitly constrains "facts must be supported by the windowed turns; do not infer beyond what was said." `extracted_by` field provides audit trail. Phase 7 active enrichment can run a verification pass.

2. **`session_id` collision across distinct contexts.** If two callers pick the same `session_id` they merge into one conversation file. Mitigation: session_ids are namespace-local; we document caller responsibility for uniqueness. We don't enforce.

3. **2GB model + extraction memory pressure on small machines.** Mitigation: `extraction.enabled = false` is the default. Documented resource cost. Gemma 3 1B (~600MB) is the lower-resource alternative configured via `quaid model pull gemma-3-1b` + `quaid extraction set-model gemma-3-1b`.

4. **DB pages table grows fast.** A power user with 100 turns/day for a year is ~36K turn pages plus extraction output. Inside SQLite + vec0's comfort zone (per the Q3 sizing analysis: ~250–400 MB DB) but pushes us closer to Phase 8 / `#134` scale targets faster than docs-only usage. Accepted; Phase 8 is on the roadmap.

## Resolutions to v3 design questions

| v3 question | Resolution |
|---|---|
| Is SLM extraction synchronous or background queue? | Background queue. Debounced (5s) plus session-close trigger. `memory_close_session` forces immediate flush. |
| What is the 3-5 turn context window boundary? | `session_id` is the boundary. Cross-session extraction not supported. Window size configurable via `extraction.window_turns`, default 5. |
| How does contradiction resolution interact with ADD-only immutability? | ADD-only supersede chain, keyed off type-specific structured frontmatter (`about`/`chose`/`what`). No mutation of existing fact pages. `superseded_by` index makes head-only filtering free. |
| Model download: lazy or eager? | Eager. `quaid extraction enable` triggers download with progress UI. Manual `quaid model pull <alias>` available as fallback. Daemon never silently downloads. |
| What structured format for extracted facts? | Hybrid: type-specific structured frontmatter (one or two required fields per type) + SLM-written prose body. Frontmatter drives dedup/supersede; prose drives FTS5 / vec0 retrieval. |

## Future work (not in Phase 5)

- Entity extraction shares the SLM pipeline established here (Phase 6 / `#107`).
- Active enrichment reads supersede chains and propagates updates to related facts (Phase 7 / `#136`).
- DAB §8 Conversation Memory section becomes a regression gate alongside §4 once Phase 5 ships.
- Multi-worker extraction if single-worker throughput becomes a problem at scale.
- Web UI for browsing / bulk-correcting facts is a future possibility but vault + Obsidian is the supported path for v1.
