CREATE TABLE IF NOT EXISTS inventory (
    book_id INTEGER PRIMARY KEY REFERENCES book(id) ON DELETE CASCADE,
    price NUMERIC(10, 2) NOT NULL CHECK (price >= 0),
    stock_count INTEGER NOT NULL CHECK (stock_count >= 0),
    cover_url VARCHAR(255)
);