extern crate rgtk;

use rgtk::*;
use std::io::fs::PathExtensions;
use std::io::fs;

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
                        for child in children.iter() {
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

    for path_str in state.projects.iter() {
        let path = Path::new(path_str);
        update_project_tree_node(state, &path, None);
    }
}
