extern crate rgtk;

use rgtk::*;
use std::io::fs::PathExtensions;

fn save_project(
    state: &mut ::utils::State,
    tree: &mut gtk::TreeView,
    filename: String)
{
    state.projects.insert(filename.clone());
    state.selection = Some(filename.clone());
    ::utils::write_prefs(state);
    ::ui::update_project_tree(state, tree);
}

pub fn new_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    let chooser = gtk::FileChooserDialog::new(
        "New Project",
        None,
        gtk::FileChooserAction::Save
    ).unwrap();
    chooser.run();
    if let Some(filename) = chooser.get_filename() {
        save_project(state, tree, filename);
        // TODO: cargo new filename --bin
    }
    chooser.destroy();
}

pub fn import_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    let chooser = gtk::FileChooserDialog::new(
        "Import",
        None,
        gtk::FileChooserAction::SelectFolder
    ).unwrap();
    chooser.run();
    if let Some(filename) = chooser.get_filename() {
        save_project(state, tree, filename);
    }
    chooser.destroy();
}

pub fn rename_file(state: &mut ::utils::State) {
    if let Some(_) = ::utils::get_selected_path(state) {
        // TODO: show dialog with a text field
    }
}

pub fn remove_item(state: &mut ::utils::State) {
    if let Some(_) = ::utils::get_selected_path(state) {
        // TODO: show dialog with confirmation buttons
    }
}

pub fn save_selection(state: &mut ::utils::State) {
    let path = ::utils::get_selected_path(state);
    if path.is_some() {
        state.selection = path;
        ::utils::write_prefs(state);
        ::ui::update_project_buttons(state);
    }
}

pub fn remove_expansion(state: &mut ::utils::State, iter: &gtk::TreeIter) {
    if let Some(path_str) = state.tree_model.get_value(iter, 1).get_string() {
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
    }
}

pub fn add_expansion(state: &mut ::utils::State, iter: &gtk::TreeIter) {
    if let Some(path_str) = state.tree_model.get_value(iter, 1).get_string() {
        state.expansions.insert(path_str);
        ::utils::write_prefs(state);
    }
}
