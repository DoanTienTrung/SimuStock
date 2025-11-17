use crate::{MainWindow, StockData};
use data_io::*;
use core_sim::*;
use slint::{SharedString, ModelRc, VecModel, Image, SharedPixelBuffer};

pub fn load_csv_file(ui: &MainWindow) {
    
    let csv_path = "data/CafeF.HSX.Upto10.11.2025.csv";

    // Load danh sách ticker từ CSV
    let tickers_result = load_available_tickers(csv_path);

    if let Ok(tickers) = tickers_result {
        //  Chuyển dữ liệu cho UI
        let mut shared_tickers = Vec::new();
        for t in &tickers {
            shared_tickers.push(SharedString::from(t.as_str()));
        }
        let ticker_model = ModelRc::new(VecModel::from(shared_tickers));

        //  Cập nhật UI
        ui.set_available_tickers(ticker_model);
        ui.set_csv_path(csv_path.into());
        ui.set_csv_loaded(true);

        // chọn ticker đầu tiên và load thông tin
        if !tickers.is_empty() {
            let default_ticker = &tickers[0];
            ui.set_selected_ticker(default_ticker.clone().into());
            update_stock_info(ui, default_ticker);  
        }

    } else {
        eprintln!("Error loading CSV: {:?} - app_logic.rs:34", tickers_result.err().unwrap());
    }
}

/// Cập nhật thông tin ticker và hiển thị trên UI
pub fn update_stock_info(ui: &MainWindow, ticker: &str) {
    // Lấy csv_path từ UI (đã được set khi load CSV)
    let csv_path = ui.get_csv_path().to_string();

    match get_stock_info(&csv_path, ticker) {
        Ok((ticker_name, date_range, record_count, last_price)) => {
            // Cập nhật thông tin ticker trên UI
            let stock_data = StockData {
                ticker: ticker_name.clone().into(),
                date_range: date_range.into(),
                record_count: record_count as i32,
                last_price: last_price as f32,
            };
            ui.set_stock_data(stock_data);

            // Tự động cập nhật Initial Price = Last Price
            let mut params = ui.get_sim_params();
            params.initial_price = last_price as f32;
            ui.set_sim_params(params);

            println!("✓ Loaded ticker: {} (Last Price: {:.2}) - app_logic.rs:59", ticker_name, last_price);
        }
        Err(e) => {
            eprintln!("Error getting stock info: {} - app_logic.rs:62", e);
        }
    }
}

pub fn estimate_parameters(ui: &MainWindow) {
    let csv_path = ui.get_csv_path();
    let ticker = ui.get_selected_ticker();
    
    match load_closes_for_ticker(&csv_path.to_string(), &ticker.to_string()) {
        Ok(closes) => {
            let returns = calculate_log_returns(&closes);
            let mu = calculate_mean(&returns);
            let sigma = calculate_stdev(&returns, mu);
            
            let mut params = ui.get_sim_params();
            params.mu = mu as f32;
            params.sigma = sigma as f32;
            ui.set_sim_params(params);
            
            println!("Estimated parameters: μ = {:.6}, σ = {:.6} - app_logic.rs:82", mu, sigma);
        }
        Err(e) => {
            eprintln!("Error estimating parameters: {} - app_logic.rs:85", e);
        }
    }
}

pub fn run_simulation(ui: &MainWindow) {
    ui.set_simulation_running(true);
    
    let params = ui.get_sim_params();
    let csv_path = ui.get_csv_path();
    let ticker = ui.get_selected_ticker();
    
    // Load historical data 
    let historical_returns = if params.model_type.as_str() == "Bootstrap" {
        match load_closes_for_ticker(&csv_path.to_string(), &ticker.to_string()) {
            Ok(closes) => Some(calculate_log_returns(&closes)),
            Err(e) => {
                eprintln!("Error loading historical data: {} - app_logic.rs:102", e);
                ui.set_simulation_running(false);
                return;
            }
        }
    } else {
        None
    };
    
    let model = if params.model_type.as_str() == "Bootstrap" {
        SimulationModel::Bootstrap { 
            historical_returns: historical_returns.unwrap() 
        }
    } else {
        SimulationModel::GBM { 
            mu: params.mu as f64, 
            sigma: params.sigma as f64 
        }
    };
    
    let config = SimulationConfig {
        initial_price: params.initial_price as f64,
        horizon_days: params.horizon_days as usize,
        num_paths: params.num_paths as usize,
        dt: params.dt as f64,
        model,
        use_antithetic: params.use_antithetic,
        seed: Some(params.seed as u64),
    };
    
    let result = core_sim::run_simulation(config);
    
    // Calculate final prices
    let final_prices: Vec<f64> = result.paths.iter()
        .map(|path| path[path.len() - 1])
        .collect();
    
    let stats = calculate_summary_stats(&final_prices);
    let var95 = calculate_var(&final_prices, params.initial_price as f64, 0.95);
    
    let summary = format!(
        "Simulation Results:\n\
        Execution Time: {} ms\n\
        Number of Paths: {}\n\
        Horizon: {} days\n\n\
        Final Price Statistics:\n\
        Mean: {:.2}\n\
        Std Dev: {:.2}\n\
        Median: {:.2}\n\
        Min: {:.2}\n\
        Max: {:.2}\n\n\
        Percentiles:\n\
        P5: {:.2}\n\
        P25: {:.2}\n\
        P75: {:.2}\n\
        P95: {:.2}\n\n\
        Risk Metrics:\n\
        VaR95: {:.2} ({:.1}%)",
        result.execution_time_ms,
        result.paths.len(),
        params.horizon_days,
        stats.mean,
        stats.std_dev,
        stats.median,
        stats.min,
        stats.max,
        stats.p5,
        stats.p25,
        stats.p75,
        stats.p95,
        var95,
        (var95 / params.initial_price as f64) * 100.0
    );
    
    ui.set_summary_stats(summary.into());
    
    // Set individual statistics 
    ui.set_stat_mean(format!("{:.2}", stats.mean).into());
    ui.set_stat_std(format!("{:.2}", stats.std_dev).into());
    ui.set_stat_median(format!("{:.2}", stats.median).into());
    ui.set_stat_min(format!("{:.2}", stats.min).into());
    ui.set_stat_max(format!("{:.2}", stats.max).into());
    ui.set_stat_p5(format!("{:.2}", stats.p5).into());
    ui.set_stat_p25(format!("{:.2}", stats.p25).into());
    ui.set_stat_p75(format!("{:.2}", stats.p75).into());
    ui.set_stat_p95(format!("{:.2}", stats.p95).into());
    ui.set_stat_var95(format!("{:.2} ({:.1}%)", var95, (var95 / params.initial_price as f64) * 100.0).into());
    ui.set_execution_time(format!("{} ms", result.execution_time_ms).into());
    
    // Store data for export
    store_simulation_data(result.paths.clone(), final_prices.clone());
    
    // Generate charts
    generate_charts(ui, &result.paths, &final_prices);
    
    ui.set_simulation_running(false);
}

pub fn export_csv(ui: &MainWindow) {
    let summary = ui.get_summary_stats();
    if summary.is_empty() {
        println!("No simulation data to export. Please run simulation first. - app_logic.rs:203");
        return;
    }
    
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    
    // Export summary statistics
    let summary_filename = format!("simulation_summary_{}.csv", timestamp);
    let summary_content = format!(
        "Metric,Value\n\
        Ticker,{}\n\
        Model Type,{}\n\
        Initial Price,{}\n\
        Horizon Days,{}\n\
        Number of Paths,{}\n\
        Mean,{}\n\
        Standard Deviation,{}\n\
        Median,{}\n\
        Minimum,{}\n\
        Maximum,{}\n\
        P5,{}\n\
        P25,{}\n\
        P75,{}\n\
        P95,{}\n\
        VaR95,{}\n\
        Execution Time,{}\n",
        ui.get_selected_ticker(),
        ui.get_sim_params().model_type,
        ui.get_sim_params().initial_price,
        ui.get_sim_params().horizon_days,
        ui.get_sim_params().num_paths,
        ui.get_stat_mean(),
        ui.get_stat_std(),
        ui.get_stat_median(),
        ui.get_stat_min(),
        ui.get_stat_max(),
        ui.get_stat_p5(),
        ui.get_stat_p25(),
        ui.get_stat_p75(),
        ui.get_stat_p95(),
        ui.get_stat_var95(),
        ui.get_execution_time()
    );
    
    match std::fs::write(&summary_filename, summary_content) {
        Ok(_) => {
            println!("✅ Summary exported to: {} - app_logic.rs:249", summary_filename);
        }
        Err(e) => {
            println!("❌ Error exporting summary CSV: {} - app_logic.rs:252", e);
            return;
        }
    }
    
    // Export detailed simulation paths if available
    unsafe {
        if let Some((paths, final_prices)) = &LAST_SIMULATION_DATA {
            let paths_filename = format!("simulation_paths_{}.csv", timestamp);
            if let Err(e) = export_simulation_paths(paths, &paths_filename) {
                println!("❌ Error exporting paths CSV: {} - app_logic.rs:262", e);
            } else {
                println!("✅ Simulation paths exported to: {} - app_logic.rs:264", paths_filename);
            }
            
            let final_prices_filename = format!("final_prices_{}.csv", timestamp);
            if let Err(e) = export_final_prices(final_prices, &final_prices_filename) {
                println!("❌ Error exporting final prices CSV: {} - app_logic.rs:269", e);
            } else {
                println!("✅ Final prices exported to: {} - app_logic.rs:271", final_prices_filename);
            }
        }
    }
}

fn export_simulation_paths(paths: &[Vec<f64>], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    
    let mut file = std::fs::File::create(filename)?;
    
    // Write header
    write!(file, "Path")?;
    if !paths.is_empty() {
        for day in 0..paths[0].len() {
            write!(file, ",Day_{}", day)?;
        }
    }
    writeln!(file)?;
    
    // Write data
    let max_paths = paths.len().min(100);
    for (i, path) in paths.iter().take(max_paths).enumerate() {
        write!(file, "Path_{}", i + 1)?;
        for &price in path {
            write!(file, ",{:.4}", price)?;
        }
        writeln!(file)?;
    }
    
    Ok(())
}

fn export_final_prices(final_prices: &[f64], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    
    let mut file = std::fs::File::create(filename)?;
    writeln!(file, "Path,Final_Price")?;
    
    for (i, &price) in final_prices.iter().enumerate() {
        writeln!(file, "{},{:.4}", i + 1, price)?;
    }
    
    Ok(())
}

pub fn export_chart(_ui: &MainWindow) {
    unsafe {
        if let Some((paths, final_prices)) = &LAST_SIMULATION_DATA {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let mut exported_files = Vec::new();
            
            // Export price paths chart
            let price_paths_filename = format!("price_paths_{}.png", timestamp);
            if let Err(e) = export_price_paths_chart(paths, &price_paths_filename) {
                println!("Error exporting price paths chart: {} - app_logic.rs:326", e);
            } else {
                exported_files.push(price_paths_filename);
            }
            
            // Export histogram
            let histogram_filename = format!("histogram_{}.png", timestamp);
            if let Err(e) = export_histogram_chart(final_prices, &histogram_filename) {
                println!("Error exporting histogram: {}  321 - app_logic.rs:334", e);
            } else {
                exported_files.push(histogram_filename);
            }
            
            if !exported_files.is_empty() {
                println!("Charts exported to: {} - app_logic.rs:340", exported_files.join(", "));
            }
        } else {
            println!("No simulation data to export. Please run simulation first. - app_logic.rs:343");
        }
    }
}

fn export_price_paths_chart(paths: &[Vec<f64>], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;
    
    let root = BitMapBackend::new(filename, (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    if paths.is_empty() || paths[0].is_empty() {
        return Ok(());
    }
    
    // Calculate price range
    let min_price = paths.iter().flatten().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_price = paths.iter().flatten().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let price_range = max_price - min_price;
    let y_min = min_price - price_range * 0.05;
    let y_max = max_price + price_range * 0.05;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Monte Carlo Price Paths", ("Arial", 24))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0f64..(paths[0].len() - 1) as f64,
            y_min..y_max,
        )?;

    chart.configure_mesh()
        .x_desc("Days")
        .y_desc("Price")
        .axis_desc_style(("Arial", 16))
        .draw()?;

    // Draw sample paths 
    let sample_count = paths.len().min(50);
    let step = if paths.len() > sample_count { paths.len() / sample_count } else { 1 };
    
    for (i, path) in paths.iter().step_by(step).take(sample_count).enumerate() {
        let hue = (i as f64 / sample_count as f64) * 360.0;
        let color = HSLColor(hue / 360.0, 0.8, 0.5);
        
        chart.draw_series(LineSeries::new(
            path.iter().enumerate().map(|(x, &y)| (x as f64, y)),
            &color,
        ))?;
    }

    root.present()?;
    Ok(())
}

fn export_histogram_chart(final_prices: &[f64], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;
    
    let root = BitMapBackend::new(filename, (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    if final_prices.is_empty() {
        return Ok(());
    }
    
    // Calculate histogram bins
    let min_price = final_prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_price = final_prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let bin_count = 30;
    let bin_width = (max_price - min_price) / bin_count as f64;
    
    if bin_width <= 0.0 {
        return Ok(());
    }
    
    let mut bins = vec![0; bin_count];
    for &price in final_prices {
        let bin_index = ((price - min_price) / bin_width).floor() as usize;
        let bin_index = bin_index.min(bin_count - 1);
        bins[bin_index] += 1;
    }
    
    let max_count = *bins.iter().max().unwrap_or(&1);
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Terminal Price Distribution", ("Arial", 24))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(min_price..max_price, 0..max_count)?;

    chart.configure_mesh()
        .x_desc("Final Price")
        .y_desc("Frequency")
        .axis_desc_style(("Arial", 16))
        .draw()?;

    chart.draw_series(
        bins.iter().enumerate().map(|(i, &count)| {
            let x0 = min_price + i as f64 * bin_width;
            let x1 = x0 + bin_width;
            Rectangle::new([(x0, 0), (x1, count)], BLUE.mix(0.7).filled())
        })
    )?;

    root.present()?;
    Ok(())
}

fn generate_charts(ui: &MainWindow, paths: &[Vec<f64>], final_prices: &[f64]) {
    // Generate price paths chart
    if let Ok(chart_data) = crate::charts::create_price_paths_chart(paths, 800, 400) {
        // Convert Vec<u8> RGBA to SharedPixelBuffer
        let buffer = SharedPixelBuffer::clone_from_slice(&chart_data, 800, 400);
        let image = Image::from_rgba8(buffer);
        ui.set_chart_image(image);
    }
    
    // Generate histogram
    if let Ok(histogram_data) = crate::charts::create_histogram(final_prices, 800, 400) {
        // Convert Vec<u8> RGBA to SharedPixelBuffer
        let buffer = SharedPixelBuffer::clone_from_slice(&histogram_data, 800, 400);
        let image = Image::from_rgba8(buffer);
        ui.set_histogram_image(image);
    }
}

// Store simulation data for export
static mut LAST_SIMULATION_DATA: Option<(Vec<Vec<f64>>, Vec<f64>)> = None;

pub fn store_simulation_data(paths: Vec<Vec<f64>>, final_prices: Vec<f64>) {
    unsafe {
        LAST_SIMULATION_DATA = Some((paths, final_prices));
    }
}