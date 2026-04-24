# Using GigaBrain v0.9.6 as an OpenClaw Harness

GigaBrain `v0.9.6` works well as the memory and knowledge layer for agents running on OpenClaw. OpenClaw talks to GigaBrain over MCP, while GigaBrain keeps a local SQLite brain synchronized with one or more markdown collections.

This release changes the recommended setup:

- Old one-shot ingest: `gbrain import <path>`
- New live-sync workflow: `gbrain collection add <name> <path>` then `gbrain serve`

When `gbrain serve` starts on macOS/Linux, it starts the MCP server and the live watcher for active collections. There is no separate sync daemon to run.

## Prerequisites

- macOS or Linux host for `gbrain serve`
- `gbrain` `v0.9.6`
- An initialized brain database
- An OpenClaw install that supports MCP server configuration

Initialize the database once:

```bash
gbrain init ~/brain.db
```

## Attach a vault as a collection

For ongoing sync with Obsidian or any markdown vault, attach the directory as a collection instead of using `gbrain import`.

```bash
gbrain collection add notes ~/Documents/Obsidian
```

This creates the collection metadata in the v5 schema and performs the initial reconcile. In `v0.9.6`, the benchmark on a 350-page vault is about 5 seconds for `collection add`.

Collections are backed by the new schema tables introduced in `v0.9.6`:

- `collections`
- `file_state`
- `embedding_jobs`
- `raw_imports`
- `collection_owners`

Use `.gbrainignore` at the vault root to exclude files or patterns from sync:

```gitignore
.obsidian/
Templates/
Daily/*.tmp.md
archive/**
```

## Configure OpenClaw

OpenClaw should launch `gbrain serve` as an MCP server. Put the GigaBrain block in `openclaw.json` under `mcp.servers`.

Recommended `openclaw.json` snippet:

```json
{
  "mcp": {
    "servers": {
      "gbrain": {
        "command": "gbrain",
        "args": ["serve"],
        "env": {
          "GBRAIN_DB": "/Users/alice/brain.db"
        }
      }
    }
  }
}
```

The important part is `GBRAIN_DB`: it must point at the `brain.db` file that OpenClaw agents should use.

If you want to test the server outside OpenClaw first:

```bash
GBRAIN_DB=~/brain.db gbrain serve
```

## Live sync workflow

The normal OpenClaw workflow is:

1. Initialize the database with `gbrain init`.
2. Attach one or more vaults with `gbrain collection add`.
3. Configure OpenClaw to spawn `gbrain serve`.
4. Start OpenClaw.
5. Edit markdown files in the vault. The watcher started by `gbrain serve` reconciles changes into `brain.db` automatically.

On `v0.9.6`, `gbrain serve` owns the watcher lifecycle on Unix/macOS. File edits, creates, and deletes are picked up automatically after the server starts.

Deleted pages are not hard-deleted immediately. `v0.9.6` adds a quarantine lifecycle so pages with preserved DB-side state can be reviewed and restored instead of being dropped blindly.

## MCP usage patterns

OpenClaw agents should treat the GigaBrain MCP tools as the durable memory interface. `v0.9.6` exposes 17 tools, including the new `brain_collections` status tool.

### `brain_query` vs `brain_search`

Use `brain_query` for:

- natural-language questions
- synthesis across multiple pages
- semantic retrieval when wording may not match exactly

Use `brain_search` for:

- exact keywords
- names, titles, tags, or phrases likely to appear verbatim
- fast recall when you know the text you want

Benchmarks from the `v0.9.6` 350-page DAB run:

- `collection add`: 5s
- FTS query: 26ms
- semantic query: 93ms
- all checks: passed

### When to use `brain_put`

Use `brain_put` when the agent is intentionally creating or updating durable knowledge in the brain, not for temporary scratch work.

For updates to an existing page:

1. Call `brain_get` first.
2. Read the current `version`.
3. Send `brain_put` with `expected_version`.

That preserves optimistic concurrency and avoids blind overwrites.

### Use `brain_collections` for health checks

`brain_collections` is the right first check when OpenClaw can query the MCP server but results look stale or writes are blocked.

It returns a JSON array of collection status records. Check these fields first:

- `state`
- `page_count`
- `last_sync_at`
- `embedding_queue_depth`
- `ignore_parse_errors`
- `needs_full_sync`
- `recovery_in_progress`
- `integrity_blocked`
- `restore_in_progress`

Example request:

```json
{}
```

Example response:

```json
[
  {
    "name": "notes",
    "root_path": "/Users/alice/Documents/Obsidian",
    "state": "active",
    "writable": true,
    "is_write_target": true,
    "page_count": 350,
    "last_sync_at": "2026-04-25T09:01:00Z",
    "embedding_queue_depth": 0,
    "ignore_parse_errors": null,
    "needs_full_sync": false,
    "recovery_in_progress": false,
    "integrity_blocked": null,
    "restore_in_progress": false
  }
]
```

If `ignore_parse_errors` is non-null, fix the `.gbrainignore` file. If `needs_full_sync` is `true` or `state` is not `active`, treat the collection as unhealthy until reconcile or restore finishes.

## Recommended operating model

- Use `gbrain collection add` for vault-backed knowledge sources.
- Keep `gbrain import` for one-shot bulk ingest, not live vault sync.
- Let OpenClaw spawn `gbrain serve`; that keeps MCP and watcher lifecycle in one process.
- Run `brain_query` first for agent reasoning, then fall back to `brain_search` for exact recall.
- Use `brain_put` only for durable page updates.
- Poll `brain_collections` during startup or incident diagnosis.

