use std::collections::HashMap;
use std::io::Write;
use std::ops::Range;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::model::AlignedRow;

/// Check if difft (difftastic) is available on PATH.
pub fn is_available() -> bool {
    Command::new("difft")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Enrich aligned rows with syntax-aware change ranges from difft.
/// On any failure, returns Ok(()) silently — the word-level diff remains as fallback.
pub fn enrich_rows(rows: &mut [AlignedRow], old: &str, new: &str, path: &Path) -> Result<()> {
    let (left_map, right_map) = match compute_ranges(old, new, path) {
        Ok(maps) => maps,
        Err(_) => return Ok(()),
    };

    for row in rows.iter_mut() {
        if row.kind != crate::model::RowKind::Changed {
            continue;
        }
        if let Some(ln) = row.left_line_no {
            if let Some(ranges) = left_map.get(&ln) {
                row.left_changed_ranges = ranges.clone();
            }
        }
        if let Some(ln) = row.right_line_no {
            if let Some(ranges) = right_map.get(&ln) {
                row.right_changed_ranges = ranges.clone();
            }
        }
    }

    Ok(())
}

/// Run difft and parse its JSON output into per-line change range maps.
/// Returns (lhs_map, rhs_map) keyed by 1-based line number.
fn compute_ranges(
    old: &str,
    new: &str,
    path: &Path,
) -> Result<(
    HashMap<usize, Vec<Range<usize>>>,
    HashMap<usize, Vec<Range<usize>>>,
)> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");

    let mut old_file = tempfile::Builder::new()
        .suffix(&format!(".{}", ext))
        .tempfile()
        .context("creating temp file for old content")?;
    old_file
        .write_all(old.as_bytes())
        .context("writing old content")?;

    let mut new_file = tempfile::Builder::new()
        .suffix(&format!(".{}", ext))
        .tempfile()
        .context("creating temp file for new content")?;
    new_file
        .write_all(new.as_bytes())
        .context("writing new content")?;

    let output = Command::new("difft")
        .env("DFT_UNSTABLE", "yes")
        .arg("--display")
        .arg("json")
        .arg("--context")
        .arg("0")
        .arg(old_file.path())
        .arg(new_file.path())
        .output()
        .context("running difft")?;

    if !output.status.success() {
        anyhow::bail!("difft exited with status {}", output.status);
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let difft_output: DifftOutput =
        serde_json::from_str(&json_str).context("parsing difft JSON")?;

    let mut left_map: HashMap<usize, Vec<Range<usize>>> = HashMap::new();
    let mut right_map: HashMap<usize, Vec<Range<usize>>> = HashMap::new();

    for chunk in &difft_output.chunks {
        for entry in chunk {
            if let Some(ref lhs) = entry.lhs {
                let ranges: Vec<Range<usize>> =
                    lhs.changes.iter().map(|c| c.start..c.end).collect();
                left_map.entry(lhs.line_number).or_default().extend(ranges);
            }
            if let Some(ref rhs) = entry.rhs {
                let ranges: Vec<Range<usize>> =
                    rhs.changes.iter().map(|c| c.start..c.end).collect();
                right_map.entry(rhs.line_number).or_default().extend(ranges);
            }
        }
    }

    Ok((left_map, right_map))
}

#[derive(Deserialize)]
struct DifftOutput {
    chunks: Vec<Vec<DifftEntry>>,
    #[allow(dead_code)]
    status: String,
}

#[derive(Deserialize)]
struct DifftEntry {
    lhs: Option<DifftSide>,
    rhs: Option<DifftSide>,
}

#[derive(Deserialize)]
struct DifftSide {
    line_number: usize,
    changes: Vec<DifftChange>,
}

#[derive(Deserialize)]
struct DifftChange {
    start: usize,
    end: usize,
    #[allow(dead_code)]
    content: Option<String>,
    #[allow(dead_code)]
    highlight: Option<String>,
}
