use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};

use crate::model::{ChangedFile, ContentData, FileStatus};

pub fn repo_root() -> Result<PathBuf> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to run git rev-parse")?;

    if !out.status.success() {
        return Err(anyhow!("not inside a git repository"));
    }

    let root = String::from_utf8(out.stdout).context("git output was not valid utf-8")?;
    Ok(PathBuf::from(root.trim()))
}

pub fn collect_changed_files(repo_root: &Path) -> Result<Vec<ChangedFile>> {
    let out = Command::new("git")
        .arg("status")
        .arg("--porcelain=v2")
        .arg("--untracked-files=all")
        .arg("-z")
        .current_dir(repo_root)
        .output()
        .context("failed to run git status")?;

    if !out.status.success() {
        return Err(anyhow!("git status failed"));
    }

    let fields: Vec<&[u8]> = out
        .stdout
        .split(|b| *b == 0)
        .filter(|entry| !entry.is_empty())
        .collect();

    let mut files = Vec::new();
    let mut i = 0usize;
    while i < fields.len() {
        let entry = fields[i];
        if entry.starts_with(b"#") {
            i += 1;
            continue;
        }

        let text = String::from_utf8_lossy(entry);

        if text.starts_with("? ") {
            let path = text[2..].to_string();
            if path.ends_with('/') {
                i += 1;
                continue;
            }
            files.push(ChangedFile::new(
                PathBuf::from(path),
                FileStatus {
                    staged: false,
                    unstaged: true,
                    untracked: true,
                },
            ));
            i += 1;
            continue;
        }

        if text.starts_with("1 ") || text.starts_with("2 ") || text.starts_with("u ") {
            let parts: Vec<&str> = text.split_whitespace().collect();
            if parts.len() < 2 {
                i += 1;
                continue;
            }

            let xy = parts[1];
            let (staged, unstaged) = parse_xy(xy);

            let path = if text.starts_with("2 ") {
                // For rename/copy entries in -z mode, current path is in this entry and
                // original path is the next NUL field.
                let p = parts.last().copied().unwrap_or("");
                i += 1; // consume current entry
                if i < fields.len() {
                    i += 1; // consume orig path entry
                }
                p.to_string()
            } else {
                let p = parts.last().copied().unwrap_or("").to_string();
                i += 1;
                p
            };

            if !path.is_empty() {
                files.push(ChangedFile::new(
                    PathBuf::from(path),
                    FileStatus {
                        staged,
                        unstaged,
                        untracked: false,
                    },
                ));
            }
            continue;
        }

        i += 1;
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}

pub fn load_file_contents(repo_root: &Path, file: &mut ChangedFile) -> Result<()> {
    let old = if file.status.untracked {
        ContentData::Text(String::new())
    } else {
        read_head_content(repo_root, &file.path)?
    };
    let new = read_worktree_content(repo_root, &file.path)?;

    file.old_content = Some(old);
    file.new_content = Some(new);
    Ok(())
}

fn parse_xy(xy: &str) -> (bool, bool) {
    let mut chars = xy.chars();
    let x = chars.next().unwrap_or('.');
    let y = chars.next().unwrap_or('.');

    let staged = x != '.' && x != ' ';
    let unstaged = y != '.' && y != ' ';
    (staged, unstaged)
}

fn read_head_content(repo_root: &Path, path: &Path) -> Result<ContentData> {
    let spec = format!("HEAD:{}", path.to_string_lossy());
    let out = Command::new("git")
        .arg("show")
        .arg(spec)
        .current_dir(repo_root)
        .output()
        .context("failed to run git show")?;

    if !out.status.success() {
        // File may not exist at HEAD (e.g. newly added file in index only).
        return Ok(ContentData::Text(String::new()));
    }

    Ok(bytes_to_content(out.stdout))
}

fn read_worktree_content(repo_root: &Path, path: &Path) -> Result<ContentData> {
    let full = repo_root.join(path);
    if full.is_dir() {
        return Ok(ContentData::Text("[directory]".to_string()));
    }
    match std::fs::read(&full) {
        Ok(bytes) => Ok(bytes_to_content(bytes)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            Ok(ContentData::Text(String::new()))
        }
        Err(err) => Err(err).with_context(|| format!("failed to read {:?}", full)),
    }
}

fn bytes_to_content(bytes: Vec<u8>) -> ContentData {
    match String::from_utf8(bytes) {
        Ok(text) => ContentData::Text(text),
        Err(_) => ContentData::Binary,
    }
}
