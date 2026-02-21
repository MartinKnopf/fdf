Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Dashboard/query/schema changes
Source of Truth: src/main.rs, src/app.rs, src/git.rs

# Observability and Debugging

## Telemetry Sources
- Logs: stderr/stdout from local run (`cargo run`, direct binary execution).
- Metrics: none centralized today (local/manual only).
- Traces: none centralized today.

## Key Signals
- Startup failure rate: any non-zero exit at launch.
- Interactive stability: no panic during key input/render loop.
- Diff correctness signal: selected file shows expected line mapping versus `git diff` sanity check.

## Query Cookbook
- Symptom: app exits immediately with git error
  - Logs query: inspect terminal output for `not inside a git repository` or `git status failed`.
  - Metrics query: N/A.
  - Trace query: N/A.
  - Next action: confirm current directory is a git worktree; run `git rev-parse --is-inside-work-tree`.
- Symptom: panic during rendering/navigation
  - Logs query: rerun with `RUST_BACKTRACE=1` and capture stack trace.
  - Metrics query: N/A.
  - Trace query: N/A.
  - Next action: reproduce with minimal file set and inspect scroll/jump bounds in `src/app.rs`.
- Symptom: diff output appears incorrect
  - Logs query: compare selected file against `git show HEAD:<path>` and worktree file content.
  - Metrics query: N/A.
  - Trace query: N/A.
  - Next action: validate alignment behavior in `src/diff.rs` and record repro in known issues.

## Debugging Workflow
1. Confirm symptom and time window.
2. Reproduce in the smallest possible git repository state.
3. Correlate error output with failing module (`git`, `diff`, `app`, or `ui`).
4. Validate hypothesis with focused command or test.
5. Apply fix and re-verify startup/navigation behavior.
