use axum::{extract::Form, response::Html, routing::get, Router};

pub async fn show_form_book() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
<html lang="ru">
    <head>
        <meta charset="UTF-8">
        <title>Управление книгами</title>
        <style>
            /* Небольшие стили для удобства */
            form { display: flex; flex-direction: column; gap: 15px; max-width: 300px; }
            label { display: flex; flex-direction: column; font-weight: bold; }
            input { padding: 8px; margin-top: 5px; }
            input[type="submit"] { background-color: #4CAF50; color: white; cursor: pointer; border: none; }
        </style>
    </head>
    <body>
        <h1>Добавить книгу</h1>
        <form action="/" method="post">
            <!-- Поле ID (обычно скрыто или только для чтения при создании) -->
            <label for="id">
                ID (u64):
                <input type="number" name="id" id="id" min="0" placeholder="Автоматически" readonly>
            </label>

            <label for="title">
                Название:
                <input type="text" name="title" id="title" required placeholder="Введите название книги">
            </label>

            <label for="author">
                Автор:
                <input type="text" name="author" id="author" required placeholder="Имя автора">
            </label>

            <label for="isbn">
                ISBN:
                <input type="text" name="isbn" id="isbn" required placeholder="Например, 978-3-16-148410-0">
            </label>

            <input type="submit" value="Сохранить книгу">
        </form>
    </body>
    </html>
        "#,
    )
}

pub async fn show_form_author() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}