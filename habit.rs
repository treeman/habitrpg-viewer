#![allow(dead_code)]
#![feature(struct_variant)]

#![feature(globs)]
#![feature(macro_rules)]

// For regex usage
#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate serialize;
extern crate core;
extern crate time;

use std::io::File;
use serialize::{json, Decodable};
use core::fmt::{Show};
use time::now;
use std::os;
use std::io;
use std::io::fs;

//use api::conn::get;
use api::id::Id;
//use api::task::*;
use api::user::*;
use api::request::*;

mod api;

fn print<T:Show>(tasks: Vec<T>) {
    for t in tasks.iter() {
        println!("{}", t);
    }
}

fn parse_user(_: &Id) -> User {
    let s = match File::open(&Path::new("data/user")).read_to_str() {
        Ok(v) => v,
        Err(e) => fail!("Failed to read: {}", e)
    };

    //let s = get("https://beta.habitrpg.com/api/v2/user", id);
    let obj = match json::from_str(s.as_slice()) {
        Ok(v) => v,
        Err(e) => fail!("json parse error: {}", e)
    };
    //println!("{}", obj.to_pretty_str());

    let mut decoder = json::Decoder::new(obj.clone());
    let user: User = match Decodable::decode(&mut decoder) {
        Ok(v) => v,
        Err(e) => fail!("Decoding error: {}", e)
    };

    //println!("User");
    //println!("{}", user);

    user
}

fn create_dir(dir: &Path) {
    if dir.is_file() {
        fail!("dir: {} is a file, aborting.", dir.display());
    }
    if !dir.is_dir() {
        println!("Creating dir: {}", dir.display());
        match fs::mkdir(dir, io::UserRWX) {
            Ok(_) => (),
            Err(e) => fail!("Failed to create dir: {}", e),
        };
    }
}

fn main() {
    let homedir = match os::homedir() {
        Some(d) => d,
        None => fail!("Could not find your homedir!"),
    };
    //println!("Your homedir is: {}", homedir.display());

    // TODO make it lazy!
    let configdir = homedir.join(".habitrpg-viewer");
    println!("Config dir: {}", configdir.display());
    create_dir(&configdir);

    let cachedir = configdir.join("cache");
    create_dir(&cachedir);

    let id = Id::from_file(&Path::new("id.json"));
    println!("Registering with");
    println!("  api_token: {}", id.api_token);
    println!("  user_id: {}", id.user_id);

    // Lazy fetching of urls.
    let r = fetch(Party, &cachedir, &id);
    println!("Got: {}", r);

    //if shall_update(&Path::new("habit")) {
        //println!("Yup");
    //}
    return;

    let user = parse_user(&id);

    println!("Found user: {}", user.profile.name);
    println!("  {:u} habits", user.habits.len());
    println!("  {:u} dailys", user.dailys.len());
    println!("  {:u} todos", user.todos.len());

    println!("level {:u} {:s}", user.stats.lvl, user.stats.class);
    println!("   {:u}/{:u} hp", user.stats.hp, user.stats.maxHealth);
    println!("   {:u}/{:u} mp", user.stats.mp, user.stats.maxMP);
    println!("   {:f}/{:f} xp", user.stats.exp, user.stats.toNextLevel);

    //println!("Habits");
    //print(user.habits);
    println!("\nDailys\n-------");
    // TODO better filtering.
    for t in user.dailys.iter() {
        if !t.text.as_slice().starts_with("#") {
            println!("{}", t);
        }
    }
    println!("\nTodos\n-----");
    // TODO better filtering
    for t in user.todos.iter() {
        if !t.completed {
            println!("{}", t);
        }
    }
    //println!("Rewards");
    //print(user.rewards);

    //println!("Server status: {}", get("https://beta.habitrpg.com/api/v2/status", &id));

    //let s = get("https://beta.habitrpg.com/api/v2/groups/party", &id);
    ////println!("Have {}", s);
    //let obj = match json::from_str(s.as_slice()) {
        //Ok(v) => v,
        //Err(e) => fail!("json parse error: {}", e)
    //};
    //println!("{}", obj.to_pretty_str());

    //let mut decoder = json::Decoder::new(json_object);
    //let tasks: Vec<Task> = match Decodable::decode(&mut decoder) {
        //Ok(v) => v,
        //Err(e) => fail!("Decoding error: {}", e)
    //};

    //println!("Found in tasks.json");
    //println!("{}", tasks);
}

