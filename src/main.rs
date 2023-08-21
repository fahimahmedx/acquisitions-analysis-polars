use polars::prelude::*;

fn load(path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(path)?.has_header(true).finish()
}

fn main() {
    let acquisitions = load("data/Acquisitions.csv");
    println!("{:?}", acquisitions);
}
