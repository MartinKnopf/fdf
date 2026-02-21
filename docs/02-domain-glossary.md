Last Updated: 2026-02-20
Status: active
Audience: both
Update Trigger: New domain concept or rename
Source of Truth: src/model.rs, src/app.rs, src/git.rs, src/diff.rs

# Domain Glossary

## Terms
### Changed File
Definition: A repository path with staged, unstaged, or untracked changes included in the viewer tree.
In Code: `src/model.rs` (`ChangedFile`)
Synonyms: modified file entry
Forbidden Alternate Meaning: Entire commit-level patch object

### File Status Indicator
Definition: Compact marker describing staged/unstaged/untracked state shown in the tree.
In Code: `src/model.rs` (`FileStatus::indicator`)
Synonyms: status badge
Forbidden Alternate Meaning: Git branch status

### Aligned Row
Definition: One rendered line pair in the side-by-side view, including optional line numbers and row kind.
In Code: `src/model.rs` (`AlignedRow`)
Synonyms: diff row
Forbidden Alternate Meaning: Patch hunk header

### Row Kind
Definition: Semantic row classification (`Equal`, `Changed`, `Insert`, `Delete`) used for color and navigation logic.
In Code: `src/model.rs` (`RowKind`)
Synonyms: row type
Forbidden Alternate Meaning: Git status code

### Change Block
Definition: One contiguous run of non-`Equal` aligned rows used for `n`/`N` navigation.
In Code: `src/app.rs` (`change_block_starts`)
Synonyms: edit block
Forbidden Alternate Meaning: Entire file-level change

### Viewport Rows
Definition: Number of vertical rows available for diff content after accounting for widget borders.
In Code: `src/app.rs` (`viewport_rows`), `src/ui.rs` (`viewport_rows`)
Synonyms: visible rows
Forbidden Alternate Meaning: Total file line count
