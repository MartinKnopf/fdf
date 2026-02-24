mod app;
mod comments;
mod diff;
mod git;
mod input;
mod model;
mod tree;
mod ui;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Result};
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::App;
use crate::comments::Comments;
use crate::input::map_key;

fn main() -> Result<()> {
    let (comments_path,) = parse_args()?;

    let comments = match comments_path {
        Some(path) => comments::load_comments(&path)?,
        None => Comments::default(),
    };

    let repo_root = git::repo_root()?;
    let mut app = App::new(repo_root, comments)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let run_result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    run_result
}

fn parse_args() -> Result<(Option<PathBuf>,)> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut comments_path: Option<PathBuf> = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-c" | "--comments" => {
                i += 1;
                if i >= args.len() {
                    bail!("Missing path argument for -c/--comments");
                }
                comments_path = Some(PathBuf::from(&args[i]));
            }
            other => {
                bail!("Unknown argument: {}. Use -h for help.", other);
            }
        }
        i += 1;
    }

    Ok((comments_path,))
}

fn print_help() {
    print!(
        "\
fdf — a read-only terminal git diff viewer

USAGE
    fdf [OPTIONS]

OPTIONS
    -c, --comments <path>   Load a YAML comments file to display annotations
                            alongside the diff (toggle with 'c' in the UI)
    -h, --help              Print this help message and exit

KEYBINDINGS
    j / k           Scroll down / up
    h / l           Scroll left / right
    Ctrl-d / Ctrl-u Page down / up
    n / N           Jump to next / previous change block
    g g / G         Jump to top / bottom
    J / K           Select next / previous file
    H / L           Scroll tree left / right
    b               Toggle file tree panel
    c               Toggle comments panel (requires -c)
    R               Refresh diff from disk
    q               Quit

COMMENTS FILE
    The comments file is a YAML document that attaches annotations to specific
    lines in the diff. Comments appear in a third panel to the right of the
    WORKTREE pane. Long comments are automatically word-wrapped to fit the
    panel width, and padding rows are inserted into the diff panels so all
    three columns stay vertically aligned.

    Line numbers refer to the new-file (WORKTREE / right-side) line numbers
    as shown in the diff viewer.

  Schema
    files:
      '<file-path>':              # path relative to repo root
        comment: '<text>'         # optional file-level comment (shown in title)
        lines:
          <line>: '<text>'        # line-level comment keyed by new-file line number

  Example
    files:
      'src/main.rs':
        comment: 'Refactored entry point for clarity'
        lines:
          12: 'Replaced unwrap with proper error handling'
          34: 'New helper function extracted from main loop'
      'src/lib.rs':
        lines:
          5: 'Added public re-export for downstream crates'
"
    );
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| {
            app.set_viewport_rows(ui::viewport_rows(frame.area()));
            ui::render(frame, app);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.on_action(map_key(key))?;
            }
        }
    }

    Ok(())
}
