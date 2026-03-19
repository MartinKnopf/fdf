mod app;
mod comments;
mod diff;
mod difft;
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

use crate::app::{App, ShellCommand};
use crate::comments::Comments;
use crate::input::map_key;

fn main() -> Result<()> {
    let (comments_path, use_difft, syntax_theme) = parse_args()?;

    if let Some(ref theme) = syntax_theme {
        ui::set_syntax_theme(theme).map_err(|e| anyhow::anyhow!(e))?;
    }

    let comments = match comments_path {
        Some(path) => comments::load_comments(&path)?,
        None => Comments::default(),
    };

    let repo_root = git::repo_root()?;
    let mut app = App::new(repo_root, comments, use_difft)?;

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

fn parse_args() -> Result<(Option<PathBuf>, bool, Option<String>)> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut comments_path: Option<PathBuf> = None;
    let mut use_difft = false;
    let mut syntax_theme: Option<String> = None;
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
            "-d" | "--use-difft" => {
                use_difft = true;
            }
            "-s" | "--syntax-theme" => {
                i += 1;
                if i >= args.len() {
                    bail!("Missing theme name for -s/--syntax-theme");
                }
                syntax_theme = Some(args[i].clone());
            }
            other => {
                bail!("Unknown argument: {}. Use -h for help.", other);
            }
        }
        i += 1;
    }

    Ok((comments_path, use_difft, syntax_theme))
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
    -d, --use-difft         Use difftastic for syntax-aware sub-line diffing
                            (requires 'difft' on PATH, falls back silently)
    -s, --syntax-theme <name>
                            Set syntax highlighting theme for unchanged lines
                            Available: base16-eighties.dark, base16-mocha.dark,
                            base16-ocean.dark (default), base16-ocean.light,
                            Dracula, InspiredGitHub, Solarized (dark),
                            Solarized (light)
    -h, --help              Print this help message and exit

KEYBINDINGS
    j / k           Scroll down / up
    h / l           Scroll left / right
    Ctrl-d / Ctrl-u Page down / up
    n / N           Jump to next / previous change block
    g g / G         Jump to top / bottom
    J / K           Select next / previous file
    H / L           Scroll tree left / right
    u               Stage / unstage selected file
    C               Run git commit (suspends TUI)
    P               Run git push (suspends TUI, wait for key)
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

        if let Some(cmd) = app.shell_command.take() {
            run_shell_command(terminal, app, &cmd)?;
        }
    }

    Ok(())
}

/// Suspend the TUI, run an interactive command, then resume and refresh.
fn run_shell_command(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    cmd: &ShellCommand,
) -> Result<()> {
    // Leave TUI
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Run the command interactively
    let status = std::process::Command::new(&cmd.args[0])
        .args(&cmd.args[1..])
        .current_dir(&app.repo_root)
        .status();

    match &status {
        Ok(s) if !s.success() => {
            eprintln!("\nCommand exited with {}", s);
        }
        Err(e) => {
            eprintln!("\nFailed to run {}: {}", cmd.args[0], e);
        }
        _ => {}
    }

    if cmd.wait_for_key {
        eprintln!("\nPress Enter or q to return to fdf...");
        wait_for_enter_or_q();
    }

    // Resume TUI regardless of command outcome
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    // Refresh file list to reflect any changes
    app.on_action(crate::input::Action::Refresh)?;

    Ok(())
}

/// Block until the user presses Enter or q (in cooked/line mode).
fn wait_for_enter_or_q() {
    // We're already in cooked mode (raw mode disabled), so we need raw mode
    // briefly to read individual keypresses without waiting for Enter.
    let _ = enable_raw_mode();
    loop {
        if let Ok(true) = event::poll(Duration::from_millis(200)) {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    let _ = disable_raw_mode();
}
