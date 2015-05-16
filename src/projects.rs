use gtk::traits::*;
use gtk::{self, widgets};
use std::fs::PathExt;
use std::path::Path;
use std::process::Command;

fn remove_expansions_for_path(prefs: &mut ::utils::Prefs, path_str: &String) {
    for expansion_str in prefs.expansions.clone().iter() {
        if !Path::new(expansion_str).exists() ||
            path_str == expansion_str ||
            ::utils::is_parent_path(path_str, expansion_str)
        {
            prefs.expansions.remove(expansion_str);
        }
    }
}

fn save_project(prefs: &mut ::utils::Prefs, path_str: &String) {
    prefs.projects.insert(path_str.clone());
    ::utils::write_prefs(prefs);
}

pub fn new_project(prefs: &mut ::utils::Prefs) {
    let dialog = widgets::FileChooserDialog::new(
        "New Project",
        None,
        gtk::FileChooserAction::Save,
        [("Save", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]
    );
    if dialog.run() == gtk::ResponseType::Ok as i32 {
        if let Some(path_str) = dialog.get_filename() {
            let path_ref: &str = path_str.as_ref();
            let path = Path::new(path_ref);
            if let Some(name_os_str) = path.file_name() {
                if let Some(name_str) = name_os_str.to_str() {
                    if let Some(parent_path) = path.parent() {
                        match Command::new("cargo").arg("new").arg(name_str).arg("--bin")
                            .current_dir(parent_path).status()
                        {
                            Ok(_) => save_project(prefs, &path_str),
                            Err(e) => println!("Error creating {}: {}", name_str, e)
                        }
                    }
                }
            }
        }
    }
    dialog.destroy();
}

pub fn import_project(prefs: &mut ::utils::Prefs) {
    let dialog = widgets::FileChooserDialog::new(
        "Import",
        None,
        gtk::FileChooserAction::SelectFolder,
        [("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]
    );
    if dialog.run() == gtk::ResponseType::Ok as i32 {
        if let Some(path_str) = dialog.get_filename() {
            save_project(prefs, &path_str);
        }
    }
    dialog.destroy();
}

pub fn rename_file(ui: &::utils::UI, prefs: &mut ::utils::Prefs, fd: i32) {
    if let Some(_) = ::utils::get_selected_path(ui) {
        let dialog = widgets::FileChooserDialog::new(
            "Rename",
            None,
            gtk::FileChooserAction::Save,
            [("Save", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]
        );
        if dialog.run() == gtk::ResponseType::Ok as i32 {
            if let Some(path_str) = dialog.get_filename() {
                prefs.selection = Some(path_str.clone());
                ::utils::write_prefs(prefs);
                ::ffi::send_message(fd, format!("Move {}", path_str).as_ref());
            }
        }
        dialog.destroy();
    }
}

pub fn remove_item(ui: &::utils::UI, prefs: &mut ::utils::Prefs, fd: i32) {
    if let Some(path_str) = ::utils::get_selected_path(ui) {
        if let Some(dialog) = widgets::MessageDialog::new_with_markup(
            Some(&ui.window),
            gtk::DialogFlags::Modal,
            gtk::MessageType::Question,
            gtk::ButtonsType::OkCancel,
            if prefs.projects.contains(&path_str) {
                "Remove this project? It WILL NOT be deleted from the disk."
            } else {
                "Remove this file? It WILL be deleted from the disk."
            }
        ) {
            if dialog.run() == gtk::ResponseType::Ok as i32 {
                if prefs.projects.contains(&path_str) {
                    prefs.projects.remove(&path_str);
                    remove_expansions_for_path(prefs, &path_str);
                    ::utils::write_prefs(prefs);
                    ::ffi::send_message(fd, "bd");
                } else {
                    ::ffi::send_message(fd, "call delete(expand('%')) | bdelete!");
                }
            }
            dialog.destroy();
        }
    }
}

pub fn set_selection(ui: &::utils::UI, prefs: &mut ::utils::Prefs, fd: i32) {
    if let Some(path_str) = ::utils::get_selected_path(ui) {
        prefs.selection = Some(path_str.clone());
        ::utils::write_prefs(prefs);
        ::ffi::send_message(fd, format!("e {}", path_str).as_ref());
    }
}

pub fn remove_expansion(ui: &::utils::UI, prefs: &mut ::utils::Prefs, iter: &widgets::TreeIter) {
    if let Some(path_str) = ui.tree_model.get_value(iter, 1).get_string() {
        remove_expansions_for_path(prefs, &path_str);
        ::utils::write_prefs(prefs);
    }
}

pub fn add_expansion(ui: &::utils::UI, prefs: &mut ::utils::Prefs, iter: &widgets::TreeIter) {
    if let Some(path_str) = ui.tree_model.get_value(iter, 1).get_string() {
        prefs.expansions.insert(path_str);
        ::utils::write_prefs(prefs);
    }
}
