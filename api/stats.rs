
//#[deriving(Show, Encodable, Decodable)]
//pub struct Buffs {
    //constitution: uint,
    //intelligence: uint,
    //perception: uint,
    //strength: uint,
    //stealth: uint,
    //// snowball: bool ?
    //// streaks: bool ?
//}

// TODO custom Show
#[deriving(Show, Encodable, Decodable)]
pub struct Stats {
    //buffs: Buffs,
    pub class: String, // Or class...
    pub con: uint,
    pub int: uint,
    pub per: uint,
    pub str: uint,
    pub exp: f32,
    pub gp: f32,
    pub hp: uint,
    pub mp: uint,
    pub maxHealth: uint,
    pub maxMP: uint,
    pub lvl: uint,
    pub points: uint, // ??
    pub toNextLevel: f32, // uint?
    // training ?
}
