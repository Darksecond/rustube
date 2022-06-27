use sqlx::sqlite::SqlitePool;
use futures::prelude::*;
use time::{OffsetDateTime, Date};
use super::channel::Channel;
use super::playstate::{Playstate, PlaystateFromQuery};

#[derive(Debug)]
pub struct UpsertVideo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub published_at: Date,
    pub duration: u32,
    pub channel_id: String,
}

#[derive(Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub published_at: Date,
    pub duration: u32,
    pub channel: Channel,
    pub playstate: Option<Playstate>,
}

impl Video {
    pub fn percentage_watched(&self) -> u32 {
        if let Some(playstate) = &self.playstate {
            playstate.position * 100 / self.duration
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct VideoFromQuery {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub published_at: i64, //TODO Use string
    pub duration: i64,
    pub channel_id: String,
    pub channel_title: String,
    pub playstate_id: Option<String>,
    pub playstate_position: Option<i64>,
    pub playstate_modified_at: Option<i64>,
}

impl VideoFromQuery {
    pub fn into_video(self) -> Video {
        let playstate = if let Some(id) = self.playstate_id {
            Some(PlaystateFromQuery {
                id,
                position: self.playstate_position.unwrap(),
                modified_at: self.playstate_modified_at.unwrap(),
            }.into_playstate())
        } else {
            None
        };

        Video {
            id: self.id,
            title: self.title,
            description: self.description,
            path: self.path.into(),
            published_at: OffsetDateTime::from_unix_timestamp(self.published_at).unwrap().date(),
            duration: self.duration as u32,
            channel: Channel {
                id: self.channel_id,
                title: self.channel_title,
            },
            playstate,
        }
    }
}

pub async fn upsert_video(video: UpsertVideo, db: &SqlitePool) -> Result<(), sqlx::Error> {
    let timestamp = video.published_at.with_hms(0,0,0).unwrap().assume_utc().unix_timestamp();
    let duration = video.duration as i64;

    sqlx::query!(r#"
                 INSERT INTO videos
                 (id, title, description, path, published_at, channel_id, duration)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 ON CONFLICT(id) DO UPDATE SET title=$2,description=$3,path=$4, published_at=$5,channel_id=$6,duration=$7
                 "#,
                    video.id,
                    video.title,
                    video.description,
                    video.path,
                    timestamp,
                    video.channel_id,
                    duration)
        .fetch_optional(db)
        .await?;

    Ok(())
}

//TODO playstate
pub async fn list_videos(db: &SqlitePool) -> Result<Vec<Video>, sqlx::Error> {
    let videos: Vec<_> = sqlx::query_as!(VideoFromQuery, r#"
                                         SELECT videos.id, videos.title, videos.description, videos.path, videos.published_at, videos.duration, videos.channel_id,
                                         channels.title AS channel_title,
                                         playstates.id as "playstate_id?", playstates.position as "playstate_position?", playstates.modified_at as "playstate_modified_at?"
                                         FROM videos
                                         LEFT JOIN playstates ON videos.id = playstates.id
                                         INNER JOIN channels ON channels.id = videos.channel_id
                                         ORDER BY published_at DESC
                                         "#)
        .fetch(db)
        .map_ok(VideoFromQuery::into_video)
        .try_collect()
        .await?;

    Ok(videos)
}

//TODO playstate
pub async fn list_videos_for_channel(channel_id: &str, db: &SqlitePool) -> Result<Vec<Video>, sqlx::Error> {
    let videos: Vec<_> = sqlx::query_as!(VideoFromQuery, r#"
                                         SELECT videos.id, videos.title, videos.description, videos.path, videos.published_at, videos.duration, videos.channel_id,
                                         channels.title AS channel_title,
                                         playstates.id as "playstate_id?", playstates.position as "playstate_position?", playstates.modified_at as "playstate_modified_at?"
                                         FROM videos
                                         INNER JOIN channels ON channels.id = videos.channel_id
                                         LEFT JOIN playstates ON playstates.id = videos.id
                                         WHERE videos.channel_id = $1
                                         ORDER BY published_at DESC
                                         "#,
                                         channel_id)
        .fetch(db)
        .map_ok(VideoFromQuery::into_video)
        .try_collect()
        .await?;

    Ok(videos)
}

pub async fn get_video(id: &str, db: &SqlitePool) -> Result<Option<Video>, sqlx::Error> {
    let video = sqlx::query_as!(VideoFromQuery, r#"
                                         SELECT videos.id, videos.title, videos.description, videos.path, videos.published_at, videos.duration, videos.channel_id,
                                         channels.title AS channel_title,
                                         playstates.id as "playstate_id?", playstates.position as "playstate_position?", playstates.modified_at as "playstate_modified_at?"
                                         FROM videos
                                         INNER JOIN channels ON channels.id = videos.channel_id
                                         LEFT JOIN playstates ON playstates.id = videos.id
                                         WHERE videos.id = $1
                                "#, id)
        .fetch_optional(db)
        .await?
        .map(VideoFromQuery::into_video);

    Ok(video)
}
