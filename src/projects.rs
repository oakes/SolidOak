extern crate rgtk;

use rgtk::*;

pub fn create_pane() -> gtk::Box {
    let new_project_button = gtk::Button::new_with_label("New Project").unwrap();
    new_project_button.connect(gtk::signals::ButtonReleaseEvent::new(|_| {
        println!("new project");
        true
    }));

    let import_button = gtk::Button::new_with_label("Import").unwrap();
    import_button.connect(gtk::signals::ButtonReleaseEvent::new(|_| {
        println!("import");
        true
    }));

    let rename_button = gtk::Button::new_with_label("Rename").unwrap();
    rename_button.connect(gtk::signals::ButtonReleaseEvent::new(|_| {
        println!("rename");
        true
    }));

    let remove_button = gtk::Button::new_with_label("Remove").unwrap();
    remove_button.connect(gtk::signals::ButtonReleaseEvent::new(|_| {
        println!("remove");
        true
    }));

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
