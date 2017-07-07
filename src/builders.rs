use gtk::*;
use vte::Terminal;
use std::path::Path;

pub fn show_builder(ui: &mut ::utils::UI, prefs: &::utils::Prefs) {
    let mut should_show = false;

    if let Some(ref path_str) = prefs.selection {
        if let Some(ref project_path) = ::utils::get_project_path(prefs, Path::new(path_str)) {
            if ::utils::is_project_root(prefs, project_path) {
                if !ui.builders.contains_key(project_path) {
                    let term = Terminal::new();
                    term.show_all();
                    ui.build_terms.add(&term);
                    ui.builders.insert(project_path.clone(), (term, -1));
                }
                if let Some(&(ref term, _)) = ui.builders.get(project_path) {
                    ui.build_terms.set_visible_child(term);
                    should_show = true;
                }
            }
        }
    }

    ui.build_buttons.set_sensitive(should_show);
    if should_show {
        ui.build_terms.show_all();
    } else {
        ui.build_terms.hide();
    }
}

pub fn run_builder(ui: &mut ::utils::UI, prefs: &::utils::Prefs, args: &[&str]) {
    if let Some(project_path) = ::utils::get_selected_project_path(ui, prefs) {
        if let Some(project_path_str) = project_path.to_str() {
            if let Some(&mut(ref mut term, ref mut current_pid)) = ui.builders.get_mut(&project_path) {
                match term.fork_command(project_path_str.as_ref(), args) {
                    Ok(pid) => { *current_pid = pid },
                    Err(s) => {
                        term.feed(s.as_ref());
                        term.feed("\r\n");
                    }
                }
            }
        }
    }
}

fn stop_process(term: &mut Terminal, current_pid: &mut i32) {
    if *current_pid >= 0 {
        ::ffi::kill_process(*current_pid);
        term.feed("===Finished===\r\n");
        *current_pid = -1;
    }
}

pub fn stop_builder(ui: &mut ::utils::UI, prefs: &::utils::Prefs) {
    if let Some(project_path) = ::utils::get_selected_project_path(ui, prefs) {
        if let Some(&mut(ref mut term, ref mut current_pid)) = ui.builders.get_mut(&project_path) {
            stop_process(term, current_pid);
        }
    }
}

pub fn stop_builders(ui: &mut ::utils::UI) {
    for (_, mut builder) in ui.builders.iter_mut() {
        let (ref mut term, ref mut current_pid) : (Terminal, i32) = *builder;
        stop_process(term, current_pid);
    }
}

pub fn set_builders_font_size(ui: &mut ::utils::UI, prefs: &::utils::Prefs) {
    for (_, mut builder) in ui.builders.iter_mut() {
        let (ref mut term, _) : (Terminal, i32) = *builder;
        term.set_font_size(prefs.font_size);
    }
}
