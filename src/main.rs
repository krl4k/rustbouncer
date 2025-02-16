use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, error, Level};
use std::sync::Arc;

mod config;
mod pool;

use config::Config;
use pool::ConnectionPool;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let config = Config::from_env();
    
    check_postgres_connection(&config.pg_host, config.pg_port).await?;
    
    // Initialize the connection pool
    let pool = ConnectionPool::new(
        config.max_connections,
        config.pg_host.clone(),
        config.pg_port,
    );
    
    let listener = TcpListener::bind(format!("{}:{}", 
        config.listen_addr, config.listen_port)).await?;
    
    info!("Listening on {}:{}", config.listen_addr, config.listen_port);

    loop {
        match listener.accept().await {
            Ok((client_stream, addr)) => {
                info!("New connection from: {}", addr);
                
                let pool = Arc::clone(&pool);
                tokio::spawn(handle_connection(client_stream, pool));
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn check_postgres_connection(pg_host: &str, pg_port: u16) -> Result<()> {
    info!("Checking PostgreSQL connection...");
    match TcpStream::connect(format!("{}:{}", pg_host, pg_port)).await {
        Ok(_) => {
            info!("Successfully connected to PostgreSQL");
            Ok(())
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL: {}", e);
            Err(anyhow::anyhow!("PostgreSQL connection check failed: {}", e))
        }
    }
}

async fn handle_connection(mut client_stream: TcpStream, pool: Arc<ConnectionPool>) -> Result<()> {
    // Get a connection from the pool
    let mut pg_conn = pool.get_connection().await?;
    
    // Get mutable reference to the underlying connection
    let pg_stream = pg_conn.get_mut().unwrap();

    match tokio::io::copy_bidirectional(&mut client_stream, pg_stream).await {
        Ok((from_client, from_pg)) => {
            info!("Connection closed. Bytes from client: {}, from pg: {}", from_client, from_pg);
            Ok(())
        }
        Err(e) => {
            error!("Error in bidirectional copy: {}", e);
            Err(anyhow::anyhow!("Bidirectional copy failed: {}", e))
        }
    }
}
