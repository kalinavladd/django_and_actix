use crate::AppState;
use actix_web::{App, web};
use actix_web::web::{Data, Path};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{self, Error, FromRow, PgPool};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub session_key: String,
    pub session_data: String,
    pub expire_date: DateTime<Utc>,
}

impl Session {
    pub async fn get_session_data_by_session_id(state: Data<AppState>, session_key: String) -> Result<Session, Error>{
        let session_data = sqlx::query_as::<_, Session>(
            "SELECT * FROM django_session WHERE session_key = $1;"
        )
            .bind(session_key)
            .fetch_optional(&state.db)
            .await?
            .unwrap();
        Ok(session_data)
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Post {
    id: i64,
    title: String,
    text: String,
    author_id: i32,
}

impl Post {
    pub async fn get_one_post(state: Data<AppState>, post_id: i32) -> Result<Post, Error> {
        let post = sqlx::query_as::<_, Post>(
            "SELECT id, title, text, author_id FROM blog_post WHERE id = $1;",
        )
        .bind(post_id)
        .fetch_optional(&state.db)
        .await?
        .unwrap();
        Ok(post)
    }

    pub async fn get_all_posts_by_author(state: Data<AppState>, author_id: i32,) -> Result<Vec<Post>, Error> {
        let posts = sqlx::query_as::<_, Post>(
            "SELECT id, title, text, author_id FROM blog_post WHERE author_id = $1;",
        )
        .bind(author_id)
        .fetch_all(&state.db)
        .await?;
        Ok(posts)
    }

    pub async fn get_all_posts(state: Data<AppState>) -> Result<Vec<Post>, Error> {
        let posts = sqlx::query_as::<_, Post>(
            "SELECT id, title, text, author_id FROM blog_post;",
        )
        .fetch_all(&state.db)
        .await?;
        Ok(posts)
    }
}
