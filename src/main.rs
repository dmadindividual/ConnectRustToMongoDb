use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
    let options = ClientOptions::parse_with_resolver_config(&client_url, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;

    println!("Connected to MongoDB");

    for name in client.list_database_names(None, None).await? {
        println!("Database: {}", name);
    }

    Ok(())
}
 