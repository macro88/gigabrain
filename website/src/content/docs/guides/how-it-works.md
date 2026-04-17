---
title: How It Works
description: Compiled truth + append-only timelines, stored in SQLite with local vector embeddings.
---

GigaBrain combines high-performance local embeddings with robust relational metadata to create a private, lightning-fast knowledge engine. Unlike traditional cloud-based RAG pipelines, everything runs on the edge — zero-latency interactions, absolute privacy.

## The compiled knowledge model

GigaBrain adapts Andrej Karpathy's compiled knowledge model. Every page has two zones:

**Above the line — compiled truth.** Always current. Rewritten when new information arrives. This is the "what we believe now" view, optimized for answering questions quickly and accurately.

**Below the line — timeline.** Append-only. Never rewritten. This is the evidence base — what happened, when, and where it came from. Your full history in one append-only log.

## Metadata via SQLite

We use **SQLite** for relational indexing. This allows for complex filtering, tag-based retrieval, and maintaining high-integrity document state without the overhead of a managed database service.

- ACID-compliant local storage — your data is safe
- Zero-config deployment — one file, no server, no Docker
- Full cross-platform portability — copy `brain.db` anywhere

Pages are stored in a single `brain.db` file with a typed, temporal link graph for relationships like `works_at`, `founded`, `knows`, and custom edge types you define.

## Local embeddings via the Candle engine

Using the Rust-native **Candle** framework, GigaBrain executes **BGE-small-en-v1.5** embeddings locally with CPU-optimized SIMD instructions.

Two release channels give you control over binary size vs. first-run latency:

- **Airgapped** — BGE-small weights embedded at compile time. No network required, ever.
- **Online** — weights downloaded and cached on first semantic use. Smaller binary, same privacy thereafter.

Your data never leaves the machine. No API keys. No inference costs. No usage logs.

## FTS5 hybrid search

Why choose between semantic and keyword search? GigaBrain uses **SQLite FTS5** extensions alongside vector similarity and ranks results with a **Reciprocal Rank Fusion (RRF)** algorithm.

- **`gbrain search "machine learning"`** — pure keyword, FTS5-powered
- **`gbrain query "who has worked with Jensen Huang?"`** — semantic hybrid, sub-40ms

The result is a ranked list that combines the precision of keyword matching with the contextual intelligence of embedding similarity.

## The thin CLI

The CLI is deliberately minimal. The workflows live in markdown `SKILL.md` files that agents read and follow. Any MCP-compatible AI client — Claude Code, Continue, or your own — can connect over stdio and use the full tool surface.
