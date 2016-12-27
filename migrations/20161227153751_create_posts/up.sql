CREATE TABLE posts (
  id BIGSERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  title VARCHAR NOT NULL DEFAULT 'New Post',
  published BOOLEAN NOT NULL DEFAULT 'f'
)
