CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    time TIMESTAMP default NOW(),
    title VARCHAR NOT NULL,
    text TEXT NOT NULL
)