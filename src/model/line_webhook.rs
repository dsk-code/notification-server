use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    pub destination: String,
    pub events: Vec<EventType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum EventType {
    Message(MessageEvent),
    Follow(FollowEvent),
    Unfollow(UnfollowEvent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Option<SourceType>,
    #[serde(rename = "webhookEventId")]
    pub webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    pub delivery_context: DeliveryContext,
    pub message: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Option<SourceType>,
    #[serde(rename = "webhookEventId")]
    pub webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    pub delivery_context: DeliveryContext,
    pub follow: Follow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfollowEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Option<SourceType>,
    #[serde(rename = "webhookEventId")]
    pub webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    pub delivery_context: DeliveryContext,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum SourceType {
    User(UserSource),
    Group(GroupSource),
    Room(RoomSource),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSource {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupSource {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomSource {
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryContext {
    #[serde(rename = "isRedelivery")]
    pub is_redelivery: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    Text(TextMessage),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextMessage {
    pub id: String,
    #[serde(rename = "quoteToken")]
    pub quote_token: String,
    pub text: String,
    pub emojis: Option<Vec<Emoji>>,
    pub mention: Option<Mention>,
    #[serde(rename = "quotedMessageId")]
    pub quoted_message_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Emoji {
    pub index: usize,
    // pub length: usize,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    pub mentionees: Vec<Mentionee>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mentionee {
    pub index: usize,
    pub length: usize,
    #[serde(rename = "type")]
    pub mentionee_type: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Follow {
    #[serde(rename = "isUnblocked")]
    pub is_unblocked: bool,
}

