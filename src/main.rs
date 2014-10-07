#![feature(globs)]
extern crate rgtk;

use rgtk::*;

mod builders;
mod editors;
mod projects;

fn main() {
    gtk::init();

    let width = 1242;
    let height = 768;
    let editor_height = ((height as f32) * 0.8) as i32;

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("SolidOak");
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(width, height);

    window.connect(gtk::signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    let project_pane = ::projects::create_pane();
    let editor_pane = ::editors::create_pane();
    let build_pane = ::builders::create_pane();

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
