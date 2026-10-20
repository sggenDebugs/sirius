use sqlx::{PgPool, FromRow};
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct Board {
    pub id: i64,
    pub title: String,
    pub created_at: DateTime<Utc>,
}

pub async fn get_all_boards(pool: &PgPool) -> Result<Vec<Board>, sqlx::Error> {
    let boards = sqlx::query_as!(
        Board,
        "SELECT id, title, created_at FROM board"
    )
    .fetch_all(pool)
    .await?;

    Ok(boards)
}

pub async fn create_board(pool: &PgPool, id: i32, title: &str) -> Result<i32, sqlx::Error> {
    let inserted_id: (i32,) = sqlx::query_as(
        "INSERT INTO board (id, title) VALUES ($1, $2) RETURNING id"
    )
    .bind(id)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(inserted_id.0)
}

pub async fn rename_board_title(pool: &PgPool, id: i32, title: &str) -> Result<i32, sqlx::Error> {
    let inserted_id: (i32,) = sqlx::query_as(
        "UPDATE board SET title = $2 WHERE id = $1 RETURNING id"
    )
    .bind(id)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(inserted_id.0)
}

pub async fn delete_board(pool: &PgPool, id: i32) -> Result<i32, sqlx::Error> {
    let inserted_id: (i32,) = sqlx::query_as(
        "DELETE FROM board WHERE id = $1 RETURNING id"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(inserted_id.0)
}