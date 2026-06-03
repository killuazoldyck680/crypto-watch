
use serde::{Deserialize, Serialize};
use clap:: {Parser,Subcommand};
use std::fs::File;
use std::io::Read;




#[derive(Serialize,Deserialize,Debug)]
struct Config {
    default_currency : String


}
#[derive(Deserialize,Debug)]
struct BitcoinPrice {
    usd: f64
}
#[derive(Deserialize,Debug)]
struct CoinResponse {
    bitcoin: BitcoinPrice
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

async fn main() {
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

    }
    None => {
        println!("Please specify a subcommand, e.g., cargo run -- price");
    }
   }


}