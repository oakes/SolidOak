extern crate rgtk;

use rgtk::*;
use std::io::fs::PathExtensions;

pub fn new_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    let chooser = gtk::FileChooserDialog::new(
        "New Project",
        None,
        gtk::FileChooserAction::Save).unwrap();
    chooser.run();
    match chooser.get_filename() {
        Some(filename_str) => {
            state.projects.insert(filename_str);
            ::ui::update_project_tree(tree, state);
            ::utils::write_prefs(state);
        },
        None => {}
    };
    chooser.destroy();
}

pub fn import_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    let chooser = gtk::FileChooserDialog::new(
        "Import",
        None,
        gtk::FileChooserAction::SelectFolder).unwrap();
    chooser.run();
    match chooser.get_filename() {
        Some(filename_str) => {
            state.projects.insert(filename_str);
            ::ui::update_project_tree(tree, state);
            ::utils::write_prefs(state);
        },
        None => {}
    };
    chooser.destroy();
}

pub fn rename_file(state: &mut ::utils::State) {
    match ::utils::get_selected_path(state) {
        Some(_) => {},
        None => {}
    };
}

pub fn remove_item(state: &mut ::utils::State) {
    match ::utils::get_selected_path(state) {
        Some(_) => {},
        None => {}
    };
}

pub fn update_selection(state: &mut ::utils::State) {
    state.selection = ::utils::get_selected_path(state);
    ::utils::write_prefs(state);
    ::ui::update_project_buttons(state);
}

pub fn remove_expansion(state: &mut ::utils::State, iter: &gtk::TreeIter) {
    match state.tree_model.get_value(iter, 1).get_string() {
        Some(path_str) => {
            for p in state.expansions.clone().iter() {
                if *p == path_str ||
                    !Path::new(p).exists() ||
                    (p.starts_with(path_str.as_slice()) &&
                    !::utils::are_siblings(&path_str, p))
                {
                    state.expansions.remove(p);
                }
            }
            ::utils::write_prefs(state);
        },
        None => {}
    };
}

pub fn add_expansion(state: &mut ::utils::State, iter: &gtk::TreeIter) {
    match state.tree_model.get_value(iter, 1).get_string() {
        Some(path_str) => {
            state.expansions.insert(path_str);
            ::utils::write_prefs(state);
        },
        None => {}
    };
}
