// TODO cleanup!
//
//use core::fmt::{Show, Formatter};
//use serialize::{Encodable, Decodable, Decoder, Encoder};
use serialize::{Encodable, Decoder, Encoder};
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
    pub habits: Vec<Habit>,
    pub dailys: Vec<Daily>,
    pub todos: Vec<Todo>,
    pub rewards: Vec<Reward>,
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

