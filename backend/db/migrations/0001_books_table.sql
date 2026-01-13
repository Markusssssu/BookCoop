CREATE TABLE IF NOT EXISTS Books (
    book_id SERIAL PRIMARY KEY, 
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    genre VARCHAR(100),
    page_count INT
);

INSERT INTO Books (title, author, genre, page_count) VALUES
('1984', 'Джордж Оруэлл', 'Антиутопия', 328),
('Скотный двор', 'Джордж Оруэлл', 'Сатира', 112),
('Гордость и предубеждение', 'Джейн Остин', 'Роман', 279);

