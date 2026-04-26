# Schema Migration Spec

**Change:** `brain_config` table renamed to `quaid_config`. `SCHEMA_VERSION` bumped. Default DB directory and filename updated. This is a **breaking schema change** per the breaking-schema-changes skill.

## DDL change

```sql
-- Before
CREATE TABLE brain_config (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) STRICT;

-- After
CREATE TABLE quaid_config (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) STRICT;
```

All references to `brain_config` in `src/core/db.rs` and any other Rust source files must be updated to `quaid_config`.

## SCHEMA_VERSION

The `SCHEMA_VERSION` constant in `src/core/db.rs` must be incremented by 1 from its current value.

## Default paths

| Before | After |
|--------|-------|
| `~/.gbrain/` | `~/.quaid/` |
| `~/.gbrain/brain.db` | `~/.quaid/memory.db` |

The `dirs::home_dir()` lookup in `src/core/db.rs` (or wherever the default path is constructed) must reflect the new directory and filename.

## Migration policy

**No automatic migration.** Existing databases created with `gbrain` are incompatible with `quaid`. On detecting a `SCHEMA_VERSION` mismatch (including detecting old schema with `brain_config` but no `quaid_config`), the binary must return a clear, non-zero error:

```
Error: database schema version mismatch.
  Found version N, expected M.
  To migrate: export your data with the old gbrain binary, then run:
    quaid init ~/.quaid/memory.db
    quaid import <export-directory>
```

No fallback, no silent upgrade, no `brain_config` alias reading.

## Invariants

1. `src/schema.sql` must contain `quaid_config` and must NOT contain `brain_config`.
2. `SCHEMA_VERSION` in `src/core/db.rs` must be greater than the version in the last release tag.
3. The DDL change, `SCHEMA_VERSION` bump, and all test fixture updates must land in a single atomic commit.
4. `cargo test` must pass in that commit with no schema-related failures.
5. The default DB path used when no `--db` / `QUAID_DB` is provided must be `~/.quaid/memory.db`.

## Validation

- `rg "brain_config" src/` → zero matches.
- `rg "\.gbrain" src/` → zero matches.
- `rg "brain\.db" src/` → zero matches.
- `cargo test` → green.
