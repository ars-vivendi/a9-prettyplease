use syn::{Item, Stmt, UseTree};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseGroup {
    Std,
    External,
    CrateLocal,
}

pub fn classify_use(item: &syn::ItemUse) -> UseGroup {
    let root_ident = use_tree_root_ident(&item.tree);
    match root_ident.as_deref() {
        Some("std" | "alloc" | "core") => UseGroup::Std,
        Some("crate" | "super" | "self") => UseGroup::CrateLocal,
        _ => UseGroup::External,
    }
}

fn use_tree_root_ident(tree: &UseTree) -> Option<String> {
    match tree {
        UseTree::Path(path) => Some(path.ident.to_string()),
        UseTree::Name(name) => Some(name.ident.to_string()),
        UseTree::Rename(rename) => Some(rename.ident.to_string()),
        UseTree::Glob(_) => None,
        UseTree::Group(group) => group.items.first().and_then(use_tree_root_ident),
    }
}

pub fn should_blank_between_items(prev: &Item, next: &Item) -> bool {
    match (prev, next) {
        (Item::Use(prev_use), Item::Use(next_use)) => {
            let prev_group = classify_use(prev_use);
            let next_group = classify_use(next_use);
            prev_group != next_group
        }
        _ => true,
    }
}

/// Returns a Vec of length `stmts.len()` where `result[i]` is true
/// if a blank line should be inserted BEFORE `stmts[i]`.
/// `result[0]` is always false (no blank line before the first statement).
pub fn stmt_blank_lines(stmts: &[Stmt]) -> Vec<bool> {
    let len = stmts.len();
    let mut blanks = vec![false; len];
    if len <= 1 {
        return blanks;
    }
    for i in 1..len {
        let prev = &stmts[i - 1];
        let next = &stmts[i];
        blanks[i] = match (prev, next) {
            (Stmt::Local(_), Stmt::Local(_)) => false,
            _ => true,
        };
    }
    blanks
}
