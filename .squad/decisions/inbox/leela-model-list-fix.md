# Decision: `gbrain model list` subcommand wiring fix

**Date:** 2026-04-17  
**Author:** Leela  
**PR:** #60 (flexible-model-resolution revision)

## What was wrong

Fry's implementation of the `flexible-model-resolution` change wired the `model` command as a unit
variant in the `Commands` enum:

```rust
/// List known embedding model aliases
Model,
```

This made `gbrain model` work, but `gbrain model list` (the spec-required shape) returned an error.
The `--json` flag was also wired as the global `cli.json` flag, so it only worked as
`gbrain model --json`, not `gbrain model list --json`.

## What was fixed

Changed `Commands::Model` from a unit variant to a subcommand-bearing variant:

```rust
/// Manage embedding models
Model {
    #[command(subcommand)]
    command: ModelCommands,
},
```

Added a `ModelCommands` enum with a single `List { json: bool }` variant so the canonical CLI shape
`gbrain model list [--json]` now parses correctly.

Updated `EarlyCommand::Model` to carry the `json: bool` value extracted from the subcommand,
so it no longer relies on the global `--json` flag.

Updated the dispatch match arms and unreachable guard accordingly.

Replaced the single old test (`gbrain model` → `EarlyCommand::Model`) with two tests that verify
the corrected shapes: `gbrain model list` and `gbrain model list --json`.

## Validation

- `cargo check --quiet`: clean.
- `cargo test --lib --quiet`: 389/389 passed.
- `cargo test --test roundtrip_semantic --test roundtrip_raw`: passed.
- `corpus_reality` test failure is pre-existing and unrelated (link-error on the test binary, not
  caused by this change).
