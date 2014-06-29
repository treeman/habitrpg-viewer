
// Could instead use a HashMap?
#[deriving(Show, Encodable, Decodable)]
pub struct Tag {
    pub id: String,
    pub name: String,
}
