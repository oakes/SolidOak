extern crate libc;
extern crate neovim;
extern crate rgtk;
extern crate "rustc-serialize" as rustc_serialize;

use libc::c_int;
use rgtk::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::fs::PathExt;
use std::io::Write;
use std::old_io::timer;
use std::ops::Deref;
use std::time::duration::Duration;

mod builders;
mod native;
mod projects;
mod ui;
mod utils;

fn gui_main(pty: &mut gtk::VtePty, read_fd: c_int, write_fd: c_int, pid: c_int) {
    gtk::init();

    // create the window

    let title = format!("SolidOak {}.{}.{}",
                        option_env!("CARGO_PKG_VERSION_MAJOR").unwrap(),
                        option_env!("CARGO_PKG_VERSION_MINOR").unwrap(),
                        option_env!("CARGO_PKG_VERSION_PATCH").unwrap());
    let mut quit_app = false;
    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    window.set_title(title.as_slice());
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(utils::WINDOW_WIDTH, utils::WINDOW_HEIGHT);
    window.connect(gtk::signals::DeleteEvent::new(&mut |_| {
        native::send_message(write_fd, "qall!");
        native::close_fd(read_fd);
        native::close_fd(write_fd);
        quit_app = true;
        true
    }));

    // create the panes

    let new_button = gtk::Button::new_with_label("New Project").unwrap();
    let import_button = gtk::Button::new_with_label("Import").unwrap();
    let rename_button = gtk::Button::new_with_label("Rename").unwrap();
    let remove_button = gtk::Button::new_with_label("Remove").unwrap();

    let mut project_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    project_buttons.set_size_request(-1, -1);
    project_buttons.add(&new_button);
    project_buttons.add(&import_button);
    project_buttons.add(&rename_button);
    project_buttons.add(&remove_button);

    let mut project_tree = gtk::TreeView::new().unwrap();
    let selection = project_tree.get_selection().unwrap();
    let column_types = [ffi::glib::g_type_string, ffi::glib::g_type_string];
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

    let mut project_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    project_pane.set_size_request(-1, -1);
    project_pane.pack_start(&project_buttons, false, true, 0);
    project_pane.pack_start(&scroll_pane, true, true, 0);

    let mut editor_pane = gtk::VteTerminal::new().unwrap();
    editor_pane.set_pty(pty);
    editor_pane.watch_child(pid);
    editor_pane.set_size_request(-1, (utils::EDITOR_HEIGHT_PCT * (utils::WINDOW_HEIGHT as f32)) as i32);

    let mut build_pane = gtk::Stack::new().unwrap();

    let mut content = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    content.pack_start(&editor_pane, true, true, 0);
    content.pack_start(&build_pane, false, true, 0);

    let mut hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    hbox.pack_start(&project_pane, false, true, 0);
    hbox.pack_start(&content, true, true, 0);
    window.add(&hbox);

    // show the window

    window.show_all();

    // populate the project tree

    let mut state = ::utils::State{
        projects: HashSet::new(),
        expansions: HashSet::new(),
        builders: HashMap::new(),
        selection: None,
        window: &window,
        tree_model: &model,
        tree_store: &store,
        tree_selection: &selection,
        rename_button: &rename_button,
        remove_button: &remove_button,
        is_refreshing_tree: false
    };

    ::utils::read_prefs(&mut state);
    ::ui::update_project_tree(&mut state, &mut project_tree);
    ::projects::set_selection(&mut state, &mut project_tree, write_fd);

    // connect to the signals

    new_button.connect(gtk::signals::Clicked::new(&mut || {
        ::projects::new_project(&mut state, &mut project_tree);
    }));
    import_button.connect(gtk::signals::Clicked::new(&mut || {
        ::projects::import_project(&mut state, &mut project_tree);
    }));
    rename_button.connect(gtk::signals::Clicked::new(&mut || {
        ::projects::rename_file(&mut state, write_fd);
    }));
    remove_button.connect(gtk::signals::Clicked::new(&mut || {
        ::projects::remove_item(&mut state, &mut project_tree, write_fd);
    }));
    selection.connect(gtk::signals::Changed::new(&mut || {
        ::projects::set_selection(&mut state, &mut project_tree, write_fd);
    }));
    project_tree.connect(gtk::signals::RowCollapsed::new(&mut |iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        ::projects::remove_expansion(&mut state, &iter);
    }));
    project_tree.connect(gtk::signals::RowExpanded::new(&mut |iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        ::projects::add_expansion(&mut state, &iter);
    }));

    // listen for bufenter events

    let cmd = "au BufEnter * call rpcnotify(1, 'bufenter', fnamemodify(bufname(''), ':p'))";
    native::send_message(write_fd, cmd);

    // make read_fd non-blocking so we can check it while also checking for GUI events

    native::set_non_blocking(read_fd);

    // loop over GUI events and respond to messages from nvim

    loop {
        gtk::main_iteration_do(false);

        if let Some(recv_arr) = native::recv_message(read_fd) {
            if let Some(neovim::Object::String(event_name)) = recv_arr.get(1) {
                match event_name.as_slice() {
                    "bufenter" => {
                        if let Some(neovim::Object::Array(event_args)) = recv_arr.get(2) {
                            if let Some(neovim::Object::String(path_str)) = event_args.get(0) {
                                state.selection = Some(path_str);
                                ::utils::write_prefs(&state);
                            }
                        }
                    },
                    _ => (),
                }
            }
            ::ui::update_project_tree(&mut state, &mut project_tree);
            ::builders::show_builder(&mut state, &mut build_pane);
        }

        if quit_app {
            break;
        }

        timer::sleep(Duration::milliseconds(10));
    }
}

fn main() {
    // create data dir
    let home_dir = ::utils::get_home_dir();
    let data_dir = home_dir.deref().join(::utils::DATA_DIR);
    if !data_dir.exists() {
        match fs::create_dir(&data_dir) {
            Ok(_) => {
                for res in ::utils::DATA_CONTENT.iter() {
                    let mut res_path = data_dir.clone();
                    for part in res.path {
                        res_path.push(part);
                    }
                    if let Some(parent) = res_path.parent() {
                        fs::create_dir_all(parent).ok();
                    }
                    if let Some(mut f) = fs::File::create(&res_path).ok() {
                        f.write_all(res.data.as_bytes()).ok();
                    }
                }
                if let Some(path_str) = data_dir.to_str() {
                    println!("Created data dir at {}", path_str);
                }
            },
            Err(e) => { println!("Error creating data dir: {}", e) }
        }
    }

    // set $VIM to the data dir if it isn't already set
    if env::var("VIM").is_err() {
        if let Some(path_str) = data_dir.to_str() {
            env::set_var("VIM", path_str);
        }
    }

    // create config file
    let config_file = home_dir.deref().join(::utils::CONFIG_FILE);
    if !config_file.exists() {
        match fs::File::create(&config_file) {
            Ok(mut f) => {
                f.write_all(::utils::CONFIG_CONTENT.as_bytes()).ok();
                if let Some(path_str) = config_file.to_str() {
                    println!("Created config file at {}", path_str);
                }
            },
            Err(e) => { println!("Error creating config file: {}", e) }
        }
    }

    // collect the args into a vector and add the config file path
    let mut args_vec : Vec<String> = env::args().collect();
    if let Some(path_str) = config_file.to_str() {
        args_vec.push_all(&["-u".to_string(), path_str.to_string()]);
    }

    // if the no window flag was used, start up neovim without a gui
    let args_set : HashSet<String> = env::args().collect();
    if args_set.contains(::utils::NO_WINDOW_FLAG) {
        args_vec.retain(|arg| arg.as_slice() != ::utils::NO_WINDOW_FLAG);
        neovim::main_setup(&args_vec);
        neovim::main_loop();
        return;
    }

    // takes care of piping stdin/stdout between the gui and nvim
    let mut pty = gtk::VtePty::new().unwrap();

    // two anonymous pipes for msgpack-rpc between the gui and nvim
    let nvim_gui = native::new_pipe(); // to nvim from gui
    let gui_nvim = native::new_pipe(); // to gui from nvim

    // split into two processes
    let pid = native::fork_process();

    if pid > 0 { // the gui process
        gui_main(&mut pty, gui_nvim[0], nvim_gui[1], pid);
    } else { // the nvim process
        // prepare this process to be piped into the gui
        pty.child_setup();

        // start nvim
        neovim::main_setup(&args_vec);
        neovim::channel_from_fds(nvim_gui[0], gui_nvim[1]);
        neovim::main_loop();
    }
}
