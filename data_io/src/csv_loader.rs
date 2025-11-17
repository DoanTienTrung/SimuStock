use csv::ReaderBuilder;
use std::error::Error;
use crate::stock_price::StockPrice;

pub fn load_closes_for_ticker(
    csv_path: &str,
    ticker: &str
) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    let mut closes = Vec::new();

    for result in reader.deserialize() {
        let record: StockPrice = result?;
        if record.ticker == ticker {
            closes.push(record.close);
        }
    }

    Ok(closes)
}

/// Load all tickers 
pub fn load_available_tickers(csv_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    let mut tickers = std::collections::HashSet::new();

    for result in reader.deserialize() {
        let record: StockPrice = result?;
        tickers.insert(record.ticker);
    }

    let mut ticker_list: Vec<String> = tickers.into_iter().collect();
    ticker_list.sort();
    Ok(ticker_list)
}

/// Get info ticker
pub fn get_stock_info(csv_path: &str, ticker: &str) -> Result<(String, String, usize, f64), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: StockPrice = result?;
        if record.ticker == ticker {
            records.push(record);
        }
    }

    if records.is_empty() {
        return Err(format!("No data found for ticker {}", ticker).into());
    }

    let first_date = &records[0].date;
    let last_date = &records[records.len() - 1].date;
    let record_count = records.len();
    let last_price = records[records.len() - 1].close;

    let date_range = format!("{} to {}", first_date, last_date);

    Ok((ticker.to_string(), date_range, record_count, last_price))
}