extern crate rgtk;

use rgtk::*;

pub fn create_pane() -> gtk::TextView { 
    let build_pane = gtk::TextView::new().unwrap();

    build_pane
}
