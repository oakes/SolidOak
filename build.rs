use std::old_io::Command;
use std::env;

fn main() {
    let curr_dir = Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap());
    Command::new("git").arg("submodule").arg("update").arg("--init").cwd(&curr_dir).status().unwrap();
}
