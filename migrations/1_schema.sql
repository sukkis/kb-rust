DROP TABLE IF EXISTS articles;

CREATE TABLE IF NOT EXISTS articles (
  id serial PRIMARY KEY,
  title TEXT NOT NULL,
  notebook TEXT NOT NULL,
  tags TEXT[] NOT NULL,
  content TEXT NOT NULL
);
