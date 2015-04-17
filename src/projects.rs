use gtk::traits::*;
use gtk::{self, widgets};
use std::fs::PathExt;
use std::num::FromPrimitive;
use std::path::Path;
use std::process::Command;

fn remove_expansions_for_path(state: &mut ::utils::State, path_str: &String) {
    for expansion_str in state.expansions.clone().iter() {
        if !Path::new(expansion_str).exists() ||
            path_str == expansion_str ||
            ::utils::is_parent_path(path_str, expansion_str)
        {
            state.expansions.remove(expansion_str);
        }
    }
}

fn save_project(state: &mut ::utils::State, tree: &mut widgets::TreeView, path_str: &String) {
    state.projects.insert(path_str.clone());
    ::utils::write_prefs(state);
    ::ui::update_project_tree(state, tree);
}

pub fn new_project(state: &mut ::utils::State, tree: &mut widgets::TreeView) {
    let dialog = widgets::FileChooserDialog::new(
        "New Project",
        None,
        gtk::FileChooserAction::Save,
        [("Save", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]
    );
    if let Some(gtk::ResponseType::Ok) = FromPrimitive::from_i32(dialog.run()) {
        if let Some(path_str) = dialog.get_filename() {
            let path_ref: &str = path_str.as_ref();
            let path = Path::new(path_ref);
            if let Some(name_os_str) = path.file_name() {
                if let Some(name_str) = name_os_str.to_str() {
                    if let Some(parent_path) = path.parent() {
                        match Command::new("cargo").arg("new").arg(name_str).arg("--bin")
                            .current_dir(parent_path).status()
                        {
                            Ok(_) => save_project(state, tree, &path_str),
                            Err(e) => println!("Error creating {}: {}", name_str, e)
                        }
                    }
                }
            }
        }
    }
    dialog.destroy();
}

pub fn import_project(state: &mut ::utils::State, tree: &mut widgets::TreeView) {
    let dialog = widgets::FileChooserDialog::new(
        "Import",
        None,
        gtk::FileChooserAction::SelectFolder,
        [("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]
    );
    if let Some(gtk::ResponseType::Ok) = FromPrimitive::from_i32(dialog.run()) {
        if let Some(path_str) = dialog.get_filename() {
            save_project(state, tree, &path_str);
        }
    }
    dialog.destroy();
}

pub fn rename_file(state: &mut ::utils::State, fd: i32) {
    if let Some(_) = ::utils::get_selected_path(state) {
        let dialog = widgets::FileChooserDialog::new(
            "Rename",
            None,
            gtk::FileChooserAction::Save,
            [("Save", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]
        );
        if let Some(gtk::ResponseType::Ok) = FromPrimitive::from_i32(dialog.run()) {
            if let Some(path_str) = dialog.get_filename() {
                state.selection = Some(path_str.clone());
                ::utils::write_prefs(&state);
                ::ffi::send_message(fd, format!("Move {}", path_str).as_ref());
            }
        }
        dialog.destroy();
    }
}

pub fn remove_item(state: &mut ::utils::State, tree: &mut widgets::TreeView, fd: i32) {
    if let Some(path_str) = ::utils::get_selected_path(state) {
        if let Some(dialog) = widgets::MessageDialog::new_with_markup(
            Some(state.window.clone()),
            gtk::DialogFlags::Modal,
            gtk::MessageType::Question,
            gtk::ButtonsType::OkCancel,
            if state.projects.contains(&path_str) {
                "Remove this project? It WILL NOT be deleted from the disk."
            } else {
                "Remove this file? It WILL be deleted from the disk."
            }
        ) {
            if let Some(gtk::ResponseType::Ok) = FromPrimitive::from_i32(dialog.run()) {
                if state.projects.contains(&path_str) {
                    state.projects.remove(&path_str);
                    remove_expansions_for_path(state, &path_str);
                    ::utils::write_prefs(state);
                    ::ui::update_project_tree(state, tree);
                    ::ffi::send_message(fd, "bd");
                } else {
                    ::ffi::send_message(fd, "call delete(expand('%')) | bdelete!");
                }
            }
            dialog.destroy();
        }
    }
}

pub fn set_selection(state: &mut ::utils::State, tree: &mut widgets::TreeView, fd: i32) {
    if !state.is_refreshing_tree {
        if let Some(path_str) = ::utils::get_selected_path(state) {
            state.selection = Some(path_str.clone());
            ::utils::write_prefs(state);
            ::ui::update_project_tree(state, tree);
            ::ffi::send_message(fd, format!("e {}", path_str).as_ref());
        }
    }
}

pub fn remove_expansion(state: &mut ::utils::State, iter: &widgets::TreeIter) {
    if let Some(path_str) = state.tree_model.get_value(iter, 1).get_string() {
        remove_expansions_for_path(state, &path_str);
        ::utils::write_prefs(state);
    }
}

pub fn add_expansion(state: &mut ::utils::State, iter: &widgets::TreeIter) {
    if let Some(path_str) = state.tree_model.get_value(iter, 1).get_string() {
        state.expansions.insert(path_str);
        ::utils::write_prefs(state);
    }
}
