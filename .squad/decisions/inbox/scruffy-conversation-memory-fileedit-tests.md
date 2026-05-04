2026-05-04T07:22:12.881+08:00

- Decision: Treat `extracted/_history/**` as a reserved sidecar path in file-edit coverage proofs and verify repeated manual edits preserve one linear supersede chain (`old predecessor -> new archive -> head`).
- Why: The risky regression is silent history forking or accidental sidecar re-ingest, not happy-path head creation.
- Test impact: Keep one focused integration file that covers chain relinking, whitespace no-op, type/path gating, and sidecar behavior under the Windows coverage lane.
