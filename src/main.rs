use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use clap:: {Parser,Subcommand};
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use anyhow::{Context, Result};




#[derive(Serialize,Deserialize,Debug)]
struct Config {
    default_currency : String


}


#[derive(Parser,Debug)]
#[command(name = "crypto-watch", version = "1.0", about = "Track live crypto prices")]

struct CLI {
    #[arg(short,long)]
    currency: Option<String>,

    #[command(subcommand)]
    subcommand: Option<Commands>,
}

#[derive(Subcommand,Debug)]
enum Commands {
    Price,
}

#[tokio::main]

async fn main() -> Result<()> {
   let cli = CLI::parse(); 

   let mut currency = cli.currency.clone();

   if currency.is_none() {
    println!("No currency flag provided. Checking crypto_config.json...");

    if let Ok(mut file) = File::open("crypto_config.json") {
        let mut contents = String::new();

        if file.read_to_string(& mut contents).is_ok() {
            if let Ok(parsed_config) = serde_json::from_str::<Config>(&contents) {
               println!("Successfully loaded currency from config file!") ;
               currency = Some(parsed_config.default_currency)
            }
        }
    }
   }

   let new_currency = currency.unwrap_or_else(|| "usd".to_string());
   println!("🎯 Final selected currency to look up: {}", new_currency);

   

   match &cli.subcommand {
    Some(Commands::Price) => {
            println!("Price subcommand detected! Ready to fetch data...");
            
            let pb = ProgressBar::new_spinner();
            pb.set_message("Fetching live crypto prices...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // 1. Build a client with a custom User-Agent so CoinGecko knows we aren't a malicious bot
            let client = reqwest::Client::builder()
                .user_agent("CryptoWatchCLI/1.0")
                .build()
                .context("Failed to initialize the HTTP network client connection engine")?;

            let target_currency = new_currency.to_lowercase();

            let url = format!("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies={}",target_currency);


            let response_result = client
                .get(url)
                .send()
                .await;

            match response_result {
                Ok(response) => {
                    pb.finish_and_clear();
                    
                    // 2. Read raw response text directly
                    if let Ok(raw_text) = response.text().await {
                        // 3. Parse dynamically into a generic Serde JSON Value object
                        if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&raw_text) {
                            
                            // 4. Safely extract the floating point price using index mapping
                            if let Some(price) = json_data["bitcoin"][&target_currency].as_f64() {
                                println!("🎉 Connection Successful!");
                                println!("💰 Live Bitcoin Price: {:.2} {}", price, target_currency.to_uppercase());
                            } else {
                                println!("❌ CoinGecko changed its layout. Raw output received:\n{}", raw_text);
                            }
                        } else {
                            println!("❌ Failed to parse response string into JSON object.");
                        }
                    } else {
                        println!("❌ Could not read string buffer from response.");
                    }
                }
                Err(e) => {
                    pb.finish_and_clear();
                    println!("❌ Network Error: {}", e);
                }
            }
        }

        None => {
    println!("Please specify a subcommand, e.g., cargo run -- price");
   }

   }
   

   Ok(())
}