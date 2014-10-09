use rgtk::*;
use std::collections::HashSet;

pub struct State<'a> {
    pub projects: HashSet<String>,
    pub expansions: HashSet<String>,
    pub selection: Option<String>,
    pub tree_model: &'a gtk::TreeModel,
}
