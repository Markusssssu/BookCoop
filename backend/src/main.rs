mod application;
mod domain;
mod infrastructure;
mod presentation;

use axum::{routing::get, Router, http::StatusCode};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use sqlx::postgres::PgPoolOptions;
use std::{env, io, time::{Duration, Instant}};
use tokio::sync::mpsc;

// Вкладки интерфейса
#[derive(PartialEq)]
enum Tab { Dashboard, Database }

// Состояние приложения
struct App {
    active_tab: Tab,
    server_logs: Vec<String>,
    cpu_activity: Vec<u64>,
    db_table_rows: Vec<Vec<String>>,
    table_state: TableState,
}

impl App {
    fn new() -> Self {
        Self {
            active_tab: Tab::Dashboard,
            server_logs: vec!["[SYSTEM] Инициализация...".into()],
            cpu_activity: vec![0; 50],
            db_table_rows: vec![],
            table_state: TableState::default(),
        }
    }

    // Ручная реализация выбора следующей строки (совместима со всеми версиями Ratatui)
    fn next_row(&mut self) {
        if self.db_table_rows.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => if i >= self.db_table_rows.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous_row(&mut self) {
        if self.db_table_rows.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => if i == 0 { self.db_table_rows.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // 1. Подготовка терминала
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Каналы связи (Logs и DB Data)
    let (log_tx, mut log_rx) = mpsc::channel::<String>(100);
    let (data_tx, mut data_rx) = mpsc::channel::<Vec<Vec<String>>>(1);

    // 3. База данных
    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    let pool_clone = pool.clone();

    // 4. Запуск HTTP Сервера
    let srv_log = log_tx.clone();
    tokio::spawn(async move {
        let app = Router::new().route("/api/health", get(|| async { StatusCode::OK }));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
        let _ = srv_log.send("[HTTP] Сервер запущен на http://localhost:8080".into()).await;
        axum::serve(listener, app.into_make_service()).await.unwrap();
    });

    // 5. Инициализация состояния
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(200);

    // 6. Главный цикл отрисовки
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Вкладки
                    Constraint::Min(10),   // Контент
                    Constraint::Length(1), // Подсказки
                ])
                .split(f.size());

            // Вкладки
            let titles = vec![" 1. Dashboard ", " 2. Database Explorer "];
            let tabs = Tabs::new(titles)
                .select(if app.active_tab == Tab::Dashboard { 0 } else { 1 })
                .block(Block::default().borders(Borders::ALL).title(" BookCoop Server Control "))
                .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            f.render_widget(tabs, chunks[0]);

            // Контент в зависимости от вкладки
            match app.active_tab {
                Tab::Dashboard => {
                    let inner_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[1]);

                    // График
                    let sparkline = Sparkline::default()
                        .block(Block::default().title(" Нагрузка CPU (Live) ").borders(Borders::ALL))
                        .data(&app.cpu_activity)
                        .style(Style::default().fg(Color::Green));
                    f.render_widget(sparkline, inner_chunks[0]);

                    // Логи
                    let log_items: Vec<ListItem> = app.server_logs.iter().rev().take(20)
                        .map(|s| ListItem::new(s.as_str())).collect();
                    let list = List::new(log_items)
                        .block(Block::default().title(" Системные логи ").borders(Borders::ALL));
                    f.render_widget(list, inner_chunks[1]);
                }
                Tab::Database => {
                    let rows = app.db_table_rows.iter().map(|r| {
                        Row::new(r.iter().map(|c| Cell::from(c.as_str())))
                    });

                    let table = Table::new(rows, [
                        Constraint::Length(10), // ID
                        Constraint::Min(30),   // Title
                        Constraint::Length(15), // Status
                    ])
                        .header(Row::new(vec!["ID", "Название книги", "Инфо"]).style(Style::default().fg(Color::Yellow)))
                        .block(Block::default().title(" Данные из таблицы 'books' ").borders(Borders::ALL))
                        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
                        .highlight_symbol(">> ");

                    f.render_stateful_widget(table, chunks[1], &mut app.table_state);
                }
            }

            // Подсказки
            let help = Paragraph::new(" [1,2]: Вкладки | [↑/↓]: Навигация | [Q]: Выход ");
            f.render_widget(help, chunks[2]);
        })?;

        // Чтение сообщений из каналов
        while let Ok(msg) = log_rx.try_recv() {
            app.server_logs.push(msg);
        }
        if let Ok(data) = data_rx.try_recv() {
            app.db_table_rows = data;
        }

        // Обработка ввода
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('1') => app.active_tab = Tab::Dashboard,
                    KeyCode::Char('2') => {
                        app.active_tab = Tab::Database;
                        let p = pool_clone.clone();
                        let d_tx = data_tx.clone();
                        tokio::spawn(async move {
                            // ВАЖНО: Замените SQL запрос под вашу структуру БД
                            let query = "SELECT id::text, title FROM books LIMIT 20";
                            if let Ok(records) = sqlx::query_as::<_, (String, String)>(query).fetch_all(&p).await {
                                let rows = records.into_iter()
                                    .map(|(id, title)| vec![id, title, "Online".to_string()])
                                    .collect();
                                let _ = d_tx.send(rows).await;
                            }
                        });
                    }
                    KeyCode::Down => app.next_row(),
                    KeyCode::Up => app.previous_row(),
                    _ => {}
                }
            }
        }

        // Анимация графика
        if last_tick.elapsed() >= tick_rate {
            let val = if app.active_tab == Tab::Dashboard { rand::random::<u64>() % 10 } else { 2 };
            app.cpu_activity.push(val);
            app.cpu_activity.remove(0);
            last_tick = Instant::now();
        }
    }

    // Восстановление терминала
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
