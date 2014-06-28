use core::fmt::{Show, Formatter, Result};
use std::io::File;
use serialize::{json, Decodable};

// API_TOKEN and USER_ID for habitrpg identifiers.
#[deriving(Decodable)]
pub struct Id  {
    pub api_token: String,
    pub user_id: String,
}

impl Show for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "api_token: {} user_id: {}", self.api_token, self.user_id)
    }
}

impl Id {
    pub fn from_file(path: &Path) -> Id {
        let contents = match File::open(path).read_to_str() {
            Ok(v) => v,
            Err(e) => fail!("Failed to read: {}", e)
        };

        let json_object = match json::from_str(contents.as_slice()) {
            Ok(v) => v,
            Err(e) => fail!("json parse error: {}", e)
        };
        let mut decoder = json::Decoder::new(json_object);
        let res = match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => fail!("Decoding error: {}", e)
        };

        res
    }
}

