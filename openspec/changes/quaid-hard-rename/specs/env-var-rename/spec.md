# Env Var Rename Spec

**Change:** All `GBRAIN_*` environment variables renamed to `QUAID_*`.

## Full rename table

| Old variable | New variable | Used in |
|--------------|--------------|---------|
| `GBRAIN_DB` | `QUAID_DB` | `src/main.rs`, docs |
| `GBRAIN_MODEL` | `QUAID_MODEL` | `src/main.rs`, docs |
| `GBRAIN_CHANNEL` | `QUAID_CHANNEL` | `scripts/install.sh`, docs |
| `GBRAIN_INSTALL_DIR` | `QUAID_INSTALL_DIR` | `scripts/install.sh` |
| `GBRAIN_VERSION` | `QUAID_VERSION` | `scripts/install.sh` |
| `GBRAIN_NO_PROFILE` | `QUAID_NO_PROFILE` | `scripts/install.sh` |
| `GBRAIN_RELEASE_API_URL` | `QUAID_RELEASE_API_URL` | `scripts/install.sh` |
| `GBRAIN_RELEASE_BASE_URL` | `QUAID_RELEASE_BASE_URL` | `scripts/install.sh` |

## Invariants

1. No `GBRAIN_*` variable name appears in any source file, script, workflow, or documentation in the final implementation.
2. The `clap` `env()` attributes in `src/main.rs` must reference `QUAID_DB` and `QUAID_MODEL`.
3. `scripts/install.sh` must reference only `QUAID_*` variables.
4. All docs and SKILL.md examples must use `QUAID_*` names.
5. No forwarding shim reads `GBRAIN_*` and exports as `QUAID_*`.

## Validation

- `rg "GBRAIN_" --type-add "text:*.{rs,md,sh,yml,yaml,toml}" -t text` → zero matches (excluding `.squad/` history files).
