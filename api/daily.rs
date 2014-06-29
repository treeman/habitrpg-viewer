use core::fmt::{Show, Formatter, FormatError};

use api::date::Date;
use api::clean_text;

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
    // repeat
    pub completed: bool,
    //history: Vec<String>, // TODO
}

impl Show for Daily {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let mut s = format!("{:s} ", clean_text(self.text.as_slice()));
        if self.completed {
            s = s.append("(Done)");
        };
        write!(f, "{:s}", s)
    }
}

