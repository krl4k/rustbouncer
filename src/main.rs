use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, error, Level};

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let config = Config::from_env();
    
    check_postgres_connection(&config.pg_host, config.pg_port).await?;
    
    let listener = TcpListener::bind(format!("{}:{}", 
        config.listen_addr, config.listen_port)).await?;
    
    info!("Listening on {}:{}", config.listen_addr, config.listen_port);

    loop {
        match listener.accept().await {
            Ok((client_stream, addr)) => {
                info!("New connection from: {}", addr);
                
                tokio::spawn(handle_connection(client_stream, config.pg_host.clone(), config.pg_port));
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

async fn handle_connection(mut client_stream: TcpStream, pg_host: String, pg_port: u16) -> Result<()> {
    // Connect to PostgreSQL
    let mut pg_stream = TcpStream::connect(format!("{}:{}", pg_host, pg_port))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to PostgreSQL: {}", e))?;

    match tokio::io::copy_bidirectional(&mut client_stream, &mut pg_stream).await {
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
