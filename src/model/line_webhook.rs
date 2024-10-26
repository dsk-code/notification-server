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
    Follow(FollowEvent),
    Unfollow(UnfollowEvent),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: u64,
    pub source: Option<Source>,
    #[serde(rename = "webhookEventId")]
    pub webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    pub delivery_context: DeliveryContext,
    pub follow: Follow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfollowEvent {
    pub mode: String,
    pub timestamp: u64,
    pub source: Option<Source>,
    #[serde(rename = "webhookEventId")]
    pub webhook_event_id: String,
    #[serde(rename = "deliveryContext")]
    pub delivery_context: DeliveryContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryContext {
    #[serde(rename = "isRedelivery")]
    pub is_redelivery: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Follow {
    #[serde(rename = "isUnblocked")]
    pub is_unblocked: bool,
}
