use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Clone, Debug, Type, sqlx::FromRow)]
#[sqlx(type_name = "commentType")]
pub struct Comment {
    pub id: i32,
    pub restaurant_id: i32,
    pub name: String,
    pub text: String,
    pub likes: i32,
    pub dislikes: i32,
    // pub date: String
}

#[derive(Debug, Clone, Deserialize, Serialize, Type, sqlx::FromRow)]
pub struct NewComment {
    pub name: String,
    pub text: String,
    // pub date: String,
}
