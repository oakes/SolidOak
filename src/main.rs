#![feature(globs)]

extern crate libc;
extern crate neovim;
extern crate rgtk;
extern crate serialize;

use rgtk::*;
use std::collections::HashSet;
use std::io::fs;
use std::io::fs::PathExtensions;

mod projects;
mod ui;
mod utils;

mod ffi {
    pub use libc::{c_int, c_uchar, c_void};
    pub use libc::funcs::posix88::unistd::{close, pipe, read, write};
    pub use libc::types::os::arch::c95::size_t;

    extern "C" {
        pub fn fork () -> c_int;
        pub fn kill (pid: c_int, sig: c_int);
    }
}

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

    // create the window

    let title = format!("SolidOak {}.{}.{}",
                        option_env!("CARGO_PKG_VERSION_MAJOR").unwrap(),
                        option_env!("CARGO_PKG_VERSION_MINOR").unwrap(),
                        option_env!("CARGO_PKG_VERSION_PATCH").unwrap());
    let mut window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
    window.set_title(title.as_slice());
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(width, height);

    window.connect(gtk::signals::DeleteEvent::new(|_| {
        unsafe {
            ffi::close(read_fd);
            ffi::close(write_fd);
            ffi::kill(pid, 15);
        }
        gtk::main_quit();
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

    ::utils::read_prefs(&mut state);
    ::ui::update_project_tree(&mut state, &mut project_tree);

    // connect to the signals

    new_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::new_project(&mut state, &mut project_tree);
    }));
    import_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::import_project(&mut state, &mut project_tree);
    }));
    rename_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::rename_file(&mut state);
    }));
    remove_button.connect(gtk::signals::Clicked::new(|| {
        ::projects::remove_item(&mut state);
    }));
    selection.connect(gtk::signals::Changed::new(|| {
        ::projects::save_selection(&mut state);
    }));
    project_tree.connect(gtk::signals::RowCollapsed::new(|iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        ::projects::remove_expansion(&mut state, &iter);
    }));
    project_tree.connect(gtk::signals::RowExpanded::new(|iter_raw, _| {
        let iter = gtk::TreeIter::wrap_pointer(iter_raw);
        ::projects::add_expansion(&mut state, &iter);
    }));

    // show the window

    window.show_all();
    gtk::main();
}

fn main() {
    // create data dir
    let home_dir = ::utils::get_home_dir();
    let data_dir = home_dir.join(::utils::DATA_DIR);
    if !data_dir.exists() {
        match fs::mkdir(&data_dir, ::std::io::USER_DIR) {
            Ok(_) => {
                for res in ::utils::DATA_CONTENT.iter() {
                    let res_path = data_dir.join_many(res.path);
                    ::std::io::fs::mkdir_recursive(&res_path.dir_path(), ::std::io::USER_DIR).ok();
                    ::std::io::File::create(&res_path).write(res.data.as_bytes()).ok();
                }
                println!("Created data dir at {}", data_dir.as_str().unwrap());
            },
            Err(e) => { println!("Error creating data dir: {}", e) }
        }
    }

    // set $VIM to the data dir if it isn't already set
    if ::std::os::getenv("VIM").is_none() {
        ::std::os::setenv("VIM", data_dir.as_str().unwrap());
    }

    // create config file
    let config_file = home_dir.join(::utils::CONFIG_FILE);
    if !config_file.exists() {
        match ::std::io::File::create(&config_file).write(::utils::CONFIG_CONTENT.as_bytes()) {
            Ok(_) => { println!("Created config file at {}", config_file.as_str().unwrap()) },
            Err(e) => { println!("Error creating config file: {}", e) }
        }
    }

    // takes care of piping stdin/stdout between the gui and nvim
    let mut pty = gtk::VtePty::new().unwrap();

    // two pairs of anonymous pipes for msgpack-rpc between the gui and nvim
    let mut nvim_from_gui : [ffi::c_int, ..2] = [0, ..2];
    let mut gui_from_nvim : [ffi::c_int, ..2] = [0, ..2];
    unsafe {
        ffi::pipe(nvim_from_gui.as_mut_ptr());
        ffi::pipe(gui_from_nvim.as_mut_ptr());
    };

    // split into two processes
    let pid = unsafe { ffi::fork() };

    if pid > 0 { // the gui process
        // listen for messages from nvim
        spawn(proc() {
            // send attach_ui message
            {
                let mut arr = neovim::Array::new();
                arr.add_integer(80);
                arr.add_integer(24);
                let msg = neovim::serialize_request(1, "attach_ui", &arr);
                let msg_ptr = msg.as_slice().as_ptr() as *const ffi::c_void;
                unsafe { ffi::write(nvim_from_gui[1], msg_ptr, msg.len() as ffi::size_t) };
            }

            // listen for messages
            let mut buf : [ffi::c_uchar, ..100] = [0, ..100];
            unsafe {
                loop {
                    let n = ffi::read(gui_from_nvim[0], buf.as_mut_ptr() as *mut ffi::c_void, 100);
                    if n < 0 {
                        break;
                    } else if n > 0 {
                        let msg = ::std::string::String::from_raw_buf_len(buf.as_ptr(), n as uint);
                        println!("Received: {}", msg);
                    }
                }
            }
        });

        // start the gui
        gui_main(&mut pty, gui_from_nvim[0], gui_from_nvim[1], pid);
    } else { // the nvim process
        // prepare this process to be piped into the gui
        pty.child_setup();

        // start nvim
        let mut args = ::std::os::args().clone();
        args.push_all(&["-u".to_string(), config_file.as_str().unwrap().to_string()]);
        neovim::run_with_fds(args, nvim_from_gui[0], gui_from_nvim[1]);
    }
}
