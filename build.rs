use std::process::Command;
use std::env;

fn main() {
    let curr_dir = env::current_dir().unwrap();
    Command::new("git").arg("submodule").arg("update").arg("--init")
        .current_dir(&curr_dir).status().unwrap();
    #[cfg(target_os="windows")]
    println!("cargo:rustc-flags=-L C:\\msys64\\mingw64\\lib");
}
