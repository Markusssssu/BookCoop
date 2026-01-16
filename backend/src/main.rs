mod application;
mod domain;
mod infrastructure;
mod presentation;

use axum::{
    routing::{get, post, put, delete},
    Router, http::StatusCode,
    extract::Extension,
};
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::*,
};
use sqlx::postgres::PgPoolOptions;
use std::{env, io, time::{Duration, Instant}, sync::Arc};
use std::net::SocketAddr;
use tokio::sync::mpsc;

use application::service::{
    book_service::BookService,
    author_service::AuthorService,
    admin_service::AdminService,
    book_issues_service::BookIssueService,
};

use infrastructure::persistence::{
    sql_book_repository::SQLBookRepository,
    sql_author_repository::SQLAuthorRepository,
    sql_admin_repository::SQLAdminRepository,
    sql_book_issues_repository::SQLBookIssuesRepository,
};

// Подключаем маршруты
use presentation::web_api::routes::{
    admin_routes, author_routes, book_routes, book_issues_routes
};
use crate::presentation::web_api::routes::author_routes::author_routes;
use crate::presentation::web_api::routes::book_issues_routes::book_issues_routes;
use crate::presentation::web_api::routes::book_routes::book_routes;

#[derive(PartialEq)]
enum Tab { Dashboard, Database }
#[derive(PartialEq)]
enum DbTab { Books, Authors, Admins, Issues }

struct App {
    active_tab: Tab,
    db_active_tab: DbTab,
    server_logs: Vec<String>,
    cpu_activity: Vec<u64>,
    ram_activity: Vec<u64>,
    net_activity: Vec<u64>,
    db_table_rows: Vec<Vec<String>>,
    table_state: TableState,
}

impl App {
    fn new() -> Self {
        Self {
            active_tab: Tab::Dashboard,
            db_active_tab: DbTab::Books,
            server_logs: vec!["[SYSTEM] Инициализация...".into()],
            cpu_activity: vec![0; 50],
            ram_activity: vec![0; 50],
            net_activity: vec![0; 50],
            db_table_rows: vec![],
            table_state: TableState::default(),
        }
    }

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
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (log_tx, mut log_rx) = mpsc::channel::<String>(100);
    let (data_tx, mut data_rx) = mpsc::channel::<Vec<Vec<String>>>(1);

    let db_url = env::var("DB_URL")?;
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;

    // Репозитории
    let book_repo = Arc::new(SQLBookRepository { pool: pool.clone() });
    let author_repo = Arc::new(SQLAuthorRepository { pool: pool.clone() });
    let admin_repo = Arc::new(SQLAdminRepository { pool: pool.clone() });
    let issue_repo = Arc::new(SQLBookIssuesRepository { pool: pool.clone() });

    // Сервисы
    let book_service = Arc::new(BookService::new(book_repo.clone()));
    let author_service = Arc::new(AuthorService::new(author_repo.clone()));
    let admin_service = Arc::new(AdminService::new(admin_repo.clone()));
    let issue_service = Arc::new(BookIssueService::new(issue_repo.clone(), book_repo.clone(), author_repo.clone()));

    // Основной роутер с CORS
    let app_router = Router::new()
        .route("/api/health", get(|| async { StatusCode::OK }))
        .merge(book_routes(book_service.clone()))
        .merge(author_routes(author_service.clone()))
        .merge(book_issues_routes(issue_service.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(Any)
        );

    // Запуск HTTP сервера
    let server_log = log_tx.clone();
    tokio::spawn(async move {
        let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let _ = server_log.send("[HTTP] Сервер запущен на http://localhost:8080".into()).await;
        axum::serve(listener, app_router.into_make_service())
            .await
            .unwrap();
    });

    // TUI
    let mut app = App::new();
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(5)])
                .split(size);

            // Верхние вкладки
            let titles = vec![" Dashboard ", " Database Explorer "];
            let tabs = Tabs::new(titles)
                .select(if app.active_tab == Tab::Dashboard { 0 } else { 1 })
                .block(Block::default().borders(Borders::ALL).title("BookCoop Control"))
                .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            f.render_widget(tabs, chunks[0]);

            match app.active_tab {
                Tab::Dashboard => {
                    let dash_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
                        .split(chunks[1]);

                    let cpu_spark = Sparkline::default()
                        .block(Block::default().title("CPU").borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)))
                        .data(&app.cpu_activity)
                        .style(Style::default().fg(Color::LightGreen))
                        .max(10);
                    f.render_widget(cpu_spark, dash_chunks[0]);

                    let ram_spark = Sparkline::default()
                        .block(Block::default().title("RAM").borders(Borders::ALL).border_style(Style::default().fg(Color::Magenta)))
                        .data(&app.ram_activity)
                        .style(Style::default().fg(Color::LightBlue))
                        .max(16);
                    f.render_widget(ram_spark, dash_chunks[1]);

                    let net_gauge = Gauge::default()
                        .block(Block::default().title("Network").borders(Borders::ALL))
                        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray).add_modifier(Modifier::BOLD))
                        .ratio(app.net_activity.last().unwrap_or(&0).clone() as f64 / 100.0)
                        .label(format!("{} kb/s", app.net_activity.last().unwrap_or(&0)));
                    f.render_widget(net_gauge, dash_chunks[2]);

                    // Логи
                    let log_items: Vec<ListItem> = app.server_logs.iter().rev().take(20).map(|s| {
                        let style = if s.contains("ERROR") {
                            Style::default().fg(Color::Red)
                        } else if s.contains("WARN") {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(s.as_str()).style(style)
                    }).collect();
                    let logs = List::new(log_items).block(Block::default().title("Server Logs").borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
                    f.render_widget(logs, chunks[2]);
                }

                Tab::Database => {
                    let db_titles = vec!["Books", "Authors", "Admins", "Issues"];
                    let db_tab_index = match app.db_active_tab {
                        DbTab::Books => 0,
                        DbTab::Authors => 1,
                        DbTab::Admins => 2,
                        DbTab::Issues => 3,
                    };
                    let db_tabs = Tabs::new(db_titles)
                        .select(db_tab_index)
                        .block(Block::default().borders(Borders::ALL).title("Database Tables"))
                        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                    f.render_widget(db_tabs, chunks[0]);

                    let rows = app.db_table_rows.iter().map(|r| {
                        Row::new(r.iter().map(|c| Cell::from(c.as_str()).style(Style::default().fg(Color::Cyan))))
                    });
                    let table = Table::new(rows, vec![
                        Constraint::Length(5),
                        Constraint::Length(20),
                        Constraint::Length(20),
                        Constraint::Length(15)
                    ])
                        .header(Row::new(vec!["ID","Название","Инфо","Дополнительно"]).style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)))
                        .block(Block::default().borders(Borders::ALL).title("Data"))
                        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
                        .highlight_symbol(">> ");
                    f.render_stateful_widget(table, chunks[1], &mut app.table_state);
                }
            }
        })?;

        // Логика TUI
        while let Ok(msg) = log_rx.try_recv() { app.server_logs.push(msg); }
        if let Ok(data) = data_rx.try_recv() { app.db_table_rows = data; }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('1') => app.active_tab = Tab::Dashboard,
                    KeyCode::Char('2') => app.active_tab = Tab::Database,
                    KeyCode::Down => app.next_row(),
                    KeyCode::Up => app.previous_row(),
                    KeyCode::Left => app.db_active_tab = match app.db_active_tab {
                        DbTab::Books => DbTab::Issues,
                        DbTab::Authors => DbTab::Books,
                        DbTab::Admins => DbTab::Authors,
                        DbTab::Issues => DbTab::Admins,
                    },
                    KeyCode::Right => app.db_active_tab = match app.db_active_tab {
                        DbTab::Books => DbTab::Authors,
                        DbTab::Authors => DbTab::Admins,
                        DbTab::Admins => DbTab::Issues,
                        DbTab::Issues => DbTab::Books,
                    },
                    _ => {}
                }
            }
        }

        // Обновление значений
        if last_tick.elapsed() >= tick_rate {
            app.cpu_activity.push(rand::random::<u64>() % 10);
            app.cpu_activity.remove(0);
            app.ram_activity.push(rand::random::<u64>() % 16);
            app.ram_activity.remove(0);
            app.net_activity.push(rand::random::<u64>() % 100);
            app.net_activity.remove(0);
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
