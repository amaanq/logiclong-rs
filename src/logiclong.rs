#[derive(Debug)]
pub struct LogicLong {
    pub low: i32,
    pub high: i32,
}

#[derive(Debug)]

pub enum LogicLongError {
    InvalidTag(String),
}

impl LogicLong {
    pub fn new(low: i32, high: i32) -> LogicLong {
        LogicLong { low, high }
    }

    pub fn new_from_tag(tag: String) -> Result<LogicLong, LogicLongError> {
        return LogicLong::from_tag(tag);
    }

    pub fn from_tag(tag: String) -> Result<LogicLong, LogicLongError> {
        let arr: Vec<char> = vec![
            '0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V',
        ];
        let tag = tag.replace("#", "").replace(" ", "");
        let mut total: i64 = 0;
        let base: i64 = 14;

        // iterate backwards
        for (i, c) in tag.chars().rev().enumerate() {
            // get index of c in arr
            let index = arr
                .iter()
                .position(|&x| x == c)
                .ok_or_else(|| LogicLongError::InvalidTag(tag.clone()))?;
            // total += index times 14 to the power of i
            total += index as i64 * base.pow(i as u32);
        }
        Ok(LogicLong {
            low: (total as i32 % 256),
            high: (total as i32 / 256),
        })
    }

    pub fn to_tag(&self) -> String {
        let arr: Vec<char> = vec![
            '0', '2', '8', '9', 'P', 'Y', 'L', 'Q', 'G', 'R', 'J', 'C', 'U', 'V',
        ];
        let mut tag = String::new();
        let mut total = self.low + self.high * 0x100;
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
