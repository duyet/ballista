# Ballista Client

A distributed query execution client built on [Apache Arrow Ballista](https://github.com/apache/arrow-ballista), demonstrating how to connect to a Ballista cluster and execute distributed queries against CSV and Parquet files.

## Features

- **Distributed Query Processing**: Leverages Ballista's distributed execution engine
- **Multiple File Formats**: Supports CSV and Parquet data sources
- **DataFusion Integration**: Uses DataFusion's powerful DataFrame API
- **Production-Ready**: Configurable, observable, and thoroughly tested

## Quick Start

### 1. Install Ballista Components

```bash
cargo install --locked ballista-scheduler
cargo install --locked ballista-executor
```

### 2. Start the Cluster

**Start the scheduler** (in terminal 1):
```bash
RUST_LOG=info ballista-scheduler
```

**Start executor(s)** (in separate terminals):
```bash
# Executor 1
RUST_LOG=info ballista-executor --bind-port 50051 -c 4

# Executor 2 (optional, for true distributed processing)
RUST_LOG=info ballista-executor --bind-port 50052 -c 4

# Executor 3 (optional)
RUST_LOG=info ballista-executor --bind-port 50053 -c 4
```

### 3. Run the Client

```bash
# Build and run
cargo run

# Or with custom configuration
BALLISTA_SCHEDULER=df://localhost:50050 cargo run
```

## Configuration

Configure the client using environment variables:

- `BALLISTA_SCHEDULER`: Scheduler address (default: `df://localhost:50050`)
- `RUST_LOG`: Logging level (e.g., `info`, `debug`, `trace`)

## Development

```bash
# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Security audit
cargo audit
```

## Architecture

See [CLAUDE.md](./CLAUDE.md) for detailed architecture documentation, API notes, and development guidelines.

## License

MIT License - see [LICENSE](./LICENSE) for details
