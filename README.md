# fdf

## What It Does
`fdf` is a native terminal diff viewer for git repositories. It shows changed files in a left-side tree with staged/unstaged indicators and renders a full-file side-by-side comparison (`HEAD` vs worktree) for the selected file. Focused on fast keyboard-driven inspection and staging of changes.

## Local Development Entrypoints
- Validate compile: `cargo check`
- Build release binary: `cargo build --release`
- Run in this repository: `cargo run`
- Run from another git repository: `/path/to/fdf/target/release/fdf`

## First Run
1. `cargo build --release`
2. `cd /path/to/your/git/repository`
3. `/absolute/path/to/fdf/target/release/fdf`
4. Press `q` to quit.

## Documentation
- Docs index: `docs/00-index.md`
- Agent guide: `AGENTS.md`
- Architecture overview: `docs/10-architecture-overview.md`

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `k` | Next / previous file |
| `h` / `l` | Tree scroll left / right (file switch when tree hidden) |
| `J` / `K` | Scroll down / up |
| `H` / `L` | Scroll left / right |
| `Ctrl-d` / `Ctrl-u` | Page down / up |
| `g g` | Go to top |
| `G` | Go to bottom |
| `n` / `N` | Next / previous change |
| `Space` | Stage / unstage selected file |
| `!` | Checkout file (discard changes, confirm with `y`) |
| `d` | Delete file (confirm with `y`) |
| `C` | Run `git commit` (suspends TUI, resumes after) |
| `p` | Run `git pull` (suspends TUI, waits for Enter/q) |
| `P` | Run `git push` (suspends TUI, waits for Enter/q) |
| `b` | Toggle file tree |
| `c` | Toggle comments pane |
| `R` | Refresh file list |
| `?` | Show keybindings help |
| `Esc` | Close help overlay |
| `q` | Quit |

## High-Level Architecture
The app has a small modular architecture:
- `src/main.rs`: terminal lifecycle and event/render loop.
- `src/input.rs`: keybinding to action mapping.
- `src/app.rs`: state machine for selection, scrolling, and navigation.
- `src/git.rs`: git status/content loading via git CLI.
- `src/diff.rs`: full-file line alignment for side-by-side rendering.
- `src/tree.rs` and `src/ui.rs`: file tree construction and terminal UI rendering.

See `docs/10-architecture-overview.md` for full data-flow and cross-cutting concerns.
