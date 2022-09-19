
CREATE TABLE tags (
  video_id TEXT NOT NULL,
  title TEXT NOT NULL,

  PRIMARY KEY(video_id, title)
);
