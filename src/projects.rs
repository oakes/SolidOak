extern crate rgtk;

use rgtk::*;

fn new_project() {
    let chooser = gtk::FileChooserDialog::new(
        "New Project",
        None,
        gtk::enums::file_chooser_action::CreateFolder).unwrap();
    chooser.run();
    let filename = chooser.get_filename();
    if filename.is_some() {
        println!("{}", filename.unwrap());
    }
    chooser.destroy();
}

fn import_project() {
    let chooser = gtk::FileChooserDialog::new(
        "Import",
        None,
        gtk::enums::file_chooser_action::SelectFolder).unwrap();
    chooser.run();
    let filename = chooser.get_filename();
    if filename.is_some() {
        println!("{}", filename.unwrap());
    }
    chooser.destroy();
}

fn rename_project() {
    
}

fn remove_project() {
    
}

pub fn create_pane() -> gtk::Box {
    let new_project_button = gtk::Button::new_with_label("New Project").unwrap();
    new_project_button.connect(gtk::signals::Clicked::new(new_project));

    let import_button = gtk::Button::new_with_label("Import").unwrap();
    import_button.connect(gtk::signals::Clicked::new(import_project));

    let rename_button = gtk::Button::new_with_label("Rename").unwrap();
    rename_button.connect(gtk::signals::Clicked::new(rename_project));

    let remove_button = gtk::Button::new_with_label("Remove").unwrap();
    remove_button.connect(gtk::signals::Clicked::new(remove_project));

    let mut project_buttons = gtk::Box::new(gtk::orientation::Horizontal, 0).unwrap();
    project_buttons.set_size_request(-1, -1);
    project_buttons.add(&new_project_button);
    project_buttons.add(&import_button);
    project_buttons.add(&rename_button);
    project_buttons.add(&remove_button);

    let mut project_pane = gtk::Box::new(gtk::orientation::Vertical, 0).unwrap();
    let project_tree = gtk::TreeView::new().unwrap();
    project_pane.set_size_request(-1, -1);
    project_pane.pack_start(&project_buttons, false, true, 0);
    project_pane.pack_start(&project_tree, true, true, 0);

    project_pane
}
