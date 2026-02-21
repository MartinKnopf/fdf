Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Test architecture or policy changes
Source of Truth: src/, tests/, Cargo.toml

# Test Strategy

## Testing Pyramid
- Unit: pure transformations (diff alignment, tree shaping, status parsing helpers).
- Integration: module boundary behavior (`git` adapter + app state transitions).
- End-to-end: critical keyboard journeys in a real terminal session.

## Coverage Priorities
- Highest: correctness of changed-file discovery and side-by-side row alignment.
- Medium: navigation behavior (`n`/`N`, paging, `gg`/`G`) and scroll bounds.
- Lower: static labels and cosmetic styling details.

## Flaky Test Policy
- Quarantine rule: mark flaky tests and exclude from blocking gate only with tracked debt item.
- Fix SLA: triage within 2 business days; remediation or rollback within 7 days.
