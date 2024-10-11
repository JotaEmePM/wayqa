use std::error::Error;

mod core;

async fn tokio_main() -> Result<(), Box<dyn Error>> {
    core::app::main()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = tokio_main().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } else {
        Ok(())
    }    
}
