🚀 CryptoWatch CLI
A high-performance, asynchronous command-line utility built in Rust to track real-time cryptocurrency prices. This tool interfaces directly with the CoinGecko API, features local JSON configuration fallback layers, handles dynamic global parameters, and supports thread-safe append logging for historical financial data analytics.

✨ Core Features Implemented
Asynchronous Runtime Engine: Engineered using tokio to handle non-blocking asynchronous network requests natively.

Robust Argument Parsing & Global Routing: Implemented using clap (Derive API) to manage structured subcommands (price) alongside persistent global flags (-c, --currency).

Dynamic Currency Normalization: Built-in automated string case-handling to guarantee flawless API payload parsing regardless of user input casing (e.g., USD, eur, jpy).

Resilient Multi-Layer Configurations: Automated configuration fallback architecture. If no runtime currency flag is provided, the application evaluates and imports settings from a local crypto_config.json payload using serde.

Production-Grade Error Handling: Zero panics. Replaced unwrap sequences with anyhow::Context for elegant, linear error propagation (? operator) and expressive contextual diagnostics.

Asynchronous Network Pipeline: Managed using reqwest equipped with custom User-Agent spoofing blocks to prevent automated bot mitigation or rate-limit blocks.

Aesthetic Terminal UI Elements: Features dynamic UI spinner states via indicatif to ensure a smooth, professional client experience during active network roundtrips.

Hard-Drive Log Appender Engine: Integrated explicit transactional appending routines via std::fs::OpenOptions to save chronological snapshot logs directly into price_history.txt via a conditional --save flag.

🛠️ Tech Stack & Dependencies
The application leverages standard, industry-standard crates to achieve enterprise-grade reliability and performance:

tokio: The multi-threaded async platform engine for Rust.

clap: Robust, type-safe command-line argument parsing.

reqwest: High-level, async HTTP network client architecture.

serde & serde_json: Generic serialization and deserialization ecosystem.

anyhow: Flexible, conversational error tracking and structural contexts.

indicatif: Sleek terminal status bars and progress spinners.

🚀 Installation & Setup
1. Clone the Repository
git clone https://github.com/killuazoldyck680/crypto-watch.git
cd crypto-watch

2. Configure Local Fallbacks (Optional)
Create a file named crypto_config.json in the root directory to specify your default tracking currency when runtime flags are absent:
{
"default_currency": "usd"
}

3. Build and Compile
cargo build --release

📖 Usage Manual
The application expects arguments structured logically around the global configuration and specific operational subcommands.

Standard Subcommand Execution
To execute the live price engine tracking your default currency configuration:
cargo run -- price

Dynamic Currency Override
To force the backend runtime engine to map market pricing to an explicit foreign fiat index (e.g., Euros or Japanese Yen):
cargo run -- -c eur price
cargo run -- -c jpy price

Analytical History File Appending
To display the live market pricing on-screen while simultaneously creating/appending a structured transaction record inside price_history.txt:
cargo run -- price --save

Shortcut layout format:
cargo run -- -c eur price -s

📄 Example Output
Terminal Session Layout
$ cargo run -- -c eur price --save
🎯 Final selected currency to look up: eur
Price subcommand detected! Ready to fetch data...
🎉 Connection Successful!
💰 Live Bitcoin Price: 51432.00 EUR
💾 Saved price snapshot safely into price_history.txt!

Generated History File State (price_history.txt)
Live Bitcoin Price: 55266.00 USD
Live Bitcoin Price: 51432.00 EUR
Live Bitcoin Price: 8645100.00 JPY

🛡️ Architecture & Error Handling Blueprint
This codebase enforces strict production rules:

No Manual Panics: Deep nesting layouts or unchecked .unwrap() hooks have been refactored entirely.

Contextual Bubbling: Low-level networking or system I/O errors are intercepted and layered with semantic descriptions:

let client = reqwest::Client::builder()
.user_agent("CryptoWatchCLI/1.0")
.build()
.context("Failed to initialize the HTTP network client connection engine")?; 

