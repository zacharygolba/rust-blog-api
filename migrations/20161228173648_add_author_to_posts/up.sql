ALTER TABLE posts
ADD COLUMN author_id BIGINT NOT NULL,
ADD FOREIGN KEY (author_id) REFERENCES authors(id);
CREATE INDEX posts_author_id_fkey_idx ON posts(author_id);
