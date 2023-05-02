use serde::{Deserialize, Serialize};
use serenity::builder::EditMessage;
use serenity::http::CacheHttp;
use serenity::json;
use serenity::json::Value;
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, MessageId};

/// Lightweight version of [`serenity::model::channel::Message`] that only supports
/// [`edit`](MessageShim::edit) and [`delete`](MessageShim::delete)
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MessageShim {
    pub message_id: MessageId,
    channel_id: ChannelId,
}

impl MessageShim {
    /// See [`serenity::model::channel::Message::edit`]
    pub async fn edit<'a, F>(&self, cache_http: impl CacheHttp, f: F) -> serenity::Result<()>
    where
        F: for<'b> FnOnce(&'b mut EditMessage<'a>) -> &'b mut EditMessage<'a>,
    {
        let mut builder = EditMessage::default();
        f(&mut builder);
        let map = json::hashmap_to_json_map(builder.0);

        let http = cache_http.http();
        http.edit_message_and_attachments(
            self.channel_id.0,
            self.message_id.0,
            &Value::from(map),
            builder.1,
        )
        .await?;
        Ok(())
    }

    /// See [`serenity::model::channel::Message::delete`]
    pub async fn delete(&self, cache_http: impl CacheHttp) -> serenity::Result<()> {
        self.channel_id
            .delete_message(&cache_http.http(), self.message_id)
            .await
    }
}

impl From<Message> for MessageShim {
    fn from(message: Message) -> Self {
        Self {
            message_id: message.id,
            channel_id: message.channel_id,
        }
    }
}

impl From<&Message> for MessageShim {
    fn from(message: &Message) -> Self {
        Self {
            message_id: message.id,
            channel_id: message.channel_id,
        }
    }
}
