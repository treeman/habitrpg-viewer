// TODO cleanup!
//
//use core::fmt::{Show, Formatter};
//use serialize::{Encodable, Decodable, Decoder, Encoder};
use serialize::{Encodable, Decoder, Encoder};
//use std::result::Result;
//use std::fmt::FormatError;

use api::date::Date;
use api::task::Task;

#[deriving(Show, Encodable, Decodable)]
pub struct Achievements {
    pub beastMaster: bool,
    //challenges: Vec<?>,
    pub perfect: uint,
    //quests: {
        //"id": uint,
    //},
    pub streak: uint,
    pub ultimateGear: bool,
}

//pub struct Flags { // Skip?
    //classSelected: bool,
    //...
//}

#[deriving(Show, Encodable, Decodable)]
pub struct Items {
    pub currentMount: String,
    pub currentPet: String,
    // eggs
    // food
    // gear
    // hatchingPotions
    // lastDrop ?
    // mounts
    // pets
    // quests ?
    // special ?
}

// TODO move
#[deriving(Show, Encodable, Decodable)]
pub struct Quest {
    // completed: "null" wut?
    pub key: String,
    // progess?
}

// TODO should parse from other place?
#[deriving(Show, Encodable, Decodable)]
pub struct Party {
    // ordering
    pub quest: Quest,
}

// TODO what for?
#[deriving(Show, Encodable, Decodable)]
pub enum Attribute {
    Strength,
    Constituion,
    Intelligence,
    Perception,
}

//#[deriving(Show, Encodable, Decodable)]
//pub struct Buffs {
    //constitution: uint,
    //intelligence: uint,
    //perception: uint,
    //strength: uint,
    //stealth: uint,
    //// snowball: bool ?
    //// streaks: bool ?
//}

// TODO custom Show
#[deriving(Show, Encodable, Decodable)]
pub struct Stats {
    //buffs: Buffs,
    pub class: String, // Or class...
    pub con: uint,
    pub int: uint,
    pub per: uint,
    pub str: uint,
    pub exp: f32,
    pub gp: f32,
    pub hp: uint,
    pub mp: uint,
    pub maxHealth: uint,
    pub maxMP: uint,
    pub lvl: uint,
    pub points: uint, // ??
    pub toNextLevel: f32, // uint?
    // training ?
}

// Could instead use a HashMap?
#[deriving(Show, Encodable, Decodable)]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[deriving(Show, Encodable, Decodable)]
pub struct Profile {
    pub name: String,
}

// TODO custom Show
#[deriving(Show, Encodable, Decodable)]
pub struct User {
    pub achievements: Achievements,
    pub dailys: Vec<Task>, // w00t?
    pub habits: Vec<Task>,
    pub todos: Vec<Task>,
    pub rewards: Vec<Task>,
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

