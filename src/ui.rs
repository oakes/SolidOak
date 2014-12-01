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
    state: &mut ::utils::State,
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

            //let tree_path = state.tree_model.get_path(&iter).unwrap();
            //state.project_tree.expand_row(&tree_path, false);

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

pub fn update_project_tree(state: &mut ::utils::State) {
    state.tree_store.clear();

    for path in sort_string_paths(&state.projects).iter() {
        add_node(state, path, None);
    }

    if state.selection.is_none() {
        let path = get_first_path(state);
        state.project_tree.set_cursor(&path.unwrap(), None, false);
    }

    update_project_buttons(state);
}
