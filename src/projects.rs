extern crate rgtk;

use rgtk::*;

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

pub fn rename_project(state: &mut ::utils::State) {
    
}

pub fn remove_project(state: &mut ::utils::State) {
    
}
