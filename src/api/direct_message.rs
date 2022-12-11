use super::TwitterApi;
use crate::authorization::Authorization;
use crate::data::DirectMessage;
use crate::id::IntoNumericId;
use crate::meta::TweetsMeta;
use crate::query::GetConversationRequestBuilder;
use crate::requests::DirectMessageBuilder;

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_dm_with_participant(
        &self,
        participant_id: impl IntoNumericId,
    ) -> GetConversationRequestBuilder<A, Vec<DirectMessage>, TweetsMeta> {
        GetConversationRequestBuilder::new(
            self,
            self.url(format!("dm_conversations/with/{participant_id}/dm_events"))
                .unwrap(),
        )
    }

    pub fn get_dm_for_conversation(
        &self,
        conversation_id: impl IntoNumericId,
    ) -> GetConversationRequestBuilder<A, Vec<DirectMessage>, TweetsMeta> {
        GetConversationRequestBuilder::new(
            self,
            self.url(format!("dm_conversations/{conversation_id}/dm_events"))
                .unwrap(),
        )
    }

    pub fn post_dm_to_conversation(
        &self,
        conversation_id: impl IntoNumericId,
    ) -> DirectMessageBuilder<A> {
        DirectMessageBuilder::new(
            self,
            self.url(format!("dm_conversations/{conversation_id}/messages"))
                .unwrap(),
        )
    }

    pub fn post_dm_to_participant(
        &self,
        participant_id: impl IntoNumericId,
    ) -> DirectMessageBuilder<A> {
        DirectMessageBuilder::new(
            self,
            self.url(format!("dm_conversations/with/{participant_id}/messages"))
                .unwrap(),
        )
    }
}
