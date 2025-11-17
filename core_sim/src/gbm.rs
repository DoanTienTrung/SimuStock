use rand::distributions::Distribution;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::StandardNormal;
use rayon::prelude::*;


/// Simulate một path GBM với seed cố định
///
/// Tham số:
/// - initial_price: Giá khởi đầu (S_0)
/// - mu: Drift (μ) - tốc độ tăng trưởng trung bình
/// - sigma: Volatility (σ) - độ biến động
/// - days: Số ngày mô phỏng
/// - dt: Bước thời gian (thường = 1.0 cho daily)
/// - seed: Seed cho random number generator (để tái tạo kết quả)
pub fn simulate_one_path(
    initial_price: f64,
    mu: f64,
    sigma: f64,
    days: usize,
    dt: f64,
    seed: u64,
) -> Vec<f64> {
    // Tạo RNG từ seed (cùng seed → cùng kết quả)
    let mut rng = StdRng::seed_from_u64(seed);

    // Vector lưu giá, bắt đầu với giá khởi đầu
    let mut prices = vec![initial_price];

    // Mô phỏng từng ngày
    for _ in 0..days {
        // Lấy giá hiện tại (giá cuối cùng trong vector)
        let current_price = prices[prices.len() - 1];

        // Lấy số random Z từ phân phối chuẩn N(0,1)
        let z: f64 = StandardNormal.sample(&mut rng);

        // Tính drift: (μ - 0.5σ²)Δt
        let drift = (mu - 0.5 * sigma.powi(2)) * dt;

        // Tính phần random: σ√Δt * Z
        let random = sigma * dt.sqrt() * z;

        // Công thức GBM: S_{t+Δt} = S_t * exp(drift + random)
        let next_price = current_price * (drift + random).exp();

        prices.push(next_price);
    }

    prices
}

/// Simulate nhiều paths GBM song song (parallel)
///
/// Mỗi path sẽ có seed khác nhau = base_seed + index
/// Ví dụ: base_seed=42 → path 0 dùng seed 42, path 1 dùng seed 43,...
pub fn simulate_multiple_paths(
    initial_price: f64,
    mu: f64,
    sigma: f64,
    days: usize,
    dt: f64,
    num_paths: usize,
    base_seed: u64,
) -> Vec<Vec<f64>> {
    (0..num_paths)
        .into_par_iter()
        .map(|i| {
            // Mỗi path có seed riêng = base_seed + index
            let seed = base_seed + i as u64;
            simulate_one_path(initial_price, mu, sigma, days, dt, seed)
        })
        .collect()
}

/// Simulate với Antithetic Variates (giảm variance)
///
/// Kỹ thuật: Với mỗi số random Z, tạo 2 paths:
/// - Path 1 dùng +Z
/// - Path 2 dùng -Z (antithetic = đối nghịch)
/// → Giảm variance, kết quả chính xác hơn với cùng số paths
pub fn simulate_with_antithetic(
    initial_price: f64,
    mu: f64,
    sigma: f64,
    days: usize,
    dt: f64,
    num_paths: usize,
    base_seed: u64,
) -> Vec<Vec<f64>> {
    // Tạo num_paths/2 cặp paths (mỗi cặp = 1 path +Z và 1 path -Z)
    let half_paths = num_paths / 2;
    let mut all_paths = Vec::with_capacity(num_paths);

    for pair_index in 0..half_paths {
        // Mỗi cặp paths có seed riêng
        let seed = base_seed + pair_index as u64;
        let mut rng = StdRng::seed_from_u64(seed);

        // 2 paths bắt đầu từ cùng giá
        let mut prices1 = vec![initial_price];
        let mut prices2 = vec![initial_price];

        // Mô phỏng từng ngày
        for _ in 0..days {
            // Lấy 1 số random Z
            let z: f64 = StandardNormal.sample(&mut rng);

            // Tính các thành phần chung
            let drift = (mu - 0.5 * sigma.powi(2)) * dt;
            let random_term = sigma * dt.sqrt();

            // Path 1: dùng +Z
            let current1 = prices1[prices1.len() - 1];
            let next1 = current1 * (drift + random_term * z).exp();
            prices1.push(next1);

            // Path 2: dùng -Z (antithetic)
            let current2 = prices2[prices2.len() - 1];
            let next2 = current2 * (drift + random_term * (-z)).exp();
            prices2.push(next2);
        }

        // Thêm cả 2 paths vào kết quả
        all_paths.push(prices1);
        all_paths.push(prices2);
    }

    all_paths
}