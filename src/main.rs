#![feature(collections, core, libc, old_io, path_ext, std_misc)]

extern crate libc;
extern crate neovim;
extern crate racer;
extern crate rgtk;
extern crate "rustc-serialize" as rustc_serialize;

use rgtk::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, PathExt};
use std::io::Write;
use std::old_io::timer;
use std::ops::Deref;
use std::ffi::AsOsStr;
use std::time::duration::Duration;

mod builders;
mod ffi;
mod projects;
mod ui;
mod utils;

fn gui_main(pty: &mut gtk::VtePty, read_fd: i32, write_fd: i32, pid: i32) {
    gtk::init();

    // create the left pane

    let new_button = gtk::Button::new_with_label("New Project").unwrap();
    let import_button = gtk::Button::new_with_label("Import").unwrap();
    let rename_button = gtk::Button::new_with_label("Rename").unwrap();
    let remove_button = gtk::Button::new_with_label("Remove").unwrap();

    let mut project_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    project_buttons.add(&new_button);
    project_buttons.add(&import_button);
    project_buttons.add(&rename_button);
    project_buttons.add(&remove_button);

    let mut project_tree = gtk::TreeView::new().unwrap();
    let selection = project_tree.get_selection().unwrap();
    let column_types = [glib::Type::String, glib::Type::String];
    let store = gtk::TreeStore::new(&column_types).unwrap();
    let model = store.get_model().unwrap();
    project_tree.set_model(&model);
    project_tree.set_headers_visible(false);
    project_tree.set_can_focus(false);

    let mut scroll_pane = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll_pane.add(&project_tree);

    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    project_tree.append_column(&column);

    let mut left_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    left_pane.pack_start(&project_buttons, false, true, 0);
    left_pane.pack_start(&scroll_pane, true, true, 0);

    // create the right pane

    let save_button = gtk::Button::new_with_label("Save").unwrap();
    let undo_button = gtk::Button::new_with_label("Undo").unwrap();
    let redo_button = gtk::Button::new_with_label("Redo").unwrap();
    let font_minus_button = gtk::Button::new_with_label("Font -").unwrap();
    let font_plus_button = gtk::Button::new_with_label("Font +").unwrap();
    let mut easy_mode_button = gtk::ToggleButton::new_with_label("Easy Mode").unwrap();
    let editor_separator = gtk::Separator::new(gtk::Orientation::Horizontal).unwrap();
    let close_button = gtk::Button::new_with_label("X").unwrap();

    let mut editor_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    editor_buttons.add(&save_button);
    editor_buttons.add(&undo_button);
    editor_buttons.add(&redo_button);
    editor_buttons.add(&font_minus_button);
    editor_buttons.add(&font_plus_button);
    editor_buttons.add(&easy_mode_button);
    editor_buttons.pack_start(&editor_separator, true, false, 0);
    editor_buttons.add(&close_button);

    let mut editor_term = gtk::VteTerminal::new().unwrap();
    editor_term.set_pty(pty);
    editor_term.watch_child(pid);
    editor_term.set_size_request(-1, (utils::EDITOR_HEIGHT_PCT * (utils::WINDOW_HEIGHT as f32)) as i32);

    let run_button = gtk::Button::new_with_label("Run").unwrap();
    let build_button = gtk::Button::new_with_label("Build").unwrap();
    let test_button = gtk::Button::new_with_label("Test").unwrap();
    let clean_button = gtk::Button::new_with_label("Clean").unwrap();
    let stop_button = gtk::Button::new_with_label("Stop").unwrap();

    let mut build_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    build_buttons.add(&run_button);
    build_buttons.add(&build_button);
    build_buttons.add(&test_button);
    build_buttons.add(&clean_button);
    build_buttons.add(&stop_button);

    let mut build_terms = gtk::Stack::new().unwrap();

    let mut build_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    build_pane.pack_start(&build_buttons, false, true, 0);
    build_pane.pack_start(&build_terms, true, true, 0);

    let mut resizer = gtk::Paned::new(gtk::Orientation::Vertical).unwrap();
    resizer.add1(&editor_term);
    resizer.add2(&build_pane);

    let mut right_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    right_pane.pack_start(&editor_buttons, false, true, 0);
    right_pane.pack_start(&resizer, true, true, 0);

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
        ffi::send_message(write_fd, "qall!");
        ffi::close_fd(read_fd);
        ffi::close_fd(write_fd);
        quit_app = true;
        true
    }));

    let mut window_pane = gtk::Paned::new(gtk::Orientation::Horizontal).unwrap();
    window_pane.add1(&left_pane);
    window_pane.add2(&right_pane);

    window.add(&window_pane);
    window.show_all();

    // populate the project tree

    let mut state = ::utils::State{
        projects: HashSet::new(),
        expansions: HashSet::new(),
        builders: HashMap::new(),
        selection: None,
        easy_mode: true,
        font_size: 12,
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

    easy_mode_button.set_active(state.easy_mode);
    ::ffi::send_message(write_fd, if state.easy_mode { "set im" } else { "set noim" });
    editor_term.set_font_size(state.font_size);
    editor_term.grab_focus();

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

    save_button.connect(gtk::signals::Clicked::new(&mut || {
        ::ffi::send_message(write_fd, "w");
    }));
    undo_button.connect(gtk::signals::Clicked::new(&mut || {
        ::ffi::send_message(write_fd, "undo");
    }));
    redo_button.connect(gtk::signals::Clicked::new(&mut || {
        ::ffi::send_message(write_fd, "redo");
    }));
    font_minus_button.connect(gtk::signals::Clicked::new(&mut || {
        if state.font_size > ::utils::MIN_FONT_SIZE {
            state.font_size -= 1;
            ::utils::write_prefs(&state);
            editor_term.set_font_size(state.font_size);
            ::builders::set_builders_font_size(&mut state);
        }
    }));
    font_plus_button.connect(gtk::signals::Clicked::new(&mut || {
        if state.font_size < ::utils::MAX_FONT_SIZE {
            state.font_size += 1;
            ::utils::write_prefs(&state);
            editor_term.set_font_size(state.font_size);
            ::builders::set_builders_font_size(&mut state);
        }
    }));
    easy_mode_button.connect(gtk::signals::Clicked::new(&mut || {
        state.easy_mode = easy_mode_button.get_active();
        ::utils::write_prefs(&state);
        ::ffi::send_message(write_fd, if state.easy_mode { "set im" } else { "set noim" });
    }));
    close_button.connect(gtk::signals::Clicked::new(&mut || {
        ::ffi::send_message(write_fd, "bd");
    }));

    run_button.connect(gtk::signals::Clicked::new(&mut || {
        ::builders::stop_builder(&mut state);
        ::builders::run_builder(&mut state, &["cargo", "run"]);
    }));
    build_button.connect(gtk::signals::Clicked::new(&mut || {
        ::builders::stop_builder(&mut state);
        ::builders::run_builder(&mut state, &["cargo", "build", "--release"]);
    }));
    test_button.connect(gtk::signals::Clicked::new(&mut || {
        ::builders::stop_builder(&mut state);
        ::builders::run_builder(&mut state, &["cargo", "test"]);
    }));
    clean_button.connect(gtk::signals::Clicked::new(&mut || {
        ::builders::stop_builder(&mut state);
        ::builders::run_builder(&mut state, &["cargo", "clean"]);
    }));
    stop_button.connect(gtk::signals::Clicked::new(&mut || {
        ::builders::stop_builder(&mut state);
    }));

    // listen for events

    ffi::send_message(write_fd, "au BufEnter * call rpcnotify(1, 'bufenter', fnamemodify(bufname(''), ':p'))");
    ffi::send_message(write_fd, "au VimLeave * call rpcnotify(1, 'vimleave')");

    // make read_fd non-blocking so we can check it while also checking for GUI events

    ffi::set_non_blocking(read_fd);

    // loop over GUI events and respond to messages from nvim

    loop {
        gtk::main_iteration_do(false);

        if let Some(recv_arr) = ffi::recv_message(read_fd) {
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
                    "vimleave" => { quit_app = true; }
                    _ => (),
                }
            }
            ::ui::update_project_tree(&mut state, &mut project_tree);
            ::builders::show_builder(&mut state, &mut build_terms);
            ::builders::set_builders_font_size(&mut state);
        }

        if quit_app {
            break;
        }

        timer::sleep(Duration::milliseconds(10));
    }

    ::builders::stop_builders(&mut state);
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

    // set $VIM if it isn't already set
    if env::var("VIM").is_err() {
        if let Some(path_str) = data_dir.to_str() {
            env::set_var("VIM", path_str);
        }
    }

    // set $RACER_CMD_PATH
    if let Ok(path) = env::current_exe() {
        if let Some(path_str) = path.as_os_str().to_str() {
            env::set_var("RACER_CMD_PATH", path_str);
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

    // collect the args into a set and vector
    let args_set: HashSet<String> = env::args().collect();
    let mut args_vec: Vec<String> = env::args().collect();

    // if the racer flag was used, run racer
    if args_set.contains(::utils::RACER_FLAG) {
        args_vec.retain(|arg| arg.as_slice() != ::utils::RACER_FLAG);
        racer::racer_main(&args_vec);
        return;
    }

    // add the config file path
    if let Some(path_str) = config_file.to_str() {
        args_vec.push_all(&["-u".to_string(), path_str.to_string()]);
    }

    // if the no window flag was used, start up neovim without a gui
    if args_set.contains(::utils::NO_WINDOW_FLAG) {
        args_vec.retain(|arg| arg.as_slice() != ::utils::NO_WINDOW_FLAG);
        neovim::main_setup(&args_vec);
        neovim::main_loop();
        return;
    }

    // takes care of piping stdin/stdout between the gui and nvim
    let mut pty = gtk::VtePty::new().unwrap();

    // two anonymous pipes for msgpack-rpc between the gui and nvim
    let nvim_gui = ffi::new_pipe(); // to nvim from gui
    let gui_nvim = ffi::new_pipe(); // to gui from nvim

    // split into two processes
    let pid = ffi::fork_process();

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
