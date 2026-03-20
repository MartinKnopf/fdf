use std::path::PathBuf;

use anyhow::Result;

use crate::comments::{self, Comments};
use crate::diff::align_full_file;
use crate::git;
use crate::input::Action;
use crate::model::{AlignedRow, ChangedFile, ContentData, TreeRow};
use crate::tree;

pub struct ShellCommand {
    pub args: Vec<String>,
    pub wait_for_key: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingAction {
    Checkout(usize),
    Delete(usize),
}

pub struct App {
    pub repo_root: PathBuf,
    pub files: Vec<ChangedFile>,
    pub tree_rows: Vec<TreeRow>,
    pub show_tree: bool,
    pub show_comments: bool,
    pub comments: Comments,
    pub comment_wrap_width: usize,
    pub tree_h_scroll: usize,
    pub selected_file_idx: usize,
    pub v_scroll: usize,
    pub h_scroll: usize,
    pub viewport_rows: usize,
    pub highlight_epoch: u64,
    pub g_prefix_pending: bool,
    pub show_help: bool,
    pub pending_confirm: Option<PendingAction>,
    pub should_quit: bool,
    pub shell_command: Option<ShellCommand>,
    pub use_difft: bool,
}

impl App {
    pub fn new(repo_root: PathBuf, comments: Comments, use_difft: bool) -> Result<Self> {
        let files = git::collect_changed_files(&repo_root)?;
        let tree = tree::build_tree(&files);
        let tree_rows = tree::flatten_tree(&tree, &files);

        let use_difft = use_difft && crate::difft::is_available();

        let mut app = Self {
            repo_root,
            files,
            tree_rows,
            show_tree: true,
            show_comments: false,
            comments,
            comment_wrap_width: 0,
            tree_h_scroll: 0,
            selected_file_idx: 0,
            v_scroll: 0,
            h_scroll: 0,
            viewport_rows: 1,
            highlight_epoch: 0,
            g_prefix_pending: false,
            show_help: false,
            pending_confirm: None,
            should_quit: false,
            shell_command: None,
            use_difft,
        };

        if !app.files.is_empty() {
            app.ensure_selected_loaded()?;
        }

        Ok(app)
    }

    pub fn on_action(&mut self, action: Action) -> Result<()> {
        // While help overlay is shown, only allow closing it.
        if self.show_help {
            match action {
                Action::CloseOverlay | Action::ShowHelp | Action::Quit => {
                    self.show_help = false;
                }
                _ => {}
            }
            return Ok(());
        }

        // While a destructive confirmation is pending, y confirms, anything else cancels.
        if let Some(pending) = self.pending_confirm.take() {
            if matches!(action, Action::ConfirmYes) {
                match pending {
                    PendingAction::Checkout(idx) => {
                        if let Some(file) = self.files.get(idx) {
                            git::checkout_file(&self.repo_root, file)?;
                            self.refresh()?;
                        }
                    }
                    PendingAction::Delete(idx) => {
                        if let Some(file) = self.files.get(idx) {
                            let full_path = self.repo_root.join(&file.path);
                            std::fs::remove_file(&full_path)?;
                            self.refresh()?;
                        }
                    }
                }
            }
            return Ok(());
        }

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
            Action::ToggleComments => {
                if !self.comments.is_empty() {
                    self.show_comments = !self.show_comments;
                }
            }
            Action::Refresh => self.refresh()?,
            Action::ToggleStage => self.toggle_stage()?,
            Action::GitCommit => {
                self.shell_command = Some(ShellCommand {
                    args: vec!["git".into(), "commit".into()],
                    wait_for_key: false,
                });
            }
            Action::GitPull => {
                self.shell_command = Some(ShellCommand {
                    args: vec!["git".into(), "pull".into()],
                    wait_for_key: true,
                });
            }
            Action::GitPush => {
                self.shell_command = Some(ShellCommand {
                    args: vec!["git".into(), "push".into()],
                    wait_for_key: true,
                });
            }
            Action::TreeScrollLeft => {
                if self.show_tree {
                    self.tree_h_scroll = self.tree_h_scroll.saturating_sub(1);
                } else {
                    self.select_prev_file()?;
                }
            }
            Action::TreeScrollRight => {
                if self.show_tree {
                    self.tree_h_scroll = self.tree_h_scroll.saturating_add(1);
                } else {
                    self.select_next_file()?;
                }
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
            Action::CheckoutFile => {
                let idx = self.selected_file_idx;
                if let Some(file) = self.files.get(idx) {
                    if file.status.unstaged && !file.status.untracked {
                        self.pending_confirm = Some(PendingAction::Checkout(idx));
                    }
                }
            }
            Action::DeleteFile => {
                let idx = self.selected_file_idx;
                if self.files.get(idx).is_some() {
                    self.pending_confirm = Some(PendingAction::Delete(idx));
                }
            }
            Action::ConfirmYes => {}
            Action::ShowHelp => self.show_help = true,
            Action::CloseOverlay => {}
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
        self.selected_file().and_then(|f| {
            if self.show_comments {
                f.display_rows.as_ref()
            } else {
                f.aligned_rows.as_ref()
            }
        })
    }

    pub fn selected_comment_rows(&self) -> Option<&Vec<comments::CommentRow>> {
        self.selected_file().and_then(|f| f.comment_rows.as_ref())
    }

    pub fn set_viewport_rows(&mut self, rows: usize) {
        self.viewport_rows = rows.max(1);
        self.clamp_scroll();
    }

    /// Called by UI each frame to communicate the comment pane content width.
    /// Ensures the selected file's display cache is up-to-date.
    pub fn set_comment_pane_width(&mut self, width: usize) {
        self.comment_wrap_width = width;
        self.ensure_display_rows();
    }

    /// Ensure display_rows / comment_rows are computed for the selected file
    /// at the current wrap width. Recomputes only when the wrap width changed
    /// or the cache is empty, so switching back to a previously viewed file is
    /// effectively free and the highlight cache still hits (stable pointer).
    fn ensure_display_rows(&mut self) {
        let idx = self.selected_file_idx;
        let wrap = self.comment_wrap_width;
        let Some(file) = self.files.get_mut(idx) else {
            return;
        };

        // Already computed at this width — nothing to do.
        if file.display_rows.is_some() && file.display_wrap_width == wrap {
            return;
        }

        let Some(ref aligned) = file.aligned_rows else {
            return;
        };

        let file_comments = self.comments.for_file(&file.path);
        let (display, crows) = comments::expand_rows_with_comments(aligned, file_comments, wrap);

        file.display_rows = Some(display);
        file.comment_rows = Some(crows);
        file.display_wrap_width = wrap;
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

        let new_idx = file_indices[new_pos];
        if new_idx == self.selected_file_idx {
            return Ok(());
        }
        self.selected_file_idx = new_idx;
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

        let new_idx = file_indices[new_pos];
        if new_idx == self.selected_file_idx {
            return Ok(());
        }
        self.selected_file_idx = new_idx;
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
                    let mut rows = align_full_file(old, new);
                    if self.use_difft {
                        let _ = crate::difft::enrich_rows(&mut rows, old, new, &file.path);
                    }
                    rows
                }
                _ => vec![AlignedRow {
                    left_line_no: None,
                    right_line_no: None,
                    left_text: "[binary or non-utf8 file]".to_string(),
                    right_text: "[binary or non-utf8 file]".to_string(),
                    kind: crate::model::RowKind::Changed,
                    left_changed_ranges: Vec::new(),
                    right_changed_ranges: Vec::new(),
                }],
            };
            file.aligned_rows = Some(rows);
        }

        self.clamp_scroll();
        Ok(())
    }

    fn toggle_stage(&mut self) -> Result<()> {
        if let Some(file) = self.files.get(self.selected_file_idx) {
            git::toggle_stage(&self.repo_root, file)?;
            self.refresh()?;
        }
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

    fn center_on_row(&mut self, row: usize) {
        let half = self.viewport_rows / 2;
        self.v_scroll = row.saturating_sub(half).min(self.max_v_scroll());
    }

    fn jump_next_change(&mut self) {
        let Some(rows) = self.selected_rows() else {
            return;
        };
        let starts = change_block_starts(rows);
        if starts.is_empty() {
            return;
        }

        let center = self.v_scroll + self.viewport_rows / 2;
        if let Some(next) = starts.iter().copied().find(|idx| *idx > center) {
            self.center_on_row(next);
        } else if let Some(first) = starts.first().copied() {
            self.center_on_row(first);
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

        let center = self.v_scroll + self.viewport_rows / 2;
        if let Some(prev) = starts
            .iter()
            .copied()
            .rev()
            .find(|idx| *idx < center)
        {
            self.center_on_row(prev);
        } else if let Some(last) = starts.last().copied() {
            self.center_on_row(last);
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
            show_comments: false,
            comments: crate::comments::Comments::default(),
            comment_wrap_width: 0,
            tree_h_scroll: 0,
            selected_file_idx: 0,
            v_scroll: 0,
            h_scroll: 0,
            viewport_rows: 1,
            highlight_epoch: 0,
            g_prefix_pending: false,
            show_help: false,
            pending_confirm: None,
            should_quit: false,
            shell_command: None,
            use_difft: false,
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
