use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy the config.toml file to the output directory
    fs::copy("config.toml", out_dir.join("config.toml")).unwrap();

    // Print the path to the config.toml file
    println!("cargo:rerun-if-changed=config.toml");
}
