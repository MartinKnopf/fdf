Last Updated: 2026-02-21
Status: active
Audience: both
Update Trigger: Any module addition/removal in `src/`, any event loop/input model changes, or git backend changes.
Source of Truth: `src/main.rs`, `src/app.rs`, `src/ui.rs`, `src/git.rs`, `src/diff.rs`, `src/tree.rs`, `src/model.rs`, `src/input.rs`, `Cargo.toml`

# Architecture Overview

## System Context
`fdf` is a local, read-only terminal application that visualizes git working tree changes side-by-side (`HEAD` vs worktree) for one repository.

- Runtime boundary:
  - Inputs: keyboard events and local git/worktree state.
  - Outputs: terminal UI rendering only.
  - Side effects: subprocess calls to `git`, filesystem reads for file content.
- Non-goals in current architecture:
  - No mutation of git state (`add`, `reset`, `checkout`, etc.).
  - No network calls.
  - No daemon/service mode.

## Component Map and Responsibilities
### High-level module map

| Component | File(s) | Responsibility |
|---|---|---|
| Bootstrap and lifecycle | `src/main.rs` | Initialize repo + app state, set terminal raw mode/alternate screen, run event/render loop, restore terminal on exit. |
| Application state machine | `src/app.rs` | Own all interactive state (`selected_file_idx`, scroll offsets, viewport size), dispatch actions, lazy-load file content and aligned rows, enforce scroll bounds. |
| Input translation | `src/input.rs` | Map raw `crossterm` key events to domain actions (`Action`). |
| Git data adapter | `src/git.rs` | Discover repo root, parse changed files from `git status --porcelain=v2 -z`, load `HEAD` and worktree content. |
| Tree builder | `src/tree.rs` | Build hierarchical path tree from changed files and flatten it into UI rows with status labels. |
| Diff alignment engine | `src/diff.rs` | Convert full old/new file text into aligned side-by-side rows with line numbers and row kinds. |
| UI rendering | `src/ui.rs` | Render tree pane, diff panes, syntax highlighting, and vertical scrollbar with change markers + viewport thumb. |
| Domain model | `src/model.rs` | Shared structs/enums (`ChangedFile`, `FileStatus`, `AlignedRow`, `RowKind`, `TreeNode`, `TreeRow`). |

### External dependencies

| Dependency | Used in | Why |
|---|---|---|
| `ratatui` | `src/main.rs`, `src/ui.rs` | Layout and widget rendering in terminal. |
| `crossterm` | `src/main.rs`, `src/input.rs` | Raw mode, alternate screen, key event polling. |
| `similar` | `src/diff.rs` | Line-level diff ops used to produce aligned full-file rows. |
| `syntect` | `src/ui.rs` | Language-aware syntax highlighting converted into terminal spans. |
| `anyhow` | most modules | Error propagation with context. |

## Request/Data Flow
### End-to-end flow

```text
main()
  -> git::repo_root()
  -> App::new()
       -> git::collect_changed_files()
       -> tree::build_tree() + tree::flatten_tree()
       -> ensure_selected_loaded() for first file
            -> git::load_file_contents()
                 -> git show HEAD:<path>
                 -> fs read <worktree path>
            -> diff::align_full_file()
  -> run loop:
       draw frame (ui::render)
       poll key event
       map_key -> Action
       app.on_action(Action)
       (loop)
```

### Interaction flow (per keypress)
1. `run()` receives `Event::Key` from `crossterm`.
2. `input::map_key()` maps raw key to `Action`.
3. `app.on_action()` mutates state:
   - file selection (`Shift+K`/`Shift+J`)
   - repository refresh (`Shift+R`) to reload changed files, rebuild tree rows, and reload the selected file
   - vertical scrolling (`j/k`, `Ctrl+d/u`, `gg`, `G`)
   - diff horizontal scrolling (`h`/`l`)
   - file tree horizontal scrolling (`Shift+H`/`Shift+L`)
   - diff block navigation with wrap (`n` / `N`)
   - file tree visibility toggle (`b`)
   - quit (`q`)
4. `ui::render()` reads immutable `App` state and re-renders:
   - optional left file tree (`TreeRow` list)
   - aligned rows window
   - rightmost scrollbar (change markers + viewport thumb)

### Data ownership and caching
- `App.files: Vec<ChangedFile>` is the canonical per-file data store.
- Each `ChangedFile` lazily caches:
  - `old_content` (`HEAD`)
  - `new_content` (worktree)
  - `aligned_rows` (computed once per file selection lifecycle)
- Re-selecting a file reuses cached rows instead of recomputing.

## Cross-Cutting Concerns
### Error handling and terminal safety
- All fallible operations return `anyhow::Result`.
- Terminal is always restored on loop exit path (`disable_raw_mode`, leave alternate screen, show cursor) in `src/main.rs`.

### Consistency and invariants
- File tree labels derive from `FileStatus::indicator()` in `src/model.rs` and `src/tree.rs`.
- Vertical scroll is clamped to `max_v_scroll()` after every action (`src/app.rs`).
- Viewport size is fed from `ui::viewport_rows(frame.area())` into `App` every draw (`src/main.rs`).

### Performance profile
- Initial load reads file list only.
- File contents and aligned diff rows are loaded/computed lazily on first selection.
- Diff rendering slices precomputed rows by viewport (`skip/take`) instead of recomputing diff.

### Text/binary and filesystem edge cases
- Non-UTF8 blobs are marked `ContentData::Binary`; UI shows placeholder row.
- Untracked directories are filtered out during status parsing.
- Missing `HEAD` version (new file) and missing worktree file (deleted path) gracefully map to empty text.
- Syntax highlighting uses `syntect` token/extension matching and falls back to plain text when no syntax matches.

### Git contract
- Source of changed files: `git status --porcelain=v2 --untracked-files=all -z`.
- Left pane baseline for content: `git show HEAD:<path>`.
- Right pane baseline for content: direct worktree file read.

## Placement Guidance for New Logic
- New keyboard behavior: extend `Action` and `map_key` in `src/input.rs`, then handle in `App::on_action`.
- New per-file derived view data: add field to `ChangedFile` in `src/model.rs`, compute in `App::ensure_selected_loaded`.
- New git state source/parsing: isolate in `src/git.rs`; keep `App` unaware of raw git output format.
- New UI widgets/panes: keep layout and rendering details in `src/ui.rs`; avoid business logic there.

## Open Questions
- None currently tracked in this file.
