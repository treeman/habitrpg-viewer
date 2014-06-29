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

use std::io::File;
use serialize::{json, Decodable};
use core::fmt::{Show};

//use api::conn::get;
use api::id::Id;
use api::task::*;
use api::user::*;

mod api;

fn print<T:Show>(tasks: Vec<T>) {
    for t in tasks.iter() {
        println!("{}", t);
    }
}

// From a list of tasks.
fn get_tasks(o: &json::Json) -> Vec<Task> {
    //println!("Have: {}", json_object.to_pretty_str());
    let mut tasks = Vec::new();
    match o.clone() {
        json::List(x) => {
            for t in x.iter() {
                //println!("Parsing:");
                //println!("{}", t.to_pretty_str());
                let mut decoder = json::Decoder::new(t.clone());
                let res: Task = match Decodable::decode(&mut decoder) {
                    Ok(v) => v,
                    Err(e) => fail!("Decoding error: {}", e)
                };
                //println!("Res:");
                //println!("{}", res);
                tasks.push(res);
            };
        },
        _ => (),
    };
    tasks
}

fn filter_habits<'a>(tasks: &'a Vec<Task>) -> Vec<&'a Task> {
    tasks.iter().filter(|x: &&Task| {
        match *x.clone() {
            Habit { .. } => true,
            _ => false,
        }
    }).collect()
}
fn filter_dailies<'a>(tasks: &'a Vec<Task>) -> Vec<&'a Task> {
    tasks.iter().filter(|x: &&Task| {
        match *x.clone() {
            Daily { .. } => true,
            _ => false,
        }
    }).collect()
}
fn filter_todos<'a>(tasks: &'a Vec<Task>) -> Vec<&'a Task> {
    tasks.iter().filter(|x: &&Task| {
        match *x.clone() {
            Todo { .. } => true,
            _ => false,
        }
    }).collect()
}
fn filter_rewards<'a>(tasks: &'a Vec<Task>) -> Vec<&'a Task> {
    tasks.iter().filter(|x: &&Task| {
        match *x.clone() {
            Reward { .. } => true,
            _ => false,
        }
    }).collect()
}

fn print_dailies(o: &json::Json) {
    let tasks = get_tasks(o);
    let dailies = filter_dailies(&tasks);
    print(dailies);
}

fn test_tasks() {
    let contents = match File::open(&Path::new("user-tasks")).read_to_str() {
        Ok(v) => v,
        Err(e) => fail!("Failed to read: {}", e)
    };

    let obj = match json::from_str(contents.as_slice()) {
        Ok(v) => v,
        Err(e) => fail!("json parse error: {}", e)
    };

    let tasks = get_tasks(&obj);
    let habits = filter_habits(&tasks);
    let dailies = filter_dailies(&tasks);
    let todos = filter_todos(&tasks);
    let rewards = filter_rewards(&tasks);

    println!("Habits");
    print(habits);
    println!("Dailies");
    print(dailies);
    println!("Todos");
    print(todos);
    println!("Rewards");
    print(rewards);
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

fn main() {
    let id = Id::from_file(&Path::new("id.json"));
    println!("Registering with");
    println!("  api_token: {}", id.api_token);
    println!("  user_id: {}", id.user_id);

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
    //println!("Dailies");
    //print(user.dailys);
    //println!("Todos");
    //print(user.todos);
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

