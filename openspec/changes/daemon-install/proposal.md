# Daemon install

## Why

`quaid serve` is now the long-lived runtime required for live vault sync and MCP access, but operators still have to wire it into launchd or systemd manually. That keeps deployment truth scattered across shell snippets instead of a first-class Quaid workflow.

## What Changes

- Add a `quaid daemon` command group for `install`, `uninstall`, `start`, `stop`, and `status`.
- Generate launchd and systemd unit definitions that wrap `quaid serve` with the existing local-first and zero-network constraints.
- Expand `quaid status` so operators can see whether the daemon is installed, running, and attached to the expected database.

## Impact

- `src/commands/` gains daemon lifecycle commands and platform-specific unit rendering helpers.
- Docs gain an operator guide for background service installation and status checks.
- No vault-sync behavior changes in this stub; it only defines the follow-up change boundary.
