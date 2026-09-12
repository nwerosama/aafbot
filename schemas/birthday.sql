CREATE TABLE IF NOT EXISTS birthdays (
  user_id BIGINT PRIMARY KEY,
  birth_date DATE NOT NULL
);

CREATE TABLE IF NOT EXISTS birthday_conf (
  channel_id BIGINT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS birthday_announcements (
  announced_on DATE PRIMARY KEY
);
