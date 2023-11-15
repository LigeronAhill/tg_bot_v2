use crate::models::tg::{Message, MessageToBot};

pub fn message_unwrap(msg: Message) -> MessageToBot {
    MessageToBot::new(
        msg.chat.id,
        format!("Я только что получила это: {}", msg.text),
    )
}
