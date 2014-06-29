use core::fmt::{Show, Formatter};
use serialize::{Encodable, Decodable, Decoder, Encoder};
use std::result::Result;
use std::fmt::FormatError;

use api::date::Date;

// TODO what for?
#[deriving(Show, Encodable, Decodable)]
pub enum Attribute {
    Strength,
    Constituion,
    Intelligence,
    Perception,
}


#[deriving(Show, Encodable)]
pub enum TaskType {
    HabitType,
    DailyType,
    TodoType,
    RewardType,
}

impl<E, D:Decoder<E>> Decodable<D, E> for TaskType {
    fn decode(d: &mut D) -> Result<TaskType, E> {
        let x = match d.read_str() {
            Ok(x) => x,
            Err(_) => fail!("Failed to read task type"),
        };
        match x.as_slice() {
            "habit" => Ok(HabitType),
            "daily" => Ok(DailyType),
            "todo" => Ok(TodoType),
            "reward" => Ok(RewardType),
            _ => fail!("Decoding missing task type: {}", x),
        }
    }
}

// TODO remove enum struct types,
// make Habit, Daily, Todo, Reward classes instead!
#[deriving(Encodable)]
pub enum Task {
    Habit {
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
    },

    Daily {
        text: String,
        //attribute: String, // "str" Some value...
        priority: f32,
        value: f32,
        notes: String,
        dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
        id: String,
        streak: uint,
        // checklist
        // collapseChecklist
        // repeat
        completed: bool,
        //history: Vec<String>, // TODO
    },

    Todo {
        text: String,
        //attribute: String, // "str" wut?
        priority: f32,
        value: f32,
        notes: String,
        dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
        id: String,
        // checklist
        // collapseChecklist
        completed: bool,
        // archived: bool,
        // dateCompleted: Date,
        // challenge?
    },

    Reward {
        text: String,
        //attribute: String, // "str" wut?
        priority: f32,
        cost: f32,
        notes: String,
        dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
        id: String,
    },
}

impl<E, D:Decoder<E>> Decodable<D, E> for Task {
    fn decode(d: &mut D) -> Result<Task, E> {
        d.read_struct("Task", 1,
            |d: &mut D| -> Result<Task, E> {
                let t: TaskType = require(d, "type");
                match t {
                    HabitType =>
                        Ok(Habit {
                            text: require(d, "text"),
                            priority: require(d, "priority"),
                            value: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                            down: require(d, "down"),
                            up: require(d, "up"),
                        }),
                    DailyType =>
                        Ok(Daily {
                            text: require(d, "text"),
                            priority: require(d, "priority"),
                            value: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                            streak: require(d, "streak"),
                            completed: require(d, "completed"),
                        }),
                    TodoType =>
                        Ok(Todo {
                            text: require(d, "text"),
                            priority: require(d, "priority"),
                            value: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                            completed: require(d, "completed"),
                            //archived: require(d, "archived"),
                            //archived: false,
                            //archived: {
                                ////let res = default(d, "archived", false);
                                //let res = match d.read_struct_field("archived", 0, |d: &mut D| {
                                        //// Also possible: d.read_uint()
                                        //// etc. But this is general for all Decodables!
                                        ////Decodable::decode(d)
                                        //d.read_bool()
                                    //})
                                //{
                                    //Ok(s) => s,
                                    //Err(e) => false,
                                //};
                                //println!("archived = {}", res);
                                //false
                            //}
                        }),
                    RewardType =>
                        Ok(Reward {
                            text: require(d, "text"),
                            priority: require(d, "priority"),
                            cost: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                        }),
                }
            }
        )
    }
}

impl Show for Task {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &Habit {
                text: ref text,
                ..
            } => {
                write!(f, "Habit: {}", text)
            },
            &Daily {
                text: ref text,
                ..
            } => {
                write!(f, "Daily: {}", text)
            },
            &Todo {
                text: ref text,
                ..
            } => {
                write!(f, "Todo: {}", text)
            },
            &Reward {
                text: ref text,
                ..
            } => {
                write!(f, "Reward: {}", text)
            },
        }
        //write!(f, "prefix: {} code: {} param: {}",
               //self.prefix, self.code, self.param)
    }
}

fn require<E, D:Decoder<E>, T:Decodable<D, E>>(d: &mut D, field: &str) -> T {
    match d.read_struct_field(field, 0, |d: &mut D| {
            // Also possible: d.read_uint()
            // etc. But this is general for all Decodables!
            Decodable::decode(d)
        })
    {
        Ok(s) => s,
        Err(_) => fail!("Failed to decode field: {}", field),
    }
}

fn default<E, D:Decoder<E>, T:Decodable<D, E>>(d: &mut D, field: &str, default: T) -> T {
    match d.read_struct_field(field, 0, |d: &mut D| {
            // Also possible: d.read_uint()
            // etc. But this is general for all Decodables!
            Decodable::decode(d)
        })
    {
        Ok(s) => {
            println!("Found {}", field);
            s
        },
        Err(_) => {
            println!("Did not find {}", field);
            default
        },
    }
}

