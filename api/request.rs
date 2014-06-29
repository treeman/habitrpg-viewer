use std::io::File;
use time::now;

use api::id::Id;
use api::conn::get;


pub enum Request {
    Status,
    Party,
    User,
}

impl Request {
    pub fn to_string(&self) -> String {
        match *self {
            Status => "status",
            Party => "party",
            User => "user",
        }.to_string()
    }

    fn id(&self) -> String {
        match *self {
            Status => "status",
            Party => "groups/party",
            User => "user",
        }.to_string()
    }

    // Return Url instead?
    pub fn url(&self) -> String {
        format!("https://beta.habitrpg.com/api/v2/{}", self.id())
    }
}

pub fn fetch(req: Request, cachedir: &Path, id: &Id) -> String {
    let cache_file = cachedir.join(req.to_string());
    //println!("Checking {}", cache_file.display());
    if shall_update(&cache_file) {
        //println!("Sending request to: {}", req.url());

        let content = get(req.url().as_slice(), id);
        let mut f = match File::create(&cache_file) {
            Ok(f) => f,
            Err(e) => fail!("Couldn't create {}: {}", cache_file.display(), e),
        };
        //println!("Updating cache for {} ({})", req.url(), cache_file.display());
        match f.write_str(content.as_slice()) {
            Err(e) => fail!("Write failed: {}", e),
            _ => (),
        };
        content.to_string()
    }
    else {
        //println!("Reading from cache {} ({})", req.url(), cache_file.display());
        match File::open(&cache_file).read_to_str() {
            Ok(s) => s,
            Err(e) => fail!("Failed to read {}: {}", cache_file.display(), e),
        }
    }
}

// Seconds is a good enough resolution.
fn seconds_since_modify(path: &Path) -> Option<u64> {
    match path.stat() {
        Ok(stat) => {
            let mod_ms = stat.modified;
            let now_sec = now().to_timespec().sec as u64;
            Some(now_sec - mod_ms / 1000)
        },
        Err(_) => None,
    }
}

fn minutes_since_modify(path: &Path) -> Option<f32> {
    seconds_since_modify(path).and_then(|sec: u64| { Some(sec as f32 / 60.0) })
}

fn shall_update(path: &Path) -> bool {
    match minutes_since_modify(path) {
        Some(min) => {
            //println!("{} minutes since modify", min);
            min > 10.0
        },
        None => true,
    }
}

