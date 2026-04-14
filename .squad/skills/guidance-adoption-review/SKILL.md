---
name: "guidance-adoption-review"
description: "Review external coding guidance before turning it into standing repo policy."
domain: "review"
confidence: "high"
source: "earned"
---

## Context
Use this skill when a team wants to adopt an external handbook, style guide, or agent skill as default guidance. The goal is not to restate the source, but to decide whether it fits the repo's architecture, maturity, and quality bar without introducing brittle or noisy rules.

## Patterns
### Read repo context first
- Read project decisions, current architecture, and crate shape before judging the guidance.
- Distinguish binary-vs-library advice, early-stage-vs-mature-project advice, and default practice vs specialized technique.

### Classify guidance into three buckets
- **Strong guidance:** broad defaults that improve code review quality with low downside.
- **Optional guidance:** useful techniques that depend on context, scale, or measured need.
- **Reject or downscope:** absolute rules that would create cargo-culting, noise, or repo-policy mismatch.

### Review for cargo-cult risk
- Be skeptical of “always/never” wording.
- Downgrade rules that force premature abstraction, excessive linting, or over-specified testing style.
- Prefer review heuristics over blanket mandates when trade-offs are situational.

## Examples
- A Rust handbook may be adopted strongly for ownership discipline, error handling, and lint hygiene, while keeping type-state, snapshot testing, and pedantic lint groups optional.
- Library-specific rules such as `#![deny(missing_docs)]` should not become repo-wide policy in a primarily binary crate without deliberate buy-in.

## Anti-Patterns
- Adopting an external guide verbatim as team law.
- Treating specialized patterns as default expectations.
- Enforcing exact tool commands or lint groups without checking current CI and repo maturity.
- Ignoring architecture context when deciding whether guidance applies.
