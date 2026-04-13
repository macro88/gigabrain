# Ceremonies

> Team meetings that happen before or after work. Each squad configures their own.

## Proposal Kickoff

| Field | Value |
|-------|-------|
| **Trigger** | auto |
| **When** | before |
| **Condition** | any meaningful code, docs, docs-site, benchmark, or test change |
| **Facilitator** | Leela |
| **Participants** | lead + relevant owners |
| **Time budget** | focused |
| **Enabled** | ✅ yes |

**Agenda:**
1. Confirm the user goal and affected surfaces
2. Write or refine the OpenSpec change proposal
3. Identify reviewers, tests, docs, and benchmark implications
4. Decide what can run in parallel immediately

---

## Design Review

| Field | Value |
|-------|-------|
| **Trigger** | auto |
| **When** | before |
| **Condition** | multi-agent task involving 2+ agents modifying shared systems |
| **Facilitator** | Leela |
| **Participants** | all-relevant |
| **Time budget** | focused |
| **Enabled** | ✅ yes |

**Agenda:**
1. Review the task and requirements
2. Agree on interfaces and contracts between components
3. Identify risks and edge cases
4. Assign action items

---

## Retrospective

| Field | Value |
|-------|-------|
| **Trigger** | auto |
| **When** | after |
| **Condition** | build failure, test failure, or reviewer rejection |
| **Facilitator** | Leela |
| **Participants** | all-involved |
| **Time budget** | focused |
| **Enabled** | ✅ yes |

**Agenda:**
1. What happened? (facts only)
2. Root cause analysis
3. What should change?
4. Action items for next iteration
