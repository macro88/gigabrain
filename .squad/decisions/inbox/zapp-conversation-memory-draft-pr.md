# Zapp conversation-memory draft PR

- **Timestamp:** 2026-05-04T07:22:12.881+08:00
- **Change:** `conversation-memory-foundations`
- **Scope:** Draft PR truth boundary for `feat/slm-conversation-mem`

## Decision

Open a draft PR against `main`, but scope the claim to the pushed schema/supersede foundations slice plus the OpenSpec truth repair only.

## Why

The branch compare currently carries broader roadmap and planning ancestry than the implementation that is actually landed on `feat/slm-conversation-mem`. Narrowing the PR body to the pushed slice keeps reviewer, docs, and launch messaging aligned with reality while the larger conversation-memory change is still in progress.

## Guardrails

- State explicitly that the larger `conversation-memory-foundations` change is still in progress.
- Do not claim `memory_add_turn`, queue worker or extraction runtime behavior, or release readiness.
- Keep the PR in draft until the wider implementation actually lands and is pushed.
