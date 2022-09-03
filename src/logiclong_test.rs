// make a test
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::logiclong::LogicLong;

    #[test]
    fn test_logic_long() {
        let logic_long = LogicLong::from_tag("80CY2LC2R".to_string());
        println!("{:#?}", logic_long);

        let logic_long = LogicLong::new(0, 1).unwrap();
        println!("{:#?}", logic_long);

        let logic_long = LogicLong::from_str("#willnotworkandlow/high=0,tag=\"\"");
        println!("{:#?}", logic_long);

        let logic_long = LogicLong::from_str("#2PP");
        println!("{:#?}", logic_long);

        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        println!(
            "{:?}",
            nums.iter().fold((0, 0), |(max, sum), &x| {
                let sum = sum + x;
                (max.max(sum), sum.max(0))
            })
        );
    }
}
