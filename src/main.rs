use ballista::prelude::*;
use datafusion::prelude::{col, lit, CsvReadOptions, ParquetReadOptions};

#[tokio::main]
async fn main() -> Result<()> {
    // create configuration
    let config = BallistaConfig::builder()
        .set("ballista.shuffle.service", "true")
        .build()?;

    // connect to Ballista scheduler
    let ctx = BallistaContext::remote("localhost", 50050, &config).await?;

    let csv_file = "testdata/test.csv";
    process_csv(&ctx, csv_file).await?;

    let parquet_file = "testdata/test.parquet";
    process_parquet(&ctx, parquet_file).await?;

    Ok(())
}

async fn process_csv(ctx: &BallistaContext, csv_file: &str) -> Result<()> {
    // define the query using the DataFrame trait
    let df = ctx
        .read_csv(csv_file, CsvReadOptions::new())
        .await?
        .select_columns(&["c1", "c2"])?;

    df.show().await?;

    Ok(())
}

async fn process_parquet(ctx: &BallistaContext, parquet_file: &str) -> Result<()> {
    // define the query using the DataFrame trait
    let df = ctx
        .read_parquet(parquet_file, ParquetReadOptions::default())
        .await?
        .select_columns(&["id", "bool_col", "timestamp_col"])?
        .filter(col("id").gt(lit(1)))?;

    df.show().await?;

    Ok(())
}

