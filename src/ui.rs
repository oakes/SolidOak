extern crate rgtk;

use rgtk::*;
use std::collections::HashSet;
use std::io::fs::PathExtensions;
use std::io::fs;

fn sort_paths(paths: &Vec<Path>) -> Vec<Path> {
    let mut paths_vec = paths.clone();
    paths_vec.sort_by(|a, b| {
        let leaf_a = a.filename_str();
        let leaf_b = b.filename_str();

        leaf_a.cmp(&leaf_b)
    });
    paths_vec
}

fn sort_string_paths(paths: &HashSet<String>) -> Vec<String> {
    let mut paths_vec : Vec<String> = paths.clone().into_iter().collect();
    paths_vec.sort_by(|a, b| {
        let path_a = Path::new(a);
        let leaf_a = path_a.filename_str();

        let path_b = Path::new(b);
        let leaf_b = path_b.filename_str();

        leaf_a.cmp(&leaf_b)
    });
    paths_vec
}

fn update_project_tree_node(
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

            if node.is_dir() {
                match fs::readdir(node) {
                    Ok(children) => {
                        for child in sort_paths(&children).iter() {
                            update_project_tree_node(state, child, Some(&iter));
                        }
                    },
                    Err(e) => println!("Error updating tree: {}", e)
                }
            }
        }
    }

    iter.drop();
}

pub fn update_project_tree(state: &::utils::State) {
    state.tree_store.clear();

    for path_str in sort_string_paths(&state.projects).iter() {
        let path = Path::new(path_str);
        update_project_tree_node(state, &path, None);
    }
}
