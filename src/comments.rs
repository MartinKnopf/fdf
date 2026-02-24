use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::model::AlignedRow;

/// Raw YAML structure for the comments file.
///
/// Expected format:
/// ```yaml
/// files:
///   'path/to/file1':
///     comment: 'The changes in this file are about...'
///     lines:
///       10: 'This change was made to...'
///       319: 'Here I changed x to y because...'
/// ```
#[derive(Debug, Deserialize)]
struct CommentsFile {
    files: HashMap<String, FileCommentEntry>,
}

#[derive(Debug, Deserialize)]
struct FileCommentEntry {
    comment: Option<String>,
    lines: Option<HashMap<usize, String>>,
}

/// Parsed comments for one file.
#[derive(Debug, Clone, Default)]
pub struct FileComments {
    /// Top-level comment for the file.
    pub comment: Option<String>,
    /// Line-level comments keyed by new-file (right-side) line number.
    pub line_comments: HashMap<usize, String>,
}

/// All parsed comments, keyed by file path string.
#[derive(Debug, Clone, Default)]
pub struct Comments {
    pub files: HashMap<String, FileComments>,
}

impl Comments {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn for_file(&self, path: &Path) -> Option<&FileComments> {
        let key = path.to_string_lossy();
        self.files.get(key.as_ref())
    }
}

/// Load and parse comments from a YAML file path.
pub fn load_comments(path: &Path) -> Result<Comments> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read comments file: {}", path.display()))?;
    parse_comments(&content)
}

fn parse_comments(yaml: &str) -> Result<Comments> {
    let raw: CommentsFile = serde_yaml::from_str(yaml).context("Failed to parse comments YAML")?;

    let mut files = HashMap::new();
    for (path, entry) in raw.files {
        files.insert(
            path,
            FileComments {
                comment: entry.comment,
                line_comments: entry.lines.unwrap_or_default(),
            },
        );
    }

    Ok(Comments { files })
}

/// A single display row that parallels aligned diff rows, holding comment text
/// (if any) for that visual line.
#[derive(Debug, Clone)]
pub struct CommentRow {
    pub text: String,
}

/// Wrap a single line of text to fit within `width` characters.
/// Uses word-boundary wrapping, falling back to character wrapping for long words.
fn wrap_line(line: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![line.to_string()];
    }
    if line.chars().count() <= width {
        return vec![line.to_string()];
    }

    let mut wrapped = Vec::new();
    let mut current = String::new();
    let mut current_len = 0;

    for word in line.split(' ') {
        let word_len = word.chars().count();

        if current_len == 0 {
            // First word on this line.
            if word_len <= width {
                current.push_str(word);
                current_len = word_len;
            } else {
                // Character-wrap the long word.
                for ch in word.chars() {
                    if current_len >= width {
                        wrapped.push(current);
                        current = String::new();
                        current_len = 0;
                    }
                    current.push(ch);
                    current_len += 1;
                }
            }
        } else {
            // Need a space separator: current_len + 1 (space) + word_len
            let needed = current_len + 1 + word_len;
            if needed <= width {
                current.push(' ');
                current.push_str(word);
                current_len = needed;
            } else {
                // Flush current line, start new one with this word.
                wrapped.push(current);
                current = String::new();
                current_len = 0;

                if word_len <= width {
                    current.push_str(word);
                    current_len = word_len;
                } else {
                    // Character-wrap the long word.
                    for ch in word.chars() {
                        if current_len >= width {
                            wrapped.push(current);
                            current = String::new();
                            current_len = 0;
                        }
                        current.push(ch);
                        current_len += 1;
                    }
                }
            }
        }
    }

    if !current.is_empty() {
        wrapped.push(current);
    }

    if wrapped.is_empty() {
        vec![String::new()]
    } else {
        wrapped
    }
}

/// Expand aligned rows with padding rows where multiline or wrapped comments
/// exceed the space available. Returns the expanded aligned rows and a parallel
/// vec of comment text lines.
///
/// `file_comments` — the comments for the current file (if any).
/// `wrap_width` — max character width for comment text; lines longer than this
///                are word-wrapped (0 means no wrapping).
///
/// When a comment on new-file line N spans more visual lines than the diff row
/// occupies, extra padding rows are inserted. Padding rows have no line numbers,
/// empty text, and `RowKind::Equal`.
pub fn expand_rows_with_comments(
    rows: &[AlignedRow],
    file_comments: Option<&FileComments>,
    wrap_width: usize,
) -> (Vec<AlignedRow>, Vec<CommentRow>) {
    let fc = match file_comments {
        Some(fc) => fc,
        None => {
            // No comments at all — just produce empty comment rows, no expansion.
            let comment_rows = rows
                .iter()
                .map(|_| CommentRow {
                    text: String::new(),
                })
                .collect();
            return (rows.to_vec(), comment_rows);
        }
    };

    let mut expanded_rows: Vec<AlignedRow> = Vec::with_capacity(rows.len());
    let mut comment_rows: Vec<CommentRow> = Vec::with_capacity(rows.len());

    for row in rows {
        // Look up comment by right-side line number.
        let comment_text = row
            .right_line_no
            .and_then(|ln| fc.line_comments.get(&ln))
            .cloned()
            .unwrap_or_default();

        // Split on explicit newlines, then wrap each resulting line.
        let visual_lines: Vec<String> = if comment_text.is_empty() {
            vec![String::new()]
        } else {
            comment_text
                .lines()
                .flat_map(|line| {
                    if wrap_width > 0 {
                        wrap_line(line, wrap_width)
                    } else {
                        vec![line.to_string()]
                    }
                })
                .collect()
        };

        // First line goes on the actual row.
        expanded_rows.push(row.clone());
        comment_rows.push(CommentRow {
            text: visual_lines[0].clone(),
        });

        // If the comment has more visual lines, insert padding rows.
        for extra_line in &visual_lines[1..] {
            expanded_rows.push(AlignedRow {
                left_line_no: None,
                right_line_no: None,
                left_text: String::new(),
                right_text: String::new(),
                kind: crate::model::RowKind::Equal,
            });
            comment_rows.push(CommentRow {
                text: extra_line.to_string(),
            });
        }
    }

    (expanded_rows, comment_rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AlignedRow, RowKind};

    #[test]
    fn parse_basic_comments_yaml() {
        let yaml = r#"
files:
  'src/main.rs':
    comment: 'Refactored entry point'
    lines:
      5: 'Added error handling'
      12: 'New import'
  'src/lib.rs':
    lines:
      1: 'Module doc'
"#;
        let comments = parse_comments(yaml).unwrap();
        assert_eq!(comments.files.len(), 2);

        let main = comments.files.get("src/main.rs").unwrap();
        assert_eq!(main.comment.as_deref(), Some("Refactored entry point"));
        assert_eq!(main.line_comments.get(&5).unwrap(), "Added error handling");
        assert_eq!(main.line_comments.get(&12).unwrap(), "New import");

        let lib = comments.files.get("src/lib.rs").unwrap();
        assert!(lib.comment.is_none());
        assert_eq!(lib.line_comments.get(&1).unwrap(), "Module doc");
    }

    #[test]
    fn expand_rows_inserts_padding_for_multiline_comment() {
        let rows = vec![
            AlignedRow {
                left_line_no: Some(1),
                right_line_no: Some(1),
                left_text: "old1".into(),
                right_text: "new1".into(),
                kind: RowKind::Changed,
            },
            AlignedRow {
                left_line_no: Some(2),
                right_line_no: Some(2),
                left_text: "same".into(),
                right_text: "same".into(),
                kind: RowKind::Equal,
            },
        ];

        let mut line_comments = HashMap::new();
        line_comments.insert(1, "Line one\nLine two\nLine three".to_string());

        let fc = FileComments {
            comment: None,
            line_comments,
        };

        let (expanded, comments) = expand_rows_with_comments(&rows, Some(&fc), 0);

        // Original 2 rows + 2 padding rows = 4 total
        assert_eq!(expanded.len(), 4);
        assert_eq!(comments.len(), 4);

        // First row keeps original data
        assert_eq!(expanded[0].right_line_no, Some(1));
        assert_eq!(comments[0].text, "Line one");

        // Padding rows have no line numbers
        assert_eq!(expanded[1].left_line_no, None);
        assert_eq!(expanded[1].right_line_no, None);
        assert_eq!(comments[1].text, "Line two");

        assert_eq!(expanded[2].left_line_no, None);
        assert_eq!(expanded[2].right_line_no, None);
        assert_eq!(comments[2].text, "Line three");

        // Original second row still present
        assert_eq!(expanded[3].right_line_no, Some(2));
        assert!(comments[3].text.is_empty());
    }

    #[test]
    fn expand_rows_no_comments_no_change() {
        let rows = vec![AlignedRow {
            left_line_no: Some(1),
            right_line_no: Some(1),
            left_text: "x".into(),
            right_text: "x".into(),
            kind: RowKind::Equal,
        }];

        let (expanded, comments) = expand_rows_with_comments(&rows, None, 0);
        assert_eq!(expanded.len(), 1);
        assert_eq!(comments.len(), 1);
        assert!(comments[0].text.is_empty());
    }

    #[test]
    fn wrap_line_short_text_no_wrap() {
        let result = wrap_line("hello world", 20);
        assert_eq!(result, vec!["hello world"]);
    }

    #[test]
    fn wrap_line_word_boundary() {
        let result = wrap_line("hello world foo bar", 11);
        assert_eq!(result, vec!["hello world", "foo bar"]);
    }

    #[test]
    fn wrap_line_long_word_char_wrap() {
        let result = wrap_line("abcdefghij", 4);
        assert_eq!(result, vec!["abcd", "efgh", "ij"]);
    }

    #[test]
    fn wrap_line_mixed_words_and_long() {
        let result = wrap_line("hi abcdefghij bye", 6);
        assert_eq!(result, vec!["hi", "abcdef", "ghij", "bye"]);
    }

    #[test]
    fn expand_rows_wraps_long_comment() {
        let rows = vec![AlignedRow {
            left_line_no: Some(1),
            right_line_no: Some(1),
            left_text: "code".into(),
            right_text: "code".into(),
            kind: RowKind::Changed,
        }];

        let mut line_comments = HashMap::new();
        line_comments.insert(1, "This is a long comment that wraps".to_string());

        let fc = FileComments {
            comment: None,
            line_comments,
        };

        // wrap_width=20: "This is a long" | "comment that wraps"
        let (expanded, comments) = expand_rows_with_comments(&rows, Some(&fc), 20);

        assert_eq!(expanded.len(), 2);
        assert_eq!(comments.len(), 2);

        // First row has original data
        assert_eq!(expanded[0].right_line_no, Some(1));
        assert_eq!(comments[0].text, "This is a long");

        // Padding row for wrapped text
        assert_eq!(expanded[1].left_line_no, None);
        assert_eq!(expanded[1].right_line_no, None);
        assert_eq!(comments[1].text, "comment that wraps");
    }
}
