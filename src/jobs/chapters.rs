use super::{JobContext, JobError};
use sqlx::sqlite::SqlitePool;
use crate::model::video::{Video, list_videos};
use crate::model::chapter::{UpsertChapter, upsert_chapters};

async fn process_entry(video: Video, db: &SqlitePool) -> Result<(), JobError> {
    let info = ffprobe::get_info(&video.path).await?;

    let chapters: Vec<_> = info.chapters.into_iter().map(|c| {
        UpsertChapter {
            title: c.tags.title,
            start_time: (c.start / 1000) as _,
        }
    }).collect();

    upsert_chapters(&video.id, &chapters, db)
        .await?;

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

mod ffprobe {
    use std::path::Path;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Info {
        pub chapters: Vec<Chapter>,
    }

    #[derive(Deserialize)]
    pub struct Chapter {
        pub start: u64,
        pub tags: Tags,
    }

    #[derive(Deserialize)]
    pub struct Tags {
        pub title: String,
    }

    pub async fn get_info(path: impl AsRef<Path>) -> anyhow::Result<Info> {
        use tokio::process::Command;
        let data = Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-print_format")
            .arg("json")
            .arg("-show_chapters")
            .arg(path.as_ref())
            .output()
            .await?;

        let data = serde_json::from_slice(&data.stdout)?;

        Ok(data)
    }
}
