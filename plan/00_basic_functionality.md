### Phase 0: Basic TCP Proxy (MVP)
Goal: Create a simple proxy that can accept connections and transfer data.

1. **Project Setup**
   - Basic connection pool
   - Basic configuration (host, port, max_connections)
   - Basic logging

2. **Simple TCP Proxy**
   - Accept TCP connections from clients
   - Create a connection to Postgres
   - Simple data transfer between them without parsing
   - Basic error handling (connection break)

```mermaid
sequenceDiagram
    participant Client
    participant Proxy
    participant Postgres
    
    Client->>Proxy: TCP Connect
    Proxy->>Postgres: TCP Connect
    
    loop Data Transfer
        Client->>Proxy: Data
        Proxy->>Postgres: Forward Data
        Postgres->>Proxy: Response Data
        Proxy->>Client: Forward Response
    end
    
    Note over Proxy: Handle disconnects
```

