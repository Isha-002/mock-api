use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    error::Error,
    types::comment::{Comment, GetUserComment, NewComment},
};

use super::Store;

impl Store {
    pub async fn get_comments(
        &self,
        restaurant_id: i32,
        account_id: Uuid,
    ) -> Result<Vec<GetUserComment>, Error> {
        match sqlx::query(
        "SELECT
        comments.id,
        comments.restaurant_id,
        comments.account_id,
        comments.text,
        comments.rating,
        comments.created_on,
        account.name,
        COUNT(CASE WHEN comment_votes.vote_type = 1 THEN 1 END) AS likes,
        COUNT(CASE WHEN comment_votes.vote_type = -1 THEN 1 END) AS dislikes,
        MAX(CASE WHEN comment_votes.account_id = $1 THEN comment_votes.vote_type END) AS current_user_vote
        FROM comments
        LEFT JOIN account ON comments.account_id = account.id 
        LEFT JOIN comment_votes ON comments.id = comment_votes.comment_id 
        WHERE comments.restaurant_id = $2
        GROUP BY comments.id, comments.restaurant_id, comments.account_id, comments.text, comments.rating, comments.created_on, account.name;",
    )
    .bind(account_id)
    .bind(restaurant_id)
    .map(|row: PgRow| GetUserComment {
    comment: Comment {
        id: row.get("id"),
        restaurant_id: row.get("restaurant_id"),
        account_id: row.get("account_id"),
        text: row.get("text"),
        rating: row.get("rating"),
        created_on: row.get("created_on"),
    },
    name: row.get("name"),
    likes: row.get("likes"),
    dislikes: row.get("dislikes"),
    current_user_vote: row.get("current_user_vote"),
    })
    .fetch_all(&self.connection)
    .await
    {
        Ok(comments) => Ok(comments),
        Err(e) => Err(Error::database_query_error(e)),
    }
    }
    pub async fn add_comment(
        &self,
        restaurant_id: i32,
        comment: NewComment,
        account_id: Uuid,
    ) -> Result<Comment, Error> {
        match sqlx::query(
            "INSERT INTO comments (restaurant_id, account_id, text, rating)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (restaurant_id, account_id) DO NOTHING
            RETURNING *;",
        )
        .bind(restaurant_id)
        .bind(account_id)
        .bind(comment.text)
        .bind(comment.rating)
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            restaurant_id: row.get("restaurant_id"),
            account_id: row.get("account_id"),
            text: row.get("text"),
            rating: row.get("rating"),
            created_on: row.get("created_on"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(comments) => Ok(comments),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn delete_comment(&self, comment_id: i32, account_id: Uuid) -> Result<bool, Error> {
        match sqlx::query(
            "DELETE FROM comments
            WHERE id = $1
            AND (account_id = $2 OR EXISTS (
            SELECT 1 FROM account WHERE id = $2 AND role = 'admin'
            ));",
        )
        .bind(comment_id)
        .bind(account_id)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn comment_vote(
        &self,
        account_id: Uuid,
        comment_id: i32,
        vote_type: i32,
    ) -> Result<i32, Error> {
        match sqlx::query(
            "WITH delete_vote AS (
              DELETE FROM comment_votes
              WHERE account_id = $1 
                AND comment_id = $2 
                AND vote_type = $3
              RETURNING 0 AS vote_result
            ),
            upsert_vote AS (
              INSERT INTO comment_votes (account_id, comment_id, vote_type)
              VALUES ($1, $2, $3)
              ON CONFLICT (account_id, comment_id)
              DO UPDATE SET vote_type = EXCLUDED.vote_type
              WHERE NOT EXISTS (SELECT 1 FROM delete_vote) 
              RETURNING vote_type AS vote_result 
            )
            SELECT vote_result FROM delete_vote
            UNION ALL
            SELECT vote_result FROM upsert_vote;",
        )
        .bind(account_id)
        .bind(comment_id)
        .bind(vote_type)
        .map(|row| row.get("vote_result"))
        .fetch_one(&self.connection)
        .await
        {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }
}
