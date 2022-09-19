use sqlx::sqlite::SqlitePool;
use super::{Video, get_video};
use futures::prelude::*;

struct IndexFromQuery {
    pub video_id: String,
}

pub struct UpsertVideoIndex {
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel_title: String,
    pub tags: Vec<String>,
}

//TODO Refactor to not use get_video
pub async fn match_videos(term: &str, db: &SqlitePool) -> Result<Vec<Video>, sqlx::Error> {
    let term = term.trim();

    // sqlite/sqlx can't handle empty terms.
    if term.is_empty() {
        return Ok(Vec::new());
    }

    let indexes: Vec<_> = sqlx::query_as!(IndexFromQuery, r#"
                                          SELECT video_id as "video_id!: String" FROM video_index WHERE video_index MATCH $1 ORDER BY rank
                                          "#, term)
        .fetch(db)
        .try_collect()
        .await?;

    let mut videos = Vec::with_capacity(indexes.len());

    for index in indexes {
        if let Some(video) = get_video(&index.video_id, db).await? {
            videos.push(video);
        }
    }
    Ok(videos)
}

pub async fn upsert_video_index(video: UpsertVideoIndex, db: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut transaction = db.begin().await?;
    let tags = video.tags.join(" ");
    sqlx::query!(r#"DELETE FROM video_index WHERE video_id = $1"#, video.id).execute(&mut transaction).await?;
    sqlx::query!(r#"
                 INSERT INTO video_index
                 (video_id, title, description, channel, tags)
                 VALUES
                 ($1, $2, $3, $4, $5)
                 "#, video.id, video.title, video.description, video.channel_title, tags)
        .execute(&mut transaction)
        .await?;
    transaction.commit().await?;
    Ok(())
}
