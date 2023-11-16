use crate::models::tg::Message;

pub fn message_unwrap(msg: &Message) -> String {
    let input = &msg.text;
    let words = input.split_whitespace();
    let mut result = String::new();
    for word in words {
        result.push_str(word)
    }
    result
}
