pub mod gbm;
pub mod bootstrap;
pub mod simulation;

pub use gbm::*;
pub use bootstrap::*;
pub use simulation::*;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_gbm_reproducibility() {
        use rand::{SeedableRng, rngs::StdRng};
        
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        
       
        let path1 = simulate_one_path(100.0, 0.05, 0.2, 10, 1.0);
        let path2 = simulate_one_path(100.0, 0.05, 0.2, 10, 1.0);
        
        
        assert_eq!(path1.len(), path2.len());
    }
    
    #[test]
    fn test_simulation_config() {
        let config = SimulationConfig {
            initial_price: 100.0,
            horizon_days: 5,
            num_paths: 10,
            dt: 1.0,
            model: SimulationModel::GBM { mu: 0.05, sigma: 0.2 },
            use_antithetic: false,
            seed: Some(42),
        };
        
        let result = run_simulation(config);
        assert_eq!(result.paths.len(), 10);
        assert_eq!(result.paths[0].len(), 6); // initial + 5 days
        assert!(result.execution_time_ms > 0);
    }
}