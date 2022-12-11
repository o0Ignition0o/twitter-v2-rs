use super::Attachments;
use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "arbitrary_precision", derive(Eq))]
pub struct DirectMessage {
    pub id: NumericId,
    pub text: String,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<OffsetDateTime>,
    pub sender_id: NumericId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
}
