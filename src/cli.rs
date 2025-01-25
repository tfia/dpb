use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,
    /// Clear the database before starting
    #[arg(short, long)]
    pub flush_data: bool,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bind_address: String,
    pub bind_port: u16,
}