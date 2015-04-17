use gtk::traits::*;
use gtk::widgets;
use std::cell::Cell;
use std::path::Path;

pub fn show_builder(state: &mut ::utils::State, build_buttons: &mut widgets::Box, build_terms: &mut widgets::Stack) {
    let mut should_show = false;

    if let Some(ref path_str) = state.selection {
        if let Some(ref project_path) = ::utils::get_project_path(state, Path::new(path_str)) {
            if ::utils::is_project_root(state, project_path) {
                if !state.builders.contains_key(project_path) {
                    let mut term = widgets::VteTerminal::new().unwrap();
                    term.show_all();
                    build_terms.add(&term);
                    state.builders.insert(project_path.clone(), (term, Cell::new(-1)));
                }
                if let Some(&(ref term, _)) = state.builders.get(project_path) {
                    build_terms.set_visible_child(term);
                    should_show = true;
                }
            }
        }
    }

    build_buttons.set_sensitive(should_show);
    if should_show {
        build_terms.show_all();
    } else {
        build_terms.hide();
    }
}

pub fn run_builder(state: &mut ::utils::State, args: &[&str]) {
    if let Some(project_path) = ::utils::get_selected_project_path(state) {
        if let Some(project_path_str) = project_path.to_str() {
            if let Some(&mut(ref mut term, ref current_pid)) = state.builders.get_mut(&project_path) {
                match term.fork_command(project_path_str.as_ref(), args) {
                    Ok(pid) => current_pid.set(pid),
                    Err(s) => {
                        term.feed(s.as_ref());
                        term.feed("\r\n");
                    }
                }
            }
        }
    }
}

fn stop_process(term: &mut widgets::VteTerminal, current_pid: &Cell<i32>) {
    let pid = current_pid.get();
    if pid >= 0 {
        ::ffi::kill_process(pid);
        term.feed("===Finished===\r\n");
        current_pid.set(-1);
    }
}

pub fn stop_builder(state: &mut ::utils::State) {
    if let Some(project_path) = ::utils::get_selected_project_path(&state) {
        if let Some(&mut(ref mut term, ref current_pid)) = state.builders.get_mut(&project_path) {
            stop_process(term, current_pid);
        }
    }
}

pub fn stop_builders(state: &mut ::utils::State) {
    for (_, mut builder) in state.builders.iter_mut() {
        let (ref mut term, ref current_pid) : (widgets::VteTerminal, Cell<i32>) = *builder;
        stop_process(term, current_pid);
    }
}

pub fn set_builders_font_size(state: &mut ::utils::State) {
    for (_, mut builder) in state.builders.iter_mut() {
        let (ref mut term, _) : (widgets::VteTerminal, Cell<i32>) = *builder;
        term.set_font_size(state.font_size);
    }
}
