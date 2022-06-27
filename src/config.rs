use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Config {
    // See https://docs.rs/sqlx/0.4.0-beta.1/sqlx/sqlite/struct.SqliteConnectOptions.html for URL options
    /// The connection url for the sqlite database to use.
    #[clap(short, long, env)]
    pub database_url: String,

    /// The port to listen on.
    #[clap(short, long, env, default_value_t = 3000)]
    pub port: u16,

    /// The root path for the videos.
    #[clap(short = 'r', long, env)]
    pub video_root: PathBuf,
}
