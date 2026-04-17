### Decision: Lower assertion object-length guard from 6 to 3 characters

**By:** Fry
**Date:** 2026-04-17
**Issue:** #38
**Branch:** `squad/38-assertion-extraction-tightening`

**What:** Reduced the minimum object-length filter in `has_minimum_object_length()` from 6 to 3 characters.

**Why:** The original 6-char guard was designed to suppress prose noise before extraction was scoped to `## Assertions` sections. After the section-scoping fix (commit `193d587`), extraction already requires explicit user intent via the `## Assertions` heading. The 6-char threshold became a regression — dropping valid short entities like Acme (4), Meta (4), CEO (3), CTO (3) that users intentionally wrote in structured sections.

**Trade-off:** 3 chars still blocks regex noise ("it", "a", "an") while accepting all real entity names and acronyms. The `## Assertions` section scope is the primary trust boundary; the length guard is a secondary defense against regex artifacts.

**Tests added:** 3 new regression tests covering short valid objects, very-short noise rejection, and the 3-char boundary case. Full suite (16 assertion tests + integration) passes.
