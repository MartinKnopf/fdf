use std::path::PathBuf;

use anyhow::Result;

use crate::diff::align_full_file;
use crate::git;
use crate::input::Action;
use crate::model::{AlignedRow, ChangedFile, ContentData, TreeRow};
use crate::tree;

pub struct App {
    pub repo_root: PathBuf,
    pub files: Vec<ChangedFile>,
    pub tree_rows: Vec<TreeRow>,
    pub show_tree: bool,
    pub tree_h_scroll: usize,
    pub selected_file_idx: usize,
    pub v_scroll: usize,
    pub h_scroll: usize,
    pub viewport_rows: usize,
    pub highlight_epoch: u64,
    pub g_prefix_pending: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(repo_root: PathBuf) -> Result<Self> {
        let files = git::collect_changed_files(&repo_root)?;
        let tree = tree::build_tree(&files);
        let tree_rows = tree::flatten_tree(&tree, &files);

        let mut app = Self {
            repo_root,
            files,
            tree_rows,
            show_tree: true,
            tree_h_scroll: 0,
            selected_file_idx: 0,
            v_scroll: 0,
            h_scroll: 0,
            viewport_rows: 1,
            highlight_epoch: 0,
            g_prefix_pending: false,
            should_quit: false,
        };

        if !app.files.is_empty() {
            app.ensure_selected_loaded()?;
        }

        Ok(app)
    }

    pub fn on_action(&mut self, action: Action) -> Result<()> {
        if matches!(action, Action::PrefixG) {
            if self.g_prefix_pending {
                self.go_top();
                self.g_prefix_pending = false;
            } else {
                self.g_prefix_pending = true;
            }
            self.clamp_scroll();
            return Ok(());
        }

        if !matches!(action, Action::None) {
            self.g_prefix_pending = false;
        }

        match action {
            Action::SelectPrevFile => self.select_prev_file()?,
            Action::SelectNextFile => self.select_next_file()?,
            Action::ToggleTree => {
                self.show_tree = !self.show_tree;
            }
            Action::Refresh => self.refresh()?,
            Action::TreeScrollLeft => {
                self.tree_h_scroll = self.tree_h_scroll.saturating_sub(1);
            }
            Action::TreeScrollRight => {
                self.tree_h_scroll = self.tree_h_scroll.saturating_add(1);
            }
            Action::ScrollDown => {
                self.v_scroll = self.v_scroll.saturating_add(1);
            }
            Action::ScrollUp => {
                self.v_scroll = self.v_scroll.saturating_sub(1);
            }
            Action::PageDown => self.page_down(),
            Action::PageUp => self.page_up(),
            Action::ScrollLeft => {
                self.h_scroll = self.h_scroll.saturating_sub(1);
            }
            Action::ScrollRight => {
                self.h_scroll = self.h_scroll.saturating_add(1);
            }
            Action::GoBottom => self.go_bottom(),
            Action::NextChange => self.jump_next_change(),
            Action::PrevChange => self.jump_prev_change(),
            Action::Quit => self.should_quit = true,
            Action::None => {}
            Action::PrefixG => {}
        }
        self.clamp_scroll();
        Ok(())
    }

    pub fn selected_file(&self) -> Option<&ChangedFile> {
        self.files.get(self.selected_file_idx)
    }

    pub fn selected_rows(&self) -> Option<&Vec<AlignedRow>> {
        self.selected_file().and_then(|f| f.aligned_rows.as_ref())
    }

    pub fn set_viewport_rows(&mut self, rows: usize) {
        self.viewport_rows = rows.max(1);
        self.clamp_scroll();
    }

    fn select_prev_file(&mut self) -> Result<()> {
        if self.files.is_empty() {
            return Ok(());
        }

        let file_indices: Vec<usize> = self.tree_rows.iter().filter_map(|r| r.file_index).collect();

        if file_indices.is_empty() {
            return Ok(());
        }

        let current_pos = file_indices
            .iter()
            .position(|&idx| idx == self.selected_file_idx);

        let new_pos = match current_pos {
            Some(0) => file_indices.len() - 1,
            Some(pos) => pos - 1,
            None => file_indices.len() - 1,
        };

        self.selected_file_idx = file_indices[new_pos];
        self.reset_scroll();
        self.ensure_selected_loaded()
    }

    fn select_next_file(&mut self) -> Result<()> {
        if self.files.is_empty() {
            return Ok(());
        }

        let file_indices: Vec<usize> = self.tree_rows.iter().filter_map(|r| r.file_index).collect();

        if file_indices.is_empty() {
            return Ok(());
        }

        let current_pos = file_indices
            .iter()
            .position(|&idx| idx == self.selected_file_idx);

        let new_pos = match current_pos {
            Some(pos) => (pos + 1) % file_indices.len(),
            None => 0,
        };

        self.selected_file_idx = file_indices[new_pos];
        self.reset_scroll();
        self.ensure_selected_loaded()
    }

    fn ensure_selected_loaded(&mut self) -> Result<()> {
        if self.files.is_empty() {
            return Ok(());
        }

        let idx = self.selected_file_idx;
        let file = &mut self.files[idx];

        if file.old_content.is_none() || file.new_content.is_none() {
            git::load_file_contents(&self.repo_root, file)?;
        }

        if file.aligned_rows.is_none() {
            let rows = match (&file.old_content, &file.new_content) {
                (Some(ContentData::Text(old)), Some(ContentData::Text(new))) => {
                    align_full_file(old, new)
                }
                _ => vec![AlignedRow {
                    left_line_no: None,
                    right_line_no: None,
                    left_text: "[binary or non-utf8 file]".to_string(),
                    right_text: "[binary or non-utf8 file]".to_string(),
                    kind: crate::model::RowKind::Changed,
                }],
            };
            file.aligned_rows = Some(rows);
        }

        self.clamp_scroll();
        Ok(())
    }

    fn refresh(&mut self) -> Result<()> {
        let files = git::collect_changed_files(&self.repo_root)?;
        self.apply_refreshed_files(files);
        self.ensure_selected_loaded()
    }

    fn apply_refreshed_files(&mut self, files: Vec<ChangedFile>) {
        let previous_selected_path = self.selected_file().map(|file| file.path.clone());
        let previous_selected_idx = self.selected_file_idx;

        let tree = tree::build_tree(&files);
        let tree_rows = tree::flatten_tree(&tree, &files);

        self.files = files;
        self.tree_rows = tree_rows;
        self.highlight_epoch = self.highlight_epoch.wrapping_add(1);

        if self.files.is_empty() {
            self.selected_file_idx = 0;
            self.reset_scroll();
            return;
        }

        let next_selected_idx = previous_selected_path
            .as_ref()
            .and_then(|path| self.files.iter().position(|file| &file.path == path))
            .unwrap_or_else(|| previous_selected_idx.min(self.files.len() - 1));

        self.selected_file_idx = next_selected_idx;

        let selection_preserved = previous_selected_path
            .as_ref()
            .map(|path| self.files[next_selected_idx].path == *path)
            .unwrap_or(false);

        if !selection_preserved {
            self.reset_scroll();
        }
    }

    fn reset_scroll(&mut self) {
        self.v_scroll = 0;
        self.h_scroll = 0;
    }

    fn page_down(&mut self) {
        let step = (self.viewport_rows / 2).max(1);
        self.v_scroll = self.v_scroll.saturating_add(step);
    }

    fn page_up(&mut self) {
        let step = (self.viewport_rows / 2).max(1);
        self.v_scroll = self.v_scroll.saturating_sub(step);
    }

    fn go_top(&mut self) {
        self.v_scroll = 0;
    }

    fn go_bottom(&mut self) {
        self.v_scroll = self.max_v_scroll();
    }

    fn jump_next_change(&mut self) {
        let Some(rows) = self.selected_rows() else {
            return;
        };
        let starts = change_block_starts(rows);
        if starts.is_empty() {
            return;
        }

        if let Some(next) = starts.iter().copied().find(|idx| *idx > self.v_scroll) {
            self.v_scroll = next;
        } else if let Some(first) = starts.first().copied() {
            self.v_scroll = first;
        }
    }

    fn jump_prev_change(&mut self) {
        let Some(rows) = self.selected_rows() else {
            return;
        };
        let starts = change_block_starts(rows);
        if starts.is_empty() {
            return;
        }

        if let Some(prev) = starts
            .iter()
            .copied()
            .rev()
            .find(|idx| *idx < self.v_scroll)
        {
            self.v_scroll = prev;
        } else if let Some(last) = starts.last().copied() {
            self.v_scroll = last;
        }
    }

    fn clamp_scroll(&mut self) {
        let max_scroll = self.max_v_scroll();
        self.v_scroll = self.v_scroll.min(max_scroll);
    }

    fn max_v_scroll(&self) -> usize {
        let total_rows = self.selected_rows().map(|r| r.len()).unwrap_or(0);
        total_rows.saturating_sub(self.viewport_rows)
    }
}

fn change_block_starts(rows: &[AlignedRow]) -> Vec<usize> {
    let mut starts = Vec::new();
    let mut in_change = false;

    for (idx, row) in rows.iter().enumerate() {
        if row.kind != crate::model::RowKind::Equal {
            if !in_change {
                starts.push(idx);
            }
            in_change = true;
        } else {
            in_change = false;
        }
    }

    starts
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::App;
    use crate::input::Action;
    use crate::model::{ChangedFile, FileStatus};

    fn app_for_test() -> App {
        App {
            repo_root: PathBuf::new(),
            files: Vec::new(),
            tree_rows: Vec::new(),
            show_tree: true,
            tree_h_scroll: 0,
            selected_file_idx: 0,
            v_scroll: 0,
            h_scroll: 0,
            viewport_rows: 1,
            highlight_epoch: 0,
            g_prefix_pending: false,
            should_quit: false,
        }
    }

    fn changed_file(path: &str) -> ChangedFile {
        ChangedFile::new(
            PathBuf::from(path),
            FileStatus {
                staged: false,
                unstaged: true,
                untracked: false,
            },
        )
    }

    #[test]
    fn toggle_tree_action_flips_visibility_flag() {
        let mut app = app_for_test();

        app.on_action(Action::ToggleTree)
            .expect("toggle action should succeed");
        assert!(!app.show_tree);

        app.on_action(Action::ToggleTree)
            .expect("toggle action should succeed");
        assert!(app.show_tree);
    }

    #[test]
    fn tree_horizontal_scroll_actions_adjust_tree_offset() {
        let mut app = app_for_test();

        app.on_action(Action::TreeScrollRight)
            .expect("right tree scroll should succeed");
        app.on_action(Action::TreeScrollRight)
            .expect("right tree scroll should succeed");
        assert_eq!(app.tree_h_scroll, 2);

        app.on_action(Action::TreeScrollLeft)
            .expect("left tree scroll should succeed");
        app.on_action(Action::TreeScrollLeft)
            .expect("left tree scroll should succeed");
        app.on_action(Action::TreeScrollLeft)
            .expect("left tree scroll should saturate at zero");
        assert_eq!(app.tree_h_scroll, 0);
    }

    #[test]
    fn apply_refreshed_files_preserves_selection_by_path() {
        let mut app = app_for_test();
        app.files = vec![changed_file("a.rs"), changed_file("b.rs")];
        app.selected_file_idx = 1;
        app.v_scroll = 9;
        app.h_scroll = 4;

        app.apply_refreshed_files(vec![changed_file("b.rs"), changed_file("c.rs")]);

        assert_eq!(app.selected_file_idx, 0);
        assert_eq!(app.v_scroll, 9);
        assert_eq!(app.h_scroll, 4);
    }

    #[test]
    fn apply_refreshed_files_resets_scroll_if_selection_replaced() {
        let mut app = app_for_test();
        app.files = vec![changed_file("a.rs"), changed_file("b.rs")];
        app.selected_file_idx = 1;
        app.v_scroll = 9;
        app.h_scroll = 4;

        app.apply_refreshed_files(vec![changed_file("a.rs"), changed_file("c.rs")]);

        assert_eq!(app.selected_file_idx, 1);
        assert_eq!(app.v_scroll, 0);
        assert_eq!(app.h_scroll, 0);
    }

    #[test]
    fn apply_refreshed_files_bumps_highlight_epoch() {
        let mut app = app_for_test();
        app.highlight_epoch = u64::MAX;

        app.apply_refreshed_files(vec![changed_file("a.rs")]);

        assert_eq!(app.highlight_epoch, 0);
    }
}
