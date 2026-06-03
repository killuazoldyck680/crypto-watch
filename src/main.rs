use serde::{Deserialize, Serialize};




#[derive(Serialize,Deserialize,Debug)]
struct Config {
    default_currency : String


}
#[derive(Deserialize,Debug)]
struct BitcoinPrice {
    price: f64
}
#[derive(Deserialize,Debug)]
struct CoinResponse {
    bitcoin: BitcoinPrice
}