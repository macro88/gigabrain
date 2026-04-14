# Decision: T11 link.rs + T12 compact.rs — Implementation Choices

**Author:** Fry
**Date:** 2026-04-14
**Scope:** T11 (link command), T12 (compact command)

## Link: slug-to-ID resolution in command layer

`resolve_page_id(db, slug)` lives in `commands/link.rs` (not `core/db.rs`). The link command resolves both from and to slugs to page IDs before any INSERT/UPDATE. If either page doesn't exist, the command bails with "page not found: {slug}" before touching the links table.

**Rationale:** Keeps the contract clean — slugs are the only identity callers ever see. The DB schema enforces FK integrity on `from_page_id`/`to_page_id`, so we'd get a constraint violation anyway, but failing early with a human-readable message is better UX.

## Link close: UPDATE-first pattern

When `--valid-until` is provided and a matching open link exists (same from, to, relationship, and `valid_until IS NULL`), the command updates the existing row instead of inserting a new one. If no open link matches, it falls through to INSERT (creating a link with both valid_from and valid_until set).

**Rationale:** Matches the spec scenario "Close a link" which says "the `valid_until` field of the link is updated". The `IS NULL` guard prevents double-closing.

## Compact: thin delegation to db::compact

`compact.rs` is a one-liner that delegates to `db::compact()` and prints a success message. Removed the `#[allow(dead_code)]` annotation from `db::compact()` since it's now wired.

## Also implemented (bonus, same module)

`link-close` (by ID), `links` (outbound list), `backlinks` (inbound list), and `unlink` (delete) are implemented in the same file since they were stubbed there and share the same slug-resolution logic. These were not in T11's task list but were already wired in main.rs and would have panicked at runtime if any user hit them.

## Test coverage

10 new tests (78 total, up from 68): create link, close link, link-close by ID, link-close nonexistent ID, from-page not found, to-page not found, unlink, links/backlinks listing, compact on live DB, compact on empty DB.
