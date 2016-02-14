use std::process::Command;
use std::env;

fn main() {
    let curr_dir = env::current_dir().unwrap();
    Command::new("git").arg("submodule").arg("update").arg("--init")
        .current_dir(&curr_dir).status().unwrap();
}
