# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ballista is a distributed query execution client built on top of Apache Arrow Ballista and DataFusion. This repository contains a simple client application that connects to a Ballista scheduler to execute distributed queries against CSV and Parquet files.

## Architecture

**Core Components:**
- **SessionContext**: The main client interface (from DataFusion) extended by Ballista to connect to remote schedulers
- **Query Processing**: Uses DataFusion's DataFrame API for query construction
- **Data Sources**: Supports CSV and Parquet file formats via DataFusion readers
- **Extension Trait**: Ballista provides `SessionContextExt` trait that adds `remote()` method to `SessionContext`

**Execution Flow:**
1. Client establishes connection to Ballista scheduler via `SessionContextExt::remote()`
2. Constructs queries using DataFusion's DataFrame API (`read_csv()`, `read_parquet()`, etc.)
3. Queries are sent to scheduler for distributed execution
4. Results are displayed via `.show()`

**Important API Notes:**
- Ballista 48.0 uses DataFusion 48.0 (version alignment is critical)
- Connection requires fully qualified trait syntax: `<SessionContext as ballista::prelude::SessionContextExt>::remote("df://localhost:50050")`
- The `BallistaContext` and `BallistaConfig` types were deprecated in favor of DataFusion's native `SessionContext`

## Development Commands

### Building and Running

```bash
# Build the project
cargo build

# Run the client (requires running Ballista cluster)
cargo run

# Build in release mode
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Code Quality

```bash
# Check code without building
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

## Cluster Setup Requirements

**Before running the client, you must have a Ballista cluster running:**

1. Install Ballista components:
```bash
cargo install --locked ballista-scheduler
cargo install --locked ballista-executor
```

2. Start scheduler (in one terminal):
```bash
RUST_LOG=info ballista-scheduler
```

3. Start executor(s) (in separate terminals):
```bash
RUST_LOG=info ballista-executor --bind-port 50051 -c 4
RUST_LOG=info ballista-executor --bind-port 50052 -c 4
```

**Note:** The scheduler must be running on `localhost:50050` (default) for the client to connect.

## Test Data

The `testdata/` directory contains sample files used by the client:
- `test.csv` - CSV format test data
- `test.parquet` - Parquet format test data
- `alltypes_plain.parquet` - Additional Parquet test file

When modifying queries in `main.rs`, ensure the selected columns exist in these test files.

## Key Dependencies

- **ballista** (v48.0): Distributed query execution with SessionContextExt trait
- **datafusion** (v48.0): SQL query engine and DataFrame API (must match ballista version)
- **tokio** (v1.47): Async runtime (full features enabled)
- **anyhow** (v1.0.100): Error handling

## Important Notes

- The client is async and requires the Tokio runtime (`#[tokio::main]`)
- All queries return `datafusion::common::Result<()>` for error propagation
- Ballista and DataFusion versions must be kept in sync (both at v48.0)
- Connection failures will occur if the scheduler is not running on port 50050
- Use fully qualified trait syntax for SessionContextExt::remote() to avoid ambiguity
