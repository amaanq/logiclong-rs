use std::{fmt, str::FromStr};

use lazy_static::lazy_static;
use rand::{self, Rng};
use regex::Regex;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicLong<T = u32>
where
    T: Copy + ?Sized + From<u32>,
    u32: From<T>,
{
    pub high: T,
    pub low: T,
    pub tag: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicLongError {
    InvalidTag(String),
    InvalidHighID(u32),
}

lazy_static! {
    pub static ref VALID_REGEX: Regex = Regex::new("^#[oO0289PYLQGRJCUVpylqgrjcuv]+$").unwrap();
    pub static ref FIX_REGEX: Regex = Regex::new("[^A-Z0-9]+").unwrap();
}

const ORDER: [char; 14] = ['0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V'];
const BASE: u64 = 14;

impl<T> LogicLong<T>
where
    T: Copy + ?Sized + From<u32>,
    u32: From<T>,
{
    pub fn new(high: T, low: T) -> Self {
        let mut logic_long = Self { high, low, tag: String::new() };
        logic_long.tag = logic_long.to_tag();
        logic_long
    }

    pub(crate) fn parse_tag(tag: &str) -> Result<Self, LogicLongError> {
        let tag = Self::fix_tag(tag);
        let mut total: u64 = 0;

        // iterate backwards
        for (index, char) in tag.replace('#', "").chars().rev().enumerate() {
            // get position of char in arr
            let position = ORDER
                .iter()
                .position(|&x| x == char)
                .ok_or_else(|| LogicLongError::InvalidTag(tag.clone()))?;
            // total += position times 14 to the power of index
            total += position as u64 * BASE.pow(index as u32);
        }

        let (high, low) = (((total % 256) as u32), ((total / 256) as u32));

        Ok(Self { high: high.into(), low: low.into(), tag })
    }

    /// Returns a "proper" tag, i.e. starts with # always and is purely uppercase with no 0s or Os
    #[must_use]
    pub fn is_valid_tag(tag: &str) -> bool {
        VALID_REGEX.is_match(&tag.to_uppercase().replace('O', "0"))
    }

    #[must_use]
    pub fn fix_tag(tag: &str) -> String {
        "#".to_owned() + &FIX_REGEX.replace_all(&tag.to_uppercase(), "").replace('O', "0")
    }

    #[must_use]
    /// Players have a max high of 100, clans/wars/messages seem to be much higher
    pub fn random(max_high: u32) -> Self {
        let mut rng = rand::thread_rng();
        let high = rng.gen_range(0..max_high).into();
        let low = rng.gen::<u32>().into();

        Self::new(high, low)
    }

    pub fn to_tag(&self) -> String {
        let mut tag = String::new();
        // let mut total: u64 = self.high.into() + self.low.into() * 0x100;
        let mut total = u64::from(u32::from(self.high)) + u64::from(u32::from(self.low)) * 0x100;
        let mut b14;

        while total != 0 {
            b14 = total % 14;
            total /= 14;
            tag.insert(0, ORDER[b14 as usize]);
        }
        Self::fix_tag(&tag)
    }
}

impl<T> fmt::Display for LogicLong<T>
where
    T: Copy + ?Sized + From<u32>,
    u32: From<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}

impl<T> fmt::Debug for LogicLong<T>
where
    T: Copy + ?Sized + From<u32> + fmt::Display,
    u32: From<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LogicLong {{ high: {}, low: {}, tag: {} }}", self.high, self.low, self.tag)
    }
}

impl<T> FromStr for LogicLong<T>
where
    T: Copy + ?Sized + From<u32>,
    u32: From<T>,
{
    type Err = LogicLongError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_tag(s)
    }
}

impl<T> From<LogicLong<T>> for String
where
    T: Copy + ?Sized + From<u32>,
    u32: From<T>,
{
    fn from(logic_long: LogicLong<T>) -> Self {
        logic_long.tag
    }
}

impl fmt::Display for LogicLongError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidTag(tag) => write!(f, "{tag} is not a valid tag."),
            Self::InvalidHighID(high) => write!(f, "Invalid high ID: {high}"),
        }
    }
}

impl std::error::Error for LogicLongError {}
