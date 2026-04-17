# Professor — issue #38 implementation decisions

## 2026-04-17

### 1. Minimum object guard follows the task list, not the proposal's token-count wording
- Implemented the regex guard as `object.trim().chars().count() >= 6` only.
- Reason: the task list and issue triage both specify the 6-character threshold, while the proposal text's "3 tokens" clause would reject valid single-token assertions like `founder`.

### 2. Frontmatter assertions use page title as subject, with slug fallback
- `is_a`, `works_at`, and `founded` frontmatter fields now emit assertions for the page title when present.
- If the title is empty, extraction falls back to the slug so the path remains deterministic.
