use core::fmt::{Show, Formatter, Result};
use serialize::{json, Encodable, Decodable, Decoder, Encoder};
use std::result::Result;
use std::fmt::FormatError;

use tasks::date::Date;

//#[deriving(Decodable)]
//pub struct Tags  {
    //// List of id: bool
//}

#[deriving(Show, Encodable)]
pub enum TaskType {
    HabitType,
    DailyType,
    TodoType,
    RewardType,
}

impl<E, D:Decoder<E>> Decodable<D, E> for TaskType {
    fn decode(d: &mut D) -> Result<TaskType, E> {
        let x = d.read_str().ok().unwrap();
        match x.as_slice() {
            "habit" => Ok(HabitType),
            "daily" => Ok(DailyType),
            "todo" => Ok(TodoType),
            "reward" => Ok(RewardType),
            _ => fail!("Decoding missing task type: {}", x),
        }
    }
}

#[deriving(Show, Encodable)]
pub enum Task {
    Habit {
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
    },

    Daily {
        text: String,
        attribute: String, // "str" wut?
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
        history: Vec<String>, // TODO
    },

    Todo {
        text: String,
        attribute: String, // "str" wut?
        priority: f32,
        value: f32,
        notes: String,
        dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
        id: String,
        // checklist
        // collapseChecklist
        completed: bool,
    },

    Reward {
        text: String,
        attribute: String, // "str" wut?
        priority: f32,
        value: f32,
        notes: String,
        dateCreated: Date, // "2014-06-27T18:22:05.834Z", can decode
        id: String,
    },
}

fn require<E, D:Decoder<E>, T:Decodable<D, E>>(d: &mut D, field: &str) -> T {
    match d.read_struct_field(field, 0, |d: &mut D| {
            // Also possible: d.read_uint()
            // etc. But this is general for all Decodables!
            Decodable::decode(d)
        })
    {
        Ok(s) => s,
        Err(e) => fail!("Failed to decode field: {}", field),
    }
}

fn default<E, D:Decoder<E>, T:Decodable<D, E>>(d: &mut D, field: &str, default: T) -> T {
    match d.read_struct_field(field, 0, |d: &mut D| {
            // Also possible: d.read_uint()
            // etc. But this is general for all Decodables!
            Decodable::decode(d)
        })
    {
        Ok(s) => s,
        Err(e) => default,
    }
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
                            attribute: require(d, "attribute"),
                            priority: require(d, "priority"),
                            value: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                            down: require(d, "down"),
                            up: require(d, "up"),
                            history: require(d, "history"),
                        }),
                    DailyType =>
                        Ok(Daily {
                            text: require(d, "text"),
                            attribute: require(d, "attribute"),
                            priority: require(d, "priority"),
                            value: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                            streak: require(d, "streak"),
                            completed: require(d, "completed"),
                            history: require(d, "history"),
                        }),
                    TodoType =>
                        Ok(Todo {
                            text: require(d, "text"),
                            attribute: require(d, "attribute"),
                            priority: require(d, "priority"),
                            value: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                            completed: require(d, "completed"),
                        }),
                    RewardType =>
                        Ok(Reward {
                            text: require(d, "text"),
                            attribute: require(d, "attribute"),
                            priority: require(d, "priority"),
                            value: require(d, "value"),
                            notes: require(d, "notes"),
                            dateCreated: require(d, "dateCreated"),
                            id: require(d, "id"),
                        }),
                }
            }
        )
    }
}

