Last Updated: 2026-02-20
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
   - Execute required quality gates from `docs/30-quality-gates.md`.
6. Review
   - Open PR with checklist from `docs/22-pr-review-playbook.md`.
7. Close
   - Update changelog and known issues if applicable.

## Escalation Triggers
- Product ambiguity.
- Invariant conflict.
- Security uncertainty.
- Repeated unexplained validation failure.
