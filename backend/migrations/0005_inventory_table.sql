CREATE TABLE inventory (
    item_id        SERIAL PRIMARY KEY,      -- Уникальный инвентарный номер экземпляра (например, 0001, 0002)
    book_id        INTEGER NOT NULL,        -- Ссылка на ID из таблицы Book (какая это книга)
    location       VARCHAR(100),            -- Место хранения (например, "Склад A, полка 3")
    status         VARCHAR(50) NOT NULL,    -- Статус экземпляра ("В наличии", "Выдана", "Списана")
    purchase_date  DATE,                    -- Дата приобретения экземпляра

    -- Определение внешнего ключа:
    -- Гарантирует, что book_id ссылается на существующую запись в таблице Book
    FOREIGN KEY (book_id) REFERENCES book(id)
        ON DELETE RESTRICT -- Не позволит удалить книгу из Book, пока есть её экземпляры на складе
);