Last Updated: 2026-02-21
Status: active
Audience: both
Update Trigger: Meaningful behavior/reliability/security/release change
Source of Truth: PRs, release tags

# Change Log

## 2026-02-21
- Change: Replaced arrow-key tree navigation with `Shift+H`/`Shift+J`/`Shift+K`/`Shift+L`.
- Impact: Tree navigation now uses shifted Vim-style keys (`Shift+J/K` for file selection and `Shift+H/L` for tree horizontal scroll), while lowercase `h/j/k/l` remain diff-pane navigation keys.
- References: src/input.rs, docs/10-architecture-overview.md
- Rollback Notes: Restore arrow-key mappings for `SelectPrevFile`/`SelectNextFile` and `TreeScrollLeft`/`TreeScrollRight` in `src/input.rs`.

## 2026-02-21
- Change: Added `Shift+R` keybinding to refresh repository state in-place.
- Impact: The changed-file tree and file list are reloaded from `git status`, and the currently selected path is reloaded so its diff view reflects latest worktree state without restarting `fdf`; refresh now invalidates syntax-highlight cache entries for the selected file so updated content is rendered immediately.
- References: src/input.rs, src/app.rs, src/ui.rs, docs/10-architecture-overview.md
- Rollback Notes: Remove `Action::Refresh`, `App::refresh`, and the `Shift+R` key mapping in `src/input.rs`.

## 2026-02-20
- Change: Added `Left`/`Right` keybindings for horizontal scrolling in the file tree pane.
- Impact: Long paths in the tree can be inspected without affecting diff-pane horizontal scroll (`h`/`l`).
- References: src/input.rs, src/app.rs, src/ui.rs, docs/10-architecture-overview.md
- Rollback Notes: Remove `TreeScrollLeft`/`TreeScrollRight` actions and tree label clipping offset logic.

## 2026-02-20
- Change: Replaced heuristic syntax coloring with `syntect`-based language highlighting in diff panes.
- Impact: Highlighting quality and language coverage improve while unknown syntaxes still render as plain text; change/insert/delete semantics are shown as background tint so syntax token colors remain visible; horizontal scrolling now clips already-highlighted spans to prevent unstable colors or dropped characters; syntax highlighting is now cached per selected file to reduce per-frame scroll latency and keep vertical-scroll coloring deterministic with newline-aware state transitions.
- References: Cargo.toml, src/ui.rs, docs/10-architecture-overview.md, docs/12-dependency-map.md
- Rollback Notes: Remove `syntect` dependency and restore non-library syntax styling path in `src/ui.rs`.

## 2026-02-20
- Change: Added `b` keybinding to toggle the changed-file tree pane on and off during review.
- Impact: Users can switch between split view and full-width diff view without leaving keyboard flow.
- References: src/input.rs, src/app.rs, src/ui.rs, docs/10-architecture-overview.md
- Rollback Notes: Remove `Action::ToggleTree`, `App.show_tree`, and conditional tree rendering path in `src/ui.rs`.

## 2026-02-20
- Change: Bootstrapped documentation tree from `docs/AGENT_AUTONOMY_DOCUMENTATION_BLUEPRINT.md`.
- Impact: Repository now has baseline agent/human operating documentation, quality gates, and runbooks.
- References: docs/AGENT_AUTONOMY_DOCUMENTATION_BLUEPRINT.md
- Rollback Notes: Remove generated docs files if adopting a different documentation standard.

## 2026-02-20
- Change: Added architecture overview for `fdf` module boundaries and data flow.
- Impact: Implementation placement and cross-cutting concerns are now documented for autonomous work.
- References: docs/10-architecture-overview.md
- Rollback Notes: N/A.
