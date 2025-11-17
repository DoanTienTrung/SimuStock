use crate::{gbm, bootstrap};

#[derive(Debug, Clone)]
pub enum SimulationModel {
    GBM { mu: f64, sigma: f64 },
    Bootstrap { historical_returns: Vec<f64> },
}

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub initial_price: f64,
    pub horizon_days: usize,
    pub num_paths: usize,
    pub dt: f64,
    pub model: SimulationModel,
    pub use_antithetic: bool,
    pub seed: Option<u64>,
}

pub struct SimulationResult {
    pub paths: Vec<Vec<f64>>,
    pub execution_time_ms: u128,
}

pub fn run_simulation(config: SimulationConfig) -> SimulationResult {
    let start = std::time::Instant::now();

    // Lấy seed từ config, nếu không có thì dùng 42 làm default
    let seed = config.seed.unwrap_or(42);

    let paths = match config.model {
        SimulationModel::GBM { mu, sigma } => {
            if config.use_antithetic {
                // GBM với Antithetic Variates
                gbm::simulate_with_antithetic(
                    config.initial_price,
                    mu,
                    sigma,
                    config.horizon_days,
                    config.dt,
                    config.num_paths,
                    seed, // Truyền seed vào
                )
            } else {
                // GBM thường
                gbm::simulate_multiple_paths(
                    config.initial_price,
                    mu,
                    sigma,
                    config.horizon_days,
                    config.dt,
                    config.num_paths,
                    seed, // Truyền seed vào
                )
            }
        }
        SimulationModel::Bootstrap { historical_returns } => {
            // Bootstrap simulation
            bootstrap::simulate_multiple_paths_bootstrap(
                config.initial_price,
                &historical_returns,
                config.horizon_days,
                config.num_paths,
                seed, // Truyền seed vào
            )
        }
    };

    let execution_time_ms = start.elapsed().as_millis();

    SimulationResult {
        paths,
        execution_time_ms,
    }
}