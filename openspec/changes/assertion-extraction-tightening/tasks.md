# Assertion Extraction Tightening — Implementation Checklist

**Scope:** Scope `extract_from_content` to `## Assertions` sections and frontmatter fields
only; add minimum object-length guard; add semantic similarity pre-filter for cross-page
comparison; document the structured assertion format.
Closes: #38, #55

---

## Phase A — structured extraction in `src/core/assertions.rs`

- [ ] A.1 Add `extract_assertions_section(content: &str) -> &str` helper: scan `content`
  for a line matching `^## [Aa]ssertions` and return the substring from that heading to the
  next `^##` heading (or end of string). Return an empty string if no such section exists.

- [ ] A.2 Add `extract_from_frontmatter(frontmatter: &HashMap<String, String>) -> Vec<ExtractedAssertion>`
  helper: for each key in `["is_a", "works_at", "founded"]`, if the key is present and the
  value is non-empty, construct a Triple with the page's title/slug as subject (passed in),
  the key as predicate, and the value as object. Insert these with `evidence_text = "frontmatter"`.

- [ ] A.3 Modify `extract_from_content(content: &str) -> Vec<ExtractedAssertion>` to call
  `extract_assertions_section(content)` and run regex patterns against that scoped text only.
  Do not change the regex patterns themselves. If the section is empty, return an empty vec.

- [ ] A.4 Update `extract_assertions(page: &Page, conn: &Connection)` to call both
  `extract_from_frontmatter` and `extract_from_content`. The frontmatter map must be parsed
  from `page`'s frontmatter JSON field (already available). Merge results before inserting.

- [ ] A.5 Add minimum object-length guard in `collect_pattern_matches`: after constructing a
  `Triple`, discard it if `triple.object.trim().len() < 6`. Apply before `seen.insert`.

---

## Phase B — documentation

- [ ] B.1 Add a "Structured Assertions" section to `docs/spec.md` (or create
  `docs/assertions.md` if the spec is too large): document the `## Assertions` heading
  convention, the supported frontmatter fields (`is_a`, `works_at`, `founded`), and explain
  that general body text is not scanned.

- [ ] B.2 Update `gbrain check --help` text (in `src/commands/check.rs` or equivalent) to
  mention that assertions are extracted from structured zones only.

---

## Phase C — tests

- [ ] C.1 Add unit test: page with `## Assertions` section — verify correct triples extracted.
- [ ] C.2 Add unit test: page with no `## Assertions` section — verify zero triples extracted
  (the current false-positive case).
- [ ] C.3 Add unit test: page with frontmatter `is_a: researcher` — verify triple inserted
  via frontmatter path, not regex.
- [ ] C.4 Add unit test: object shorter than 6 chars (e.g., `is_a: it`) — verify discarded.

## Phase E — semantic similarity gate for cross-page comparison (absorbs #55)

- [ ] E.1 In `check_assertions` (or the function that iterates page pairs to compare
  their assertion triples), add a pre-filter: before comparing `is_a` assertions between
  page A and page B, compute cosine similarity between A's and B's stored embeddings.
  Retrieve the most recent embedding vector for each page's `compiled_truth` chunk from
  `page_embeddings_vec_384`. If cosine similarity is below 0.6, skip the pair entirely.
  Do not compare assertions for unrelated pages.

- [ ] E.2 If either page has no stored embedding (e.g. newly imported, not yet embedded),
  fall back to the existing behavior (compare anyway) but log a debug trace noting the
  missing vector. Do not block the check command on re-embedding.

- [ ] E.3 Add a configurable threshold: read the similarity floor from the `config` table
  key `assertion_similarity_floor` (default `0.6`). Allow the user to lower it to `0.0`
  to restore the old all-pairs behavior or raise it for stricter filtering.

- [ ] E.4 Unit test: two pages with cosine similarity < 0.6 → zero contradiction pairs
  returned even if `is_a` text would match.

- [ ] E.5 Unit test: two pages with cosine similarity ≥ 0.6 → contradiction detection
  runs normally.

---

## Phase F — verification (extends Phase D)

- [ ] F.1 On the 350-page PARA test vault: run `gbrain check --all` after both extraction
  tightening (Phase A) and semantic gate (Phase E). Confirm false positive rate is near
  zero. A genuine contradiction (same-entity, different assertions) must still surface.

