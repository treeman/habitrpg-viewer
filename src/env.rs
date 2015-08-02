use std::path::{ Path, PathBuf };
use std::fs::PathExt;
//use std::os::unix::fs::*;
use std::fs;
use std::env;

use habitrpg;
use habitrpg::{ Id };

pub struct Env {
    pub configdir: PathBuf,
    pub cachedir: PathBuf,
    pub id: Id,
    pub conky: bool, // TODO refactor away
}

impl Env {
    /// Config directory defaults to ~/.habitrpg-viewer
    pub fn new() -> Env {
        let homedir = match env::home_dir() {
            Some(d) => d,
            None => panic!("Could not find your homedir!"),
        };

        let configdir = homedir.join(".habitrpg-viewer");
        let cachedir = configdir.join("cache");

        create_dir(&configdir);
        create_dir(&cachedir);

        let id: Id = habitrpg::from_path(&configdir.join("id.json"));

        Env {
            configdir: configdir,
            cachedir: cachedir,
            id: id,
            conky: false,
        }
    }
}

// TODO make it recursive.
fn create_dir(dir: &Path) {
    if dir.is_file() {
        panic!("dir: {} is a file, aborting.", dir.display());
    }
    if !dir.is_dir() {
        println!("Creating dir: {}", dir.display());
        match fs::create_dir_all(dir) {
            Ok(_) => {},
            Err(e) => panic!("Failed to create dir {}: {}", dir.display(), e),
        }
    }
}

