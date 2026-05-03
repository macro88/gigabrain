---
name: quaid-ingest
description: |
  Ingest meeting notes, articles, documents, and conversations into Quaid.
  Handles idempotent ingestion for exact-byte duplicates and vault-backed sync.
---

# Ingest Skill

## Overview

The ingest skill processes raw source documents into structured brain pages.
Re-ingesting the same exact bytes is a no-op unless `--force` is used.

## Commands

### Single file ingest

```bash
quaid ingest /path/to/document.md
quaid ingest /path/to/document.md --force  # re-ingest even if hash matches
```

The file must be valid markdown with optional YAML frontmatter. Frontmatter
fields `title`, `type`, `slug`, and `wing` are used if present; otherwise
defaults are derived from the file name and content.

### Vault-backed batch ingest

```bash
quaid collection add notes /path/to/directory/
quaid serve
```

`quaid collection add` performs the initial scan and attaches the directory as a
live-sync collection. `quaid serve` keeps the index fresh on Unix platforms.

### Export

```bash
quaid export /path/to/output/
```

Exports all pages as canonical markdown files to the output directory.
Files are written to `<output>/<slug>.md`, creating parent directories as needed.

## Idempotency

- Exact raw file bytes are the idempotency key for duplicate single-file ingest
- Active source bytes and paths are stored in `raw_imports`
- `--force` bypasses the hash check and re-ingests

## Frontmatter Handling

- `slug`: used as-is if present; otherwise derived from file path
- `title`: used as-is if present; otherwise set to slug
- `type`: used as-is if present; defaults to `concept`
- `wing`: used as-is if present; otherwise derived from slug prefix

## Filing Disambiguation

When the same entity could go in multiple wings, prefer the wing
that matches the slug prefix (e.g., `people/` → `people` wing).

