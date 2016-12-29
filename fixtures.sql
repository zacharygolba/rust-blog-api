INSERT INTO authors(
    email,
    first_name,
    last_name
) VALUES(
    'zachary.golba@postlight.com',
    'Zachary',
    'Golba'
);

DO $$
BEGIN
    FOR counter IN 1..20 LOOP
        INSERT INTO posts(
            body,
            title,
            published,
            author_id
        ) VALUES(
            'Testing...',
            concat('Test Post ', counter),
            counter > 10,
            1
        );
    END LOOP;
END; $$
