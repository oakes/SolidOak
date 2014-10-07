extern crate rgtk;

use rgtk::*;

pub fn create_pane() -> gtk::TextView { 
    let editor_pane = gtk::TextView::new().unwrap();

    editor_pane
}
