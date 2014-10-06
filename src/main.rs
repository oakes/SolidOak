#![feature(globs)]
extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn main() {
    gtk::init();

    let width = 1242;
    let height = 768;
    let editor_height = ((height as f32) * 0.8) as i32;

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("SolidOak");
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(width, height);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    let new_project_button = gtk::Button::new_with_label("New Project").unwrap();
    let import_button = gtk::Button::new_with_label("Import").unwrap();
    let rename_button = gtk::Button::new_with_label("Rename").unwrap();
    let remove_button = gtk::Button::new_with_label("Remove").unwrap();

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

    let editor_pane = gtk::TextView::new().unwrap();
    let build_pane = gtk::TextView::new().unwrap();
    editor_pane.set_size_request(-1, editor_height);
    let mut content = gtk::Box::new(gtk::orientation::Vertical, 0).unwrap();
    content.pack_start(&editor_pane, false, true, 0);
    content.pack_start(&build_pane, true, true, 0);

    let mut hbox = gtk::Box::new(gtk::orientation::Horizontal, 0).unwrap();
    hbox.pack_start(&project_pane, false, true, 0);
    hbox.pack_start(&content, true, true, 0);
    window.add(&hbox);

    window.show_all();
    gtk::main();
}