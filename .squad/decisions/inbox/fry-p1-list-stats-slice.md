# Decision: T08 list.rs + T09 stats.rs implementation choices

**Date:** 2026-04-14
**Author:** Fry
**Status:** proposed
**Scope:** T08, T09

## Decisions

### list.rs — dynamic query construction

`list_pages` builds the SQL string with optional `AND wing = ?` / `AND type = ?` clauses
using `Box<dyn ToSql>` parameter bags. This avoids four separate prepared statements
for the four filter combinations while staying injection-safe (all values are bound
parameters, never interpolated). Default limit 50 is enforced by clap's `default_value`.

### stats.rs — DB file size via pragma_database_list

Rather than threading the file path through from `main.rs`, `gather_stats` reads the
path from `SELECT file FROM pragma_database_list WHERE name = 'main'`. This keeps the
function signature clean (only `&Connection`) and works for any open database. Falls
back to 0 bytes if `fs::metadata` fails (e.g., in-memory DB).

### Test coverage

- **list.rs:** 7 tests — no filters, wing filter, type filter, combined filters,
  limit cap, empty DB, ordering by updated_at DESC.
- **stats.rs:** 4 tests — empty DB zeros, page+type counts, FTS trigger row count,
  nonzero file size.

### No main.rs changes needed

The clap dispatch in `main.rs` was already wired to call `commands::list::run` and
`commands::stats::run` with the correct signatures. No wiring changes required.
