use api::quest::*;

// TODO should parse from other place?
#[deriving(Show, Encodable, Decodable)]
pub struct Party {
    // ordering
    pub quest: Quest,
}

