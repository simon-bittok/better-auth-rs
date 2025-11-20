use betterauth::{App, Result};

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = App::run().await {
        eprintln!("Error {e}");
    }
    Ok(())
}
