test:
		diesel database reset
		psql -d rust_blog_api -f fixtures.sql
		cargo test
