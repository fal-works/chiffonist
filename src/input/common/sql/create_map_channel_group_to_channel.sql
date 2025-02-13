CREATE TABLE IF NOT EXISTS map_channel_group_to_channel (
  channel_group TEXT NOT NULL,
  channel TEXT NOT NULL,
  PRIMARY KEY (channel_group, channel)
);