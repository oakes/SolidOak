#![feature(globs)]
#![feature(if_let)]

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
    let editor_height = ((height as f32) * 0.75) as i32;

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

    let mut proj_btns = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    proj_btns.set_size_request(-1, -1);
    proj_btns.add(&new_button);
    proj_btns.add(&import_button);
    proj_btns.add(&rename_button);
    proj_btns.add(&remove_button);

    let mut proj_tree = gtk::TreeView::new().unwrap();
    let selection = proj_tree.get_selection().unwrap();
    let column_types = [glib::ffi::g_type_string, glib::ffi::g_type_string];
    let store = gtk::TreeStore::new(&column_types).unwrap();
    let model = store.get_model().unwrap();
    proj_tree.set_model(&model);
    proj_tree.set_headers_visible(false);

    let mut scroll_pane = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll_pane.add(&proj_tree);

    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    proj_tree.append_column(&column);

    let mut proj_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    proj_pane.set_size_request(-1, -1);
    proj_pane.pack_start(&proj_btns, false, true, 0);
    proj_pane.pack_start(&scroll_pane, true, true, 0);

    let editor_pane = gtk::VteTerminal::new().unwrap();
    editor_pane.set_size_request(-1, editor_height);

    let run_button = gtk::Button::new_with_label("Run").unwrap();
    let build_button = gtk::Button::new_with_label("Build").unwrap();
    let test_button = gtk::Button::new_with_label("Test").unwrap();
    let clean_button = gtk::Button::new_with_label("Clean").unwrap();
    let stop_button = gtk::Button::new_with_label("Stop").unwrap();

    let mut build_btns = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    build_btns.set_size_request(-1, -1);
    build_btns.add(&run_button);
    build_btns.add(&build_button);
    build_btns.add(&test_button);
    build_btns.add(&clean_button);
    build_btns.add(&stop_button);

    let build_term = gtk::VteTerminal::new().unwrap();

    let mut build_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    build_pane.pack_start(&build_btns, false, true, 0);
    build_pane.pack_start(&build_term, true, true, 0);

    let mut content = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    content.pack_start(&editor_pane, false, true, 0);
    content.pack_start(&build_pane, true, true, 0);

    let mut hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    hbox.pack_start(&proj_pane, false, true, 0);
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
    ::ui::update_project_tree(&mut state, &mut proj_tree);

    // connect to the signals

    new_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::new_project(&mut state, &mut proj_tree);
    }));
    import_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::import_project(&mut state, &mut proj_tree);
    }));
    rename_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::rename_file(&mut state);
    }));
    remove_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::remove_item(&mut state);
    }));
    selection.connect(gtk::signals::Changed::new(|| {
        ::projects::update_selection(&mut state);
    }));
    proj_tree.connect(gtk::signals::RowCollapsed::new(|iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        ::projects::remove_expansion(&mut state, &iter);
    }));
    proj_tree.connect(gtk::signals::RowExpanded::new(|iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        ::projects::add_expansion(&mut state, &iter);
    }));

    // show the window

    window.show_all();
    gtk::main();
}
