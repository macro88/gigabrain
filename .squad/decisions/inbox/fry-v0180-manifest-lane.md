---
recorded_at: 2026-05-04T07:22:12.881+08:00
author: Fry
change: release-v0.18.0
topic: manifest-and-doc-truth
---

# Decision

The `v0.18.0` release-bound commit should move the Cargo manifest surface to `0.18.0` and, in the same pass, repair every release-facing link or status line that still points at moved docs or an older upcoming tag.

# Why

- `release.yml` hard-fails when `Cargo.toml` does not match the pushed tag, so the branch is not truthfully releasable until the manifest and lockfile both carry `0.18.0`.
- Public install and upgrade guidance still participates in the release lane: a tag can succeed while release notes, README/download instructions, or upgrade docs still point at missing files like the old root `MIGRATION.md`.
- Keeping the version bump and the doc/link repair in one coherent release-lane commit prevents a half-prepared state where tagging would pass CI but ship broken release references.

# Consequence

- Future release prep should audit workflow release-note links, README/install docs, and web upgrade docs alongside the version bump.
- The branch can now truthfully stay in “preparing `v0.18.0` / latest public tag still older” mode until the actual tag and GitHub Release are cut.
