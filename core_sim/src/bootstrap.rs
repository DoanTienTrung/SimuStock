use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;

/// Bootstrap simulation: lấy mẫu ngẫu nhiên từ log-returns lịch sử
///
/// Phương pháp:
/// 1. Chọn ngẫu nhiên 1 log-return từ dữ liệu lịch sử
/// 2. Áp dụng: S_{t+1} = S_t * exp(return_đã_chọn)
/// 3. Lặp lại cho số ngày cần mô phỏng
///
/// Ưu điểm: Giữ nguyên phân phối thực tế của returns (không giả định phân phối chuẩn)
pub fn simulate_one_path_bootstrap(
    initial_price: f64,
    historical_returns: &[f64],
    days: usize,
    seed: u64,
) -> Vec<f64> {
    // Tạo RNG từ seed
    let mut rng = StdRng::seed_from_u64(seed);

    // Vector lưu giá
    let mut prices = vec![initial_price];

    // Mô phỏng từng ngày
    for _ in 0..days {
        let current_price = prices[prices.len() - 1];

        // Chọn ngẫu nhiên 1 index từ mảng historical_returns
        let random_index = rng.gen_range(0..historical_returns.len());

        // Lấy log-return tại index đó
        let sampled_return = historical_returns[random_index];

        // Tính giá tiếp theo: S_new = S_current * exp(return)
        let next_price = current_price * sampled_return.exp();

        prices.push(next_price);
    }

    prices
}

/// Simulate nhiều bootstrap paths song song (parallel)
///
/// Mỗi path có seed khác nhau = base_seed + index
pub fn simulate_multiple_paths_bootstrap(
    initial_price: f64,
    historical_returns: &[f64],
    days: usize,
    num_paths: usize,
    base_seed: u64,
) -> Vec<Vec<f64>> {
    (0..num_paths)
        .into_par_iter()
        .map(|i| {
            // Mỗi path có seed riêng
            let seed = base_seed + i as u64;
            simulate_one_path_bootstrap(initial_price, historical_returns, days, seed)
        })
        .collect()
}