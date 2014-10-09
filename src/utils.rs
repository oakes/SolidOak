use rgtk::*;
use std::collections::HashSet;

pub struct State {
    pub projects: HashSet<String>,
    pub expansions: HashSet<String>,
    pub selection: Option<String>,
}
