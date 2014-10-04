#![feature(globs)]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("SolidOak");
    window.set_border_width(10);
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(1242, 768);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    window.show_all();
    gtk::main();
}
