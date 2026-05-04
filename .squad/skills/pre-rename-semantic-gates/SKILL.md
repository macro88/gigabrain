# Pre-Rename Semantic Gates

Use this when a write path installs bytes on disk before it commits the database or other semantic metadata.

## Rule

If a later semantic check can still fail after the rename, the feature does **not** have an honest “rejected write” contract. The user-visible state has already changed.

## What to do

1. Identify every validation that can return a typed refusal (`conflict`, head-check failure, kind mismatch, policy gate).
2. Run those validations **before** the rename/install step whenever possible.
3. Keep the authoritative transaction-time validation too when concurrency can still invalidate the preflight result; preflight is for honest refusal, not for replacing the final race guard.
4. If post-rename validation is unavoidable, implement a real rollback of the installed bytes before returning the error.
5. Test the blocked-state outcome on the real write-through seam:
   - target file content unchanged
   - no new source/raw-import owner becomes active
   - caller receives the intended typed error, not a generic recovery wrapper

## Why

DB-only assertions are not enough on rename-before-commit paths. A test can show “no row was inserted” while the vault/source-of-truth file has already been replaced, which means the task closure and error contract are dishonest.

## Quaid fit

- `src/commands/put.rs` write-through paths
- any future extracted-fact correction path that writes markdown first and reconciles chain metadata second
