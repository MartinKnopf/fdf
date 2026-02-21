Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: Layering changes, new invariants, enforcement changes
Source of Truth: src/, docs/10-architecture-overview.md, docs/30-quality-gates.md

# Boundaries and Invariants

## Layer Boundaries
- Allowed: `main` -> `app`, `ui`, `input`, `git`
- Allowed: `app` -> `git`, `diff`, `tree`, `model`, `input`
- Allowed: `ui` -> `app`, `model`
- Allowed: `tree` -> `model`
- Allowed: `diff` -> `model`
- Allowed: `git` -> `model`
- Forbidden: `model` importing `ui`, `app`, or `git`
- Forbidden: `ui` invoking git subprocesses or filesystem scanning

## Invariants
1. Invariant: The tool does not mutate git state.
   - Rationale: Product contract is read-only diff inspection.
   - Enforcement: PR review checklist + grep for mutating git commands in `src/git.rs`.
   - Owner: ac1ifci
   - Severity: critical
2. Invariant: All keybindings map through `src/input.rs` into `Action` before state mutation.
   - Rationale: Keeps interaction behavior centralized and testable.
   - Enforcement: Code review + compile check (`cargo check`).
   - Owner: ac1ifci
   - Severity: high
3. Invariant: Vertical scrolling is bounded by content length and viewport.
   - Rationale: Prevents rendering bugs, panics, and invalid row addressing.
   - Enforcement: `App::clamp_scroll` path review in `src/app.rs`.
   - Owner: ac1ifci
   - Severity: high
4. Invariant: UI rendering remains pure (state read-only, no side-effectful I/O).
   - Rationale: Prevents frame-time stalls and hidden behavior.
   - Enforcement: Code review on `src/ui.rs`; side-effects must stay in `src/app.rs` or adapters.
   - Owner: ac1ifci
   - Severity: medium

## Violation Response
1. Stop merge.
2. Open debt/incident item in `docs/70-debt-register.md` or `docs/71-known-issues.md`.
3. Escalate to ac1ifci.
