CREATE TABLE IF NOT EXISTS book_author_category (
    book_id INTEGER NOT NULL REFERENCES book(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES category(id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, author_id, category_id) 
);
