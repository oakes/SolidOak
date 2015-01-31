use std::old_io::Command;
use std::os;

fn main() {
    let curr_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    Command::new("git").arg("submodule").arg("update").arg("--init").cwd(&curr_dir).status().unwrap();
}
