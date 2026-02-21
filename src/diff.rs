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

            rows.push(AlignedRow {
                left_line_no: left.as_ref().map(|_| left_no),
                right_line_no: right.as_ref().map(|_| right_no),
                left_text: left.unwrap_or_default(),
                right_text: right.unwrap_or_default(),
                kind,
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
        });
    }

    rows
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
