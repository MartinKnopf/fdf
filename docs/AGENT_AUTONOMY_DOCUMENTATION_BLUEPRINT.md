# Agent Autonomy Documentation Blueprint

## 1) Purpose and Usage
This file is a single bootstrap blueprint for setting up documentation that lets agents work mostly autonomously with strong quality and safety guardrails.

Use this file to create the initial documentation in a new project. It defines:
- The target documentation tree.
- The purpose of each file.
- Exactly how to fill each file.
- Copy-ready templates.
- Operating rules so docs remain useful over time.

### Outcomes this blueprint targets
- Agents can discover context quickly.
- Agents can execute tasks with explicit boundaries and quality gates.
- Agents can self-debug with documented observability paths.
- Agents escalate only when judgment is required.
- Humans can review agent work with minimal hidden tribal knowledge.

### Adoption order
1. Create root navigation and context files (`AGENTS.md`, `README.md`, `docs/00-index.md`).
2. Create architecture and workflow files (`docs/10-*`, `docs/20-*`).
3. Create quality, observability, and security files (`docs/30-*`, `docs/40-*`, `docs/50-*`).
4. Create operational registries and templates (`docs/60-*`, `docs/70-*`, `docs/80-*`).

## 2) Documentation Contract (Shared Metadata)
Use these metadata fields at the top of every documentation file.

```md
Last Updated: <YYYY-MM-DD>
Status: <draft|active|deprecated>
Audience: <agent|human|both>
Update Trigger: <what event requires refresh>
Source of Truth: <paths/links to code, config, dashboards, tests>
```

### Standard labels
- Work status: `proposed`, `in_progress`, `blocked`, `done`, `archived`
- Risk severity: `low`, `medium`, `high`, `critical`

### Writing rules
- Write for execution, not prose quality.
- Prefer explicit checklists over paragraphs.
- Name concrete files, commands, and owners.
- Make unknowns explicit in an `Open Questions` section.
- Ban vague language: avoid "should probably", "usually", "maybe".

## 3) Target Documentation Tree
Create this structure exactly (you can expand later):

```text
AGENTS.md
README.md
docs/
  00-index.md
  01-product-context.md
  02-domain-glossary.md
  10-architecture-overview.md
  11-boundaries-and-invariants.md
  12-dependency-map.md
  20-agent-workflow.md
  21-task-lifecycle.md
  22-pr-review-playbook.md
  30-quality-gates.md
  31-test-strategy.md
  40-observability-debugging.md
  41-runbooks.md
  50-security-constraints.md
  60-decisions/
    ADR-TEMPLATE.md
  61-plans/
    PLAN-TEMPLATE.md
  62-specs/
    SPEC-TEMPLATE.md
  70-debt-register.md
  71-known-issues.md
  80-change-log.md
```

## 4) File-by-File Blueprint

### `AGENTS.md`
Purpose: Entry-point instructions for autonomous agents in this repo.

When agents read it: First, before any code or docs.

Required sections:
- Mission and non-goals.
- Navigation order (which docs to read, in order).
- Hard constraints (security, architecture, coding boundaries).
- Escalation triggers.
- Definition of done.

How to fill:
- Keep it short (1-2 pages).
- Link every rule to the canonical detailed doc.
- Document commands agents are allowed to run for validation.

Common mistakes:
- Overloading with full architecture details.
- Missing escalation criteria.

Definition of done:
- A new agent can start work with no human explanation.

### `README.md`
Purpose: Human and agent orientation to the project.

When agents read it: Early, after `AGENTS.md`.

Required sections:
- What this project does.
- Local development entrypoints.
- Link to docs index.
- High-level architecture summary.

How to fill:
- Keep setup instructions accurate and minimal.
- Include one "first successful run" command sequence.

Common mistakes:
- README diverges from actual commands.

Definition of done:
- Fresh clone to running service is reproducible.

### `docs/00-index.md`
Purpose: Canonical map of all docs and the read order.

When agents read it: Before deep work.

Required sections:
- Read order by use case (`feature`, `bug`, `incident`, `refactor`).
- Quick links to every doc.
- Owner map for decision escalation.

How to fill:
- Update links whenever docs are added/renamed.
- Include "If you are blocked, go here" section.

Common mistakes:
- Dead links and missing owners.

Definition of done:
- All core workflows are navigable in under 2 minutes.

### `docs/01-product-context.md`
Purpose: Product intent and user outcomes.

When agents read it: Before planning behavior changes.

Required sections:
- Users/personas.
- Critical user journeys.
- Business and product priorities.
- Non-negotiable behavior expectations.

How to fill:
- Prefer concrete examples with expected outcomes.
- Include explicit "do not optimize for" items.

Common mistakes:
- Technical details replace product intent.

Definition of done:
- Agent can reason about correct behavior tradeoffs.

### `docs/02-domain-glossary.md`
Purpose: Shared vocabulary and canonical definitions.

When agents read it: During design/planning and review.

Required sections:
- Terms and definitions.
- Synonyms and forbidden ambiguous terms.
- Domain object cheatsheet.

How to fill:
- One short paragraph per term.
- Link each term to source code model where possible.

Common mistakes:
- Conflicting definitions across docs.

Definition of done:
- Same term means one thing everywhere.

### `docs/10-architecture-overview.md`
Purpose: Explain the system's major components and data flow.

When agents read it: Before non-trivial implementation.

Required sections:
- System context.
- Main components and responsibilities.
- Request/data flow.
- Cross-cutting concerns.

How to fill:
- Include simple diagrams (ASCII or linked images).
- Map component names to directories/packages.

Common mistakes:
- Conceptual architecture disconnected from code.

Definition of done:
- Agent can identify where new logic belongs.

### `docs/11-boundaries-and-invariants.md`
Purpose: Hard architecture boundaries and behavioral invariants.

When agents read it: Before writing code and before PR merge.

Required sections:
- Layer boundaries (allowed dependencies).
- Invariants per domain.
- Invariant enforcement mechanism (lint/test/check).
- Escalation policy for violations.

How to fill:
- Write rules as testable statements.
- For each invariant, reference the enforcement path.

Common mistakes:
- Invariants without enforcement.

Definition of done:
- Every invariant has an owner and an enforcement mechanism.

### `docs/12-dependency-map.md`
Purpose: Internal and external dependency contract.

When agents read it: During architecture changes and risk analysis.

Required sections:
- Internal module dependency graph.
- External services/libraries and criticality.
- Upgrade policy and compatibility notes.

How to fill:
- Highlight forbidden dependency directions.
- Include minimal risk notes for each critical dependency.

Common mistakes:
- No compatibility policy for critical dependencies.

Definition of done:
- Dependency change impact can be assessed quickly.

### `docs/20-agent-workflow.md`
Purpose: End-to-end workflow for autonomous execution.

When agents read it: Every task start.

Required sections:
- Task intake.
- Planning and spec requirements.
- Implementation loop.
- Validation loop.
- PR loop and merge criteria.
- Escalation points.

How to fill:
- Use step-by-step numbered flow.
- Define required artifacts at each step.

Common mistakes:
- Missing explicit stop conditions.

Definition of done:
- Agent can execute task from intake to merge with minimal ambiguity.

### `docs/21-task-lifecycle.md`
Purpose: State machine for task progress and accountability.

When agents read it: During task tracking and status updates.

Required sections:
- Lifecycle states and transitions.
- Entry/exit criteria for each state.
- Required evidence per transition.

How to fill:
- Keep transitions strict; avoid implicit jumps.
- Define who can change state.

Common mistakes:
- State transitions with no required evidence.

Definition of done:
- Any reviewer can audit task progression from artifacts.

### `docs/22-pr-review-playbook.md`
Purpose: Review standards tuned for agent-generated PRs.

When agents read it: Before opening and before finalizing PR.

Required sections:
- PR size and scoping guidance.
- Required PR description format.
- Reviewer checklist.
- Blocking vs non-blocking feedback policy.

How to fill:
- Include exact expected headings for PR body.
- Define fast-follow process for non-blocking items.

Common mistakes:
- No clarity on what blocks merge.

Definition of done:
- Reviews are consistent and high-signal.

### `docs/30-quality-gates.md`
Purpose: Canonical pass/fail criteria for code changes.

When agents read it: Before and after implementation.

Required sections:
- Gate catalog (`lint`, `test`, `type`, `security`, `perf`).
- Trigger conditions by change type.
- Pass/fail thresholds.
- Ownership and remediation path.

How to fill:
- Tie each gate to command or CI job name.
- Include "what to do on failure" guidance.

Common mistakes:
- Gates listed without thresholds.

Definition of done:
- Any agent knows exactly what must pass before merge.

### `docs/31-test-strategy.md`
Purpose: Testing philosophy and scope boundaries.

When agents read it: While planning tests.

Required sections:
- Test pyramid strategy.
- What must be unit/integration/e2e.
- Flaky test policy.
- Test data strategy.

How to fill:
- Map test types to directories and commands.
- Include coverage priorities, not just percentages.

Common mistakes:
- Overemphasis on raw coverage metrics.

Definition of done:
- Test plans are consistent with system risk profile.

### `docs/40-observability-debugging.md`
Purpose: How agents inspect logs/metrics/traces for diagnosis.

When agents read it: During bug triage and post-change verification.

Required sections:
- Available telemetry sources.
- Service-level signals and SLOs.
- Query cookbook.
- Repro-to-fix debugging workflow.

How to fill:
- Include copy-ready query examples.
- Document correlation IDs and where they originate.

Common mistakes:
- No bridge between signal and actionable next step.

Definition of done:
- Agent can isolate likely root cause using docs alone.

### `docs/41-runbooks.md`
Purpose: Operational procedures for frequent and high-risk incidents.

When agents read it: During incidents and production troubleshooting.

Required sections:
- Incident index by symptom.
- Per-incident: detection, triage, mitigation, rollback, verification.
- Escalation and communication templates.

How to fill:
- Use strict step order.
- Include hard stop points where human approval is required.

Common mistakes:
- Runbooks without rollback or verification.

Definition of done:
- Incident handling is repeatable and auditable.

### `docs/50-security-constraints.md`
Purpose: Security/privacy boundaries and mandatory controls.

When agents read it: Before touching auth/data/external integrations.

Required sections:
- Data classification.
- Secret handling rules.
- Access control model.
- Prohibited actions.
- Security review triggers.

How to fill:
- State allowed vs forbidden actions explicitly.
- Link to control implementations (code/policy/checks).

Common mistakes:
- Security guidance that is advisory, not enforceable.

Definition of done:
- Agent knows where autonomy ends and escalation begins.

### `docs/60-decisions/ADR-TEMPLATE.md`
Purpose: Standard format for architecture decisions.

When agents read it: Before major design or dependency changes.

Required sections:
- Context.
- Decision.
- Alternatives considered.
- Consequences.
- Rollback plan.

How to fill:
- Keep concise and decision-focused.
- Link affected components and migration impact.

Common mistakes:
- Decision stated without alternatives.

Definition of done:
- Future contributors can understand why a choice was made.

### `docs/61-plans/PLAN-TEMPLATE.md`
Purpose: Standard implementation planning artifact.

When agents read it: Before coding for medium/large changes.

Required sections:
- Goal and success criteria.
- Scope.
- Design approach.
- Tasks.
- Risks and mitigations.
- Validation plan.

How to fill:
- Make tasks independently verifiable.
- Include explicit out-of-scope list.

Common mistakes:
- Plans missing measurable success criteria.

Definition of done:
- Another engineer/agent can execute plan without new decisions.

### `docs/62-specs/SPEC-TEMPLATE.md`
Purpose: Functional and interface spec template.

When agents read it: Before implementing behavior or API changes.

Required sections:
- Problem statement.
- Requirements.
- Non-requirements.
- Interface contracts.
- Failure modes.
- Acceptance criteria.

How to fill:
- Use testable "must" statements.
- Include concrete examples and edge cases.

Common mistakes:
- Requirements mixed with implementation detail.

Definition of done:
- Expected behavior is unambiguous and testable.

### `docs/70-debt-register.md`
Purpose: Structured technical debt tracking.

When agents read it: During planning, refactors, and cleanup work.

Required sections:
- Debt item list with severity and impact.
- Workaround.
- Proposed fix.
- Owner and target date.

How to fill:
- Keep entries small and actionable.
- Link debt to affected modules and incidents.

Common mistakes:
- Debt list becomes a backlog dump without prioritization.

Definition of done:
- Debt items drive concrete follow-up work.

### `docs/71-known-issues.md`
Purpose: Active known defects and temporary mitigations.

When agents read it: During bug triage and before release.

Required sections:
- Issue summary.
- User impact.
- Reproduction notes.
- Temporary mitigation.
- Permanent fix status.

How to fill:
- Keep status current.
- Link related PRs, debt items, and runbooks.

Common mistakes:
- Issues stay open after fix is released.

Definition of done:
- Stakeholders can separate expected limitations from regressions.

### `docs/80-change-log.md`
Purpose: Human-readable record of meaningful project changes.

When agents read it: Before release notes and incident retrospectives.

Required sections:
- Date.
- Change summary.
- Impacted areas.
- Rollback notes.

How to fill:
- Focus on behavior and operational impact.
- Include references to PR/ADR/spec.

Common mistakes:
- Only commit hashes with no impact narrative.

Definition of done:
- Change history is understandable without diff spelunking.

## 5) Copy-Ready Templates

### Template: `AGENTS.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Process change, architecture change, quality gate change
Source of Truth: docs/00-index.md, docs/11-boundaries-and-invariants.md, docs/30-quality-gates.md

# Agent Operating Guide

## Mission
<What agents are expected to optimize for>

## Non-Goals
- <What agents must not optimize for>

## Read Order
1. docs/00-index.md
2. docs/01-product-context.md
3. docs/10-architecture-overview.md
4. docs/11-boundaries-and-invariants.md
5. docs/20-agent-workflow.md
6. docs/30-quality-gates.md
7. docs/50-security-constraints.md

## Hard Constraints
- <Constraint 1>
- <Constraint 2>

## Escalate Immediately If
- Requirement ambiguity changes user-visible behavior.
- Any boundary or invariant would be violated.
- Security/privacy uncertainty exists.
- Required quality gates repeatedly fail with unknown cause.

## Definition Of Done
- Spec/plan updated if needed.
- All required quality gates pass.
- Changelog and known-issues updated when relevant.
- PR review checklist completed.
```

Example snippet:
```md
## Escalate Immediately If
- Request requires bypassing authorization checks.
- A fix depends on undocumented production data mutations.
```

### Template: `README.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Setup command changes, architecture changes
Source of Truth: docs/00-index.md

# <Project Name>

## What It Does
<Two-to-four sentence project summary>

## First Run
1. <install command>
2. <start command>
3. <test command>

## Documentation
- Docs index: docs/00-index.md
- Agent guide: AGENTS.md

## High-Level Architecture
<Short summary with links to docs/10-architecture-overview.md>
```

### Template: `docs/00-index.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Any doc path/owner/workflow change
Source of Truth: docs/

# Documentation Index

## Read Order By Workflow
### New Feature
1. docs/01-product-context.md
2. docs/10-architecture-overview.md
3. docs/11-boundaries-and-invariants.md
4. docs/20-agent-workflow.md
5. docs/30-quality-gates.md

### Bug Fix
1. docs/71-known-issues.md
2. docs/40-observability-debugging.md
3. docs/41-runbooks.md
4. docs/30-quality-gates.md

### Incident
1. docs/41-runbooks.md
2. docs/40-observability-debugging.md
3. docs/50-security-constraints.md

## Owner Escalation Map
- Product decisions: <owner>
- Architecture decisions: <owner>
- Security decisions: <owner>

## Doc Catalog
- <list every doc with one-line purpose>
```

### Template: `docs/01-product-context.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Product strategy or priority changes
Source of Truth: <product docs/roadmap>

# Product Context

## Users and Personas
- <persona>: <goals>

## Critical Journeys
- Journey: <name>
  - Trigger: <what starts it>
  - Expected Outcome: <success>
  - Failure Cost: <impact>

## Priorities
1. <priority 1>
2. <priority 2>

## Non-Negotiable Behavior
- <must statement>

## Not Optimized For
- <explicit non-goal>
```

### Template: `docs/02-domain-glossary.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: New domain concept or rename
Source of Truth: src/, db schema, API schema

# Domain Glossary

## Terms
### <Term>
Definition: <short definition>
In Code: <path/type>
Synonyms: <if any>
Forbidden Alternate Meaning: <if any>
```

### Template: `docs/10-architecture-overview.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Component boundary or data-flow changes
Source of Truth: src/, infra configs

# Architecture Overview

## System Context
<One paragraph>

## Components
- <component>: <responsibility>, path: <path>

## Data and Request Flow
1. <step 1>
2. <step 2>
3. <step 3>

## Cross-Cutting Concerns
- Auth: <where enforced>
- Logging/Tracing: <where emitted>
- Error handling: <approach>
```

### Template: `docs/11-boundaries-and-invariants.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Layering changes, new invariants, enforcement changes
Source of Truth: architecture tests, lint config, module graph

# Boundaries and Invariants

## Layer Boundaries
- Allowed: <layer A> -> <layer B>
- Forbidden: <layer C> -> <layer A>

## Invariants
1. Invariant: <statement>
   - Rationale: <why it exists>
   - Enforcement: <test/lint/check + command/path>
   - Owner: <team/person>
   - Severity: <low|medium|high|critical>

## Violation Response
1. Stop merge.
2. Open incident/debt item.
3. Escalate to <owner>.
```

Example snippet:
```md
1. Invariant: Domain layer never imports transport adapters.
   - Enforcement: dependency linter job `deps:check-layering`
   - Severity: high
```

### Template: `docs/12-dependency-map.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Dependency addition/removal/upgrade policy change
Source of Truth: package/cargo/go manifests, lockfiles

# Dependency Map

## Internal Dependencies
- <module>: depends on <modules>

## External Dependencies
- <dependency>
  - Purpose: <why>
  - Criticality: <low|medium|high|critical>
  - Upgrade Policy: <cadence/rules>
  - Breaking Change Plan: <how handled>

## Forbidden Dependency Patterns
- <pattern>
```

### Template: `docs/20-agent-workflow.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Workflow/process/tooling changes
Source of Truth: AGENTS.md, docs/30-quality-gates.md, docs/22-pr-review-playbook.md

# Agent Workflow

## Workflow Stages
1. Intake
   - Inputs: issue/task/spec
   - Output: scope statement
2. Context Load
   - Read docs in `docs/00-index.md` order for task type.
3. Plan
   - Create/update plan from `docs/61-plans/PLAN-TEMPLATE.md`.
4. Implement
   - Keep changes scoped to plan.
5. Validate
   - Execute required quality gates.
6. Review
   - Open PR with checklist from `docs/22-pr-review-playbook.md`.
7. Close
   - Update changelog and known issues if applicable.

## Escalation Triggers
- Product ambiguity.
- Invariant conflict.
- Security uncertainty.
- Repeated unexplained validation failure.
```

Example snippet:
```md
5. Validate
   - Required gates for backend logic change: lint + unit + integration + security scan.
```

### Template: `docs/21-task-lifecycle.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Task management process changes
Source of Truth: tracker configuration

# Task Lifecycle

## States
- proposed
- in_progress
- blocked
- done
- archived

## Transitions
- proposed -> in_progress: scope + owner assigned
- in_progress -> blocked: blocking dependency documented
- in_progress -> done: acceptance criteria met and evidence linked
- done -> archived: released and documented

## Required Evidence
- Plan/spec link
- PR link
- Validation results
```

### Template: `docs/22-pr-review-playbook.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Review policy change
Source of Truth: repository branch/merge settings

# PR Review Playbook

## PR Scope
- Preferred: single objective, small diff, one risk class.

## Required PR Body
- Goal
- Scope
- Non-scope
- Risks
- Validation performed
- Follow-ups

## Reviewer Checklist
- Behavior matches spec.
- No invariant violations.
- Required quality gates passed.
- Security constraints respected.
- Docs updated where needed.

## Blocking Rules
- Block: correctness, security, invariant, missing required validation.
- Non-block: stylistic or optional optimization (file as follow-up).
```

### Template: `docs/30-quality-gates.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: CI job or threshold changes
Source of Truth: CI config, test configs

# Quality Gates

## Gate Catalog
| Gate | Command/Job | Trigger | Pass Criteria | Owner |
|---|---|---|---|---|
| lint | <cmd/job> | any code change | no errors | <owner> |
| unit_tests | <cmd/job> | logic changes | all pass | <owner> |
| integration_tests | <cmd/job> | integration changes | all pass | <owner> |
| security_scan | <cmd/job> | dependency/auth/data changes | no high/critical findings | <owner> |
| performance_check | <cmd/job> | hot path changes | within threshold | <owner> |

## Failure Handling
1. Record failing gate and context.
2. Attempt known remediation from runbook.
3. If repeated unexplained failure, escalate.
```

Example snippet:
```md
| security_scan | `ci:security` | auth/data/dependency change | zero critical, zero high without approved exception | Security |
```

### Template: `docs/31-test-strategy.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Test architecture or policy changes
Source of Truth: test directories and runner config

# Test Strategy

## Testing Pyramid
- Unit: business logic and pure functions.
- Integration: module and adapter boundaries.
- End-to-end: critical user journeys only.

## Coverage Priorities
- Highest: money/data integrity/auth flows.
- Medium: workflow orchestration.
- Lower: thin wrappers and generated code.

## Flaky Test Policy
- Quarantine rule: <policy>
- Fix SLA: <time expectation>
```

### Template: `docs/40-observability-debugging.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Dashboard/query/schema changes
Source of Truth: logging config, metrics backend, tracing backend

# Observability and Debugging

## Telemetry Sources
- Logs: <location/query tool>
- Metrics: <location/query tool>
- Traces: <location/query tool>

## Key Signals
- Error rate: <definition + threshold>
- Latency p95: <definition + threshold>
- Saturation: <definition + threshold>

## Query Cookbook
- Symptom: <name>
  - Logs query: <query>
  - Metrics query: <query>
  - Trace query: <query>
  - Next action: <what to do based on result>

## Debugging Workflow
1. Confirm symptom and time window.
2. Correlate logs -> metrics -> traces by request/correlation ID.
3. Identify likely failing component.
4. Validate hypothesis with focused test/repro.
5. Apply fix and re-verify signals.
```

Example snippet:
```md
- Symptom: elevated 5xx on write endpoint
  - Logs query: `service=api level=error route=/v1/write`
  - Next action: inspect invariant violations in domain write path.
```

### Template: `docs/41-runbooks.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Incident, topology change, mitigation changes
Source of Truth: production ops docs, dashboards, alerts

# Runbooks

## Incident Index
- <incident name> -> section link

## Runbook: <incident name>
### Detection
- Alerts/signals:

### Triage
1. Confirm scope.
2. Confirm affected components.

### Mitigation
1. <step>
2. <step>

### Rollback
1. <step>
2. <step>

### Verification
- What metrics/logs confirm recovery:

### Escalation
- Escalate when:
- Contact:
```

### Template: `docs/50-security-constraints.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Security policy/regulatory/auth/data changes
Source of Truth: security policy, IAM config, secrets manager config

# Security Constraints

## Data Classification
- Public:
- Internal:
- Sensitive:
- Restricted:

## Mandatory Controls
- Authentication: <requirements>
- Authorization: <requirements>
- Encryption: <requirements>
- Secrets handling: <requirements>
- Audit logging: <requirements>

## Prohibited Actions
- <explicit forbidden action>

## Mandatory Escalation Cases
- Access control uncertainty.
- Handling of restricted data.
- Request to bypass security checks.
```

### Template: `docs/60-decisions/ADR-TEMPLATE.md`
```md
Last Updated: <YYYY-MM-DD>
Status: draft
Audience: both
Update Trigger: New architecture decision required
Source of Truth: related specs/plans/PRs

# ADR: <Title>

## Status
<proposed|accepted|superseded>

## Context
<what forces this decision>

## Decision
<the chosen approach>

## Alternatives Considered
1. <option A> - pros/cons
2. <option B> - pros/cons

## Consequences
- Positive:
- Negative:

## Rollback Plan
<how to revert safely>

## References
- <links>
```

### Template: `docs/61-plans/PLAN-TEMPLATE.md`
```md
Last Updated: <YYYY-MM-DD>
Status: draft
Audience: both
Update Trigger: New implementation initiative
Source of Truth: issue/spec/ADR links

# Plan: <Title>

## Goal
<one clear outcome>

## Success Criteria
- <measurable criterion>

## Scope
- In scope:
- Out of scope:

## Approach
<high-level implementation strategy>

## Task Breakdown
1. <task> - output artifact
2. <task> - output artifact

## Risks and Mitigations
- Risk: <risk>, Mitigation: <action>

## Validation Plan
- Required quality gates:
- Test scenarios:
```

### Template: `docs/62-specs/SPEC-TEMPLATE.md`
```md
Last Updated: <YYYY-MM-DD>
Status: draft
Audience: both
Update Trigger: New behavior/API requirement
Source of Truth: product context and architecture docs

# Spec: <Title>

## Problem Statement
<what problem is solved>

## Requirements
1. The system must ...
2. The system must ...

## Non-Requirements
- The system will not ...

## Interfaces and Contracts
- Input:
- Output:
- Error behavior:

## Edge Cases and Failure Modes
- <case>: expected behavior

## Acceptance Criteria
- <testable criterion>
```

### Template: `docs/70-debt-register.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: New debt item, status/severity change
Source of Truth: code comments, incidents, ADRs

# Debt Register

| ID | Item | Impact | Severity | Workaround | Proposed Fix | Owner | Target Date | Status |
|---|---|---|---|---|---|---|---|---|
| DEBT-001 | <summary> | <impact> | <low|medium|high|critical> | <workaround> | <fix> | <owner> | <date> | <status> |
```

### Template: `docs/71-known-issues.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: New issue, mitigation change, fix release
Source of Truth: issue tracker, incident docs

# Known Issues

| ID | Summary | User Impact | Repro Notes | Mitigation | Permanent Fix | Status | Owner |
|---|---|---|---|---|---|---|---|
| KI-001 | <summary> | <impact> | <notes> | <mitigation> | <plan/link> | <status> | <owner> |
```

### Template: `docs/80-change-log.md`
```md
Last Updated: <YYYY-MM-DD>
Status: active
Audience: both
Update Trigger: Meaningful behavior/reliability/security/release change
Source of Truth: PRs, release tags

# Change Log

## <YYYY-MM-DD>
- Change: <what changed>
- Impact: <who/what affected>
- References: <PR/spec/ADR>
- Rollback Notes: <if relevant>
```

## 6) Operating Rules (Keep Docs Useful Over Time)

### Freshness SLA
- `docs/10-*`, `docs/11-*`, `docs/30-*`, `docs/50-*`: review every 30 days.
- `docs/41-runbooks.md` and `docs/71-known-issues.md`: update after every incident or major bug.
- `AGENTS.md` and `docs/20-agent-workflow.md`: update when process or quality gates change.

### PR linkage policy
For any code-changing PR, link at least one of:
- A spec (`docs/62-specs/...`)
- A plan (`docs/61-plans/...`)
- An ADR (`docs/60-decisions/...`)
- A debt item (`docs/70-debt-register.md`)

### Escalation triggers
Escalate to a human owner if:
- Product behavior is ambiguous.
- A boundary/invariant may be violated.
- Security/privacy interpretation is unclear.
- Required quality gates fail repeatedly without clear cause.

### Ownership policy
- Every doc has exactly one accountable owner.
- Ownership changes are documented in `docs/80-change-log.md`.

## 7) Agent Execution Playbook (Docs-Only)
Use this sequence for every task:

1. Determine task type (`feature`, `bug`, `incident`, `refactor`).
2. Follow read order in `docs/00-index.md`.
3. Confirm constraints in `docs/11-boundaries-and-invariants.md` and `docs/50-security-constraints.md`.
4. Create/update plan or spec from templates if missing.
5. Execute scoped changes.
6. Validate using `docs/30-quality-gates.md` and `docs/31-test-strategy.md`.
7. Run observability checks per `docs/40-observability-debugging.md`.
8. Open PR using `docs/22-pr-review-playbook.md`.
9. Update `docs/80-change-log.md`, and update `docs/71-known-issues.md` if relevant.
10. If blocked by escalation trigger, stop and escalate using owner map.

## 8) Documentation Acceptance Tests
Documentation system is "autonomy-ready" only when all checks pass:

1. All required files from the target tree exist.
2. Every file includes metadata fields and non-empty `Last Updated`.
3. `docs/00-index.md` links resolve and read order is complete for all workflows.
4. Every invariant in `docs/11-boundaries-and-invariants.md` has an enforcement mechanism.
5. Every quality gate has command/job, trigger, pass criteria, and owner.
6. Every runbook includes detection, triage, mitigation, rollback, verification, escalation.
7. Security doc has explicit prohibited actions and mandatory escalation cases.
8. Plan/spec/ADR templates are usable without further structural edits.
9. Debt register and known issues use structured tabular entries.
10. Changelog entries reference PR/spec/ADR links.

## 9) Rollout Plan for a New Project

### Week 1
- Create root docs: `AGENTS.md`, `README.md`, `docs/00-index.md`.
- Create product and architecture baseline: `docs/01-*`, `docs/10-*`, `docs/11-*`.
- Create workflow baseline: `docs/20-*`.

### Week 2
- Add quality/testing docs: `docs/30-*`.
- Add observability and runbooks: `docs/40-*`.
- Add security constraints: `docs/50-security-constraints.md`.

### Week 3
- Add decision/spec/plan templates: `docs/60-*`, `docs/61-*`, `docs/62-*`.
- Add debt, known issues, changelog registries: `docs/70-*`, `docs/71-*`, `docs/80-*`.
- Run documentation acceptance tests and close gaps.

### Ongoing cadence
- Monthly doc freshness review.
- Post-incident doc update within one working day.
- Quarterly audit: invariants, quality gates, and escalation owner map.

## 10) Practical Defaults
- Use numeric prefixes (`00`, `10`, `20`, etc.) for stable navigation.
- Keep `AGENTS.md` short; keep deeper details in `docs/`.
- Prefer explicit checklists over narrative text.
- Treat stale docs as production risk.
- If documentation and code conflict, fix docs or code in the same PR.
