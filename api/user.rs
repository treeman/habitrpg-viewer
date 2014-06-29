// TODO cleanup!
//
//use core::fmt::{Show, Formatter};
use serialize::{Encodable, Decoder, Decodable, Encoder, json};
//use std::result::Result;
//use std::fmt::FormatError;

use api::date::*;
use api::achievements::*;
use api::party::*;
use api::stats::*;
use api::tag::*;
use api::habit::*;
use api::daily::*;
use api::todo::*;
use api::reward::*;
use api::id::*;
use api::request;
use api::request::fetch;

#[deriving(Show, Encodable, Decodable)]
pub struct Items {
    pub currentMount: String,
    pub currentPet: String,
}

#[deriving(Show, Encodable, Decodable)]
pub struct Profile {
    pub name: String,
}

// TODO custom Show
#[deriving(Show, Encodable, Decodable)]
pub struct User {
    pub achievements: Achievements,

    // Made private so we can filter away unwanted things.
    habits: Vec<Habit>,
    dailys: Vec<Daily>,
    todos: Vec<Todo>,
    rewards: Vec<Reward>,

    // filters?
    //flags: Flags, // skip?
    // history exp/todo
    pub id: String,
    // invitations
    pub items: Items,
    pub lastCron: Date,
    // newMessages ?
    pub party: Party,
    // preferences
    // profile (name...)
    pub profile: Profile,
    pub stats: Stats,
    pub tags: Vec<Tag>, // TODO dictionary
}

impl User {
    // Fetch struct from habit
    pub fn fetch(cachedir: &Path, id: &Id) -> User {
        let o = fetch(request::User, cachedir, id);

        let obj = match json::from_str(o.as_slice()) {
            Ok(v) => v,
            Err(e) => fail!("json parse error: {}", e)
        };

        let mut decoder = json::Decoder::new(obj.clone());
        let user: User = match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => fail!("Decoding error: {}", e)
        };

        user
    }

    pub fn print_char(&self) {
        println!("{:s}, level {:u} {:s}", self.profile.name, self.stats.lvl, self.stats.class)
    }

    pub fn print_char_stats(&self) {
        println!("  {:u}/{:u} hp", self.stats.hp, self.stats.maxHealth);
        println!("  {:u}/{:u} mp", self.stats.mp, self.stats.maxMP);
        println!("  {:f}/{:f} xp", self.stats.exp, self.stats.toNextLevel);
    }

    // Filter out separators, starting with #, which I use as delimiters in habitrpg website.
    pub fn dailys<'a>(&'a self) -> Vec<&'a Daily> {
        self.dailys.iter().filter(|t: &&Daily| {
            !t.text.as_slice().starts_with("#")
        }).collect()
    }

    pub fn habits<'a>(&'a self) -> Vec<&'a Habit> {
        self.habits.iter().filter(|t: &&Habit| {
            !t.text.as_slice().starts_with("#")
        }).collect()
    }

    // Fetch unfinished todos as opposed to all todos.
    pub fn unfinished_todos<'a>(&'a self) -> Vec<&'a Todo> {
        self.todos.iter().filter(|t: &&Todo| {
            !t.completed
        }).collect()
    }

    pub fn print_todays_stats(&self) {
        // TODO
    }

    pub fn print_task_stats(&self) {
        println!("  {:u} habits", self.habits.len());
        println!("  {:u} dailys", self.dailys.len());
        println!("  {:u} todos", self.unfinished_todos().len());
    }
}

