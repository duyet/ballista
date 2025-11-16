use anyhow::{Context, Result};
use clap::Parser;
use datafusion::prelude::{col, lit, CsvReadOptions, ParquetReadOptions, SessionContext};
use std::path::PathBuf;
use tracing::{info, instrument};

/// Ballista distributed query client - demonstrating distributed query execution
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ballista scheduler address
    #[arg(
        short,
        long,
        env = "BALLISTA_SCHEDULER",
        default_value = "df://localhost:50050"
    )]
    scheduler: String,

    /// CSV file path to query
    #[arg(long, env = "CSV_FILE", default_value = "testdata/test.csv")]
    csv_file: PathBuf,

    /// Parquet file path to query
    #[arg(long, env = "PARQUET_FILE", default_value = "testdata/test.parquet")]
    parquet_file: PathBuf,

    /// Skip CSV processing
    #[arg(long)]
    skip_csv: bool,

    /// Skip Parquet processing
    #[arg(long)]
    skip_parquet: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let args = Args::parse();

    info!("🚀 Starting Ballista distributed query client");
    info!("📡 Connecting to scheduler: {}", args.scheduler);

    // Connect to Ballista scheduler
    let ctx = connect_to_scheduler(&args.scheduler).await?;
    info!("✅ Successfully connected to Ballista scheduler");

    // Process CSV file
    if !args.skip_csv {
        process_csv(&ctx, &args.csv_file).await?;
    }

    // Process Parquet file
    if !args.skip_parquet {
        process_parquet(&ctx, &args.parquet_file).await?;
    }

    info!("🎉 All queries completed successfully");
    Ok(())
}

/// Establishes connection to the Ballista scheduler
#[instrument(skip_all, fields(scheduler = %scheduler_addr))]
async fn connect_to_scheduler(scheduler_addr: &str) -> Result<SessionContext> {
    info!("Establishing connection to Ballista scheduler");

    <SessionContext as ballista::prelude::SessionContextExt>::remote(scheduler_addr)
        .await
        .with_context(|| {
            format!(
                "Failed to connect to Ballista scheduler at '{}'. \
                 Ensure the scheduler is running and accessible.",
                scheduler_addr
            )
        })
}

/// Processes a CSV file with distributed query execution
#[instrument(skip(ctx), fields(file = %csv_file.display()))]
async fn process_csv(ctx: &SessionContext, csv_file: &PathBuf) -> Result<()> {
    info!("📄 Processing CSV file");

    // Validate file exists
    if !csv_file.exists() {
        anyhow::bail!(
            "CSV file not found: '{}'. Please check the file path.",
            csv_file.display()
        );
    }

    // Execute distributed query
    let df = ctx
        .read_csv(csv_file.to_str().unwrap(), CsvReadOptions::new())
        .await
        .with_context(|| format!("Failed to read CSV file: {}", csv_file.display()))?
        .select_columns(&["c1", "c2"])
        .with_context(|| "Failed to select columns 'c1', 'c2'. Verify these columns exist.")?;

    info!("Executing query and displaying results");
    df.show()
        .await
        .context("Failed to execute query or display results")?;

    info!("✅ CSV query completed");
    Ok(())
}

/// Processes a Parquet file with distributed query execution and filtering
#[instrument(skip(ctx), fields(file = %parquet_file.display()))]
async fn process_parquet(ctx: &SessionContext, parquet_file: &PathBuf) -> Result<()> {
    info!("📊 Processing Parquet file");

    // Validate file exists
    if !parquet_file.exists() {
        anyhow::bail!(
            "Parquet file not found: '{}'. Please check the file path.",
            parquet_file.display()
        );
    }

    // Execute distributed query with filter
    let df = ctx
        .read_parquet(
            parquet_file.to_str().unwrap(),
            ParquetReadOptions::default(),
        )
        .await
        .with_context(|| format!("Failed to read Parquet file: {}", parquet_file.display()))?
        .select_columns(&["id", "bool_col", "timestamp_col"])
        .with_context(|| {
            "Failed to select columns 'id', 'bool_col', 'timestamp_col'. Verify schema."
        })?
        .filter(col("id").gt(lit(1)))
        .context("Failed to apply filter: id > 1")?;

    info!("Executing query with filter (id > 1) and displaying results");
    df.show()
        .await
        .context("Failed to execute query or display results")?;

    info!("✅ Parquet query completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        // Test default values
        let args = Args::parse_from(["ballista"]);
        assert_eq!(args.scheduler, "df://localhost:50050");
        assert_eq!(args.csv_file, PathBuf::from("testdata/test.csv"));
        assert_eq!(args.parquet_file, PathBuf::from("testdata/test.parquet"));
        assert!(!args.skip_csv);
        assert!(!args.skip_parquet);
    }

    #[test]
    fn test_args_custom_scheduler() {
        let args = Args::parse_from(["ballista", "--scheduler", "df://custom-host:9999"]);
        assert_eq!(args.scheduler, "df://custom-host:9999");
    }

    #[test]
    fn test_args_skip_flags() {
        let args = Args::parse_from(["ballista", "--skip-csv", "--skip-parquet"]);
        assert!(args.skip_csv);
        assert!(args.skip_parquet);
    }

    #[test]
    fn test_args_custom_files() {
        let args = Args::parse_from([
            "ballista",
            "--csv-file",
            "/custom/path.csv",
            "--parquet-file",
            "/custom/data.parquet",
        ]);
        assert_eq!(args.csv_file, PathBuf::from("/custom/path.csv"));
        assert_eq!(args.parquet_file, PathBuf::from("/custom/data.parquet"));
    }
}
