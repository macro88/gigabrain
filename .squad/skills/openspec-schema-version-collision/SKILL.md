---
name: "openspec-schema-version-collision"
description: "How to truth-repair an OpenSpec change when its schema target collides with the live repo"
domain: "openspec, architecture, execution planning"
confidence: "high"
source: "earned"
---

## Context

Use this when an OpenSpec proposal/design/spec/tasks package describes a schema bump or DDL surface that no longer matches the live repository. Typical symptoms: the target schema version is already shipped, the artifacts still refer to an old rejection boundary, or task DDL uses a logical field name that does not match the physical column name in `src/schema.sql`.

## Patterns

### 1. Verify the live schema target before batching

Before assigning any implementation slice, read both:

- `src/core/db.rs` for `SCHEMA_VERSION`
- `src/schema.sql` for the labeled schema version and the actual column/index names

If either already reflects the version the change claims it will introduce, the artifacts are stale and must be corrected before implementation continues.

### 2. Truth-repair all change artifacts together

When the target version is wrong, correct the full OpenSpec set together:

- `proposal.md`
- `design.md`
- `tasks.md`
- every affected `specs/**/spec.md`

Do not fix only `tasks.md`. The proposal, migration plan, and acceptance scenarios all need the same version boundary or reviewers will sign off against conflicting truths.

### 3. Resolve logical-name versus physical-name DDL drift explicitly

OpenSpec often speaks in product terms (`kind`, `session`, `status`) while SQLite may use older physical column names (`type`, legacy config keys, etc.). Before routing the first schema slice, confirm every indexed column name exists exactly as written in the live schema. If not, either:

- rewrite the artifact to the real column name, or
- add an explicit rename task to the change

Never let the first implementation slice discover this by failing at DDL apply time.

### 4. Freeze checkbox progress while the spec is lying

If partial implementation has already started, stop widening the branch until the artifacts are repaired. Checked tasks against stale version numbers or nonexistent columns create false progress and make later reviewer gating unreliable.

### 5. Reframe completed tasks as landed baseline

If the first schema slice is already checked and present in the repo, rewrite those tasks so they describe the landed baseline truthfully:

- change future-tense verbs like `Add` / `Bump` to `Keep` / `Align` / `Verify`
- fix the concrete DDL spellings (`pages.type`, guarded expressions, current schema version)
- add a short truth note that remaining work starts at the next open task

This preserves checkbox history while removing the false claim that the repo still needs a fresh schema bump.

### 6. Re-route review and PR timing after repair

Once the artifacts are corrected:

- reopen batching from the repaired schema slice
- put adversarial reviewers on any shared watcher / schema / server hotspots before code resumes
- open the draft PR early enough for CI to exercise the corrected assumptions, not after the full change is already intertwined

## Anti-Patterns

- Do not keep implementing because "the code already started."
- Do not update only the schema version constant and leave the spec scenarios on the old boundary.
- Do not assume product terminology in the spec matches the physical SQLite column names.
- Do not allow task checkboxes to move forward while the artifact baseline is still wrong.
- Do not leave already-checked schema tasks phrased as future work once that slice is live in the repo.
