use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: i32,
    pub restaurant_id: i32,
    pub account_id: Uuid,
    pub text: String,
    pub rating: i32,
    pub created_on: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewComment {
    pub restaurant_id: i32,
    pub text: String,
    pub rating: i32,
    pub created_on: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentVote {
    pub account_id: Uuid,
    pub comment_id: i32,
    pub vote_type: i32, // -1= dislike | 1= like
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUserComment {
    #[serde(flatten)]
    pub comment: Comment,
    pub name: String,
    pub likes: i64,
    pub dislikes: i64,
    pub current_user_vote: Option<i32>,
}
