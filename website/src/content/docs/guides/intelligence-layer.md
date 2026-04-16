---
title: Intelligence Layer
description: Graph traversal, contradiction detection, and knowledge gaps — Phase 2 capabilities that turn search into knowledge.
sidebar:
  badge:
    text: Phase 2
    variant: note
---

Phase 1 gives you search. Phase 2 gives you **knowledge**.

The intelligence layer adds cross-reference graph traversal, assertion-based contradiction detection, novelty filtering on ingest, automatic knowledge gap tracking, and palace room classification. These are the capabilities that separate GigaBrain from a glorified FTS5 wrapper.

---

## Graph traversal

GigaBrain maintains a **typed temporal link graph** between pages. Each link has a `relationship`, an optional `valid_from` date, and an optional `valid_until` date (for closed intervals).

### Create links

```bash
gbrain link people/alice companies/acme --relationship works_at --valid-from 2023-01-01
gbrain link people/alice people/bob --relationship mentors
gbrain link companies/acme companies/bigco --relationship acquired_by --valid-from 2025-06-01
```

### Close a link when it ends

```bash
gbrain link-close 42 --valid-until 2026-01-01
```

### Traverse the graph

```bash
# 1-hop neighborhood (default)
gbrain graph people/alice

# 2-hop, active links only
gbrain graph people/alice --depth 2

# Include historically closed links
gbrain graph people/alice --depth 2 --temporal all

# JSON output
gbrain graph people/alice --depth 2 --json
```

**Example human-readable output:**

```
people/alice
  → companies/acme (works_at)
    → companies/bigco (acquired_by)
  → people/bob (mentors)
```

**Example JSON output:**

```json
{
  "nodes": [
    { "slug": "people/alice", "page_type": "person", "summary": "Staff engineer at Acme." },
    { "slug": "companies/acme", "page_type": "company", "summary": "Enterprise SaaS." },
    { "slug": "people/bob", "page_type": "person", "summary": "Engineering manager." }
  ],
  "edges": [
    {
      "from": "people/alice",
      "to": "companies/acme",
      "relationship": "works_at",
      "valid_from": "2023-01-01",
      "valid_until": null
    },
    {
      "from": "people/alice",
      "to": "people/bob",
      "relationship": "mentors",
      "valid_from": null,
      "valid_until": null
    }
  ]
}
```

### See who links to a page

```bash
gbrain backlinks companies/acme
```

---

## Contradiction detection

GigaBrain extracts **assertions** from each page — subject/predicate/object triples derived from compiled truth text — and compares them for conflicts. When two assertions share the same subject and predicate but carry different objects, a contradiction is recorded.

### Run a check

```bash
# Check one page
gbrain check --slug people/alice

# Scan every page in the brain
gbrain check --all

# JSON output (useful for automation)
gbrain check --all --json
```

**Example output (human-readable):**

```
[people/alice] ↔ [sources/alice-profile]: Conflicting 'employer' values: 'Acme' vs 'Beta Corp'
1 contradiction(s) found across 2 pages.
```

**Example JSON output:**

```json
[
  {
    "page_slug": "people/alice",
    "other_page_slug": "sources/alice-profile",
    "type": "assertion_conflict",
    "description": "Conflicting 'employer' values: 'Acme' vs 'Beta Corp'",
    "detected_at": "2026-04-15T10:00:00Z"
  }
]
```

An empty array or "No contradictions found." means the brain is internally consistent.

### How it works

1. `gbrain check` calls `assertions::extract_assertions` on each target page — this parses compiled truth text into triples and writes them to the `assertions` table.
2. It then runs `assertions::check_assertions` which queries for same-subject, same-predicate, different-object pairs and logs them in the `contradictions` table.
3. Contradictions remain until you resolve them by updating the page content and re-running `gbrain check`.

> **Design constraint:** GigaBrain's binary does heuristic assertion extraction only. Cross-page synthesis and LLM-assisted contradiction resolution live in the `maintain` skill — run `gbrain skills` to see it.

---

## Progressive retrieval

The `query` command supports `--depth auto` to enable **token-budget progressive retrieval**. Instead of returning a fixed number of results, the engine expands its search breadth until it hits a token budget ceiling — returning richer context for complex questions.

```bash
# Standard hybrid query (FTS5 + vectors)
gbrain query "who works at Acme Corp?"

# Progressive retrieval — expands until token budget is spent
gbrain query "who works at Acme Corp?" --depth auto

# JSON output
gbrain query "who works at Acme Corp?" --depth auto --json
```

Use `--depth auto` when asking multi-entity questions or when initial results feel sparse. For simple lookups, the default mode is faster.

---

## Novelty checking on ingest

When you ingest a document, GigaBrain runs a **novelty check** before writing anything. It computes:

- **Jaccard similarity** between the incoming content's shingles and existing page content
- **Cosine similarity** between the incoming content's embedding and existing page embeddings

If either similarity exceeds the threshold (Jaccard ≥ 0.85 or cosine ≥ 0.95), the ingest is skipped as a near-duplicate and the source hash is recorded in `ingest_log` for idempotency.

```bash
gbrain ingest ./notes/meeting-notes.md --type meeting
# If near-duplicate: "Skipped: content is 91% similar to existing page people/alice"
```

Ingest is fully idempotent — running the same file twice produces the same result.

---

## Palace room classification

GigaBrain's memory palace organizes pages into **wings** (derived from the slug prefix) and **rooms** (derived from `##` heading content). Room classification happens automatically on put and ingest.

| Slug prefix | Wing |
| --- | --- |
| `people/*` | `people` |
| `companies/*` | `companies` |
| `decisions/*` | `decisions` |
| `sources/*` | `sources` |

Rooms are derived from the first `##` heading in the compiled truth section. For example, a page with `## Technical Background` under `## Skills` will be classified into the `skills` room.

Room classification is used internally to narrow vector search to the most relevant wing before expanding to the full corpus — reducing latency without precision loss.

---

## Knowledge gaps

When `gbrain query` returns results with low confidence (few matches, low scores), GigaBrain automatically logs a **knowledge gap** — a record of what the brain couldn't answer.

### View gaps

```bash
# List unresolved gaps
gbrain gaps

# Show more results
gbrain gaps --limit 50

# Include resolved gaps
gbrain gaps --resolved

# JSON output
gbrain gaps --json
```

**Example output:**

```
[42] who-funds-acme (confidence: 0.12, unresolved)
[43] alice-previous-employer (confidence: 0.08, unresolved)
2 gap(s) found.
```

**Example JSON output:**

```json
[
  {
    "id": 42,
    "query_hash": "who-funds-acme",
    "context": "who funds acme corp?",
    "confidence_score": 0.12,
    "sensitivity": "internal",
    "resolved_at": null,
    "detected_at": "2026-04-15T08:30:00Z"
  }
]
```

### Resolving gaps

Gaps are resolved by adding content that answers the question and re-running the query. The `research` skill (see `gbrain skills`) provides an agent workflow for systematically closing knowledge gaps.

---

## MCP tools for the intelligence layer

All Phase 2 capabilities are available via MCP for AI agent workflows. See the [MCP Server guide](/guides/mcp-server/) for full examples.

| Tool | Use case |
| --- | --- |
| `brain_link` | Agent creates a typed temporal link between pages |
| `brain_link_close` | Agent closes a link when a relationship ends |
| `brain_backlinks` | Agent discovers what links to a page |
| `brain_graph` | Agent traverses the knowledge graph N hops from a page |
| `brain_check` | Agent runs contradiction detection before writing |
| `brain_timeline` | Agent reads structured timeline entries for a page |
| `brain_tags` | Agent lists, adds, or removes tags |

---

## Related

- [CLI Reference](/reference/cli/) — full command flags and examples
- [MCP Server](/guides/mcp-server/) — Phase 2 MCP tool call examples
- [Architecture](/reference/architecture/) — schema and module map
