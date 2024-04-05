use actix_web::{web, App, HttpServer};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

// Your data collection function with a loop and delay
async fn collect_data_loop() -> Result<(), std::io::Error> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut prev_content: String = ctx.get_contents().unwrap();

    loop {
        let curr_content = ctx.get_contents().unwrap();
        let current_time = chrono::Local::now().format("%H:%M:%S");
        if curr_content != prev_content {
            println!("{}: Content changed!", current_time);
            prev_content = curr_content;
            println!("{:?}", prev_content);
        } else {
            println!("{}: Content unchanged.", current_time);
        }

        // Delay for 5 seconds
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Spawn a new thread for data collection (moved to async block)
    tokio::spawn(async {
        if let Err(e) = collect_data_loop().await {
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
