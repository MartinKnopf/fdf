Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Review policy change
Source of Truth: repository branch/merge settings, docs/30-quality-gates.md

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
- Behavior matches spec/plan.
- No invariant violations.
- Required quality gates passed.
- Security constraints respected.
- Docs updated where needed.

## Blocking Rules
- Block: correctness, security, invariant, missing required validation.
- Non-block: stylistic or optional optimization (file as follow-up).
