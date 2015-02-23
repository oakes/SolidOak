extern crate libc;
extern crate neovim;
extern crate rgtk;
extern crate "rustc-serialize" as rustc_serialize;

use rgtk::*;
use std::collections::HashSet;
use std::old_io::fs;
use std::old_io::fs::PathExtensions;
use std::old_io::timer;
use std::time::duration::Duration;

mod ffi;
mod projects;
mod ui;
mod utils;

fn gui_main(
    pty: &mut gtk::VtePty,
    read_fd: ffi::c_int,
    write_fd: ffi::c_int,
    pid: ffi::c_int)
{
    gtk::init();

    // constants

    let width = 1242;
    let height = 768;
    let editor_height = ((height as f32) * 0.75) as i32;
    let mut quit_app = false;

    // create the window

    let title = format!("SolidOak {}.{}.{}",
                        option_env!("CARGO_PKG_VERSION_MAJOR").unwrap(),
                        option_env!("CARGO_PKG_VERSION_MINOR").unwrap(),
                        option_env!("CARGO_PKG_VERSION_PATCH").unwrap());
    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    window.set_title(title.as_slice());
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(width, height);

    window.connect(gtk::signals::DeleteEvent::new(&mut |_| {
        ffi::send_message(write_fd, "qall!");
        unsafe {
            ffi::close(read_fd);
            ffi::close(write_fd);
        }
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

    let mut project_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    project_pane.set_size_request(-1, -1);
    project_pane.pack_start(&project_buttons, false, true, 0);
    project_pane.pack_start(&scroll_pane, true, true, 0);

    let mut editor_pane = gtk::VteTerminal::new().unwrap();
    editor_pane.set_size_request(-1, editor_height);
    editor_pane.set_pty(pty);
    editor_pane.watch_child(pid);

    let run_button = gtk::Button::new_with_label("Run").unwrap();
    let build_button = gtk::Button::new_with_label("Build").unwrap();
    let test_button = gtk::Button::new_with_label("Test").unwrap();
    let clean_button = gtk::Button::new_with_label("Clean").unwrap();
    let stop_button = gtk::Button::new_with_label("Stop").unwrap();

    let mut build_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 0).unwrap();
    build_buttons.set_size_request(-1, -1);
    build_buttons.add(&run_button);
    build_buttons.add(&build_button);
    build_buttons.add(&test_button);
    build_buttons.add(&clean_button);
    build_buttons.add(&stop_button);

    let build_term = gtk::VteTerminal::new().unwrap();

    let mut build_pane = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    build_pane.pack_start(&build_buttons, false, true, 0);
    build_pane.pack_start(&build_term, true, true, 0);

    let mut content = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    content.pack_start(&editor_pane, false, true, 0);
    content.pack_start(&build_pane, true, true, 0);

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

    ffi::send_message(write_fd, "au BufEnter * call rpcnotify(1, 'bufenter', fnamemodify(bufname(''), ':p'))");

    // make read_fd non-blocking so we can check it while also checking for GUI events

    unsafe { ffi::fcntl(read_fd, ffi::F_SETFL, ffi::O_NONBLOCK) };

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
                    _ => (),
                }
            }
            ::ui::update_project_tree(&mut state, &mut project_tree);
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
    let data_dir = home_dir.join(::utils::DATA_DIR);
    if !data_dir.exists() {
        match fs::mkdir(&data_dir, ::std::old_io::USER_DIR) {
            Ok(_) => {
                for res in ::utils::DATA_CONTENT.iter() {
                    let res_path = data_dir.join_many(res.path);
                    ::std::old_io::fs::mkdir_recursive(&res_path.dir_path(), ::std::old_io::USER_DIR).ok();
                    ::std::old_io::File::create(&res_path).write_all(res.data.as_bytes()).ok();
                }
                println!("Created data dir at {}", data_dir.as_str().unwrap());
            },
            Err(e) => { println!("Error creating data dir: {}", e) }
        }
    }

    // set $VIM to the data dir if it isn't already set
    if ::std::env::var("VIM").is_err() {
        ::std::env::set_var("VIM", data_dir.as_str().unwrap());
    }

    // create config file
    let config_file = home_dir.join(::utils::CONFIG_FILE);
    if !config_file.exists() {
        match ::std::old_io::File::create(&config_file).write_all(::utils::CONFIG_CONTENT.as_bytes()) {
            Ok(_) => { println!("Created config file at {}", config_file.as_str().unwrap()) },
            Err(e) => { println!("Error creating config file: {}", e) }
        }
    }

    // collect the args into a vector and add the config file path
    let mut args_vec : Vec<String> = ::std::env::args().collect();
    args_vec.push_all(&["-u".to_string(), config_file.as_str().unwrap().to_string()]);

    // if the no window flag was used, start up neovim without a gui
    let args_set : HashSet<String> = ::std::env::args().collect();
    if args_set.contains(::utils::NO_WINDOW_FLAG) {
        args_vec.retain(|arg| arg.as_slice() != ::utils::NO_WINDOW_FLAG);
        neovim::main_setup(&args_vec);
        neovim::main_loop();
        return;
    }

    // takes care of piping stdin/stdout between the gui and nvim
    let mut pty = gtk::VtePty::new().unwrap();

    // two anonymous pipes for msgpack-rpc between the gui and nvim
    let mut nvim_gui : [ffi::c_int; 2] = [0; 2]; // to nvim from gui
    let mut gui_nvim : [ffi::c_int; 2] = [0; 2]; // to gui from nvim
    unsafe {
        ffi::pipe(nvim_gui.as_mut_ptr());
        ffi::pipe(gui_nvim.as_mut_ptr());
    };

    // split into two processes
    let pid = unsafe { ffi::fork() };

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
