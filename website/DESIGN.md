# Design System: Quaid "Memory Echo" Identity

This document serves as the visual truth for all generated UI components.

## Core Identity
A dark, restrained, luxe design language. Warmth through typography and tone, not color saturation. No AI clichés — no brains, no circuits, no glowing particles.

**Tagline:** PERSISTENT MEMORY FOR AGENTS

## Typography
- **Display Font**: `"Recoleta", "Cormorant Garamond", "Iowan Old Style", Georgia, serif`
  - Weight: 400 (Regular)
  - Wordmark tracking: `0.18em`
  - Hero tracking: `0.14em`
  - Always lowercase for brand use
- **UI/Body Font**: `Inter, ui-sans-serif, system-ui, sans-serif`
- **Monospace**: `"SFMono-Regular", "JetBrains Mono", Consolas, monospace`

## Color Palette (Dark-first)

| Token | Hex | Usage |
|---|---|---|
| Ink | `#0D0D0D` | Primary background |
| Charcoal | `#161616` | Nav, sidebar, elevated surfaces |
| Stone | `#2A2A2A` | Cards, inputs, secondary surfaces |
| Bone | `#F7F7F5` | Primary text |
| Linen | `#D6D0C4` | Taglines, secondary text, emphasis |
| Bronze | `#A59A7A` | Accent, links, active states |
| Muted | `#8A8376` | Subdued text, captions |

## UI Elements
- **Borders**: 1px solid with bronze at low opacity (`rgba(165, 154, 122, 0.18)`)
- **Shadows**: Subtle dark shadows only. No light-mode drop shadows.
- **Cards**: Stone background (`#2A2A2A`), subtle border, `8px` radius
- **Buttons**: Primary = Linen fill with Ink text. Secondary = transparent with bronze border.
- **Icons**: Minimal line icons, bronze colored
- **Micro-interactivity**: Subtle hover translations, border opacity changes

## Brand Rules
- Wordmark is always **lowercase** `quaid` in Recoleta
- Do not highlight "ai" in the wordmark
- Do not add gradients, neon, or cyan
- Use large negative space
- Let the wordmark carry the brand
- Memory echo symbol is secondary texture only
