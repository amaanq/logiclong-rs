use lazy_static::lazy_static;
use rand::{self, Rng};
use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicLong {
    pub low: u32,
    pub high: u32,
    pub tag: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicLongError {
    InvalidTag(String),
    InvalidLowID(u32),
}

lazy_static! {
    pub static ref VALID_REGEX: Regex = Regex::new("^#[oO0289PYLQGRJCUVpylqgrjcuv]+$").unwrap();
    pub static ref FIX_REGEX: Regex = Regex::new("[^A-Z0-9]+").unwrap();
}

impl LogicLong {
    pub const ORDER: [char; 14] = [
        '0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V',
    ];
    pub(crate) const BASE: u64 = 14;

    pub fn new(low: u32, high: u32) -> Result<LogicLong, LogicLongError> {
        if low > 100 {
            return Err(LogicLongError::InvalidLowID(low));
        }

        let mut logic_long = LogicLong {
            low,
            high,
            tag: String::new(),
        };
        logic_long.tag = logic_long.to_tag();
        Ok(logic_long)
    }

    pub fn from_tag(tag: String) -> Result<LogicLong, LogicLongError> {
        LogicLong::parse_tag(tag)
    }

    pub(crate) fn parse_tag(tag: String) -> Result<LogicLong, LogicLongError> {
        let tag = LogicLong::fix_tag(tag);
        let mut total: u64 = 0;

        // iterate backwards
        for (index, char) in tag.replace('#', "").chars().rev().enumerate() {
            // get position of char in arr
            let position = LogicLong::ORDER
                .iter()
                .position(|&x| x == char)
                .ok_or_else(|| LogicLongError::InvalidTag(tag.clone()))?;
            // total += position times 14 to the power of index
            total += position as u64 * Self::BASE.pow(index as u32);
        }

        let (low, high) = (((total % 256) as u32), ((total / 256) as u32));
        if low > 100 {
            Err(LogicLongError::InvalidTag(tag))
        } else {
            Ok(LogicLong { low, high, tag })
        }
    }

    /// Returns a "proper" tag, i.e. starts with # always and is purely uppercase with no 0s or Os
    pub fn is_valid_tag(tag: String) -> bool {
        VALID_REGEX.is_match(&tag.to_uppercase().replace('O', "0"))
    }

    pub fn fix_tag(tag: String) -> String {
        "#".to_owned()
            + &FIX_REGEX
                .replace_all(&tag.to_uppercase(), "")
                .replace('O', "0")
    }

    pub fn random() -> LogicLong {
        let mut rng = rand::thread_rng();
        let low = rng.gen_range(0..100);
        let high = rng.gen::<u32>();
        // unwrapping here because low is < 100
        LogicLong::new(low, high).unwrap()
    }

    pub fn to_tag(&self) -> String {
        let arr: Vec<char> = vec![
            '0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V',
        ];
        let mut tag = String::new();
        let mut total = self.low as i64 + self.high as i64 * 0x100;
        let mut b14;

        while total != 0 {
            b14 = total % 14;
            total /= 14;
            tag.insert(0, arr[b14 as usize]);
        }
        LogicLong::fix_tag(tag)
    }
}

impl fmt::Display for LogicLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}, {})", self.to_tag(), self.low, self.high)
    }
}

impl FromStr for LogicLong {
    type Err = LogicLongError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LogicLong::parse_tag(s.to_string())
    }
}

impl From<LogicLong> for String {
    fn from(logic_long: LogicLong) -> String {
        logic_long.tag
    }
}

impl From<String> for LogicLong {
    fn from(tag: String) -> LogicLong {
        LogicLong::parse_tag(tag).unwrap_or_default()
    }
}

impl From<&str> for LogicLong {
    fn from(tag: &str) -> LogicLong {
        LogicLong::parse_tag(tag.to_owned()).unwrap_or_default()
    }
}

impl fmt::Display for LogicLongError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogicLongError::InvalidTag(tag) => write!(f, "{} is not a valid tag.", tag),
            LogicLongError::InvalidLowID(low) => write!(f, "Invalid low ID: {}", low),
        }
    }
}

impl std::error::Error for LogicLongError {}
