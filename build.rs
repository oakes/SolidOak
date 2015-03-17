use std::path::Path;
use std::process::Command;
use std::env;

fn main() {
    let curr_path_str = env::var("CARGO_MANIFEST_DIR").unwrap();
    let curr_path = Path::new(&curr_path_str);
    Command::new("git").arg("submodule").arg("update").arg("--init")
        .current_dir(&curr_path).status().unwrap();
}
