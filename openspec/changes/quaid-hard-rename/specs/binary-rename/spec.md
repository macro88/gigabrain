# Binary & Crate Rename Spec

**Change:** `gbrain` → `quaid` everywhere in Cargo and the CLI harness.

## Invariants

1. `Cargo.toml` `[package] name` must be `"quaid"`.
2. `[[bin]] name` must be `"quaid"`.
3. `clap` root command `name` attribute must be `"quaid"`.
4. `repository` field must point to `https://github.com/quaid-app/quaid`.
5. The compiled release binary on all platforms must be named `quaid` (or `quaid.exe` on Windows).
6. No `gbrain` binary alias, symlink, or wrapper is created.

## Validation

- `cargo build --release` → binary at `target/release/quaid` (not `target/release/gbrain`).
- `./target/release/quaid --version` exits 0.
- `rg 'name = "gbrain"' Cargo.toml` → zero matches.
