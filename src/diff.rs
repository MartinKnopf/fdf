use std::ops::Range;

use similar::{ChangeTag, DiffTag, TextDiff};

use crate::model::{AlignedRow, RowKind};

pub fn align_full_file(old: &str, new: &str) -> Vec<AlignedRow> {
    let diff = TextDiff::from_lines(old, new);
    let mut rows = Vec::new();
    let mut left_no = 1usize;
    let mut right_no = 1usize;

    for op in diff.ops() {
        let old_chunk: Vec<String> = diff
            .iter_changes(op)
            .filter(|c| c.tag() != ChangeTag::Insert)
            .map(|c| clean_line(c.to_string()))
            .collect();
        let new_chunk: Vec<String> = diff
            .iter_changes(op)
            .filter(|c| c.tag() != ChangeTag::Delete)
            .map(|c| clean_line(c.to_string()))
            .collect();

        if op.tag() == DiffTag::Equal {
            for line in old_chunk {
                rows.push(AlignedRow {
                    left_line_no: Some(left_no),
                    right_line_no: Some(right_no),
                    left_text: line.clone(),
                    right_text: line,
                    kind: RowKind::Equal,
                    left_changed_ranges: Vec::new(),
                    right_changed_ranges: Vec::new(),
                });
                left_no += 1;
                right_no += 1;
            }
            continue;
        }

        let max_len = old_chunk.len().max(new_chunk.len());
        for i in 0..max_len {
            let left = old_chunk.get(i).cloned();
            let right = new_chunk.get(i).cloned();
            let kind = match (&left, &right) {
                (Some(_), Some(_)) => RowKind::Changed,
                (Some(_), None) => RowKind::Delete,
                (None, Some(_)) => RowKind::Insert,
                (None, None) => RowKind::Equal,
            };

            let (left_ranges, right_ranges) = if kind == RowKind::Changed {
                compute_inline_ranges(
                    left.as_deref().unwrap_or(""),
                    right.as_deref().unwrap_or(""),
                )
            } else {
                (Vec::new(), Vec::new())
            };

            rows.push(AlignedRow {
                left_line_no: left.as_ref().map(|_| left_no),
                right_line_no: right.as_ref().map(|_| right_no),
                left_text: left.unwrap_or_default(),
                right_text: right.unwrap_or_default(),
                kind,
                left_changed_ranges: left_ranges,
                right_changed_ranges: right_ranges,
            });

            if rows.last().and_then(|r| r.left_line_no).is_some() {
                left_no += 1;
            }
            if rows.last().and_then(|r| r.right_line_no).is_some() {
                right_no += 1;
            }
        }
    }

    if rows.is_empty() {
        rows.push(AlignedRow {
            left_line_no: None,
            right_line_no: None,
            left_text: String::new(),
            right_text: String::new(),
            kind: RowKind::Equal,
            left_changed_ranges: Vec::new(),
            right_changed_ranges: Vec::new(),
        });
    }

    rows
}

/// Compute byte ranges that differ between two lines using word-level diff.
/// Returns (left_ranges, right_ranges) where each range marks changed bytes.
///
/// Uses word-level granularity (splitting on non-alphanumeric boundaries) to
/// produce clean highlights on whole tokens rather than scattered characters.
fn compute_inline_ranges(left: &str, right: &str) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let diff = TextDiff::from_words(left, right);
    let mut left_ranges = Vec::new();
    let mut right_ranges = Vec::new();
    let mut left_byte = 0usize;
    let mut right_byte = 0usize;

    for change in diff.iter_all_changes() {
        let value = change.value();
        let byte_len = value.len();
        match change.tag() {
            ChangeTag::Equal => {
                left_byte += byte_len;
                right_byte += byte_len;
            }
            ChangeTag::Delete => {
                left_ranges.push(left_byte..left_byte + byte_len);
                left_byte += byte_len;
            }
            ChangeTag::Insert => {
                right_ranges.push(right_byte..right_byte + byte_len);
                right_byte += byte_len;
            }
        }
    }

    // Merge adjacent ranges
    left_ranges = merge_ranges(left_ranges);
    right_ranges = merge_ranges(right_ranges);

    (left_ranges, right_ranges)
}

/// Merge adjacent or overlapping ranges into contiguous spans.
fn merge_ranges(ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    if ranges.is_empty() {
        return ranges;
    }
    let mut merged = vec![ranges[0].clone()];
    for r in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        if r.start <= last.end {
            last.end = last.end.max(r.end);
        } else {
            merged.push(r.clone());
        }
    }
    merged
}

fn clean_line(mut line: String) -> String {
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_ranges_for_single_word_change() {
        let (left_ranges, right_ranges) = compute_inline_ranges("hello world", "hello earth");
        // word-level diff: "world" -> "earth" as whole tokens
        assert_eq!(left_ranges, vec![6..11]);
        assert_eq!(right_ranges, vec![6..11]);
    }

    #[test]
    fn inline_ranges_empty_for_identical_lines() {
        let (left_ranges, right_ranges) = compute_inline_ranges("same line", "same line");
        assert!(left_ranges.is_empty());
        assert!(right_ranges.is_empty());
    }

    #[test]
    fn inline_ranges_full_line_when_completely_different() {
        let (left_ranges, right_ranges) = compute_inline_ranges("abc", "xyz");
        assert_eq!(left_ranges, vec![0..3]);
        assert_eq!(right_ranges, vec![0..3]);
    }

    #[test]
    fn inline_ranges_code_token_change() {
        // Simulates the kind of change the user sees: function arg replacement
        let (left_ranges, right_ranges) = compute_inline_ranges(
            "nch(JENKINS_WORKSPACE, workspaceRoot, streamName)",
            "nch(JENKINS_WORKSPACE, workspaceRoot, 'master')",
        );
        // "streamName" on left should be highlighted, "'master'" on right
        assert!(!left_ranges.is_empty());
        assert!(!right_ranges.is_empty());
        // Common prefix "nch(JENKINS_WORKSPACE, workspaceRoot, " = 38 bytes
        assert!(left_ranges[0].start >= 38);
        assert!(right_ranges[0].start >= 38);
    }

    #[test]
    fn merge_adjacent_ranges() {
        let merged = merge_ranges(vec![0..2, 2..4, 6..8]);
        assert_eq!(merged, vec![0..4, 6..8]);
    }
}
