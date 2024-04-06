use actix_web::{web, App, HttpServer};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use std::sync::Arc;
use tokio::task;

// Your data collection function with a loop and delay
async fn collect_data_loop(
    pool: Arc<Pool<SqliteConnectionManager>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut prev_content: String = ctx.get_contents().unwrap();

    loop {
        let curr_content = ctx.get_contents().unwrap();
        let current_time = chrono::Local::now().format("%H:%M:%S");
        if curr_content != prev_content {
            println!("{}: Content changed!", current_time);
            prev_content = curr_content;
            println!("{:?}", prev_content);
            // Get a connection from the pool
            let conn = pool.get().unwrap();

            // Execute a SQL query
            conn.execute(
                "INSERT INTO clipboard (content) VALUES (?1)",
                params![prev_content],
            )?;
        } else {
            println!("{}: Content unchanged.", current_time);
        }

        // Delay for 5 seconds
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the SQLite connection pool
    let manager = SqliteConnectionManager::file("db.sqlite3");
    let pool = match Pool::new(manager) {
        Ok(pool) => Arc::new(pool),
        Err(e) => {
            eprintln!("Failed to create pool: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize the SQLite database
    let conn = pool.get().unwrap();
    // Create the table
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard (
            id INTEGER PRIMARY KEY,
            label TEXT,
            content TEXT NOT NULL,
            collection TEXT NOT NULL DEFAULT 'General',
            pinned BOOLEAN NOT NULL DEFAULT 0,
            hidden BOOLEAN NOT NULL DEFAULT 0
        )",
        params![],
    ) {
        Ok(_) => println!("Table created successfully."),
        Err(e) => {
            eprintln!("Failed to create table: {}", e);
            std::process::exit(1);
        }
    }

    // Spawn a new thread for data collection (moved to async block)
    let pool_clone = Arc::clone(&pool);
    task::spawn(async move {
        if let Err(e) = collect_data_loop(pool_clone).await {
            // Handle any errors from the collection loop
            eprintln!("Error during data collection: {}", e);
        }
    });

    // Start the Actix web server
    HttpServer::new(|| {
        App::new()
            // Add your web routes here
            .route("/", web::get().to(|| async { "Hello from Actix web!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
