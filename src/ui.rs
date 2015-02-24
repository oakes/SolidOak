use rgtk::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::old_io::fs;
use std::old_io::fs::PathExtensions;

fn path_sorter(a: &Path, b: &Path) -> Ordering {
    let leaf_a = a.filename_str();
    let leaf_b = b.filename_str();
    leaf_a.cmp(&leaf_b)
}

fn sort_paths(paths: &Vec<Path>) -> Vec<Path> {
    let mut paths_vec = paths.clone();
    paths_vec.sort_by(path_sorter);
    paths_vec
}

fn sort_string_paths(paths: &HashSet<String>) -> Vec<Path> {
    let mut paths_vec = Vec::new();
    for path in paths.iter() {
        paths_vec.push(Path::new(path));
    }
    paths_vec.sort_by(path_sorter);
    paths_vec
}

pub fn update_project_buttons(state: &::utils::State) {
    if let Some(path_str) = ::utils::get_selected_path(state) {
        let is_project = state.projects.contains(&path_str);
        let path = Path::new(path_str);
        state.rename_button.set_sensitive(!path.is_dir());
        state.remove_button.set_sensitive(!path.is_dir() || is_project);
    } else {
        state.rename_button.set_sensitive(false);
        state.remove_button.set_sensitive(false);
    }
}

fn add_node(state: &::utils::State, node: &Path, parent: Option<&gtk::TreeIter>) {
    let mut iter = gtk::TreeIter::new().unwrap();

    if let Some(leaf_str) = node.filename_str() {
        if !leaf_str.starts_with(".") {
            state.tree_store.append(&mut iter, parent);
            state.tree_store.set_string(&iter, 0, leaf_str);
            state.tree_store.set_string(&iter, 1, node.as_str().unwrap());

            if node.is_dir() {
                match fs::readdir(node) {
                    Ok(children) => {
                        for child in sort_paths(&children).iter() {
                            add_node(state, child, Some(&iter));
                        }
                    },
                    Err(e) => println!("Error updating tree: {}", e)
                }
            }
        }
    }
}

fn expand_nodes(
    state: &mut ::utils::State,
    tree: &mut gtk::TreeView,
    parent: Option<&gtk::TreeIter>)
{
    let mut iter = gtk::TreeIter::new().unwrap();

    if state.tree_model.iter_children(&mut iter, parent) {
        loop {
            if let Some(path_str) = state.tree_model.get_value(&iter, 1).get_string() {
                if let Some(selection_str) = state.selection.clone() {
                    if path_str == selection_str {
                        if let Some(path) = state.tree_model.get_path(&iter) {
                            tree.set_cursor(&path, None, false);
                        }
                    }
                }

                if state.expansions.contains(&path_str) {
                    if let Some(path) = state.tree_model.get_path(&iter) {
                        tree.expand_row(&path, false);
                        expand_nodes(state, tree, Some(&iter));
                    }
                }
            }

            if !state.tree_model.iter_next(&mut iter) {
                break;
            }
        }
    }
}

pub fn update_project_tree(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    state.is_refreshing_tree = true;

    state.tree_store.clear();

    for path in sort_string_paths(&state.projects).iter() {
        add_node(state, path, None);
    }

    expand_nodes(state, tree, None);

    update_project_buttons(state);

    state.is_refreshing_tree = false;
}
