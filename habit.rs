#![feature(globs)]

extern crate serialize;
extern crate core;

use conn::get;
use id::Id;
use tasks::*;
use std::io::File;
use serialize::{json, Decodable};

mod conn;
mod id;
mod tasks;

fn main() {
    let id = Id::from_file(&Path::new("id.json"));
    println!("Registering with");
    println!("  api_token: {}", id.api_token);
    println!("  user_id: {}", id.user_id);

    //println!("Server status: {}", get("https://beta.habitrpg.com/api/v2/status", &id));

    //println!("Tasks: {}", get("https://beta.habitrpg.com/api/v2/user/tasks", &id));

    let contents = match File::open(&Path::new("tasks.json")).read_to_str() {
        Ok(v) => v,
        Err(e) => fail!("Failed to read: {}", e)
    };

    let json_object = match json::from_str(contents.as_slice()) {
        Ok(v) => v,
        Err(e) => fail!("json parse error: {}", e)
    };
    println!("Have: {}", json_object.to_pretty_str());
    let mut decoder = json::Decoder::new(json_object);
    let task: Task = match Decodable::decode(&mut decoder) {
        Ok(v) => v,
        Err(e) => fail!("Decoding error: {}", e)
    };

    println!("Found in tasks.json");
    println!("{}", task);
}
