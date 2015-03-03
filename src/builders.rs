use rgtk::*;
use std::path::Path;

fn create_builder(project_path_str: &str) -> gtk::Box {
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

    let mut build_term = gtk::VteTerminal::new().unwrap();

    let mut builder = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    builder.add(&build_buttons);
    builder.pack_start(&build_term, true, true, 0);

    run_button.connect(gtk::signals::Clicked::new(&mut || {
        match build_term.fork_command(project_path_str, &["cargo", "run"]) {
            Ok(_) => {},
            Err(s) => println!("{}", s)
        }
    }));

    builder.show_all();
    builder
}

pub fn show_builder(state: &mut ::utils::State, build_pane: &mut gtk::Stack) {
    if let Some(ref path_str) = state.selection {
        if let Some(ref project_path) = ::utils::get_project_path(state, Path::new(path_str)) {
            if let Some(project_path_str) = project_path.to_str() {
                if !state.builders.contains_key(project_path) {
                    state.builders.insert(project_path.clone(), create_builder(project_path_str));
                }
                if let Some(builder) = state.builders.get(project_path) {
                    if builder.get_parent().is_none() {
                        build_pane.add_named(builder, project_path_str.as_slice());
                    } else {
                        build_pane.set_visible_child(builder);
                    }
                }
            }
        }
    }
}
