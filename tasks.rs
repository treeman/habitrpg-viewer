use core::fmt::{Show, Formatter, Result};
use serialize::{json, Encodable, Decodable, Decoder, Encoder};
use std::result::Result;
use std::fmt::FormatError;

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

#[deriving(Encodable)]
pub enum TaskType {
    Habit,
    Daily,
    Todo,
    Reward,
}

impl<E, D:Decoder<E>> Decodable<D, E> for TaskType {
    fn decode(d: &mut D) -> Result<TaskType, E> {
        let x = d.read_str().ok().unwrap();
        match x.as_slice() {
            "habit" => Ok(Habit),
            "daily" => Ok(Daily),
            "todo" => Ok(Todo),
            "reward" => Ok(Reward),
            _ => fail!("Missing task type: {}", x),
        }
    }
}

// For now just parse 2014-06-27T18:22:05.834Z
// which is simply hardcoded. Might want to use datetime library when it has been made.
pub struct Date {
    year: uint,
    month: uint,
    day: uint,
    hour: uint,
    min: uint,
    sec: uint,
    ms: uint,
}

impl Date {
    // Not very stable, but works atm?
    pub fn from_str(s: &str) -> Option<Date> {
        let re = regex!(r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})\.(\d{3})Z");
        let caps = re.captures(s);
        caps.and_then(|x| {
            Some(Date {
                year: from_str(x.at(1)).unwrap(),
                month: from_str(x.at(2)).unwrap(),
                day: from_str(x.at(3)).unwrap(),
                hour: from_str(x.at(4)).unwrap(),
                min: from_str(x.at(5)).unwrap(),
                sec: from_str(x.at(6)).unwrap(),
                ms: from_str(x.at(7)).unwrap(),
            })
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}-{:0>2u}-{:0>2u}T{:0>2u}:{:0>2u}:{:0>2u}.{:0>3u}Z",
                self.year, self.month, self.day,
                self.hour, self.min, self.sec, self.ms)
    }
}

impl<E, D:Decoder<E>> Decodable<D, E> for Date {
    fn decode(d: &mut D) -> Result<Date, E> {
        d.read_str().and_then(|x| {
            match Date::from_str(x.as_slice()) {
                Some(x) => Ok(x),
                None => fail!("Failed to parse Date from {}", x)
            }
        })
    }
}

impl<E, S:Encoder<E>> Encodable<S, E> for Date {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_str(self.to_string().as_slice())
    }
}

impl Show for Date {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}", self.to_string().as_slice())
    }
}

#[deriving(Encodable)]
pub struct Task {
    text: String,
    attribute: String, // "str" wut?
    priority: f32,
    value: f32,
    notes: String,
    dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
    id: String,
    down: bool,
    up: bool,
    history: Vec<String>, // TODO
    ty: TaskType, // Might want to have Task be an enumeration type itself?
    //tags: Tags,

    //completed: bool, // dailies
    //streak: uint,
    //checklist: Vec<CheckItem>,
    //repeat: Repeat,
}

fn require<E, D:Decoder<E>, T:Decodable<D, E>>(d: &mut D, field: &str) -> T {
    d.read_struct_field(field, 0, |d: &mut D| {
        // Also possible: d.read_uint()
        // etc. But this is general for all Decodables!
        Decodable::decode(d)
    }).ok().unwrap()
}

impl<E, D:Decoder<E>> Decodable<D, E> for Task {
    fn decode(d: &mut D) -> Result<Task, E> {
        d.read_struct("Task", 1,
            |d: &mut D| -> Result<Task, E> {
                Ok(Task {
                    text: require(d, "text"),
                    attribute: require(d, "attribute"),
                    priority: require(d, "priority"),
                    value: require(d, "value"),
                    notes: require(d, "notes"),
                    dateCreated: require(d, "dateCreated"),
                    id: require(d, "id"),
                    down: require(d, "down"),
                    up: require(d, "up"),
                    ty: require(d, "type"),
                    //ty: require(d, "ty"),
                    history: require(d, "history"),
                })
            }
        )
    }
}

impl Show for Task {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "Task: ");
        let mut encoder = json::PrettyEncoder::new(f);
        self.encode(&mut encoder);
        Ok(())
    }
}

