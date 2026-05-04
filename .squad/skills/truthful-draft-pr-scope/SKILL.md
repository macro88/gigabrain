---
name: "truthful-draft-pr-scope"
description: "How to open a narrow, honest draft PR when the branch context is wider than the landed slice"
domain: "devrel, pull-requests, scope-truth"
confidence: "high"
source: "earned"
---

# Skill: Truthful draft PR scope

## Context

Use this when a branch needs an early draft PR, but the branch ancestry or compare view is broader than the implementation slice that is actually finished and pushed. Typical cases: stacked work on top of a docs/spec branch, inherited planning artifacts, or a large change where only the first foundation slice is landed.

## Pattern

### 1. Verify the pushed branch state first

- Confirm the branch head is pushed to origin.
- Check whether a PR already exists for the branch.
- Identify the exact pushed commits that belong to the current implementation slice.

Do not open the draft PR off local-only work and do not describe scope you have not pushed yet.

### 2. Derive the claim from landed artifacts, not proposal ambition

Read the actual changed code/tests plus the current proposal/tasks. Separate:

- what is already landed and validated
- what the larger change still intends to add later

If the proposal is broader than the landed slice, the PR body must follow the landed slice.

### 3. Name the compare noise when ancestry is wider

If the compare view includes inherited roadmap/spec/docs churn from earlier branch ancestry, say so directly in the PR notes. This prevents reviewers from assuming every file in the compare is part of the current shipped claim.

### 4. Add explicit non-claims

List the unfinished capabilities that this draft does **not** land yet. Use the concrete user-facing names reviewers would otherwise assume, such as MCP tool names, worker behavior, retrieval flags, or release readiness.

### 5. Keep the PR in draft until the non-claims become true

Only promote the PR out of draft after the wider implementation is actually landed, pushed, and the body can be rewritten without caveats.

## Anti-patterns

- Opening the draft PR before the branch is pushed
- Letting the PR body repeat the full proposal scope when only the first slice is landed
- Assuming reviewers will infer scope correctly from commit history alone
- Claiming release readiness from green local checks while major tasks remain open
