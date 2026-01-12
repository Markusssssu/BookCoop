CREATE TABLE stock_item (
    inventory_id SERIAL PRIMARY KEY,
    book_id      INT  NOT NULL REFERENCES book(id),
    location     VARCHAR(100), 
    status       VARCHAR(50) NOT NULL DEFAULT 'In Stock' 
);