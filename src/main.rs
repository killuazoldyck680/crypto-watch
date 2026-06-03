use serde::{Deserialize, Serialize};
use clap:: {Parser,Subcommand};




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