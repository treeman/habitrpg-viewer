#![feature(globs)]

extern crate serialize;
extern crate core;

use serialize::{json, Decodable};
use std::io::File;
use std::io::process::Command;
use std::*;
use core::fmt::{Show, Formatter, Result};

// API_TOKEN and USER_ID for habitrpg identifiers.
#[deriving(Decodable)]
pub struct Id  {
    api_token: String,
    user_id: String,
}

impl Show for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "api_token: {} user_id: {}", self.api_token, self.user_id)
    }
}

impl Id {
    pub fn from_file(path: &Path) -> Id {
        let contents = match File::open(path).read_to_str() {
            Ok(v) => v,
            Err(e) => fail!("Failed to read: {}", e)
        };

        let json_object = match json::from_str(contents.as_slice()) {
            Ok(v) => v,
            Err(e) => fail!("json parse error: {}", e)
        };
        let mut decoder = json::Decoder::new(json_object);
        let res = match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => fail!("Decoding error: {}", e)
        };

        res
    }
}

// Wrap GET requests and use curl.
// Did not find a stable library for it.
fn get(url: &str, id: &Id) -> String {
    let output = match Command::new("curl")
        .arg("-X").arg("GET")
        .arg("-H").arg(format!("x-api-key: {}", id.api_token))
        .arg("-H").arg(format!("x-api-user: {}", id.user_id))
        .arg(url).output()
    {
        Ok(output) => output,
        Err(e) => fail!("failed to execute process: {}", e),
    };

    let out = str::from_utf8_lossy(output.output.as_slice()).to_string();

    //println!("status: {}", output.status);
    //println!("stdout: {}", out);
    //println!("stderr: {}", str::from_utf8_lossy(output.error.as_slice()));

    out
}

fn main() {
    let id = Id::from_file(&Path::new("id.json"));
    println!("Registering with");
    println!("  api_token: {}", id.api_token);
    println!("  user_id: {}", id.user_id);

    //println!("Server status: {}", get("https://beta.habitrpg.com/api/v2/status", &id));

    println!("Tasks: {}", get("https://beta.habitrpg.com/api/v2/user/tasks", &id));
}

