#![allow(dead_code)]

use std::path::Path;
use std::fs::{ File, PathExt };
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use time;
use time::{ Timespec };

use habitrpg;
use habitrpg::{ User, Party };
use env::Env;

/// Cache url requests.
pub fn get_user(env: &Env) -> User {
    let cachefile = env.cachedir.join("user.json");

    if is_old(&cachefile) {
        let data = habitrpg::get_user_response(&env.id);
        let user: User = habitrpg::from_str(&data);
        // TODO if ok, save file.
        write_file(&cachefile, &data);
        user
    } else {
        habitrpg::from_path(&cachefile)
    }
}

pub fn get_party(env: &Env) -> Party {
    let cachefile = env.cachedir.join("party.json");

    if is_old(&cachefile) {
        let data = habitrpg::get_party_response(&env.id);
        let party: Party = habitrpg::from_str(&data);
        // TODO if ok, save file.
        write_file(&cachefile, &data);
        party
    } else {
        habitrpg::from_path(&cachefile)
    }
}

// TODO error handling!
fn write_file(path: &Path, content: &str) {
    let mut f = match File::create(path) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't create {}: {}", path.display(), e),
    };
    match f.write_all(content.as_bytes()) {
        Err(e) => panic!("Write failed: {}", e),
        _ => (),
    };
}

/// For now check if the given file is older than 5 minutes.
///
/// Or if it doesn't exist, it's old :)
fn is_old(path: &Path) -> bool {
    match path.metadata() {
        Ok(m) => {
            let last_mod = Timespec::new(m.mtime() as i64 / 1000, 0);
            let now = time::get_time();
            let duration = now - last_mod;
            duration.num_minutes() >= 5
        },
        Err(_) => true,
    }
}

