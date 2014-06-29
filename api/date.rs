use core::fmt::{Show, Formatter, FormatError};
use serialize::{Encodable, Decodable, Decoder, Encoder};
use std::result::Result;


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
        let res = caps.and_then(|x| {
            Some(Date {
                year: from_str(x.at(1)).unwrap(),
                month: from_str(x.at(2)).unwrap(),
                day: from_str(x.at(3)).unwrap(),
                hour: from_str(x.at(4)).unwrap(),
                min: from_str(x.at(5)).unwrap(),
                sec: from_str(x.at(6)).unwrap(),
                ms: from_str(x.at(7)).unwrap(),
            })
        });
        res
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
