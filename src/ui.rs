extern crate rgtk;

use rgtk::*;

pub fn update_project_tree(state: &::utils::State) {
    let iter = gtk::TreeIter::new().unwrap();
    state.tree_store.append(&iter, None);
    state.tree_store.set_string(&iter, 0, "Hello, world!");

    let child = gtk::TreeIter::new().unwrap();
    state.tree_store.append(&child, Some(&iter));
    state.tree_store.set_string(&child, 0, "Bye, world!");
}
