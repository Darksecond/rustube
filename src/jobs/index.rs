use super::{JobContext, JobError};
use crate::model::tag::get_tags;
use crate::model::video::{Video, list_videos};
use crate::model::video::index::{UpsertVideoIndex, upsert_video_index};
use sqlx::sqlite::SqlitePool;

async fn process_entry(video: Video, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let tags = get_tags(&video.id, db).await?;

        let video = UpsertVideoIndex {
            id: video.id,
            title: video.title,
            description: video.description,
            channel_title: video.channel.title,
            tags,
        };

        upsert_video_index(video, db).await?;

        Ok(())
}

pub async fn job(context: JobContext) -> Result<(), JobError> {
    let videos = list_videos(&context.db).await?;

    for video in videos {
        match process_entry(video, &context.db).await {
            Ok(()) => (),
            Err(error) => {
                tracing::warn!(%error, "error processing entry");
            },
        }
    }

    Ok(())
}
