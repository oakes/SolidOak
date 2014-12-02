extern crate rgtk;

use rgtk::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::fs;
use std::io::fs::PathExtensions;

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

fn get_first_path(state: &::utils::State) -> Option<gtk::TreePath> {
    let mut iter = gtk::TreeIter::new().unwrap();
    let model = state.tree_store.get_model().unwrap();

    model.get_iter_first(&mut iter);

    let path = model.get_path(&iter);
    iter.drop();
    path
}

pub fn update_project_buttons(state: &::utils::State) {
    state.rename_button.set_sensitive(state.selection.is_some());
    state.remove_button.set_sensitive(state.selection.is_some());
}

fn add_node(
    state: &::utils::State,
    node: &Path,
    parent: Option<&gtk::TreeIter>)
{
    let mut iter = gtk::TreeIter::new().unwrap();

    let leaf = node.filename_str();
    if leaf.is_some() {
        let leaf_str = leaf.unwrap();
        if !leaf_str.starts_with(".") {
            state.tree_store.append(&iter, parent);
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

    iter.drop();
}

fn expand_nodes(tree: &mut gtk::TreeView, state: &mut ::utils::State, parent: Option<&gtk::TreeIter>) {
    if parent.is_none() {
        tree.expand_all();
    }

    let mut iter = gtk::TreeIter::new().unwrap();

    if state.tree_model.iter_children(&mut iter, parent) {
        loop {
            let path = state.tree_model.get_value(&iter, 1).get_string();
            if path.is_some() {
                let path_str = path.unwrap();

                if state.selection.is_some() {
                    let selection_str = state.selection.clone().unwrap();
                    if path_str == selection_str {
                        let path = state.tree_model.get_path(&iter);
                        tree.set_cursor(&path.unwrap(), None, false);
                    }
                }

                if state.expansions.contains(&path_str) {
                    expand_nodes(tree, state, Some(&iter));
                } else {
                    let tree_path = state.tree_model.get_path(&iter).unwrap();
                    tree.collapse_row(&tree_path);
                }
            }

            if !state.tree_model.iter_next(&mut iter) {
                break;
            }
        }
    }

    iter.drop();
}

pub fn update_project_tree(tree: &mut gtk::TreeView, state: &mut ::utils::State) {
    state.tree_store.clear();

    for path in sort_string_paths(&state.projects).iter() {
        add_node(state, path, None);
    }

    expand_nodes(tree, state, None);

    let mut iter = gtk::TreeIter::new().unwrap();
    if !state.tree_selection.get_selected(state.tree_model, &mut iter) {
        let path = get_first_path(state);
        tree.set_cursor(&path.unwrap(), None, false);
    }
    iter.drop();

    update_project_buttons(state);
}
