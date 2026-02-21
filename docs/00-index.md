Last Updated: 2026-02-20
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

### Refactor
1. docs/10-architecture-overview.md
2. docs/11-boundaries-and-invariants.md
3. docs/12-dependency-map.md
4. docs/30-quality-gates.md
5. docs/22-pr-review-playbook.md

## If You Are Blocked, Go Here
- Product ambiguity: docs/01-product-context.md, then escalate via owner map below.
- Architecture conflict: docs/11-boundaries-and-invariants.md, then owner escalation.
- Validation failure: docs/30-quality-gates.md and docs/41-runbooks.md.
- Security uncertainty: docs/50-security-constraints.md; treat as mandatory escalation.

## Owner Escalation Map
- Product decisions: ac1ifci
- Architecture decisions: ac1ifci
- Security decisions: ac1ifci

## Doc Catalog
- docs/01-product-context.md: Product goals, personas, and behavior priorities.
- docs/02-domain-glossary.md: Canonical project vocabulary and model terms.
- docs/10-architecture-overview.md: High-level architecture and data flow.
- docs/11-boundaries-and-invariants.md: Layer rules and behavioral invariants.
- docs/12-dependency-map.md: Internal/external dependency contracts.
- docs/20-agent-workflow.md: Agent execution loop and escalation triggers.
- docs/21-task-lifecycle.md: Task state model and required delivery evidence.
- docs/22-pr-review-playbook.md: PR structure and reviewer blocking criteria.
- docs/30-quality-gates.md: Required validation commands and pass conditions.
- docs/31-test-strategy.md: Test pyramid and coverage priorities.
- docs/40-observability-debugging.md: Debug signals and troubleshooting workflow.
- docs/41-runbooks.md: Incident-specific detection, mitigation, rollback steps.
- docs/50-security-constraints.md: Security guardrails and prohibited actions.
- docs/60-decisions/ADR-TEMPLATE.md: Architecture decision record template.
- docs/61-plans/PLAN-TEMPLATE.md: Implementation plan template.
- docs/62-specs/SPEC-TEMPLATE.md: Behavior/API specification template.
- docs/70-debt-register.md: Structured technical debt backlog.
- docs/71-known-issues.md: Current user-impacting issues and mitigations.
- docs/80-change-log.md: Behavior and operational change history.
