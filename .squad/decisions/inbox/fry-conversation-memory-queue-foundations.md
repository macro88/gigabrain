# Fry — conversation memory queue foundations

- **Timestamp:** 2026-05-04T07:22:12.881+08:00
- **Decision:** For `memory.location = dedicated-collection`, auto-create a sibling collection named `<write-target>-memory` rooted at `<write-target-root>-quaid-memory` on first use.
- **Why:** This keeps conversation/extracted paths isolated from the main vault without inventing another config key in this slice, and avoids nesting the dedicated collection under the live vault root.
- **Implication:** Future MCP/CLI surfaces should treat that derived collection contract as the current truthful default unless a later OpenSpec explicitly introduces user-configurable naming or root overrides.
