Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Dependency addition/removal/upgrade policy change
Source of Truth: Cargo.toml, Cargo.lock, src/

# Dependency Map

## Internal Dependencies
- `src/main.rs`: depends on `app`, `git`, `input`, `ui`.
- `src/app.rs`: depends on `diff`, `git`, `input`, `model`, `tree`.
- `src/ui.rs`: depends on `app`, `model`.
- `src/tree.rs`: depends on `model`.
- `src/diff.rs`: depends on `model`.
- `src/git.rs`: depends on `model`.
- `src/input.rs`: standalone action mapping.
- `src/model.rs`: foundational types; should not depend on other internal modules.

## External Dependencies
- `ratatui`
  - Purpose: terminal layout/widgets rendering.
  - Criticality: high
  - Upgrade Policy: review minor updates quarterly; patch updates opportunistically.
  - Breaking Change Plan: isolate UI API changes in `src/ui.rs` and `src/main.rs`.
- `crossterm`
  - Purpose: terminal mode control and keyboard events.
  - Criticality: high
  - Upgrade Policy: review minor updates quarterly.
  - Breaking Change Plan: constrain breakage to `src/main.rs` and `src/input.rs`.
- `similar`
  - Purpose: line-diff operations used to generate aligned rows.
  - Criticality: medium
  - Upgrade Policy: update when needed for bug fixes/perf.
  - Breaking Change Plan: keep adaptation confined to `src/diff.rs`.
- `syntect`
  - Purpose: language-aware syntax highlighting for diff pane content.
  - Criticality: medium
  - Upgrade Policy: review minor updates quarterly; prioritize parser/theme bugfixes.
  - Breaking Change Plan: keep adaptation confined to `src/ui.rs`.
- `anyhow`
  - Purpose: ergonomic error propagation/context.
  - Criticality: medium
  - Upgrade Policy: patch/minor updates as available.
  - Breaking Change Plan: replace/adjust error paths per module if API changes.

## Forbidden Dependency Patterns
- Domain model (`src/model.rs`) depending on rendering or input crates.
- UI layer (`src/ui.rs`) reading git state or filesystem directly.
- Adding async runtime dependencies without documented need and ADR.
