use sqlx::sqlite::SqlitePool;
use futures::prelude::*;


#[allow(dead_code)]
#[derive(Debug)]
struct TagFromQuery {
    pub video_id: String,
    pub title: String,
}

#[tracing::instrument(skip(db))]
pub async fn upsert_tags<T: AsRef<str> + std::fmt::Debug>(video_id: &str, tags: &[T], db: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut transaction = db.begin().await?;

    sqlx::query!(r#"
                 DELETE FROM tags WHERE video_id = $1
                 "#, video_id)
        .execute(&mut transaction)
        .await?;

    for tag in tags {
        let tag = tag.as_ref();

        sqlx::query!(r#"
                     INSERT INTO tags
                     (video_id, title)
                     VALUES
                     ($1, $2)
                     "#, video_id, tag)
            .execute(&mut transaction)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}

#[tracing::instrument(skip(db))]
pub async fn get_tags(video_id: &str, db: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let tags: Vec<_> = sqlx::query_as!(TagFromQuery, r#"
                               SELECT video_id, title
                               FROM tags
                               WHERE video_id = $1
                               "#, video_id)
        .fetch(db)
        .map_ok(|tag| { tag.title })
        .try_collect()
        .await?;

    Ok(tags)
}
