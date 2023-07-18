use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tokio-console and tracing.
    let console_layer = console_subscriber::spawn();
    let format_layer = tracing_subscriber::fmt::layer().pretty().with_file(true).with_level(true).with_line_number(true).with_thread_ids(true).with_thread_names(true);
    tracing_subscriber::registry().with(console_layer).with(format_layer).init();
    
    // Do stuff
    println!("Hello, world!");
    tracing::info!("About to sleep.");

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // Done
    Ok(())
}
