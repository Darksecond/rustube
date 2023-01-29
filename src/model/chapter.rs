use sqlx::sqlite::SqlitePool;
use futures::prelude::*;

#[derive(Debug)]
pub struct Chapter {
    pub video_id: String,
    pub title: String,
    pub start_time: i64,
}

impl Chapter {
    pub fn format_time(&self) -> String {
        let minutes = self.start_time / 60;
        let seconds = self.start_time % 60;

        format!("{:02}:{:02}", minutes, seconds)
    }
}

#[derive(Debug)]
pub struct UpsertChapter {
    pub title: String,
    pub start_time: i64,
}

pub async fn upsert_chapters(id: &str, chapters: &[UpsertChapter], db: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut transaction = db.begin().await?;

    sqlx::query!(r#"
                 DELETE FROM chapters WHERE video_id=$1
                 "#, id)
        .execute(&mut transaction)
        .await?;


    for chapter in chapters {
        tracing::trace!("Inserting chapter {:?}", chapter);

        sqlx::query!(r#"
                     INSERT INTO chapters
                     (video_id, title, start_time)
                     VALUES
                     ($1, $2, $3)
                     "#, id, chapter.title, chapter.start_time)
            .execute(&mut transaction)
            .await?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn get_chapters(id: &str, db: &SqlitePool) -> Result<Vec<Chapter>, sqlx::Error> {
    let chapters: Vec<_> = sqlx::query_as!(Chapter, r#"
                                   SELECT video_id, title, start_time FROM chapters WHERE video_id=$1 ORDER BY start_time ASC
                                   "#, id)
        .fetch(db)
        .try_collect()
        .await?;

    Ok(chapters)
}
