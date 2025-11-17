# 📈 Monte Carlo Stock Price Simulator

Ứng dụng mô phỏng giá cổ phiếu bằng phương pháp Monte Carlo, được xây dựng bằng **Rust** và **Slint UI**.

## 🎯 Mục Đích

Project này implement 2 phương pháp mô phỏng giá cổ phiếu:
1. **GBM (Geometric Brownian Motion)** - Mô hình toán học chuẩn
2. **Historical Bootstrap** - Dựa trên dữ liệu lịch sử thực tế

## 🏗️ Kiến Trúc Project

Project sử dụng Rust Workspace với 3 crates:

```
stock-analyzer/
├── core_sim/          # Logic mô phỏng Monte Carlo
│   ├── gbm.rs         # Geometric Brownian Motion
│   ├── bootstrap.rs   # Historical Bootstrap
│   └── simulation.rs  # Simulation engine
├── data_io/           # Xử lý dữ liệu CSV
│   ├── csv_loader.rs  # Load dữ liệu CSV
│   ├── statistics.rs  # Tính toán thống kê
│   └── stock_price.rs # Struct dữ liệu
├── app_gui/           # Giao diện Slint
│   ├── main.rs        # Entry point
│   ├── app_logic.rs   # Business logic
│   ├── charts.rs      # Vẽ charts
│   └── ui/
│       └── main.slint # Giao diện UI
└── data/              # Dữ liệu CSV
    └── CafeF.HSX.Upto10.11.2025.csv
```

## ✨ Tính Năng

### 📊 Data Input
- ✅ Load CSV chứa dữ liệu giá lịch sử
- ✅ Chọn ticker từ dropdown
- ✅ Hiển thị thông tin: Date range, số records, giá cuối
- ✅ Tính log-returns: `r_t = ln(Close_t / Close_{t-1})`

### 🔢 Parameter Estimation
- ✅ Ước lượng μ (drift) từ dữ liệu
- ✅ Ước lượng σ (volatility) từ dữ liệu

### 🎲 Simulation Models

#### 1. GBM (Geometric Brownian Motion)
Công thức: `S_{t+Δt} = S_t × exp((μ - 0.5σ²)Δt + σ√Δt·Z)`

Tính năng:
- ✅ Parallel execution (rayon)
- ✅ Reproducible với seed
- ✅ Antithetic Variates (giảm variance)

#### 2. Historical Bootstrap
Phương pháp:
- Lấy mẫu ngẫu nhiên từ log-returns lịch sử
- Áp dụng: `S_{t+1} = S_t × exp(return_sampled)`

### 📈 Visualization
- ✅ **Price Paths Chart**: Hiển thị 20 paths mẫu với màu sắc
- ✅ **Histogram**: Phân phối giá cuối kỳ (25 bins)

### 📊 Statistics
- ✅ Mean, Std Dev, Median
- ✅ Percentiles: P5, P25, P75, P95
- ✅ **VaR95** (Value at Risk 95%)
- ✅ Execution time (milliseconds)

### 💾 Export
- ✅ **CSV Export**: Summary stats, paths, final prices
- ✅ **PNG Export**: Charts (1000x600 pixels)

## 🚀 Cài Đặt & Chạy

### Yêu Cầu
- Rust 1.70+ (`rustup`)
- Windows/Linux/macOS

### Build Project

```bash
# Clone repository
git clone <your-repo-url>
cd stock-analyzer

# Build release version
cargo build --release

# Chạy ứng dụng
cargo run --bin gui --release
```

### Chạy Tests

```bash
# Test tất cả
cargo test

# Test crate cụ thể
cargo test -p core_sim
cargo test -p data_io
```

## 📖 Hướng Dẫn Sử Dụng

### Bước 1: Load Dữ Liệu
1. Click **"Load CSV"**
2. Chọn ticker từ dropdown
3. Xem thông tin ticker (date range, records, last price)

### Bước 2: Ước Lượng Parameters
1. Click **"Estimate μ/σ from Data"**
2. Hệ thống tự động tính:
   - μ (drift) = mean của log-returns
   - σ (volatility) = std của log-returns

### Bước 3: Cấu Hình Simulation
Điều chỉnh các tham số:
- **Initial Price**: Giá khởi đầu (mặc định = last price)
- **Horizon (days)**: Số ngày mô phỏng (ví dụ: 30)
- **Number of Paths**: Số paths (ví dụ: 1000)
- **dt**: Bước thời gian (thường = 1.0 cho daily)
- **μ (mu)**: Drift (từ Estimate)
- **σ (sigma)**: Volatility (từ Estimate)
- **Random Seed**: Seed cho reproducibility (ví dụ: 42)
- **Antithetic Variates**: Bật/tắt variance reduction
- **Model Type**: Chọn GBM hoặc Bootstrap

### Bước 4: Run Simulation
1. Click **"Run Simulation"**
2. Đợi kết quả (thời gian hiển thị ở dưới)
3. Xem:
   - Price Paths Chart (20 paths mẫu)
   - Histogram (phân phối final prices)
   - Summary Statistics

### Bước 5: Export Results
- **Export to CSV**: Lưu summary, paths, final prices
- **Export Charts**: Lưu charts dạng PNG

## 🧮 Công Thức Toán Học

### Log-Returns
```
r_t = ln(S_t / S_{t-1})
```

### Drift & Volatility
```
μ = mean(r_t)
σ = std(r_t) = √(Σ(r_t - μ)² / (n-1))
```

### GBM Formula
```
S_{t+Δt} = S_t × exp((μ - 0.5σ²)Δt + σ√Δt·Z)

Trong đó:
- S_t: Giá tại thời điểm t
- μ: Drift (tốc độ tăng trưởng)
- σ: Volatility (độ biến động)
- Δt: Bước thời gian
- Z ~ N(0,1): Số ngẫu nhiên từ phân phối chuẩn
```

### Antithetic Variates
Với mỗi số ngẫu nhiên Z, tạo 2 paths:
- Path 1: Dùng +Z
- Path 2: Dùng -Z

→ Giảm variance, kết quả chính xác hơn

### VaR95 (Value at Risk)
```
VaR95 = S_0 - P5(S_T)

Trong đó:
- S_0: Giá khởi đầu
- P5(S_T): Percentile thứ 5 của giá cuối kỳ
```

Ý nghĩa: Có 95% khả năng loss không vượt quá VaR95

## 📁 Format Dữ Liệu CSV

File CSV cần có format:

```csv
<Ticker>,<DTYYYYMMDD>,<Open>,<High>,<Low>,<Close>,<Volume>
AAA,20240101,15.5,16.0,15.3,15.8,1000000
AAA,20240102,15.8,16.2,15.7,16.0,1200000
...
```

**Columns:**
- `<Ticker>`: Mã cổ phiếu (ví dụ: AAA, ACB)
- `<DTYYYYMMDD>`: Ngày (format: YYYYMMDD)
- `<Open>`: Giá mở cửa
- `<High>`: Giá cao nhất
- `<Low>`: Giá thấp nhất
- `<Close>`: Giá đóng cửa (dùng để tính log-returns)
- `<Volume>`: Khối lượng giao dịch

## 🛠️ Dependencies

### Core Dependencies
```toml
rand = "0.8"           # Random number generation
rand_distr = "0.4"     # Standard Normal distribution
rayon = "1.7"          # Parallel iteration
anyhow = "1.0"         # Error handling
```

### Data Processing
```toml
csv = "1.2"            # CSV parsing
serde = "1.0"          # Serialization
chrono = "0.4"         # Date/time handling
```

### GUI
```toml
slint = "1.8"          # UI framework
plotters = "0.3"       # Chart drawing
image = "0.24"         # Image processing
```

## 🧪 Testing

Project có unit tests cho:
- ✅ GBM reproducibility (same seed → same results)
- ✅ Simulation config validation
- ✅ Path count & length verification

## 📊 Performance

### Benchmark (1000 paths, 30 days):
- **GBM**: ~50ms (parallel với rayon)
- **Bootstrap**: ~40ms
- **Chart rendering**: ~200ms

### Parallel Execution
Code sử dụng `rayon` để chạy song song:
```rust
(0..num_paths)
    .into_par_iter()  // Parallel iterator
    .map(|i| simulate_one_path(...))
    .collect()
```

## 🔒 Reproducibility

**Random seed đảm bảo kết quả lặp lại được:**

```rust
// Cùng parameters + cùng seed = cùng kết quả
let config = SimulationConfig {
    seed: Some(42),  // Cố định seed
    ...
};

let result1 = run_simulation(config.clone());
let result2 = run_simulation(config.clone());

assert_eq!(result1.paths, result2.paths);  // ✅ Pass
```

## 📚 Tài Liệu Tham Khảo

### Monte Carlo Methods
- Hull, J. (2018). *Options, Futures, and Other Derivatives*
- Glasserman, P. (2003). *Monte Carlo Methods in Financial Engineering*

### Geometric Brownian Motion
- Black-Scholes model
- Itô's Lemma
- Log-normal distribution

### Variance Reduction
- Antithetic Variates technique
- Control Variates
- Importance Sampling

## 🤝 Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📝 License

This project is licensed under the MIT License.

## 👨‍💻 Author

Created for Week 2 Assignment - Monte Carlo Stock Price Simulator

## 🐛 Known Issues

- Ticker selection auto-update đã được fix
- Random seed reproducibility đã được implement
- Charts layout đã được tối ưu

## 🚧 Future Enhancements

Potential improvements:
- [ ] File picker dialog cho CSV
- [ ] Real-time data integration
- [ ] More variance reduction techniques
- [ ] GPU acceleration
- [ ] Web version (WASM)
- [ ] Multiple ticker comparison
- [ ] Greek calculations (Delta, Gamma, Vega)

## 📞 Support

For questions or issues:
- Open an issue on GitHub
- Contact: [trungtiendoan22@gmail.com]

---

**Happy Simulating! 📈🎲**
