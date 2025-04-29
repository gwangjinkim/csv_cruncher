use clap::Parser;
use polars::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(version, about = "A CSV cruncher that laughs at pandas' speed")]
struct Args {
    #[clap(short, long, value_name = "FILE")]
    input: String,
    #[clap(short, long, value_name = "JSON_FILE")]
    output: Option<String>,
}

fn crunch_csv<P: AsRef<Path>>(path: P) -> anyhow::Result<(f64, f64, f64)> {
    let df = LazyCsvReader::new(path) // no ? here
        .finish()? // finish() returns a LazyFrame
        .collect()?; // collect() turns it into a real DataFrame

    let values = df.column("value")?.f64()?; // now works
    let sum: f64 = values.sum().unwrap_or(0.0);
    let count = df.height() as f64;
    let mean = if count > 0.0 { sum / count } else { 0.0 };
    let max = values.max().unwrap_or(0.0);
    Ok((sum, mean, max))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (sum, mean, max) = crunch_csv(&args.input)?;
    let result = serde_json::json!({
        "sum": sum,
        "mean": mean,
        "max": max
    });
    if let Some(output) = args.output {
        std::fs::write(&output, result.to_string())?;
        println!("Results saved to {}", output);
    } else {
        println!("Results: {}", result);
    }
    Ok(())
}
