//pub mod task;
pub mod date;
pub mod id;
pub mod conn;
pub mod user;
pub mod achievements;
pub mod party;
pub mod stats;
pub mod tag;
pub mod quest;
pub mod habit;
pub mod daily;
pub mod todo;
pub mod reward;
pub mod request;
pub mod repeat;

pub fn clean_text<'a>(text: &'a str) -> &'a str {
    let re = regex!(r"^(?::[^:]*:)?\s*(.+)");
    let caps = re.captures(text);
    match caps {
        Some(x) => x.at(1),
        None => fail!("Failed to match text: {}", text),
    }
}

