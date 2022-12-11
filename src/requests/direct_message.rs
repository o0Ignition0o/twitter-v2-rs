use crate::api::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::id::{IntoNumericId, StringId};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct DirectMessageSentEvent {
    pub dm_conversation_id: StringId,
    pub dm_event_id: StringId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftDirectMessageMedia {
    pub media_ids: Vec<String>,
    pub tagged_user_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftDirectMessagePoll {
    pub options: Vec<String>,
    pub duration_minutes: u64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftDirectMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<DraftDirectMessageMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<DraftDirectMessagePoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tweet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug)]
pub struct DirectMessageBuilder<A> {
    client: TwitterApi<A>,
    url: Url,
    direct_message: DraftDirectMessage,
}

impl<A> DirectMessageBuilder<A>
where
    A: Authorization,
{
    pub(crate) fn new(client: &TwitterApi<A>, url: Url) -> Self {
        Self {
            client: client.clone(),
            url,
            direct_message: Default::default(),
        }
    }
    pub fn text(&mut self, text: String) -> &mut Self {
        self.direct_message.text = Some(text);
        self
    }
    pub fn add_media(
        &mut self,
        media_ids: impl IntoIterator<Item = impl IntoNumericId>,
        tagged_user_ids: impl IntoIterator<Item = impl IntoNumericId>,
    ) -> &mut Self {
        if let Some(media) = self.direct_message.media.as_mut() {
            media
                .media_ids
                .extend(media_ids.into_iter().map(|id| id.to_string()));
            media
                .tagged_user_ids
                .extend(tagged_user_ids.into_iter().map(|id| id.to_string()));
        } else {
            self.direct_message.media = Some(DraftDirectMessageMedia {
                media_ids: media_ids.into_iter().map(|id| id.to_string()).collect(),
                tagged_user_ids: tagged_user_ids
                    .into_iter()
                    .map(|id| id.to_string())
                    .collect(),
            });
        }
        self
    }
    pub fn poll(
        &mut self,
        options: impl IntoIterator<Item = impl ToString>,
        duration: Duration,
    ) -> &mut Self {
        self.direct_message.poll = Some(DraftDirectMessagePoll {
            options: options
                .into_iter()
                .map(|option| option.to_string())
                .collect::<Vec<_>>(),
            duration_minutes: duration.as_secs() / 60,
        });
        self
    }
    pub fn quote_tweet_id(&mut self, id: impl IntoNumericId) -> &mut Self {
        self.direct_message.quote_tweet_id = Some(id.to_string());
        self
    }
    pub async fn send(&self) -> ApiResult<A, DirectMessageSentEvent, ()> {
        self.client
            .send(
                self.client
                    .request(Method::POST, self.url.clone())
                    .json(&self.direct_message),
            )
            .await
    }
}

impl<A> Clone for DirectMessageBuilder<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            url: self.url.clone(),
            direct_message: self.direct_message.clone(),
        }
    }
}
