# Phase 1: Basic Connection Pooling
## Goals
Create a simple connection pool that can handle multiple clients effectively.

### Core Features

1. **Simple Connection Pool**
   ```mermaid
   graph TD
       subgraph Pool
           C1[Connection 1<br/>Free/Busy]
           C2[Connection 2<br/>Free/Busy]
       end
       
       Client1 --> Pool
       Client2 --> Pool
       Pool --> PostgreSQL
   ```

2. **Basic States Tracking**
   ```mermaid
   stateDiagram-v2
       [*] --> Free
       Free --> Busy: Client gets connection
       Busy --> Free: Client finished
   ```
   - Just track if connection is free or busy
   - No transaction tracking yet
   - Simple cleanup on client disconnect

3. **Simple Load Distribution**
   ```mermaid
   sequenceDiagram
       participant C as Client
       participant P as Pool
       participant PG as PostgreSQL
       
       C->>P: Connect
       alt Free connection exists
           P->>C: Give existing connection
       else No free connections
           P->>PG: Create new connection
           P->>C: Give new connection
       end
   ```

### Configuration
- Max pool size
- Server connection details
- Basic timeouts

### Monitoring
- Number of active connections
- Pool size
- Basic error counting
