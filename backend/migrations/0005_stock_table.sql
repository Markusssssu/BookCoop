CREATE TABLE stock (
    id BIGSERIAL PRIMARY KEY,
    book_id BIGINT NOT NULL REFERENCES book(id),
    quantity INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(book_id)
);
