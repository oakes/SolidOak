use rgtk::*;
use std::cell::Cell;
use std::path::Path;

fn create_builder() -> (gtk::VteTerminal, Cell<i32>) {
    let mut term = gtk::VteTerminal::new().unwrap();
    term.show_all();
    (term, Cell::new(-1))
}

pub fn show_builder(state: &mut ::utils::State, build_pane: &mut gtk::Stack) {
    if let Some(ref path_str) = state.selection {
        if let Some(ref project_path) = ::utils::get_project_path(state, Path::new(path_str)) {
            if !state.builders.contains_key(project_path) {
                state.builders.insert(project_path.clone(), create_builder());
            }
            if let Some(&(ref term, _)) = state.builders.get(project_path) {
                if term.get_parent().is_none() {
                    build_pane.add(term);
                } else {
                    build_pane.set_visible_child(term);
                }
            }
        }
    }
}

pub fn run_builder(state: &mut ::utils::State, args: &[&str]) {
    if let Some(project_path) = ::utils::get_selected_project_path(state) {
        if let Some(project_path_str) = project_path.to_str() {
            if let Some(&mut(ref mut term, ref current_pid)) = state.builders.get_mut(&project_path) {
                match term.fork_command(project_path_str.as_slice(), args) {
                    Ok(pid) => current_pid.set(pid),
                    Err(s) => term.feed(s.as_slice())
                }
            }
        }
    }
}

pub fn stop_builder(state: &::utils::State) {
    if let Some(project_path) = ::utils::get_selected_project_path(&state) {
        if let Some(&(_, ref current_pid)) = state.builders.get(&project_path) {
            let pid = current_pid.get();
            if pid >= 0 {
                ::native::kill_process(pid);
            }
            current_pid.set(-1);
        }
    }
}
