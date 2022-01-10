use datafusion::prelude::*;
use datafusion::arrow::util::pretty::print_batches;
use datafusion::arrow::record_batch::RecordBatch;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
  // register the table
  let mut ctx = ExecutionContext::new();
  ctx.register_csv("example", "/Users/luna/tmp/test.csv", CsvReadOptions::new()).await?;

  // create a plan to run a SQL query
  let df = ctx.sql("SELECT * FROM example LIMIT 100").await?;

  // execute and print results
  df.show().await?;
  Ok(())
}