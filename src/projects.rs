extern crate gtk;

use gtk::*;
use std::path::Path;
use std::fs::metadata;
use std::process::Command;

fn remove_expansions_for_path(prefs: &mut ::utils::Prefs, path_str: &String) {
    for expansion_str in prefs.expansions.clone().iter() {
        if !metadata(Path::new(expansion_str)).is_ok()||
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
    let dialog = FileChooserDialog::new::<Dialog>(
        Some("New Project"),
        None,
        gtk::FileChooserAction::Save
    );
    dialog.add_button("Save", gtk::ResponseType::Ok.into());
    dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
    if dialog.run() == gtk::ResponseType::Ok.into() {
        if let Some(path_buf) = dialog.get_filename() {
	    if let Some(path_str) = path_buf.to_str() {
                if let Some(name_os_str) = path_buf.file_name() {
                    if let Some(name_str) = name_os_str.to_str() {
                        if let Some(parent_path) = path_buf.parent() {
                            match Command::new("cargo").arg("new").arg(name_str).arg("--bin")
                                .current_dir(parent_path).status()
                            {
                                Ok(_) => save_project(prefs, &String::from(path_str)),
                                Err(e) => println!("Error creating {}: {}", name_str, e)
                            }
                        }
                    }
	        }
            }
        }
    }
    dialog.destroy();
}

pub fn import_project(prefs: &mut ::utils::Prefs) {
    let dialog = FileChooserDialog::new::<Dialog>(
        Some("Import"),
        None,
        gtk::FileChooserAction::SelectFolder,
    );
    dialog.add_button("Save", gtk::ResponseType::Ok.into());
    dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
    if dialog.run() == gtk::ResponseType::Ok.into() {
        if let Some(path_buf) = dialog.get_filename() {
	    if let Some(path_str) = path_buf.to_str() {
                save_project(prefs, &String::from(path_str));
	    }
        }
    }
    dialog.destroy();
}

pub fn rename_file(ui: &::utils::UI, prefs: &mut ::utils::Prefs, fd: i32) {
    if let Some(_) = ::utils::get_selected_path(ui) {
        let dialog = FileChooserDialog::new::<Dialog>(
            Some("Rename"),
            None,
            gtk::FileChooserAction::Save
        );
        dialog.add_button("Save", gtk::ResponseType::Ok.into());
        dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
        if dialog.run() == gtk::ResponseType::Ok.into() {
            if let Some(path_buf) = dialog.get_filename() {
	        if let Some(path_str) = path_buf.to_str() {
                    prefs.selection = Some(String::from(path_str));
                    ::utils::write_prefs(prefs);
                    ::ffi::send_message(fd, format!("Move {}", path_str).as_ref());
		}
            }
        }
        dialog.destroy();
    }
}

pub fn remove_item(ui: &::utils::UI, prefs: &mut ::utils::Prefs, fd: i32) {
    if let Some(path_str) = ::utils::get_selected_path(ui) {
        let dialog = MessageDialog::new(
            Some(&ui.window),
            gtk::DialogFlags::empty(),
            gtk::MessageType::Question,
            gtk::ButtonsType::OkCancel,
            if prefs.projects.contains(&path_str) {
                "Remove this project? It WILL NOT be deleted from the disk."
            } else {
                "Remove this file? It WILL be deleted from the disk."
            }
        );
        if dialog.run() == gtk::ResponseType::Ok.into() {
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

pub fn set_selection(ui: &::utils::UI, prefs: &mut ::utils::Prefs, fd: i32) {
    if let Some(path_str) = ::utils::get_selected_path(ui) {
        prefs.selection = Some(path_str.clone());
        ::utils::write_prefs(prefs);
        ::ffi::send_message(fd, format!("e {}", path_str).as_ref());
    }
}

pub fn remove_expansion(ui: &::utils::UI, prefs: &mut ::utils::Prefs, iter: &TreeIter) {
    if let Some(path_str) = ::utils::iter_to_str(ui, iter) {
        remove_expansions_for_path(prefs, &path_str);
        ::utils::write_prefs(prefs);
    }
}

pub fn add_expansion(ui: &::utils::UI, prefs: &mut ::utils::Prefs, iter: &TreeIter) {
    if let Some(path_str) = ::utils::iter_to_str(ui, iter) {
        prefs.expansions.insert(path_str);
        ::utils::write_prefs(prefs);
    }
}
