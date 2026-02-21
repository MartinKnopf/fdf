use std::collections::BTreeMap;
use std::path::{Component, PathBuf};

use crate::model::{ChangedFile, TreeNode, TreeRow};

pub fn build_tree(files: &[ChangedFile]) -> TreeNode {
    let mut root = TreeNode::root();
    for (idx, file) in files.iter().enumerate() {
        insert_path(&mut root, &file.path, idx);
    }
    sort_node(&mut root);
    root
}

pub fn flatten_tree(root: &TreeNode, files: &[ChangedFile]) -> Vec<TreeRow> {
    let mut rows = Vec::new();
    flatten_recursive(root, 0, files, &mut rows);
    rows
}

fn flatten_recursive(
    node: &TreeNode,
    depth: usize,
    files: &[ChangedFile],
    rows: &mut Vec<TreeRow>,
) {
    for child in &node.children {
        let label = if let Some(file_idx) = child.file_index {
            let indicator = files
                .get(file_idx)
                .map(|f| f.status.indicator())
                .unwrap_or("[ ]");
            format!("{} {}", indicator, child.name)
        } else {
            format!("{}/", child.name)
        };

        rows.push(TreeRow {
            depth,
            label,
            is_dir: child.is_dir,
            file_index: child.file_index,
        });

        if child.is_dir {
            flatten_recursive(child, depth + 1, files, rows);
        }
    }
}

fn insert_path(root: &mut TreeNode, path: &PathBuf, file_idx: usize) {
    let mut current = root;
    let components: Vec<String> = path
        .components()
        .filter_map(|comp| match comp {
            Component::Normal(seg) => Some(seg.to_string_lossy().to_string()),
            _ => None,
        })
        .collect();

    for (i, name) in components.iter().enumerate() {
        let is_last = i + 1 == components.len();

        if is_last {
            current.children.push(TreeNode {
                name: name.clone(),
                is_dir: false,
                children: Vec::new(),
                file_index: Some(file_idx),
            });
            return;
        }

        let existing_idx = current
            .children
            .iter()
            .position(|c| c.is_dir && c.name == *name)
            .unwrap_or_else(|| {
                current.children.push(TreeNode {
                    name: name.clone(),
                    is_dir: true,
                    children: Vec::new(),
                    file_index: None,
                });
                current.children.len() - 1
            });

        current = &mut current.children[existing_idx];
    }
}

fn sort_node(node: &mut TreeNode) {
    for child in &mut node.children {
        if child.is_dir {
            sort_node(child);
        }
    }

    let mut dirs = BTreeMap::new();
    let mut files = BTreeMap::new();

    for child in node.children.drain(..) {
        if child.is_dir {
            dirs.insert(child.name.clone(), child);
        } else {
            files.insert(child.name.clone(), child);
        }
    }

    node.children.extend(dirs.into_values());
    node.children.extend(files.into_values());
}
