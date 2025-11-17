use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StockPrice {
    #[serde(rename = "<Ticker>")]
    pub ticker: String,

    #[serde(rename = "<DTYYYYMMDD>")]
    pub date: String,

    #[serde(rename = "<Open>")]
    pub open: f64,

    #[serde(rename = "<High>")]
    pub high: f64,

    #[serde(rename = "<Low>")]
    pub low: f64,

    #[serde(rename = "<Close>")]
    pub close: f64,

    #[serde(rename = "<Volume>")]
    pub volume: i64,
}