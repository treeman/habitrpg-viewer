use std::io;
use std::io::fs::{ mod, PathExtensions };
use std::os;

use habitrpg::{ mod, Id };

pub struct Env {
    pub configdir: Path,
    pub cachedir: Path,
    pub id: Id,
}

impl Env {
    /// Config directory defaults to ~/.habitrpg-viewer
    pub fn new() -> Env {
        let homedir = match os::homedir() {
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
        match fs::mkdir(dir, io::USER_RWX) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create dir: {}", e),
        };
    }
}

