use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct FileStatus {
    pub staged: bool,
    pub unstaged: bool,
    pub untracked: bool,
}

impl FileStatus {
    pub fn indicator(&self) -> &'static str {
        match (self.staged, self.unstaged, self.untracked) {
            (_, _, true) => "[N]",
            (true, true, _) => "[SU]",
            (true, false, _) => "[S]",
            (false, true, _) => "[U]",
            (false, false, false) => "[ ]",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub path: PathBuf,
    pub status: FileStatus,
    pub old_content: Option<ContentData>,
    pub new_content: Option<ContentData>,
    pub aligned_rows: Option<Vec<AlignedRow>>,
}

impl ChangedFile {
    pub fn new(path: PathBuf, status: FileStatus) -> Self {
        Self {
            path,
            status,
            old_content: None,
            new_content: None,
            aligned_rows: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContentData {
    Text(String),
    Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowKind {
    Equal,
    Changed,
    Insert,
    Delete,
}

#[derive(Debug, Clone)]
pub struct AlignedRow {
    pub left_line_no: Option<usize>,
    pub right_line_no: Option<usize>,
    pub left_text: String,
    pub right_text: String,
    pub kind: RowKind,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
    pub file_index: Option<usize>,
}

impl TreeNode {
    pub fn root() -> Self {
        Self {
            name: String::new(),
            is_dir: true,
            children: Vec::new(),
            file_index: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TreeRow {
    pub depth: usize,
    pub label: String,
    pub is_dir: bool,
    pub file_index: Option<usize>,
}
