Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Incident, topology change, mitigation changes
Source of Truth: docs/40-observability-debugging.md, docs/30-quality-gates.md

# Runbooks

## Incident Index
- Startup fails outside repository -> Runbook: Not In Git Repository
- Render/input crash -> Runbook: Runtime Panic or Terminal Failure
- Incorrect file listing/status -> Runbook: Git Status Parsing Issue

## Runbook: Not In Git Repository
### Detection
- Alerts/signals:
  - Startup prints `not inside a git repository`.

### Triage
1. Confirm current working directory.
2. Run `git rev-parse --is-inside-work-tree`.

### Mitigation
1. Switch to a valid repository directory.
2. Relaunch `fdf`.

### Rollback
1. N/A (no persistent state mutation).
2. N/A.

### Verification
- App launches and renders file tree without startup error.

### Escalation
- Escalate when:
  - Repository is valid but startup still fails.
- Contact:
  - ac1ifci

## Runbook: Runtime Panic or Terminal Failure
### Detection
- Alerts/signals:
  - Panic output, stack trace, or terminal not restored after crash.

### Triage
1. Re-run with `RUST_BACKTRACE=1`.
2. Capture key sequence and file state that triggered failure.

### Mitigation
1. Restore terminal with `reset` if needed.
2. Avoid triggering key sequence until patch is applied.

### Rollback
1. Revert to last known good binary/build.
2. Record issue in `docs/71-known-issues.md`.

### Verification
- Terminal state restores correctly on exit.
- Reproduction sequence no longer crashes.

### Escalation
- Escalate when:
  - Panic is reproducible and not trivially isolated.
- Contact:
  - ac1ifci

## Runbook: Git Status Parsing Issue
### Detection
- Alerts/signals:
  - Missing changed files, wrong status indicators, or directory entries shown as files.

### Triage
1. Compare app output with `git status --porcelain=v2 --untracked-files=all -z`.
2. Capture problematic repository state (rename, untracked dir, delete, etc.).

### Mitigation
1. Add parsing fix in `src/git.rs`.
2. Add regression test or documented repro case.

### Rollback
1. Roll back parsing change if regression is broad.
2. Document temporary workaround in known issues.

### Verification
- App file tree matches git status output for affected case.

### Escalation
- Escalate when:
  - Parsing behavior disagrees with documented git porcelain format and fix is unclear.
- Contact:
  - ac1ifci
