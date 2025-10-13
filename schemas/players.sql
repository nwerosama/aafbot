CREATE TABLE IF NOT EXISTS players (
  name VARCHAR(30) NOT NULL PRIMARY KEY,
  total_played INT NOT NULL DEFAULT 0 -- total minutes
);
