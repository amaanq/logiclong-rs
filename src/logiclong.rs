use rand::{self, Rng};
use regex::Regex;
use std::fmt;

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicLong {
    pub low: u32,
    pub high: u32,
    pub tag: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LogicLongError {
    InvalidTag(String),
}

impl LogicLong {
    pub fn new(low: u32, high: u32) -> LogicLong {
        let mut logic_long = LogicLong {
            low,
            high,
            tag: String::new(),
        };
        logic_long.tag = logic_long.to_tag();
        logic_long
    }

    pub fn new_from_tag(tag: String) -> Result<LogicLong, LogicLongError> {
        return LogicLong::from_tag(tag);
    }

    pub fn from_tag(tag: String) -> Result<LogicLong, LogicLongError> {
        let arr: Vec<char> = vec![
            '0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V',
        ];
        let tag = LogicLong::fix_tag(tag);
        let mut total: u64 = 0;
        let base: u64 = 14;

        // iterate backwards
        for (i, c) in tag.chars().rev().enumerate() {
            // get index of c in arr
            let index = arr
                .iter()
                .position(|&x| x == c)
                .ok_or_else(|| LogicLongError::InvalidTag(tag.clone()))?;
            // total += index times 14 to the power of i
            total += index as u64 * base.pow(i as u32);
        }
        Ok(LogicLong {
            low: ((total % 256) as u32),
            high: ((total / 256) as u32),
            tag,
        })
    }

    pub fn fix_tag(tag: String) -> String {
        let re = Regex::new("[^A-Z0-9]+").unwrap();
        "#".to_owned()
            + &re
                .replace_all(tag.to_uppercase().as_str(), "")
                .replace("O", "0")
    }

    pub fn random() -> LogicLong {
        let mut rng = rand::thread_rng();
        let low = rng.gen_range(0..100);
        let high = rng.gen::<u32>();
        LogicLong::new(low, high)
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
        tag
    }

    // fn digit_to_char(digit: i32) -> String {
    //     if digit < 10 {
    //         return format!("{}", digit);
    //     }
    //     return format!("{}", (b'a' + digit as u8 - 10) as char);
    // }

    // fn str_base(number: i32, base: i32) -> String {
    //     if number < 0 {
    //         return format!("-{}", LogicLong::str_base(-number, base));
    //     }
    //     let (d, m) = (number / base, number % base);
    //     if d > 0 {
    //         LogicLong::str_base(d, base) + &LogicLong::digit_to_char(m)
    //     } else {
    //         LogicLong::digit_to_char(m)
    //     }
    // }

    // fn dec2rdx(mut num: i32) -> String {
    //     let mut rv = String::new();
    //     for _ in 0..4 {
    //         rv = format!("{},", num & 0xFF) + &rv;
    //         num >>= 8;
    //     }
    //     rv
    // }
}

impl fmt::Display for LogicLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}, {})", self.to_tag(), self.low, self.high)
    }
}
