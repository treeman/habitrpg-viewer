use core::fmt::{Show, Formatter, Result};
use serialize::{json, Encodable, Decodable};

#[deriving(Decodable)]
pub struct HistVal  {
    value: f32,
    date: String, // Date
}

#[deriving(Decodable)]
pub struct CheckItem  {
    text: String,
    id: String, // Id?
    completed: bool,
}

#[deriving(Decodable)]
pub struct Repeat  {
    su: bool,
    s: bool,
    f: bool,
    th: bool,
    w: bool,
    t: bool,
    m: bool
}

//#[deriving(Decodable)]
//pub struct Tags  {
    //// List of id: bool
//}


#[deriving(Decodable, Encodable)]
pub struct Task  {
    text: String,
    attribute: String, // "str" wut?
    priority: f32,
    value: f32,
    notes: String,
    dateCreated: String, // "2014-06-27T18:22:05.834Z", can decode
    id: String,
    down: bool,
    up: bool,
    history: Vec<String>,
    //tst: Option<String>,

    //type: String, // "habit", "reward", "daily", "todo"
    //tags: Tags,

    //completed: bool, // dailies
    //streak: uint,
    //checklist: Vec<CheckItem>,
    //repeat: Repeat,
}

impl Show for Task {
    fn fmt(&self, f: &mut Formatter) -> Result {
        //write!(f, "Task: {}", self.text)
        //write!(f, "api_token: {} user_id: {}", self.api_token, self.user_id)
        let encoded_str: String = json::Encoder::str_encode(self);
        write!(f, "Task: {}", encoded_str)
    }
}

