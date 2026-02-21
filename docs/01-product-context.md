Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Product strategy or priority changes
Source of Truth: README.md, tasks/TASKING.md, src/

# Product Context

## Users and Personas
- Individual developer: needs quick, keyboard-first visibility into what changed before commit or code review.
- Reviewer preparing a local review pass: needs full-file context, not only hunk snippets.

## Critical Journeys
- Journey: Review all changes in working tree
  - Trigger: user runs `fdf` inside a git repository.
  - Expected Outcome: user can cycle changed files and inspect side-by-side content quickly.
  - Failure Cost: incorrect commits or delayed reviews due to poor visibility.
- Journey: Jump between meaningful edits
  - Trigger: user navigates with `n` / `N` and paging keys.
  - Expected Outcome: user lands on the first line of each change block with reliable wrap behavior.
  - Failure Cost: navigation friction and missed diffs.

## Priorities
1. Correctness of diff content and file status indicators.
2. Fast keyboard-driven navigation in large changed files.
3. Stable terminal behavior and clean recovery on exit/error.

## Non-Negotiable Behavior
- The tool must remain read-only for git state unless explicitly approved and specified.
- The viewer must render full-file context side-by-side (`HEAD` left, worktree right).

## Not Optimized For
- Full IDE/editor functionality.
- Collaborative or remote review workflows.
- GUI/Desktop visualization.
