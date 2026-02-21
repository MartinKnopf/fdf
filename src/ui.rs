use std::cell::RefCell;
use std::path::Path;
use std::sync::OnceLock;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style as SyntectStyle, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

use crate::app::App;
use crate::model::{AlignedRow, RowKind};

#[derive(Debug)]
struct HighlightedRow {
    left: Vec<Span<'static>>,
    right: Vec<Span<'static>>,
}

#[derive(Debug)]
struct FileHighlightCache {
    file_path: String,
    epoch: u64,
    rows_ptr: usize,
    rows_len: usize,
    rows: Vec<HighlightedRow>,
}

thread_local! {
    static HIGHLIGHT_CACHE: RefCell<Option<FileHighlightCache>> = const { RefCell::new(None) };
}

pub fn render(frame: &mut Frame<'_>, app: &App) {
    if app.show_tree {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(28), Constraint::Percentage(72)])
            .split(frame.area());

        render_tree(frame, app, chunks[0]);
        render_diff(frame, app, chunks[1]);
    } else {
        render_diff(frame, app, frame.area());
    }
}

pub fn viewport_rows(area: Rect) -> usize {
    area.height.saturating_sub(2).max(1) as usize
}

fn render_tree(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let content_width = area.width.saturating_sub(2) as usize;
    let items: Vec<ListItem> = app
        .tree_rows
        .iter()
        .map(|row| {
            let indent = "  ".repeat(row.depth);
            let full_label = format!("{}{}", indent, row.label);
            let clipped_label: String = full_label
                .chars()
                .skip(app.tree_h_scroll)
                .take(content_width)
                .collect();
            let style = if row.is_dir {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(clipped_label)).style(style)
        })
        .collect();

    let mut state = ListState::default();
    if let Some(selected) = selected_tree_row_idx(app) {
        state.select(Some(selected));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .title("Changed Files")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_diff(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let right_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Length(1),
        ])
        .split(area);

    let selected_file = app.selected_file();
    let title = selected_file
        .map(|file| file.path.to_string_lossy().to_string())
        .unwrap_or_else(|| "No changes".to_string());

    let syntax = selected_file
        .map(|file| syntax_for_path(&file.path, syntax_set()))
        .unwrap_or_else(|| syntax_set().find_syntax_plain_text());

    let (left_lines, right_lines) = if let Some(file) = selected_file {
        if let Some(rows) = file.aligned_rows.as_ref() {
            with_highlighted_rows_for_file(
                &file.path,
                rows,
                syntax,
                app.highlight_epoch,
                |highlighted_rows| {
                    build_visible_rows(
                        rows,
                        highlighted_rows,
                        app.v_scroll,
                        app.h_scroll,
                        right_chunks[0].width as usize,
                        right_chunks[0].height as usize,
                    )
                },
            )
        } else {
            (
                vec![Line::from("No changed files")],
                vec![Line::from("No changed files")],
            )
        }
    } else {
        (
            vec![Line::from("No changed files")],
            vec![Line::from("No changed files")],
        )
    };

    let left = Paragraph::new(left_lines).block(
        Block::default()
            .title(format!("HEAD | {}", title))
            .borders(Borders::ALL),
    );

    let right = Paragraph::new(right_lines).block(
        Block::default()
            .title(format!("WORKTREE | {}", title))
            .borders(Borders::ALL),
    );

    frame.render_widget(left, right_chunks[0]);
    frame.render_widget(right, right_chunks[1]);

    render_scrollbar(
        frame,
        right_chunks[2],
        app.selected_rows().map(|rows| rows.as_slice()),
        app.v_scroll,
        app.viewport_rows,
    );
}

fn build_visible_rows(
    rows: &[AlignedRow],
    highlighted_rows: &[HighlightedRow],
    v_scroll: usize,
    h_scroll: usize,
    pane_width: usize,
    pane_height: usize,
) -> (Vec<Line<'static>>, Vec<Line<'static>>) {
    let viewport_height = pane_height.saturating_sub(2).max(1);
    let max_visible = rows.len().saturating_sub(v_scroll).min(viewport_height);
    let end_idx = (v_scroll + max_visible)
        .min(rows.len())
        .min(highlighted_rows.len());
    let mut left = Vec::with_capacity(max_visible);
    let mut right = Vec::with_capacity(max_visible);

    let content_width = pane_width.saturating_sub(8);

    for idx in v_scroll..end_idx {
        let row = &rows[idx];
        let highlighted = &highlighted_rows[idx];

        left.push(styled_diff_line(
            row.left_line_no,
            row.kind,
            &highlighted.left,
            h_scroll,
            content_width,
        ));
        right.push(styled_diff_line(
            row.right_line_no,
            row.kind,
            &highlighted.right,
            h_scroll,
            content_width,
        ));
    }

    (left, right)
}

fn styled_diff_line(
    line_no: Option<usize>,
    kind: RowKind,
    highlighted_spans: &[Span<'static>],
    h_scroll: usize,
    max_chars: usize,
) -> Line<'static> {
    let number = line_no
        .map(|n| format!("{:>4}", n))
        .unwrap_or_else(|| "    ".to_string());
    let base = row_style(kind);

    let mut spans = Vec::new();
    spans.push(Span::styled(format!("{} ", number), base));
    spans.extend(clip_spans(highlighted_spans, h_scroll, max_chars));

    Line::from(spans)
}

fn with_highlighted_rows_for_file<R>(
    file_path: &Path,
    rows: &[AlignedRow],
    syntax: &SyntaxReference,
    epoch: u64,
    f: impl FnOnce(&[HighlightedRow]) -> R,
) -> R {
    let file_path_key = file_path.to_string_lossy().into_owned();
    let rows_ptr = rows.as_ptr() as usize;
    let rows_len = rows.len();

    HIGHLIGHT_CACHE.with(|cache_cell| {
        let mut cache = cache_cell.borrow_mut();
        let cache_miss = cache.as_ref().map_or(true, |existing| {
            existing.epoch != epoch
                || existing.file_path != file_path_key
                || existing.rows_ptr != rows_ptr
                || existing.rows_len != rows_len
        });

        if cache_miss {
            let computed_rows = build_highlighted_rows(rows, syntax);
            *cache = Some(FileHighlightCache {
                file_path: file_path_key,
                epoch,
                rows_ptr,
                rows_len,
                rows: computed_rows,
            });
        }

        let highlighted_rows = cache
            .as_ref()
            .map(|entry| entry.rows.as_slice())
            .unwrap_or(&[]);
        f(highlighted_rows)
    })
}

fn build_highlighted_rows(rows: &[AlignedRow], syntax: &SyntaxReference) -> Vec<HighlightedRow> {
    let mut left_highlighter = HighlightLines::new(syntax, syntax_theme());
    let mut right_highlighter = HighlightLines::new(syntax, syntax_theme());
    let mut highlighted_rows = Vec::with_capacity(rows.len());

    for row in rows {
        let base = row_style(row.kind);
        highlighted_rows.push(HighlightedRow {
            left: highlight_line(&row.left_text, &mut left_highlighter, base),
            right: highlight_line(&row.right_text, &mut right_highlighter, base),
        });
    }

    highlighted_rows
}

fn clip_spans(spans: &[Span<'static>], offset: usize, max_chars: usize) -> Vec<Span<'static>> {
    if max_chars == 0 {
        return Vec::new();
    }

    let mut remaining_skip = offset;
    let mut remaining_take = max_chars;
    let mut clipped = Vec::new();

    for span in spans {
        if remaining_take == 0 {
            break;
        }

        let text = span.content.as_ref();
        let char_count = text.chars().count();

        if remaining_skip >= char_count {
            remaining_skip -= char_count;
            continue;
        }

        let start = remaining_skip;
        let take = (char_count - start).min(remaining_take);
        let chunk: String = text.chars().skip(start).take(take).collect();
        if !chunk.is_empty() {
            clipped.push(Span::styled(chunk, span.style));
        }

        remaining_skip = 0;
        remaining_take -= take;
    }

    clipped
}

fn highlight_line(
    line: &str,
    highlighter: &mut HighlightLines<'_>,
    base: Style,
) -> Vec<Span<'static>> {
    let mut line_for_highlight = String::with_capacity(line.len() + 1);
    line_for_highlight.push_str(line);
    line_for_highlight.push('\n');

    let ranges = match highlighter.highlight_line(&line_for_highlight, syntax_set()) {
        Ok(ranges) => ranges,
        Err(_) => return vec![Span::styled(line.to_string(), base)],
    };

    let mut spans: Vec<Span<'static>> = ranges
        .into_iter()
        .map(|(style, text)| Span::styled(text.to_string(), syntect_to_ratatui_style(style, base)))
        .collect();
    trim_trailing_newline(&mut spans);

    if spans.is_empty() {
        vec![Span::styled(String::new(), base)]
    } else {
        spans
    }
}

fn trim_trailing_newline(spans: &mut Vec<Span<'static>>) {
    let Some(last_idx) = spans.iter().rposition(|span| !span.content.is_empty()) else {
        return;
    };

    if !spans[last_idx].content.ends_with('\n') {
        return;
    }

    let mut content = spans[last_idx].content.to_string();
    content.pop();

    if content.is_empty() {
        spans.remove(last_idx);
    } else {
        let style = spans[last_idx].style;
        spans[last_idx] = Span::styled(content, style);
    }
}

fn syntax_set() -> &'static SyntaxSet {
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn syntax_theme() -> &'static Theme {
    static THEME: OnceLock<Theme> = OnceLock::new();
    THEME.get_or_init(|| {
        let themes = ThemeSet::load_defaults();
        themes
            .themes
            .get("base16-ocean.dark")
            .or_else(|| themes.themes.values().next())
            .cloned()
            .expect("syntect default themes should not be empty")
    })
}

fn syntax_for_path<'a>(path: &Path, syntax_set: &'a SyntaxSet) -> &'a SyntaxReference {
    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        if let Some(syntax) = syntax_set.find_syntax_by_token(file_name) {
            return syntax;
        }

        let lower_file_name = file_name.to_ascii_lowercase();
        if lower_file_name != file_name {
            if let Some(syntax) = syntax_set.find_syntax_by_token(&lower_file_name) {
                return syntax;
            }
        }
    }

    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        if let Some(syntax) = syntax_set.find_syntax_by_extension(extension) {
            return syntax;
        }

        let lower_extension = extension.to_ascii_lowercase();
        if lower_extension != extension {
            if let Some(syntax) = syntax_set.find_syntax_by_extension(&lower_extension) {
                return syntax;
            }
        }
    }

    syntax_set.find_syntax_plain_text()
}

fn syntect_to_ratatui_style(style: SyntectStyle, base: Style) -> Style {
    // Preserve diff semantic background while letting syntect own token foreground color.
    let mut mapped = base.fg(Color::Rgb(
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
    ));

    if style.font_style.contains(FontStyle::BOLD) {
        mapped = mapped.add_modifier(Modifier::BOLD);
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        mapped = mapped.add_modifier(Modifier::ITALIC);
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        mapped = mapped.add_modifier(Modifier::UNDERLINED);
    }

    mapped
}

fn row_style(kind: RowKind) -> Style {
    match kind {
        RowKind::Equal => Style::default(),
        RowKind::Changed => Style::default().bg(Color::Rgb(63, 54, 18)),
        RowKind::Insert => Style::default().bg(Color::Rgb(18, 60, 36)),
        RowKind::Delete => Style::default().bg(Color::Rgb(72, 24, 24)),
    }
}

fn selected_tree_row_idx(app: &App) -> Option<usize> {
    app.tree_rows
        .iter()
        .position(|r| r.file_index == Some(app.selected_file_idx))
}

fn render_scrollbar(
    frame: &mut Frame<'_>,
    area: Rect,
    rows: Option<&[AlignedRow]>,
    v_scroll: usize,
    viewport_rows: usize,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let height = area.height as usize;
    let mut glyphs = vec![Line::styled("│", Style::default().fg(Color::DarkGray)); height];

    if let Some(rows) = rows {
        if !rows.is_empty() {
            let total = rows.len();

            for (idx, row) in rows.iter().enumerate() {
                if row.kind == RowKind::Equal {
                    continue;
                }
                let pos = idx.saturating_mul(height) / total;
                if pos < glyphs.len() {
                    glyphs[pos] = Line::styled("╵", Style::default().fg(Color::LightGreen));
                }
            }

            let thumb_len = ((viewport_rows.max(1) * height) / total).max(1).min(height);
            let mut thumb_start = v_scroll.saturating_mul(height) / total;
            if thumb_start + thumb_len > height {
                thumb_start = height.saturating_sub(thumb_len);
            }

            for pos in thumb_start..thumb_start + thumb_len {
                if pos < glyphs.len() {
                    glyphs[pos] = Line::styled("█", Style::default().fg(Color::Green));
                }
            }
        }
    }

    frame.render_widget(Paragraph::new(glyphs), area);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::Span;
    use syntect::easy::HighlightLines;
    use syntect::highlighting::{Color as SyntectColor, FontStyle, Style as SyntectStyle};

    use crate::model::{AlignedRow, RowKind};

    use super::{
        clip_spans, highlight_line, syntax_for_path, syntax_set, syntax_theme,
        syntect_to_ratatui_style, with_highlighted_rows_for_file,
    };

    #[test]
    fn resolves_rust_syntax_by_extension() {
        let syntax = syntax_for_path(Path::new("src/main.rs"), syntax_set());
        assert_eq!(syntax.name, "Rust");
    }

    #[test]
    fn falls_back_to_plain_text_syntax_for_unknown_extension() {
        let syntax = syntax_for_path(Path::new("assets/file.unknown"), syntax_set());
        assert_eq!(syntax.name, syntax_set().find_syntax_plain_text().name);
    }

    #[test]
    fn maps_syntect_styles_to_ratatui_styles() {
        let syntect_style = SyntectStyle {
            foreground: SyntectColor {
                r: 1,
                g: 2,
                b: 3,
                a: 255,
            },
            background: SyntectColor {
                r: 10,
                g: 20,
                b: 30,
                a: 255,
            },
            font_style: FontStyle::BOLD | FontStyle::ITALIC,
        };

        let mapped = syntect_to_ratatui_style(syntect_style, Style::default());

        assert_eq!(mapped.fg, Some(Color::Rgb(1, 2, 3)));
        assert!(mapped.add_modifier.contains(Modifier::BOLD));
        assert!(mapped.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn preserves_existing_diff_background_color() {
        let syntect_style = SyntectStyle {
            foreground: SyntectColor {
                r: 200,
                g: 100,
                b: 50,
                a: 255,
            },
            background: SyntectColor {
                r: 10,
                g: 20,
                b: 30,
                a: 255,
            },
            font_style: FontStyle::BOLD,
        };

        let mapped =
            syntect_to_ratatui_style(syntect_style, Style::default().bg(Color::Rgb(50, 51, 52)));

        assert_eq!(mapped.fg, Some(Color::Rgb(200, 100, 50)));
        assert_eq!(mapped.bg, Some(Color::Rgb(50, 51, 52)));
        assert!(mapped.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn highlight_line_produces_spans() {
        let syntax = syntax_for_path(Path::new("src/main.rs"), syntax_set());
        let mut highlighter = HighlightLines::new(syntax, syntax_theme());
        let spans = highlight_line(
            "fn main() { let x = 1; }",
            &mut highlighter,
            Style::default(),
        );

        assert!(!spans.is_empty());
        assert!(spans.iter().all(|span| !span.content.is_empty()));
    }

    #[test]
    fn clip_spans_matches_plain_text_clipping() {
        let text = "let greeting = \"hello world\";";
        let syntax = syntax_for_path(Path::new("src/main.rs"), syntax_set());
        let mut highlighter = HighlightLines::new(syntax, syntax_theme());
        let highlighted = highlight_line(text, &mut highlighter, Style::default());

        let clipped = clip_spans(&highlighted, 4, 14);
        let clipped_text: String = clipped
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<Vec<_>>()
            .join("");
        let expected: String = text.chars().skip(4).take(14).collect();

        assert_eq!(clipped_text, expected);
    }

    #[test]
    fn clip_spans_preserves_styles_across_boundaries() {
        let spans = vec![
            Span::styled("abcd".to_string(), Style::default().fg(Color::Red)),
            Span::styled("efgh".to_string(), Style::default().fg(Color::Blue)),
        ];

        let clipped = clip_spans(&spans, 2, 4);

        assert_eq!(clipped.len(), 2);
        assert_eq!(clipped[0].content.as_ref(), "cd");
        assert_eq!(clipped[0].style.fg, Some(Color::Red));
        assert_eq!(clipped[1].content.as_ref(), "ef");
        assert_eq!(clipped[1].style.fg, Some(Color::Blue));
    }

    #[test]
    fn highlight_line_does_not_emit_trailing_newline() {
        let syntax = syntax_for_path(Path::new("src/main.rs"), syntax_set());
        let mut highlighter = HighlightLines::new(syntax, syntax_theme());
        let text = "let value = 1;";

        let spans = highlight_line(text, &mut highlighter, Style::default());
        let rendered: String = spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<Vec<_>>()
            .join("");

        assert_eq!(rendered, text);
    }

    #[test]
    fn highlight_state_advances_across_empty_line() {
        let syntax = syntax_for_path(Path::new("src/main.rs"), syntax_set());
        let mut highlighter = HighlightLines::new(syntax, syntax_theme());

        let comment_spans = highlight_line("// comment", &mut highlighter, Style::default());
        let comment_style = comment_spans
            .iter()
            .find(|span| span.content.contains("comment"))
            .expect("expected comment token")
            .style;

        let _ = highlight_line("", &mut highlighter, Style::default());
        let code_spans = highlight_line("let value = 1;", &mut highlighter, Style::default());
        let let_style = code_spans
            .iter()
            .find(|span| span.content.contains("let"))
            .expect("expected let token")
            .style;

        assert_ne!(let_style, comment_style);
    }

    #[test]
    fn highlight_cache_invalidates_when_epoch_changes() {
        let syntax = syntax_for_path(Path::new("cache_epoch_test.rs"), syntax_set());
        let file_path = Path::new("cache_epoch_test.rs");
        let mut rows = vec![AlignedRow {
            left_line_no: Some(1),
            right_line_no: Some(1),
            left_text: "let value = 1;".to_string(),
            right_text: "let value = 1;".to_string(),
            kind: RowKind::Equal,
        }];

        let first = with_highlighted_rows_for_file(file_path, &rows, syntax, 10, |highlighted| {
            highlighted[0]
                .left
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<Vec<_>>()
                .join("")
        });

        rows[0].left_text = "let changed = 2;".to_string();

        let stale = with_highlighted_rows_for_file(file_path, &rows, syntax, 10, |highlighted| {
            highlighted[0]
                .left
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<Vec<_>>()
                .join("")
        });
        let refreshed =
            with_highlighted_rows_for_file(file_path, &rows, syntax, 11, |highlighted| {
                highlighted[0]
                    .left
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<Vec<_>>()
                    .join("")
            });

        assert_eq!(stale, first);
        assert_eq!(refreshed, "let changed = 2;");
    }
}
