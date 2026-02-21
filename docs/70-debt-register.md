Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: New debt item, status/severity change
Source of Truth: code comments, incidents, ADRs

# Debt Register

| ID | Item | Impact | Severity | Workaround | Proposed Fix | Owner | Target Date | Status |
|---|---|---|---|---|---|---|---|---|
| DEBT-001 | No automated tests for keybinding and navigation state transitions | Regressions in interaction behavior can slip through | high | Manual smoke test in git repo | Add unit tests for `App::on_action` and change-block navigation | ac1ifci | 2026-03-15 | proposed |
| DEBT-002 | Git porcelain parser lacks dedicated regression tests for rename/delete edge cases | Incorrect file tree/status under uncommon repo states | medium | Manual comparison with `git status --porcelain=v2` | Add parser tests with captured fixture outputs | ac1ifci | 2026-03-22 | proposed |
