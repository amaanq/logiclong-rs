// make a test
#[cfg(test)]
mod tests {
    use crate::logiclong::LogicLong;

    #[test]
    fn test_logic_long() {
        let logic_long = LogicLong::from_tag("80CY2LC2R".to_string());
        println!("{:#?}", logic_long);

        let logic_long = LogicLong::new(0, 1).unwrap();
        println!("{:#?}", logic_long);

        let logic_long = LogicLong::from("#willnotworkandlow/high=0,tag=\"\"");
        println!("{:#?}", logic_long);

        let logic_long = LogicLong::from("#2PP");
        println!("{:#?}", logic_long);
    }
}
