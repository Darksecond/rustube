
CREATE TABLE playlists (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE
);

CREATE TABLE playlist_items (
  playlist_id INTEGER NOT NULL,
  item_index INTEGER NOT NULL,
  video_id TEXT NOT NULL,

  PRIMARY KEY(playlist_id, item_index)
);
