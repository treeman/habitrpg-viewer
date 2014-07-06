use core::fmt::{Show, Formatter, FormatError};
use time::*;

use api::date::Date;
use api::clean_text;
use api::repeat::Repeat;

#[deriving(Encodable, Decodable)]
pub struct Daily {
    pub text: String,
    //attribute: String, // "str" Some value...
    pub priority: f32,
    pub value: f32,
    pub notes: String,
    pub dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
    pub id: String,
    pub streak: uint,
    // checklist
    // collapseChecklist
    pub repeat: Repeat,
    pub completed: bool,
    //history: Vec<String>, // TODO
}

impl Daily {
    pub fn due_today(&self) -> bool {
        let t = now();
        match t.tm_wday {
            0 => self.repeat.su,
            1 => self.repeat.m,
            2 => self.repeat.t,
            3 => self.repeat.w,
            4 => self.repeat.th,
            5 => self.repeat.f,
            6 => self.repeat.s,
            _ => fail!("Tm.tm_wday errorenous: {}", t.tm_wday),
        }
    }
}

impl Show for Daily {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let s = format!("{:s} ", clean_text(self.text.as_slice()));
        write!(f, "{:s}", s)
    }
}

