use sqlx::sqlite::SqlitePool;

#[derive(Debug)]
pub struct Channel {
    pub id: String,
    pub title: String,
}

//TODO Do not use anyhow
pub async fn upsert_channel(channel: Channel, db: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query!(r#"
                 INSERT INTO channels
                 (id, title)
                 VALUES ($1, $2)
                 ON CONFLICT(id) DO UPDATE SET title=$2
                 "#,
                    channel.id,
                    channel.title)
        .fetch_optional(db)
        .await?;

    Ok(())
}

//TODO Do not use anyhow
pub async fn list_channels(db: &SqlitePool) -> anyhow::Result<Vec<Channel>> {
    let channels = sqlx::query_as!(Channel, r#"
                                   SELECT id, title FROM channels
                                   ORDER BY title COLLATE NOCASE ASC
                                   "#)
        .fetch_all(db).await?;

    Ok(channels)
}

pub async fn get_channel(id: &str, db: &SqlitePool) -> Result<Option<Channel>, sqlx::Error> {
    let video = sqlx::query_as!(Channel, r#"
                                         SELECT id, title
                                         FROM channels
                                         WHERE id = $1
                                "#, id)
        .fetch_optional(db)
        .await?;

    Ok(video)
}
