mod charts;
mod app_logic;

use slint::*;


slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = MainWindow::new()?;
    
    // Initialize default simulation parameters
    let default_params = SimulationParams {
        initial_price: 100.0,
        horizon_days: 30,
        num_paths: 1000,
        dt: 1.0,
        mu: 0.0,
        sigma: 0.2,
        seed: 42,
        use_antithetic: false,
        model_type: "GBM".into(),
    };
    ui.set_sim_params(default_params);
    
    // Setup callbacks
    let ui_handle = ui.as_weak();
    ui.on_load_csv_clicked(move || {
        let ui = ui_handle.unwrap();
        app_logic::load_csv_file(&ui);
    });

    // Callback khi user chọn ticker từ dropdown
    let ui_handle = ui.as_weak();
    ui.on_ticker_selected(move |ticker_name| {
        let ui = ui_handle.unwrap();
        // Cập nhật selected_ticker
        ui.set_selected_ticker(ticker_name.clone());
        // Tự động load thông tin ticker
        app_logic::update_stock_info(&ui, &ticker_name);
    });

    let ui_handle = ui.as_weak();
    ui.on_estimate_params_clicked(move || {
        let ui = ui_handle.unwrap();
        app_logic::estimate_parameters(&ui);
    });
    
    let ui_handle = ui.as_weak();
    ui.on_run_simulation_clicked(move || {
        let ui = ui_handle.unwrap();
        app_logic::run_simulation(&ui);
    });
    
    let ui_handle = ui.as_weak();
    ui.on_export_csv_clicked(move || {
        let ui = ui_handle.unwrap();
        app_logic::export_csv(&ui);
    });
    
    let ui_handle = ui.as_weak();
    ui.on_export_chart_clicked(move || {
        let ui = ui_handle.unwrap();
        app_logic::export_chart(&ui);
    });
    
    ui.run()?;
    Ok(())
}