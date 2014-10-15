#![feature(globs)]
extern crate rgtk;

use rgtk::*;
use std::collections::HashSet;

mod projects;
mod utils;

fn main() {
    gtk::init();

    // constants

    let width = 1242;
    let height = 768;
    let editor_height = ((height as f32) * 0.8) as i32;

    // window

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("SolidOak");
    window.set_window_position(gtk::window_position::Center);
    window.set_default_size(width, height);

    window.connect(gtk::signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    // project pane

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

    let mut project_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::ffi::g_type_string];
    let store = gtk::TreeStore::new(column_types).unwrap();
    let model = store.get_model().unwrap();
    project_tree.set_model(&model);
    project_tree.set_headers_visible(false);

    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    project_tree.append_column(&column);

    let mut project_pane = gtk::Box::new(gtk::orientation::Vertical, 0).unwrap();
    project_pane.set_size_request(-1, -1);
    project_pane.pack_start(&project_buttons, false, true, 0);
    project_pane.pack_start(&project_tree, true, true, 0);

    // editor pane

    let editor_pane = gtk::TextView::new().unwrap();
    editor_pane.set_size_request(-1, editor_height);

    // build pane

    let build_pane = gtk::TextView::new().unwrap();

    // content

    let mut content = gtk::Box::new(gtk::orientation::Vertical, 0).unwrap();
    content.pack_start(&editor_pane, false, true, 0);
    content.pack_start(&build_pane, true, true, 0);

    // hbox

    let mut hbox = gtk::Box::new(gtk::orientation::Horizontal, 0).unwrap();
    hbox.pack_start(&project_pane, false, true, 0);
    hbox.pack_start(&content, true, true, 0);
    window.add(&hbox);

    // state

    let mut state = ::utils::State{
        projects: HashSet::new(),
        expansions: HashSet::new(),
        selection: None,
        tree_store: &store
    };

    // populate tree

    let iter = gtk::TreeIter::new().unwrap();
    state.tree_store.append(&iter, None);
    state.tree_store.set_string(&iter, 0, "Hello, world!");

    let child = gtk::TreeIter::new().unwrap();
    state.tree_store.append(&child, Some(&iter));
    state.tree_store.set_string(&child, 0, "Bye, world!");

    // connections

    new_project_button.connect(gtk::signals::Clicked::new(|| {::projects::new_project(&mut state)}));
    import_button.connect(gtk::signals::Clicked::new(|| {::projects::import_project(&mut state)}));
    rename_button.connect(gtk::signals::Clicked::new(|| {::projects::rename_project(&mut state)}));
    remove_button.connect(gtk::signals::Clicked::new(|| {::projects::remove_project(&mut state)}));

    // show window

    window.show_all();
    gtk::main();
}
