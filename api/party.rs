use api::quest::*;

// TODO should parse from other place?
#[deriving(Show, Encodable, Decodable)]
pub struct Party {
    // ordering
    pub quest: Quest, // TODO should be optional!
    // leader
    // memberCount
    // members
    // name
}

//#[deriving(Show, Encodable, Decodable)]
//pub struct PartyMember {
    //
    // achievements
    // items
    // profile
    //      name
    // stats
    // auth
    //    timestamps
    //        created
    //        loggedin
//}

