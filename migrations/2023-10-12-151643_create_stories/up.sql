CREATE TABLE stories (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    rating INTEGER,
    comment VARCHAR,
    progress INTEGER,
    length INTEGER,
    link VARCHAR
)
