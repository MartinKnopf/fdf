Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Task management process changes
Source of Truth: tasks/TASKING.md

# Task Lifecycle

## States
- proposed
- in_progress
- blocked
- done
- archived

## Transitions
- proposed -> in_progress: scope + owner assigned.
- in_progress -> blocked: blocking dependency documented with next action.
- in_progress -> done: acceptance criteria met and evidence linked.
- done -> archived: released and documented in changelog.

## Required Evidence
- Plan/spec link
- PR link
- Validation results (`cargo fmt --check`, `cargo check`, and relevant tests)
