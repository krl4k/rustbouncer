use tokio::net::TcpStream;
use tokio::sync::{Mutex, Semaphore};
use std::collections::VecDeque;
use std::sync::Arc;
use anyhow::Result;

#[derive(Debug)]
pub struct ConnectionPool {
    connections: Mutex<VecDeque<TcpStream>>,
    semaphore: Semaphore,
    pg_host: String,
    pg_port: u16,
}

impl ConnectionPool {
    pub fn new(max_size: usize, pg_host: String, pg_port: u16) -> Arc<Self> {
        Arc::new(Self {
            connections: Mutex::new(VecDeque::with_capacity(max_size)),
            semaphore: Semaphore::new(max_size),
            pg_host,
            pg_port,
        })
    }

    pub async fn get_connection(self: &Arc<Self>) -> Result<PooledConnection> {
        let _permit = self.semaphore.acquire().await?;

        let mut connections = self.connections.lock().await;
        
        // Try to get an existing connection
        if let Some(conn) = connections.pop_front() {
            return Ok(PooledConnection::new(conn, self));
        }

        let stream = TcpStream::connect(format!("{}:{}", self.pg_host, self.pg_port)).await?;
        Ok(PooledConnection::new(stream, self))
    }

    async fn return_connection(&self, conn: TcpStream) {
        let mut connections = self.connections.lock().await;
        connections.push_back(conn);
        self.semaphore.add_permits(1);
    }
}

pub struct PooledConnection {
    connection: Option<TcpStream>,
    pool: Arc<ConnectionPool>,
}

impl PooledConnection {
    fn new(connection: TcpStream, pool: &Arc<ConnectionPool>) -> Self {
        Self {
            connection: Some(connection),
            pool: Arc::clone(pool),
        }
    }

    pub fn get_ref(&self) -> Option<&TcpStream> {
        self.connection.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut TcpStream> {
        self.connection.as_mut()
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            let pool = self.pool.clone();
            tokio::spawn(async move {
                pool.return_connection(conn).await;
            });
        }
    }
} 
