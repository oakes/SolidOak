extern crate rgtk;

use rgtk::*;

pub fn new_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    let chooser = gtk::FileChooserDialog::new(
        "New Project",
        None,
        gtk::FileChooserAction::Save).unwrap();
    chooser.run();
    let filename = chooser.get_filename();
    if filename.is_some() {
        state.projects.insert(filename.unwrap());
        ::ui::update_project_tree(tree, state);
        ::utils::write_prefs(state);
    }
    chooser.destroy();
}

pub fn import_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    let chooser = gtk::FileChooserDialog::new(
        "Import",
        None,
        gtk::FileChooserAction::SelectFolder).unwrap();
    chooser.run();
    let filename = chooser.get_filename();
    if filename.is_some() {
        state.projects.insert(filename.unwrap());
        ::ui::update_project_tree(tree, state);
        ::utils::write_prefs(state);
    }
    chooser.destroy();
}

pub fn rename_project(state: &mut ::utils::State) {
    
}

pub fn remove_project(state: &mut ::utils::State) {
    
}
