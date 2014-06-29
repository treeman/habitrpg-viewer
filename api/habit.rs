use api::date::Date;

#[deriving(Show, Encodable, Decodable)]
pub struct Habit {
    pub text: String,
    //attribute: String, // "str" wut?
    pub priority: f32,
    pub value: f32,
    pub notes: String,
    pub dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
    pub id: String,
    pub down: bool,
    pub up: bool,
    //history: Vec<String>, // TODO
}

