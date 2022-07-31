pub mod bytestream;
pub mod logiclong;
pub mod logiclong_test;
pub mod reader;
pub mod writer;


// #[derive(Describe)]
// struct LoginMessage {
//     name: String,
//     token: String,
// }

// // make unit test to test describe()
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn do_this() {
//         use super::*;
//         // let msg = Message {
//         //     name: "Message".to_string(),
//         //     token: "token".to_string(),
//         // };
//         let desc = LoginMessage::describe();
//         assert_eq!(desc, "a struct with these named fields: name, token");
//         println!("{}", desc);
//     }
// }
