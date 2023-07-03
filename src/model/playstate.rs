use sqlx::sqlite::SqlitePool;
use time::OffsetDateTime;
use time::serde::iso8601;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpsertPlaystate {
    pub id: String,
    pub position: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Playstate {
    pub id: String,
    pub position: u32,
    #[serde(with = "iso8601")]
    pub modified_at: OffsetDateTime,
}

#[derive(Debug)]
pub struct PlaystateFromQuery {
    pub id: String,
    pub position: i64,
    pub modified_at: i64,
}

impl PlaystateFromQuery {
    pub fn into_playstate(self) -> Playstate {
        Playstate {
            id: self.id,
            position: self.position as u32,
            modified_at: OffsetDateTime::from_unix_timestamp(self.modified_at).unwrap(),
        }
    }
}

pub async fn upsert_playstate(playstate: UpsertPlaystate, db: &SqlitePool) -> Result<(), sqlx::Error> {
    let modified_at = OffsetDateTime::now_utc().unix_timestamp();

    sqlx::query!(r#"
                 INSERT INTO playstates
                 (id, position, modified_at)
                 VALUES ($1, $2, $3)
                 ON CONFLICT(id) DO UPDATE SET position=$2,modified_at=$3
                 "#,
                 playstate.id,
                 playstate.position,
                 modified_at
                 )
        .fetch_optional(db)
        .await?;

    Ok(())
}

pub async fn get_playstate(id: &str, db: &SqlitePool) -> Result<Option<Playstate>, sqlx::Error> {
    let playstate = sqlx::query_as!(PlaystateFromQuery, r#"
                                         SELECT playstates.id, playstates.position, playstates.modified_at
                                         FROM playstates
                                         WHERE playstates.id = $1
                                "#, id)
        .fetch_optional(db)
        .await?
        .map(PlaystateFromQuery::into_playstate);

    Ok(playstate)
}
