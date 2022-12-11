use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{
    Bookmarked, Deleted, DirectMessage, Hidden, Liked, Retweeted, StreamRule, Tweet, TweetsCount,
    User,
};
use crate::id::IntoNumericId;
use crate::query::{GetConversationRequestBuilder, GetTweetsRequestBuilder};

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_direct_messages_with_participant(
        &self,
        participant_id: impl IntoNumericId,
    ) -> GetTweetsRequestBuilder<A, Vec<DirectMessage>, ()> {
        GetTweetsRequestBuilder::new(
            self,
            self.url(format!("dm_conversations/with/{participant_id}/dm_events"))
                .unwrap(),
        )
    }

    pub fn get_direct_messages_for_conversation(
        &self,
        conversation_id: impl IntoNumericId,
    ) -> GetConversationRequestBuilder<A, Vec<DirectMessage>, ()> {
        GetConversationRequestBuilder::new(
            self,
            self.url(format!("dm_conversations/{conversation_id}/dm_events"))
                .unwrap(),
        )
    }
}
