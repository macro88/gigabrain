# leela history

- [2026-04-29T07-04-07Z] History summarized and archived

## Learnings

- 2026-04-30T06:37:20.531+08:00 — **Batch 4 scope reconciliation:** When `tasks.md` and `implementation_plan.md` describe the same task differently, the implementation_plan is authoritative for batch scope. Fix `tasks.md` to match; never expand scope in `tasks.md` beyond what the plan intends for the batch.
- 2026-04-30T06:37:20.531+08:00 — **IPC proxy deferral pattern:** A security-critical subsystem (IPC socket with kernel peer-UID verification) must land and be reviewed as its own batch before any client code is built against it. Building client-side proxy before the server socket design is locked creates a dependency inversion and risks shipping unauthenticated writes.
- 2026-04-30T06:37:20.531+08:00 — **Scope note format in tasks.md:** When narrowing a task from its original spec description, add a `> **Scope note (Author, date):**` annotation inline below the task line explaining the narrowing rationale and which batch/tasks will complete the original scope. This creates an auditable trail without losing original intent.
- 2026-04-30T06:37:20.531+08:00 — Batch 4 target scope (v0.13.0): `12.1` (complete 13-step rename-before-commit), `12.6` (expected_version contract), `12.6a` (refuse-when-live stub, NOT proxy), `12.6b` (verify only — already closed in Batch 3), `12.7` (tests). IPC tasks (11.9, 12.6c–g) are Batch 5.
