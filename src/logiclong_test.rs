// make a test
#[cfg(test)]
mod tests {
    use crate::logiclong::LogicLong;

    #[test]
    fn test_new() {
        LogicLong::new(0_u32, 1_u32);
    }

    #[test]
    fn test_normal() {
        "#2PP".parse::<LogicLong>().unwrap();
    }

    #[test]
    fn test_no_hashtag() {
        "80CY2LC2R".parse::<LogicLong>().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
        "#willnotworkandlow/high=0,tag=\"\"".parse::<LogicLong>().unwrap();
    }
}
