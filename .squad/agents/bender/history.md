# bender history

- [2026-04-29T07-04-07Z] History summarized and archived
- [2026-04-29T06-55-46Z] Investigated BEIR Regression Gate timeout on PR #114 (release/v0.11.0). Root cause: both beir_nq and beir_fiqa ran the full 10k-doc import+embed+query pipeline before checking whether a baseline existed — both baselines are null/pending in beir.json, so CI burned the entire 60-minute budget every time with no assertion. Fixed by moving the null-baseline early-exit guard to the top of each test function. Committed as 52b46e9, pushed to release/v0.11.0. This is a test-logic fix, not a branch search/embedding regression.
