CREATE TABLE IF NOT EXISTS players (
  name VARCHAR(30) NOT NULL PRIMARY KEY,
  total_played INT NOT NULL DEFAULT 0, -- total minutes
  last_seen_server VARCHAR(30),
  last_seen_date BIGINT,
  FOREIGN KEY (last_seen_server) REFERENCES servers(friendly)
);

ALTER TABLE players
  ADD COLUMN IF NOT EXISTS last_seen_server VARCHAR(30),
  ADD COLUMN IF NOT EXISTS last_seen_date BIGINT;

CREATE TABLE IF NOT EXISTS sessions (
  name VARCHAR(30),
  server VARCHAR(30),
  playtime INT,
  PRIMARY KEY (name, server),
  FOREIGN KEY (server) REFERENCES servers(internal)
);
