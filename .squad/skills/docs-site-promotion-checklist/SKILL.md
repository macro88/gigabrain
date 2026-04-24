---
name: docs-site-promotion-checklist
version: 1.0
author: hermes
last_updated: 2026-04-25
---

# Docs-Site Promotion-Readiness Checklist

Use this skill before any public release or branch promotion to audit the docs-site surface for staleness.

## Checklist — what drifts between branches

### Tool count references (update all four when a new MCP tool lands)

1. `website/src/content/docs/guides/mcp-server.md` — Available Tools table
2. `website/src/content/docs/guides/phase3-capabilities.md` — `call` command description ("`N` MCP tools")
3. `website/src/content/docs/guides/phase3-capabilities.md` — Related section link ("all `N` MCP tool examples")
4. `website/src/content/docs/guides/getting-started.mdx` — action banner prose ("the `N` production tools")

### Version numbers (update when a new release tag is cut)

1. `website/src/content/docs/guides/install.mdx` — GitHub Releases `VERSION='...'` in CodePanel
2. `website/src/content/docs/guides/install.mdx` — all four `GBRAIN_VERSION=...` lines in the installer CodePanel
3. `README.md` — "**Status:** `vX.Y.Z` (released)" and install table rows

### Schema version references (update when schema version bumps)

1. `website/src/content/docs/guides/getting-started.mdx` — step 01 prose ("full vN schema")
2. `website/src/content/docs/contributing/contributing.md` — repo layout `schema.sql` annotation ("Full vN DDL")

### Homepage accuracy

1. `website/src/content/docs/index.mdx` — "Get running in seconds" code block must not show simulated output.
   - `gbrain serve` is stdio MCP, not HTTP. Never show `"Server active at http://..."`.
   - `gbrain init` produces a confirmation line, not detailed schema output.
   - Safe pattern: show commands only, no fake output; or use known-accurate one-line outputs.

### Roadmap completeness

1. `website/src/content/docs/contributing/roadmap.md` — add a section for every active branch with named landed capabilities and an explicit "deferred" list.
2. Version targets table: no internal issue numbers; one sentence per tag; match README.
3. If work is deferred (restore, IPC, etc.), name it explicitly as deferred — silence reads as "not planned."

## Anti-patterns to avoid

- **Fake terminal output**: Any `$ command\nSome simulated response` block will eventually diverge from reality. Prefer showing commands only, or extract real output from integration tests.
- **"N tools" without audit**: Never increment the tool count without verifying all four reference sites above.
- **Stale version pins in installer examples**: Users copy-paste these; stale pins silently install old binaries.
- **Advertising deferred work**: If a feature is on a branch but not released (restore, IPC, live recovery worker), it must be labeled "deferred" in the roadmap, not "coming soon" in feature prose.
