CREATE TABLE IF NOT EXISTS Book_Issues (
    issue_id SERIAL PRIMARY KEY,
    book_id INT NOT NULL,
    author_id INT NOT NULL, 
    issue_date DATE NOT NULL,
    return_date DATE,
    FOREIGN KEY (book_id) REFERENCES Books(book_id),
    FOREIGN KEY (author_id) REFERENCES Authors(author_id)
);

INSERT INTO Book_Issues (book_id, author_id, issue_date, return_date) VALUES
(1, 1, CURRENT_DATE, NULL); 

INSERT INTO Book_Issues (book_id, author_id, issue_date, return_date) VALUES
(2, 1, CURRENT_DATE - INTERVAL '1 day', NULL);

INSERT INTO Book_Issues (book_id, author_id, issue_date, return_date) VALUES
(3, 2, CURRENT_DATE - INTERVAL '7 days', CURRENT_DATE - INTERVAL '3 days');