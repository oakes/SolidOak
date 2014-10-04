#![feature(globs)]
extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn main() {
    gtk::init();

    let width = 1242;
    let height = 768;
    let sidebar_width = ((width as f32) * 0.32) as i32;
    let content_width = width - sidebar_width;
    let top_height = ((height as f32) * 0.8) as i32;

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("SolidOak");
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(width, height);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    let project_pane = gtk::TreeView::new().unwrap();
    let rusti_pane = gtk::TextView::new().unwrap();
    project_pane.set_size_request(sidebar_width, top_height);
    let mut vbox_1 = gtk::Box::new(gtk::orientation::Vertical, 0).unwrap();
    vbox_1.pack_start(&project_pane, false, true, 0);
    vbox_1.pack_start(&rusti_pane, true, true, 0);

    let editor_pane = gtk::TextView::new().unwrap();
    let build_pane = gtk::TextView::new().unwrap();
    editor_pane.set_size_request(content_width, top_height);
    let mut vbox_2 = gtk::Box::new(gtk::orientation::Vertical, 0).unwrap();
    vbox_2.pack_start(&editor_pane, false, true, 0);
    vbox_2.pack_start(&build_pane, true, true, 0);

    let mut hbox = gtk::Box::new(gtk::orientation::Horizontal, 0).unwrap();
    hbox.pack_start(&vbox_1, false, true, 0);
    hbox.pack_start(&vbox_2, true, true, 0);
    window.add(&hbox);

    window.show_all();
    gtk::main();
}
