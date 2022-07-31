
// make a test
#[cfg(test)]
mod tests {
    use crate::logiclong::LogicLong;

    #[test]
    fn test_logic_long() {
        let logic_long = LogicLong::new_from_tag("#QL82UUGGG".to_string());
        println!("{:#?}", logic_long);
    }
}
