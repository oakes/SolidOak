use rgtk::*;
use serialize::json;
use std::collections::HashSet;
use std::io::fs;
use std::os::homedir;

pub struct State<'a> {
    pub projects: HashSet<String>,
    pub expansions: HashSet<String>,
    pub selection: Option<String>,
    pub tree_store: &'a gtk::TreeStore,
}

#[deriving(Decodable, Encodable)]
struct Prefs {
    projects: Vec<String>,
    expansions: Vec<String>,
    selection: Option<String>
}

fn get_data_dir() -> Path {
    let home = homedir();
    let mut path = if home.is_some() {
        home.unwrap()
    } else {
        Path::new(".")
    };
    path.push(".solidoak");
    path
}

fn get_prefs(state: &State) -> Prefs {
    Prefs {
        projects: state.projects.clone().into_iter().collect(),
        expansions: state.expansions.clone().into_iter().collect(),
        selection: state.selection.clone()
    }
}

pub fn write_prefs(state: &State) {
    let prefs = get_prefs(state);
    let json_str = json::encode(&prefs);

    let mut prefs_path = get_data_dir();
    match fs::mkdir(&prefs_path, ::std::io::USER_DIR) {
        Ok(_) => {},
        Err(_) => {}
    };

    prefs_path.push("prefs.json");
    let mut f = fs::File::create(&prefs_path);
    match f.write_str(json_str.as_slice()) {
        Ok(_) => {},
        Err(e) => println!("Error writing prefs: {}", e)
    };
}

pub fn read_prefs(state: &mut State) {
    let mut prefs = get_data_dir();
    prefs.push("prefs.json");

    let mut f = fs::File::open(&prefs);
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

    if prefs_option.is_some() {
        let prefs = prefs_option.unwrap();

        state.projects.clear();
        for path in prefs.projects.iter() {
            state.projects.insert(path.clone());
        }

        state.expansions.clear();
        for path in prefs.expansions.iter() {
            state.expansions.insert(path.clone());
        }

        state.selection = prefs.selection;
    }
}
