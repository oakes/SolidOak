use rgtk::*;
use serialize::json;
use std::collections::HashSet;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::os::homedir;

pub struct State<'a> {
    pub projects: HashSet<String>,
    pub expansions: HashSet<String>,
    pub selection: Option<String>,
    pub tree_store: &'a gtk::TreeStore,
    pub tree_model: &'a gtk::TreeModel,
    pub tree_selection: &'a gtk::TreeSelection,
    pub rename_button: &'a gtk::Button,
    pub remove_button: &'a gtk::Button,
}

#[deriving(Decodable, Encodable)]
struct Prefs {
    projects: Vec<String>,
    expansions: Vec<String>,
    selection: Option<String>
}

fn get_data_dir() -> Path {
    let home = homedir();
    let mut path = match home {
        Some(p) => p,
        None => Path::new(".")
    };
    path.push(".solidoak");
    path
}

fn get_prefs_file() -> Path {
    let mut path = get_data_dir();
    path.push("prefs.json");
    path
}

fn get_prefs(state: &State) -> Prefs {
    Prefs {
        projects: state.projects.clone().into_iter().collect(),
        expansions: state.expansions.clone().into_iter().collect(),
        selection: state.selection.clone()
    }
}

pub fn are_siblings(path1: &String, path2: &String) -> bool {
    let parent_path1 = Path::new(path1).dir_path();
    let parent_path2 = Path::new(path2).dir_path();

    let parent1 = parent_path1.as_str();
    let parent2 = parent_path2.as_str();

    parent1.is_some() && parent2.is_some() &&
    parent1.unwrap() == parent2.unwrap()
}

pub fn get_selected_path(state: &State) -> Option<String> {
    let mut iter = gtk::TreeIter::new().unwrap();
    state.tree_selection.get_selected(state.tree_model, &mut iter);
    let path = state.tree_model.get_value(&iter, 1).get_string();
    iter.drop();
    path
}

pub fn create_data_dir() {
    let path = get_data_dir();
    if !path.exists() {
        match fs::mkdir(&path, ::std::io::USER_DIR) {
            Ok(_) => {},
            Err(e) => { println!("Error creating directory: {}", e) }
        };
    }
}

pub fn write_prefs(state: &State) {
    let prefs = get_prefs(state);
    let json_str = json::encode(&prefs);

    let prefs_path = get_prefs_file();
    let mut f = fs::File::create(&prefs_path);
    match f.write_str(json_str.as_slice()) {
        Ok(_) => {},
        Err(e) => println!("Error writing prefs: {}", e)
    };
}

pub fn read_prefs(state: &mut State) {
    let prefs_path = get_prefs_file();
    let mut f = fs::File::open(&prefs_path);
    let prefs_option : Option<Prefs> = match f.read_to_string() {
        Ok(json_str) => {
            match json::decode(json_str.as_slice()) {
                Ok(object) => Some(object),
                Err(e) => {
                    println!("Error decoding prefs: {}", e);
                    None
                }
            }
        },
        Err(_) => None
    };

    match prefs_option {
        Some(prefs) => {
            state.projects.clear();
            for path in prefs.projects.iter() {
                state.projects.insert(path.clone());
            }

            state.expansions.clear();
            for path in prefs.expansions.iter() {
                state.expansions.insert(path.clone());
            }

            state.selection = prefs.selection;
        },
        None => {}
    };
}
