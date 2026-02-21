Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Process change, architecture change, quality gate change
Source of Truth: docs/00-index.md, docs/11-boundaries-and-invariants.md, docs/30-quality-gates.md

# Agent Operating Guide

## Mission
Deliver safe, correct, and maintainable improvements to `fdf`, a read-only terminal git diff viewer, while preserving keyboard-driven UX and architecture boundaries.

## Non-Goals
- Expanding scope to git mutation workflows (commit/stage/reset from UI) without an explicit approved spec.
- Architecture changes without updating boundary and dependency docs.
- Skipping required validation gates for code changes.

## Read Order
1. docs/00-index.md
2. docs/01-product-context.md
3. docs/10-architecture-overview.md
4. docs/11-boundaries-and-invariants.md
5. docs/20-agent-workflow.md
6. docs/30-quality-gates.md
7. docs/50-security-constraints.md

## Hard Constraints
- Keep the application read-only with respect to git state unless a human-approved spec explicitly changes this behavior.
- Preserve module boundaries documented in `docs/11-boundaries-and-invariants.md`.
- Run required quality gates from `docs/30-quality-gates.md` before handoff.
- Update docs and changelog when behavior, policy, or workflow changes.

## Allowed Validation Commands
- `cargo fmt --check`
- `cargo check`
- `cargo test`

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
