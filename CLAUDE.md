# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ballista is a production-ready distributed query execution client built on top of Apache Arrow Ballista and DataFusion. This repository demonstrates best practices for building distributed query applications with comprehensive configuration, observability, testing, and error handling.

## Architecture

**Core Components:**
- **CLI Interface**: Built with `clap` for flexible command-line arguments and environment variable support
- **Configuration**: Environment-based configuration for scheduler address and file paths
- **SessionContext**: The main client interface (from DataFusion) extended by Ballista to connect to remote schedulers
- **Query Processing**: Uses DataFusion's DataFrame API for query construction
- **Data Sources**: Supports CSV and Parquet file formats via DataFusion readers
- **Extension Trait**: Ballista provides `SessionContextExt` trait that adds `remote()` method to `SessionContext`
- **Observability**: Structured logging with `tracing` for comprehensive visibility
- **Error Handling**: Rich error context using `anyhow::Context` for clear error messages

**Execution Flow:**
1. Parse CLI arguments and environment variables
2. Initialize structured logging with `tracing-subscriber`
3. Client establishes connection to Ballista scheduler via `SessionContextExt::remote()`
4. Validate file existence before processing
5. Constructs queries using DataFusion's DataFrame API (`read_csv()`, `read_parquet()`, etc.)
6. Queries are sent to scheduler for distributed execution
7. Results are displayed via `.show()` with comprehensive error context

**Important API Notes:**
- **Version Alignment Critical**: Ballista 49.0 requires DataFusion 49.0 (must stay in sync)
- Connection requires fully qualified trait syntax: `<SessionContext as ballista::prelude::SessionContextExt>::remote("df://localhost:50050")`
- The `BallistaContext` and `BallistaConfig` types were deprecated in favor of DataFusion's native `SessionContext`
- Renovate is configured to group ballista+datafusion updates together to prevent version mismatches

## Development Commands

### Building and Running

```bash
# Build the project
cargo build

# Run the client with default settings (requires running Ballista cluster)
cargo run

# Run with custom scheduler address
cargo run -- --scheduler df://remote-host:50050

# Run with custom files
cargo run -- --csv-file /path/to/data.csv --parquet-file /path/to/data.parquet

# Skip CSV or Parquet processing
cargo run -- --skip-csv
cargo run -- --skip-parquet

# Run with environment variables
BALLISTA_SCHEDULER=df://localhost:50050 CSV_FILE=testdata/test.csv cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Run with trace-level logging for maximum visibility
RUST_LOG=trace cargo run

# Build in release mode
cargo build --release

# Show CLI help
cargo run -- --help
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_args_parsing

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture
```

### Code Quality

```bash
# Check code without building
cargo check

# Run clippy linter (enforced in CI)
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Check formatting without modifying files (enforced in CI)
cargo fmt -- --check

# Security audit
cargo audit
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
RUST_LOG=info ballista-executor --bind-port 50053 -c 4
```

**Note:** The scheduler runs on `localhost:50050` by default, which can be customized via CLI or environment variables.

## Test Data

The `testdata/` directory contains sample files used by the client:
- `test.csv` - CSV format test data (100 rows, columns: c1, c2, c3, ...)
- `test.parquet` - Parquet format test data (columns: id, bool_col, timestamp_col, ...)
- `alltypes_plain.parquet` - Additional Parquet test file

When modifying queries in `main.rs`, ensure the selected columns exist in these test files. The client validates file existence before processing.

## Configuration

### Environment Variables

- `BALLISTA_SCHEDULER`: Scheduler address (default: `df://localhost:50050`)
- `CSV_FILE`: Path to CSV file (default: `testdata/test.csv`)
- `PARQUET_FILE`: Path to Parquet file (default: `testdata/test.parquet`)
- `RUST_LOG`: Logging level (`trace`, `debug`, `info`, `warn`, `error`)

### CLI Arguments

All environment variables can be overridden via command-line arguments:

```bash
cargo run -- --help
```

Example output:
```
Options:
  -s, --scheduler <SCHEDULER>          Ballista scheduler address [env: BALLISTA_SCHEDULER=] [default: df://localhost:50050]
      --csv-file <CSV_FILE>            CSV file path to query [env: CSV_FILE=] [default: testdata/test.csv]
      --parquet-file <PARQUET_FILE>    Parquet file path to query [env: PARQUET_FILE=] [default: testdata/test.parquet]
      --skip-csv                       Skip CSV processing
      --skip-parquet                   Skip Parquet processing
  -h, --help                           Print help
  -V, --version                        Print version
```

## Key Dependencies

- **ballista** (v49.0): Distributed query execution with SessionContextExt trait
- **datafusion** (v49.0): SQL query engine and DataFrame API (MUST match ballista version)
- **tokio** (v1.47): Async runtime (full features enabled)
- **anyhow** (v1.0.100): Error handling with context and backtraces
- **clap** (v4.5): CLI argument parsing with derive macros and environment variable support
- **tracing** (v0.1): Structured logging and instrumentation
- **tracing-subscriber** (v0.3): Log collection with environment-based filtering

## CI/CD Pipeline

The project uses GitHub Actions for continuous integration:

### Automated Checks
- **Test Suite**: Runs on stable and beta Rust toolchains
- **Formatting**: Enforces `rustfmt` standards
- **Linting**: Clippy with warnings as errors
- **Security Audit**: `cargo-audit` checks for vulnerabilities
- **Build**: Release builds with artifact uploads
- **Coverage**: Code coverage with `tarpaulin` and Codecov integration

### Dependency Management
- **Renovate**: Configured to group ballista+datafusion updates atomically
- **Automerge**: Disabled for critical dependencies, enabled for minor/patch updates of stable deps
- **Version Alignment**: Prevents ballista/datafusion version mismatches

## Important Notes

- The client is async and requires the Tokio runtime (`#[tokio::main]`)
- Error handling uses `anyhow::Result` with rich context via `.with_context()`
- **Version Alignment Critical**: Ballista 49.0 and DataFusion 49.0 must stay in sync
- Connection failures include helpful error messages indicating scheduler status
- File paths are validated before query execution
- Use fully qualified trait syntax for SessionContextExt::remote() to avoid ambiguity
- Structured logging provides visibility into connection, query execution, and errors
- All functions are instrumented with `#[instrument]` for distributed tracing

## Testing Strategy

### Unit Tests
- CLI argument parsing validation
- Configuration defaults and overrides
- File path handling

### Integration Tests (Future)
- Docker Compose-based cluster testing
- End-to-end query execution
- Error scenarios (missing scheduler, invalid files)

### Test Coverage Goals
- Maintain >70% code coverage
- Cover all error paths
- Test CLI interface thoroughly
