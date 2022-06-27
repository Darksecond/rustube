
CREATE TABLE videos (
  id TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  path TEXT NOT NULL,
  published_at INTEGER NOT NULL,
  duration INTEGER NOT NULL,
  channel_id TEXT NOT NULL
);
