use walkdir::{self, WalkDir};
use std::{env, io, process};
use std::fs::Metadata;


fn main() -> std::io::Result<()> {
    let rcm_names = vec!("RC-M", "RCM");
    let mut entry2 = String::new();
    for entry in WalkDir::new(env::current_dir()?) {
        match &entry {
            Err(_) => continue,
            Ok(inside) => { entry2 = inside.path().display().to_string(); }
        }

        for name in &rcm_names {
            if entry2.contains(name) {
                println!("Possible RC-M found. File name: {}", entry.as_ref().unwrap().path().display());
            }
        }
    }
    println!("Search finished.");
    io::Result::Ok(())
}
