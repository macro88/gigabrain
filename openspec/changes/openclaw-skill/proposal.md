# Openclaw skill bootstrap

## Why

Agents still have to hand-roll the same bootstrap sequence — initialize a database, attach a collection, install the background serve process, and wire MCP clients. That repeated setup work belongs in a dedicated agent-facing onboarding change instead of ad hoc instructions.

## What Changes

- Define an agent-facing bootstrap skill that orchestrates `quaid init`, `quaid collection add`, daemon installation, and MCP wiring in one guided flow.
- Capture the operating constraints for safe default setup, including local-first paths, collection naming, and daemon ownership checks.
- Provide truthful operator handoff output so the skill can explain what was created and what still requires manual review.

## Impact

- `.copilot/skills/` and `skills/` gain a dedicated bootstrap skill once the follow-up change lands.
- Docs gain an agent/operator setup path that points to the daemon-install change for service management.
- No current runtime behavior changes in this stub; it only reserves the follow-up spec lane.
