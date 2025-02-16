use dotenv::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listen_addr: String,
    pub listen_port: u16,
    pub pg_host: String,
    pub pg_port: u16,
    pub max_connections: usize,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Load .env file if it exists

        Self {
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string()),
            listen_port: env::var("LISTEN_PORT")
                .unwrap_or_else(|_| "6432".to_string())
                .parse()
                .expect("Invalid LISTEN_PORT"),
            pg_host: env::var("PG_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            pg_port: env::var("PG_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .expect("Invalid PG_PORT"),
            max_connections: env::var("MAX_CONNECTIONS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .expect("Invalid MAX_CONNECTIONS"),
        }
    }
} 
