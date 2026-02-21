Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: CI job or threshold changes
Source of Truth: Cargo.toml, rustfmt config, repository workflows

# Quality Gates

## Gate Catalog
| Gate | Command/Job | Trigger | Pass Criteria | Owner |
|---|---|---|---|---|
| formatting | `cargo fmt --check` | any Rust code/doc change touching Rust snippets | no formatting diffs | ac1ifci |
| compile | `cargo check` | any code change | build succeeds without errors | ac1ifci |
| tests | `cargo test` | logic changes or test updates | all tests pass (if test suite exists) | ac1ifci |
| smoke_manual | run `cargo run` in a git repo | keybinding/UI behavior changes | app launches, input works, exits with `q` | ac1ifci |
| security_review | PR checklist item | dependency/auth/data flow changes | no unreviewed critical/high security concerns | ac1ifci |

## Failure Handling
1. Record failing gate and context.
2. Attempt known remediation from `docs/41-runbooks.md`.
3. If repeated unexplained failure, escalate.
