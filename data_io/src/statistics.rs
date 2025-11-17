pub fn calculate_log_returns(closes: &[f64]) -> Vec<f64> {
    let mut returns = Vec::new();

    for i in 1..closes.len() {
        // lấy giá trị hiện tại và giá trị trước đó
        let current_price = closes[i];
        let previous_price = closes[i - 1];
        // tính ratio = giá hiện tại - giá trước
        // let ratio = current_price / previous_price;
        // tính log-returns = ln(ratio)
        let log_return = (current_price / previous_price).ln();
        returns.push(log_return);
    }
    returns
}

// tinh tong trung binh
pub fn calculate_mean(returns: &[f64]) -> f64 {
    //let mut sum = 0.0;
    let sum: f64 = returns.iter().sum();
    // for r in returns {
    //     sum = sum + r;
    // }
    sum / returns.len() as f64
}

// tính độ lệch chuẩn σ = √(Σ(xi - μ)² / (n-1))
pub fn calculate_stdev(returns: &[f64], mean: f64) -> f64 {
    let mut sum_squared_diff = 0.0;
    for r in returns {
        sum_squared_diff = sum_squared_diff + (r - mean).powf(2.0);
    }
    let n = returns.len();
    //let variance = sum_squared_diff/(n as f64 - 1.0);
    let stdev = (sum_squared_diff/(n as f64 - 1.0)).sqrt();
    
    stdev
}

pub fn find_min(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut min = values[0];
    for &value in values {
        if value < min {
            min = value;
        }
    }
    min
}

pub fn find_max(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut max = values[0];
    for &value in values {
        if value > max {
            max = value;
        }
    }
    max
}

pub fn calculate_percentile(values: &[f64], p: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (p * (sorted.len() - 1) as f64) as usize;

    sorted[index]
}

pub fn calculate_var(final_prices: &[f64], initial_price: f64, confidence: f64) -> f64 {
    let percentile = calculate_percentile(final_prices, 1.0 - confidence);
    initial_price - percentile
}

#[derive(Debug, Clone)]
pub struct SummaryStats {
    pub mean: f64,
    pub std_dev: f64,
    pub median: f64,
    pub p5: f64,
    pub p25: f64,
    pub p75: f64,
    pub p95: f64,
    pub min: f64,
    pub max: f64,
}

pub fn calculate_summary_stats(values: &[f64]) -> SummaryStats {
    let mean = calculate_mean(values);
    let std_dev = calculate_stdev(values, mean);
    
    SummaryStats {
        mean,
        std_dev,
        median: calculate_percentile(values, 0.5),
        p5: calculate_percentile(values, 0.05),
        p25: calculate_percentile(values, 0.25),
        p75: calculate_percentile(values, 0.75),
        p95: calculate_percentile(values, 0.95),
        min: find_min(values),
        max: find_max(values),
    }
}