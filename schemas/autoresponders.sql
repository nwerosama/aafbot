CREATE TABLE IF NOT EXISTS autoresponders (
  id SERIAL PRIMARY KEY,
  guild_id BIGINT NOT NULL,
  channel_id BIGINT NOT NULL,
  keywords TEXT[] NOT NULL,
  response TEXT NOT NULL,
  UNIQUE (guild_id, channel_id, keywords)
);

CREATE INDEX IF NOT EXISTS idx_autoresponder_trigger
  ON autoresponders (guild_id, channel_id) INCLUDE (keywords);
