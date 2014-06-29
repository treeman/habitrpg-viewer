use api::date::Date;

#[deriving(Show, Encodable, Decodable)]
pub struct Habit {
    text: String,
    //attribute: String, // "str" wut?
    priority: f32,
    value: f32,
    notes: String,
    dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
    id: String,
    down: bool,
    up: bool,
    //history: Vec<String>, // TODO
}

