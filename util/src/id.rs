use std::fmt;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;

pub const BYTES: usize = 16;

const SLICES: [(usize, usize); 5] = [(0, 4), (4, 6), (6, 8), (8, 10), (10, 16)];

/// Read a new ID from /dev/urandom
pub fn urandom() -> Option<[u8; BYTES]> {
    let mut f = match File::open("/dev/urandom") {
        Ok(f) => f,
        Err(_) => { return None; },
    };
    let mut id: [u8; BYTES] = [0u8; BYTES];
    let mut amt = 0;
    while amt < BYTES {
        let x = f.read(&mut id).ok()?;
        amt += x;
    }
    Some(id)
}

/// Encode 16B of random data in something aesthetically better.
pub fn encode(id: &[u8; BYTES]) -> String {
    let mut s = String::with_capacity(36);
    for &(start, limit) in SLICES.iter() {
        if start > 0 {
            s.push_str("-");
        }
        for i in start..limit {
            write!(&mut s, "{:02x}", id[i]).expect("unable to write to string");
        }
    }
    s
}

/// Turn the "aesthetically better" string back into bytes.
pub fn decode(s: &str) -> Option<[u8; BYTES]> {
    let mut result = [0u8; BYTES];
    let mut index = 0;
    let mut chars = s.chars();
    for &(start, limit) in SLICES.iter() {
        for _ in start..limit {
            let mut upper = chars.next()?;
            let mut lower = chars.next()?;
            if !upper.is_ascii_hexdigit() {
                return None;
            }
            if !lower.is_ascii_hexdigit() {
                return None;
            }
            upper.make_ascii_lowercase();
            lower.make_ascii_lowercase();
            const HEX: &str = "0123456789abcdef";
            let upper = HEX.find(upper).unwrap();
            let lower = HEX.find(lower).unwrap();

            result[index] = (upper << 4 | lower) as u8;
            index += 1;
        }
        let dash = chars.next();
        if (limit < 16 && dash != Some('-')) || (limit == 16 && dash != None) {
            return None;
        }
    }
    Some(result)
}

macro_rules! generate_id {
    ($what:ident, $prefix:literal) => {
        #[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
        pub struct $what {
            id: [u8; BYTES],
        }

        impl $what {
            pub const BOTTOM: $what = $what { id: [0u8; BYTES] };
            pub const TOP: $what = $what { id: [0xffu8; BYTES], };

            pub fn generate() -> Option<$what> {
                match urandom() {
                    Some(id) => Some($what { id }),
                    None => None
                }
            }

            pub fn from_human_readable(s: &str) -> Option<Self> {
                let prefix = $prefix;
                if !s.starts_with(prefix) {
                    return None;
                }
                match decode(&s[prefix.len()..]) {
                    Some(x) => Some(Self::new(x)),
                    None => None,
                }
            }

            pub fn human_readable(&self) -> String {
                let readable = $prefix.to_string();
                readable + &encode(&self.id)
            }

            pub fn prefix_free_readable(&self) -> String {
                encode(&self.id)
            }

            fn new(id: [u8; BYTES]) -> Self {
                Self {
                    id
                }
            }
        }

        impl Default for $what {
            fn default() -> $what {
                $what::BOTTOM
            }
        }

        impl fmt::Display for $what {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}{}", $prefix, encode(&self.id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urandom_is_nonzero() {
        assert_ne!(Some([0u8; BYTES]), urandom());
    }

    #[test]
    fn id_bytes_is_sixteen() {
        assert_eq!(BYTES, 16);
    }

    #[test]
    fn encode_id() {
        let id = [0x55u8; BYTES];
        assert_eq!(encode(&id), "55555555-5555-5555-5555-555555555555");
    }

    #[test]
    fn decode_id() {
        let id = [0x55u8; BYTES];
        assert_eq!(decode("55555555-5555-5555-5555-555555555555"), Some(id));
    }

    generate_id!(FooID, "foo:");

    #[test]
    fn generate_id() {
        let id = FooID::new([0xffu8; BYTES]);
        assert_eq!("foo:ffffffff-ffff-ffff-ffff-ffffffffffff", id.human_readable());
        assert_eq!(Some(id), FooID::from_human_readable("foo:ffffffff-ffff-ffff-ffff-ffffffffffff"));
    }
}
