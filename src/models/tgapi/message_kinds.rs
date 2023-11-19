use super::*;
use serde::{Deserialize, Serialize};
pub mod media_kinds;
use media_kinds::*;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageKind {
    Common(MessageCommon),
    NewChatMembers(MessageNewChatMembers),
    LeftChatMember(MessageLeftChatMember),
    NewChatTitle(MessageNewChatTitle),
    NewChatPhoto(MessageNewChatPhoto),
    DeleteChatPhoto(MessageDeleteChatPhoto),
    GroupChatCreated(MessageGroupChatCreated),
    SupergroupChatCreated(MessageSupergroupChatCreated),
    ChannelChatCreated(MessageChannelChatCreated),
    MessageAutoDeleteTimerChanged(MessageMessageAutoDeleteTimerChanged),
    Pinned(MessagePinned),
    Invoice(MessageInvoice),
    SuccessfulPayment(MessageSuccessfulPayment),
    ConnectedWebsite(MessageConnectedWebsite),
    WriteAccessAllowed(MessageWriteAccessAllowed),
    PassportData(MessagePassportData),
    Dice(MessageDice),
    ProximityAlertTriggered(MessageProximityAlertTriggered),
    ForumTopicCreated(MessageForumTopicCreated),
    ForumTopicEdited(MessageForumTopicEdited),
    ForumTopicClosed(MessageForumTopicClosed),
    ForumTopicReopened(MessageForumTopicReopened),
    GeneralForumTopicHidden(MessageGeneralForumTopicHidden),
    GeneralForumTopicUnhidden(MessageGeneralForumTopicUnhidden),
    VideoChatScheduled(MessageVideoChatScheduled),
    VideoChatStarted(MessageVideoChatStarted),
    VideoChatEnded(MessageVideoChatEnded),
    VideoChatParticipantsInvited(MessageVideoChatParticipantsInvited),
    WebAppData(MessageWebAppData),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageCommon {
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub author_signature: Option<String>,
    pub forward: Option<Forward>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<DateTime<Utc>>,
    pub media_kind: MediaKind,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub is_topic_message: bool,
    pub is_automatic_forward: bool,
    pub has_protected_content: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub kind: InlineKeyboardButtonKind,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InlineKeyboardButtonKind {
    Url(Url),
    LoginUrl(LoginUrl),
    CallbackData(String),
    WebApp(WebAppInfo),
    SwitchInlineQuery(String),
    SwitchInlineQueryCurrentChat(String),
    CallbackGame(CallbackGame),
    Pay(True),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAppInfo {
    pub url: Url,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUrl {
    pub url: Url,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackGame;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Forward {
    pub date: DateTime<Utc>,
    pub from: ForwardedFrom,
    pub signature: Option<String>,
    pub message_id: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ForwardedFrom {
    User(User),
    Chat(Chat),
    SenderName(String),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageNewChatMembers {
    pub new_chat_members: Vec<User>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageLeftChatMember {
    pub left_chat_member: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageNewChatTitle {
    pub new_chat_title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageNewChatPhoto {
    pub new_chat_photo: Vec<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageDeleteChatPhoto {
    pub delete_chat_photo: True,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageGroupChatCreated {
    pub group_chat_created: True,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSupergroupChatCreated {
    pub supergroup_chat_created: True,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageChannelChatCreated {
    pub channel_chat_created: True,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageMessageAutoDeleteTimerChanged {
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessagePinned {
    pub pinned: Box<Message>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInvoice {
    pub invoice: Invoice,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSuccessfulPayment {
    pub successful_payment: SuccessfulPayment,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulPayment {
    pub currency: Currency,
    pub total_amount: i32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: OrderInfo,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageConnectedWebsite {
    pub connected_website: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageWriteAccessAllowed {
    pub write_access_allowed: WriteAccessAllowed,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WriteAccessAllowed;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessagePassportData {
    pub passport_data: PassportData,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElement {
    pub hash: String,
    pub kind: EncryptedPassportElementKind,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EncryptedPassportElementKind {
    PersonalDetails(EncryptedPassportElementPersonalDetails),
    Passport(EncryptedPassportElementPassport),
    DriverLicense(EncryptedPassportElementDriverLicense),
    IdentityCard(EncryptedPassportElementIdentityCard),
    InternalPassport(EncryptedPassportElementInternalPassport),
    Address(EncryptedPassportElementAddress),
    UtilityBill(EncryptedPassportElementUtilityBill),
    BankStatement(EncryptedPassportElementBankStatement),
    RentalAgreement(EncryptedPassportElementRentalAgreement),
    PassportRegistration(EncryptedPassportElementPassportRegistration),
    EncryptedPassportElement(EncryptedPassportElementTemporaryRegistration),
    PhoneNumber(EncryptedPassportElementPhoneNumber),
    Email(EncryptedPassportElementEmail),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageDice {
    pub dice: Dice,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dice {
    pub emoji: DiceEmoji,
    pub value: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DiceEmoji {
    Dice,
    Darts,
    Basketball,
    Football,
    Bowling,
    SlotMachine,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementPersonalDetails {
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementPassport {
    pub data: String,
    pub front_side: PassportFile,
    pub selfie: PassportFile,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassportFile {
    pub file: FileMeta,
    pub date: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementDriverLicense {
    pub data: String,
    pub front_side: PassportFile,
    pub reverse_side: PassportFile,
    pub selfie: PassportFile,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementIdentityCard {
    pub data: String,
    pub front_side: PassportFile,
    pub reverse_side: PassportFile,
    pub selfie: PassportFile,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementInternalPassport {
    pub data: String,
    pub front_side: PassportFile,
    pub selfie: PassportFile,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementAddress {
    pub data: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementUtilityBill {
    pub files: Vec<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementBankStatement {
    pub files: Vec<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementRentalAgreement {
    pub files: Vec<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementPassportRegistration {
    pub files: Vec<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementTemporaryRegistration {
    pub files: Vec<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementPhoneNumber {
    pub phone_number: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedPassportElementEmail {
    pub email: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageProximityAlertTriggered {
    pub proximity_alert_triggered: ProximityAlertTriggered,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageForumTopicCreated {
    pub forum_topic_created: ForumTopicCreated,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: [u8; 3],
    pub icon_custom_emoji_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageForumTopicEdited {
    pub forum_topic_edited: ForumTopicEdited,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageForumTopicClosed {
    pub forum_topic_closed: ForumTopicClosed,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTopicClosed;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageForumTopicReopened {
    pub forum_topic_reopened: ForumTopicReopened,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTopicReopened;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageGeneralForumTopicHidden {
    pub general_forum_topic_hidden: GeneralForumTopicHidden,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralForumTopicHidden;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageGeneralForumTopicUnhidden {
    pub general_forum_topic_unhidden: GeneralForumTopicUnhidden,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralForumTopicUnhidden;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageVideoChatScheduled {
    pub video_chat_scheduled: VideoChatScheduled,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatScheduled {
    pub start_date: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageVideoChatStarted {
    pub video_chat_started: VideoChatStarted,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatStarted {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageVideoChatEnded {
    pub video_chat_ended: VideoChatEnded,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatEnded {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageVideoChatParticipantsInvited {
    pub video_chat_participants_invited: VideoChatParticipantsInvited,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatParticipantsInvited {
    pub users: Option<Vec<User>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageWebAppData {
    pub web_app_data: WebAppData,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}
