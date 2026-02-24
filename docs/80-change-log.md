Last Updated: 2026-02-23
Status: active
Audience: both
Update Trigger: Meaningful behavior/reliability/security/release change
Source of Truth: PRs, release tags

# Change Log

## 2026-02-23
- Change: File selection (`Shift+J`/`Shift+K`) wraps around at boundaries and short-circuits when selection is unchanged.
- Impact: `K` at the first file jumps to the last file; `J` at the last file jumps to the first. Repeated presses at a boundary that would select the same file (single-file case) now return immediately without reloading.
- References: src/app.rs

## 2026-02-23
- Change: Added `-h`/`--help` CLI flag with full usage, keybinding reference, and comments file schema documentation.
- Impact: Running `fdf -h` prints help text and exits without entering the TUI.
- References: src/main.rs

## 2026-02-23
- Change: Display row caching moved from single `DisplayCache` on `App` to per-file fields on `ChangedFile` with `display_wrap_width` staleness key.
- Impact: Switching between previously viewed files no longer recomputes display rows or invalidates the syntax highlight cache, eliminating lag on file switches with comments enabled.
- References: src/app.rs, src/model.rs

## 2026-02-23
- Change: Added comments feature — YAML-sourced annotations displayed alongside diffs.
- Impact: A new `-c`/`--comments <path>` CLI argument loads a YAML file with per-file and per-line annotations. Press `c` to toggle a third panel (Comments) to the right of the WORKTREE pane. Comments are word-wrapped to fit the panel width; multiline comments insert padding rows in the diff panels to keep all three columns vertically aligned. New module `src/comments.rs` handles YAML parsing, word wrapping, and row expansion. New dependencies: `serde`, `serde_yaml`.
- References: src/comments.rs, src/app.rs, src/input.rs, src/main.rs, src/model.rs, src/ui.rs, Cargo.toml, docs/10-architecture-overview.md, docs/11-boundaries-and-invariants.md, docs/12-dependency-map.md
- Rollback Notes: Remove `src/comments.rs`, revert `src/app.rs`/`src/input.rs`/`src/main.rs`/`src/model.rs`/`src/ui.rs` changes, remove `serde`/`serde_yaml` from `Cargo.toml`.

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
