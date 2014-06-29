

#[deriving(Show, Encodable, Decodable)]
pub struct Achievements {
    pub beastMaster: bool,
    //challenges: Vec<?>,
    pub perfect: uint,
    //quests: {
        //"id": uint,
    //},
    pub streak: uint,
    pub ultimateGear: bool,
}
