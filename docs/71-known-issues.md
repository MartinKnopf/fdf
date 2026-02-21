Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: New issue, mitigation change, fix release
Source of Truth: issue tracker, incident docs

# Known Issues

| ID | Summary | User Impact | Repro Notes | Mitigation | Permanent Fix | Status | Owner |
|---|---|---|---|---|---|---|---|
| KI-001 | Interactive terminal mode may fail in restricted/non-TTY environments | App may exit with terminal/permission errors instead of opening UI | Run binary from non-interactive execution context | Run `fdf` directly in an interactive terminal session inside a git repo | Detect unsupported TTY early and display explicit guidance | in_progress | ac1ifci |
