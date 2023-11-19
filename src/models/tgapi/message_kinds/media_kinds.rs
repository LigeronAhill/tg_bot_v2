use super::*;
use mime::Mime;
pub mod mime_serde;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaKind {
    Animation(MediaAnimation),
    Audio(MediaAudio),
    Contact(MediaContact),
    Document(MediaDocument),
    Game(MediaGame),
    Venue(MediaVenue),
    Location(MediaLocation),
    Photo(MediaPhoto),
    Poll(MediaPoll),
    Sticker(MediaSticker),
    Text(MediaText),
    Video(MediaVideo),
    VideoNote(MediaVideoNote),
    Voice(MediaVoice),
    Migration(ChatMigration),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaAnimation {
    pub animation: Animation,
    pub caption: Option<String>,
    pub caption_entities: Vec<MessageEntity>,
    pub has_media_spoiler: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animation {
    pub file: FileMeta,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    #[serde(with = "mime_serde::opt_deser")]
    pub mime_type: Option<Mime>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMeta {
    pub id: String,
    pub unique_id: String,
    pub size: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoSize {
    pub file: FileMeta,
    pub width: u32,
    pub height: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaAudio {
    pub audio: Audio,
    pub caption: Option<String>,
    pub caption_entities: Vec<MessageEntity>,
    pub media_group_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Audio {
    pub file: FileMeta,
    pub duration: u32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    #[serde(with = "mime_serde::opt_deser")]
    pub mime_type: Option<Mime>,
    pub thumb: Option<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaContact {
    pub contact: Contact,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<UserId>,
    pub vcard: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaDocument {
    pub document: Document,
    pub caption: Option<String>,
    pub caption_entities: Vec<MessageEntity>,
    pub media_group_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub file: FileMeta,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    #[serde(with = "mime_serde::opt_deser")]
    pub mime_type: Option<Mime>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaGame {
    pub game: Game,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaVenue {
    pub venue: Venue,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaLocation {
    pub location: Location,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaPhoto {
    pub photo: Vec<PhotoSize>,
    pub caption: Option<String>,
    pub caption_entities: Vec<MessageEntity>,
    pub has_media_spoiler: bool,
    pub media_group_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaPoll {
    pub poll: Poll,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaSticker {
    pub sticker: Sticker,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sticker {
    pub file: FileMeta,
    pub width: u16,
    pub height: u16,
    pub kind: StickerKind,
    pub format: StickerFormat,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StickerKind {
    Regular { premium_animation: Option<FileMeta> },
    Mask { mask_position: MaskPosition },
    CustomEmoji { custom_emoji_id: String },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MaskPosition {
    pub point: MaskPoint,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MaskPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StickerFormat {
    Raster,
    Animated,
    Video,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaText {
    pub text: String,
    pub entities: Vec<MessageEntity>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaVideo {
    pub video: Video,
    pub caption: Option<String>,
    pub caption_entities: Vec<MessageEntity>,
    pub has_media_spoiler: bool,
    pub media_group_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub file: FileMeta,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    #[serde(with = "mime_serde::opt_deser")]
    pub mime_type: Option<Mime>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaVideoNote {
    pub video_note: VideoNote,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoNote {
    pub file: FileMeta,
    pub length: u32,
    pub duration: u32,
    pub thumb: Option<PhotoSize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaVoice {
    pub voice: Voice,
    pub caption: Option<String>,
    pub caption_entities: Vec<MessageEntity>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voice {
    pub file: FileMeta,
    pub duration: u32,
    #[serde(with = "mime_serde::opt_deser")]
    pub mime_type: Option<Mime>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatMigration {
    To { chat_id: ChatId },
    From { chat_id: ChatId },
}
