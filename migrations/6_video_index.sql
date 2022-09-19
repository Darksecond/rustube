
CREATE VIRTUAL TABLE video_index USING fts5 (
  video_id UNINDEXED,
  title,
  description,
  channel,
  tags,
);
