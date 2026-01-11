CREATE TABLE book_author (
    book_id BIGINT REFERENCES book(id) ON DELETE CASCADE,
    author_id BIGINT REFERENCES author(id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, author_id)
);
