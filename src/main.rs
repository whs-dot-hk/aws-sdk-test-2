use aws_sdk_imagebuilder as imagebuilder;
use aws_sdk_secretsmanager as secretsmanager;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = aws_config::load_from_env().await;
    let client = imagebuilder::Client::new(&config);

    let client = secretsmanager::Client::new(&config);

    Ok(())
}
