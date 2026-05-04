# Atomic Queue State Transitions

## Use when

- A SQLite-backed queue has retries, leases, or worker crash recovery
- A worker-facing `mark_failed` / `requeue` path currently does `SELECT attempts` followed by `UPDATE`
- You need tests that reflect the real `pending -> running -> pending|failed` lifecycle

## Pattern

1. Make retry/fail transitions a single `UPDATE` against `status = 'running'`.
2. Compute `attempts = attempts + 1` and terminal/non-terminal status inside SQL.
3. Make lease expiry recovery use the same atomic style so stale workers cannot resurrect or double-count jobs.
4. Test retries by dequeuing between failures; do not call `mark_failed` repeatedly on a `pending` row.

## Why

Separate `SELECT` then `UPDATE` retry logic creates TOCTOU bugs once lease expiry or concurrent workers exist. Atomic SQL keeps stale workers from overwriting a newer `running` or `failed` state and makes retry caps truthful.
