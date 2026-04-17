---
title: Why GigaBrain
description: A local-first brain that scales past thousands of markdown files — no cloud, no API keys, no compromise.
---

The current landscape of knowledge management is fractured by dependency. We rely on centralized servers to host our insights, making our collective memory vulnerable to service outages, policy changes, and the whims of tech conglomerates.

**GigaBrain** is an "architectural" approach to software — built with the same philosophy as a physical library. Heavy, stable, and entirely yours. A personal knowledge base should be a lifelong asset, not a temporary lease.

> "Intelligence should not require a handshake with the cloud."

## Zero cloud, zero keys

No subscription models. No API keys to manage. Your intelligence stays within the physical confines of your hardware, operating with the raw speed of local silicon.

Git doesn't scale past ~5,000 markdown files. At that size, a wiki-brain becomes slow to clone, painful to search, and structurally opaque. GigaBrain replaces the file system with a purpose-built SQLite layer that handles millions of notes without breaking a sweat.

## Radical privacy

In an era of data harvesting, GigaBrain acts as a digital bunker. Your thoughts are never indexed, never trained upon, and never transmitted.

Most existing knowledge tools either require a GUI, lock your data in a SaaS platform, or rely on internet and API keys for anything semantic. GigaBrain runs entirely offline — on a plane, in an air-gapped environment, without any ongoing costs.

## Instant retrieval

Latency is the enemy of thought. By leveraging local vector databases, GigaBrain provides millisecond responses across millions of documents.

The hybrid search combines **FTS5 keyword precision** with **BGE-small semantic intelligence** using Reciprocal Rank Fusion. You get the best of both without choosing between them.

## Future proof

Built on open standards — Markdown and SQLite. Your knowledge base remains readable and queryable long after proprietary platforms disappear.

The single `brain.db` file is a standard SQLite database. You can open it with any SQLite client, export it, back it up with `cp`, and query it directly with SQL. No lock-in, no migrations, no vendor.

## Built for an agent-first world

GigaBrain is designed for an agent-first world where your knowledge layer needs to:

- Live in a single file you own completely
- Do full-text **and** semantic search natively
- Expose an MCP server for any AI client
- Work offline (plane / air-gapped) with zero ongoing costs

Any MCP-compatible AI client — Claude Code, Continue, or your own tools — can connect and query your brain over stdio with 16 production-ready tools.
