use super::{JobContext, JobError};
use std::path::Path;
use serde::Deserialize;
use time::Date;
use sqlx::sqlite::SqlitePool;
use crate::model::channel::{upsert_channel, Channel};
use crate::model::video::{upsert_video, UpsertVideo};
use crate::model::tag::upsert_tags;

#[derive(Deserialize, Debug)]
pub struct YtVideo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel_id: Option<String>,
    pub channel: Option<String>,
    pub uploader_id: String,
    pub uploader: String,
    pub duration: f64,
    upload_date: String,
    pub tags: Option<Vec<String>>,
}

impl YtVideo {
    pub fn to_channel(&self) -> Channel {
        let channel_id = self.channel_id.as_ref().unwrap_or(&self.uploader_id);
        let channel = self.channel.as_ref().unwrap_or(&self.uploader);
        Channel {
            id: channel_id.clone(),
            title: channel.clone(),
        }
    }

    pub fn to_upsert_video(&self, path: &Path) -> UpsertVideo {
        let channel_id = self.channel_id.as_ref().unwrap_or(&self.uploader_id);
        let mp4_path = path
            .to_string_lossy()
            .to_string();

        UpsertVideo {
            id: self.id.clone(),
            description: self.description.clone(),
            duration: self.duration as u32,
            path: mp4_path,
            title: self.title.clone(),
            published_at: self.upload_date(),
            channel_id: channel_id.clone(),
        }
    }
}

impl YtVideo {
    pub fn upload_date(&self) -> Date {
        let format = time::format_description::parse("[year][month][day]").expect("Could not format format");
        Date::parse(&self.upload_date, &format).expect("Could not parse date")
    }
}

async fn process_entry(path: &Path, db: &SqlitePool) -> Result<(), JobError> {
    let json = tokio::fs::read_to_string(path.with_extension("info.json")).await?;
    let json: YtVideo = serde_json::from_str(&json)?;

    upsert_channel(json.to_channel(), db).await?;
    upsert_video(json.to_upsert_video(path), db).await?;
    if let Some(tags) = json.tags {
        upsert_tags(&json.id, tags.as_slice(), db).await?;
    }

    Ok(())
}

pub async fn job(context: JobContext) -> Result<(), JobError> {
    let root = &context.config.video_root;

    tracing::info!(?root, "Scanning directory");

    let entries = wax::walk("**/*.mp4", root)?
        .filter_map(|e| e.ok());

    for entry in entries {
        match process_entry(entry.path(), &context.db).await {
            Ok(_) => (),
            Err(error) => {
                //TODO Log path?
                tracing::warn!(%error, "error processing entry");
            },
        }
    }

    Ok(())
}
