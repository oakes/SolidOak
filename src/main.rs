#![feature(globs)]
extern crate neovim;
extern crate rgtk;
extern crate serialize;

//use neovim::*;
use rgtk::*;
use std::collections::HashSet;

mod projects;
mod ui;
mod utils;

fn main() {
    gtk::init();

    // constants

    let width = 1242;
    let height = 768;
    let editor_height = ((height as f32) * 0.8) as i32;

    // create the window

    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    window.set_title("SolidOak");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(width, height);

    window.connect(gtk::signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    // create the panes

    let new_button = gtk::Button::new_with_label("New Project").unwrap();
    let import_button = gtk::Button::new_with_label("Import").unwrap();
    let rename_button = gtk::Button::new_with_label("Rename").unwrap();
    let remove_button = gtk::Button::new_with_label("Remove").unwrap();

    let mut project_buttons =
        gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    project_buttons.set_size_request(-1, -1);
    project_buttons.add(&new_button);
    project_buttons.add(&import_button);
    project_buttons.add(&rename_button);
    project_buttons.add(&remove_button);

    let mut project_tree = gtk::TreeView::new().unwrap();
    let selection = project_tree.get_selection().unwrap();
    let column_types = [glib::ffi::g_type_string, glib::ffi::g_type_string];
    let store = gtk::TreeStore::new(&column_types).unwrap();
    let model = store.get_model().unwrap();
    project_tree.set_model(&model);
    project_tree.set_headers_visible(false);

    let mut scroll_pane = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll_pane.add(&project_tree);

    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    project_tree.append_column(&column);

    let mut project_pane =
        gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    project_pane.set_size_request(-1, -1);
    project_pane.pack_start(&project_buttons, false, true, 0);
    project_pane.pack_start(&scroll_pane, true, true, 0);

    let editor_pane = gtk::TextView::new().unwrap();
    editor_pane.set_size_request(-1, editor_height);

    let build_pane = gtk::TextView::new().unwrap();

    let mut content = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    content.pack_start(&editor_pane, false, true, 0);
    content.pack_start(&build_pane, true, true, 0);

    let mut hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    hbox.pack_start(&project_pane, false, true, 0);
    hbox.pack_start(&content, true, true, 0);
    window.add(&hbox);

    // populate the project tree

    let mut state = ::utils::State{
        projects: HashSet::new(),
        expansions: HashSet::new(),
        selection: None,
        tree_model: &model,
        tree_store: &store,
        tree_selection: &selection,
        rename_button: &rename_button,
        remove_button: &remove_button,
    };

    ::utils::create_data_dir();
    ::utils::read_prefs(&mut state);
    ::ui::update_project_tree(&mut project_tree, &mut state);

    // connect to the signals

    new_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::new_project(&mut project_tree, &mut state)
    }));
    import_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::import_project(&mut project_tree, &mut state)
    }));
    rename_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::rename_project(&mut state)
    }));
    remove_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::remove_project(&mut state)
    }));
    selection.connect(gtk::signals::Changed::new(|| {
        let mut iter = gtk::TreeIter::new().unwrap();
        selection.get_selected(&model, &mut iter);
        let path_str = model.get_value(&iter, 1).get_string();
        state.selection = path_str;
        ::utils::write_prefs(&state);

        ::ui::update_project_buttons(&mut state);
    }));
    project_tree.connect(gtk::signals::RowCollapsed::new(|iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        let path_str = model.get_value(&iter, 1).get_string();
        if path_str.is_some() {
            state.expansions.remove(&path_str.unwrap());
            ::utils::write_prefs(&state);
        }
    }));
    project_tree.connect(gtk::signals::RowExpanded::new(|iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        let path_str = model.get_value(&iter, 1).get_string();
        if path_str.is_some() {
            state.expansions.insert(path_str.unwrap());
            ::utils::write_prefs(&state);
        }
    }));

    // show the window

    window.show_all();
    gtk::main();
}
