use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub mod dics;
use dics::*;
pub mod message_kinds;
use message_kinds::*;
pub mod chat_permissions;
use chat_permissions::*;
use url::Url;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update {
    pub id: i32,
    pub kind: UpdateKind,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Poll),
    PollAnswer(PollAnswer),
    MyChatMember(ChatMemberUpdated),
    ChatMember(ChatMemberUpdated),
    ChatJoinRequest(ChatJoinRequest),
    Error(Value),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub thread_id: Option<i32>,
    pub date: DateTime<Utc>,
    pub chat: Chat,
    pub via_bot: Option<User>,
    pub kind: MessageKind,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<ChatType>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: Currency,
    pub total_amount: i32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: OrderInfo,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub is_closed: bool,
    pub total_voter_count: i32,
    pub is_anonymous: bool,
    pub poll_type: PollType,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<u8>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<u16>,
    pub close_date: Option<DateTime<Utc>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: DateTime<Utc>,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub date: DateTime<Utc>,
    pub bio: Option<String>,
    pub invite_link: Option<ChatInviteLink>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: ChatId,
    pub kind: ChatKind,
    pub photo: Option<ChatPhoto>,
    pub pinned_message: Option<Box<Message>>,
    pub message_auto_delete_time: Option<u32>,
    pub has_hidden_members: bool,
    pub has_aggressive_anti_spam_enabled: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: bool,
    pub added_to_attachment_menu: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserId(pub u64);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMember {
    pub user: User,
    pub kind: ChatMemberKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatMemberKind {
    Owner(Owner),
    Administrator(Administrator),
    Member,
    Restricted(Restricted),
    Left,
    Banned(Banned),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<DateTime<Utc>>,
    pub member_limit: Option<u32>,
    pub pending_join_request_count: Option<u32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PollOption {
    pub text: String,
    pub voter_count: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PollType {
    Quiz,
    Regular,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEntity {
    pub kind: MessageEntityKind,
    pub offset: usize,
    pub length: usize,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre { language: Option<String> },
    TextLink { url: Url },
    TextMention { user: User },
    CustomEmoji { custom_emoji_id: String },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatType {
    Sender,
    Private,
    Group,
    Supergroup,
    Channel,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingAddress {
    pub country_code: CountryCode,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatId(pub i64);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatKind {
    Public(ChatPublic),
    Private(ChatPrivate),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatPublic {
    pub title: Option<String>,
    pub kind: PublicChatKind,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub has_protected_content: Option<True>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct True;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PublicChatKind {
    Channel(PublicChatChannel),
    Group(PublicChatGroup),
    Supergroup(PublicChatSupergroup),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicChatChannel {
    pub username: Option<String>,
    pub linked_chat_id: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicChatGroup {
    pub permissions: Option<ChatPermissions>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicChatSupergroup {
    pub username: Option<String>,
    pub active_usernames: Option<Vec<String>>,
    pub is_forum: bool,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<u32>,
    pub linked_chat_id: Option<i64>,
    pub location: Option<ChatLocation>,
    pub join_to_send_messages: Option<True>,
    pub join_by_request: Option<True>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owner {
    pub custom_title: Option<String>,
    pub is_anonymous: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Administrator {
    pub custom_title: Option<String>,
    pub is_anonymous: bool,
    pub can_be_edited: bool,
    pub can_manage_chat: bool,
    pub can_change_info: bool,
    pub can_post_messages: bool,
    pub can_edit_messages: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_invite_users: bool,
    pub can_restrict_members: bool,
    pub can_pin_messages: bool,
    pub can_manage_topics: bool,
    pub can_promote_members: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Restricted {
    pub until_date: UntilDate,
    pub is_member: bool,
    pub can_send_messages: bool,
    pub can_send_media_messages: bool,
    pub can_send_other_messages: bool,
    pub can_add_web_page_previews: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_pin_messages: bool,
    pub can_manage_topics: bool,
    pub can_send_polls: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UntilDate {
    Date(DateTime<Utc>),
    Forever,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Banned {
    pub until_date: UntilDate,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatPrivate {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<True>,
    pub has_restricted_voice_and_video_messages: Option<True>,
}
