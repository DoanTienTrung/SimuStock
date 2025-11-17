use plotters::prelude::*;

pub fn create_price_paths_chart(
    paths: &[Vec<f64>],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    if paths.is_empty() || paths[0].is_empty() {
       
        return Ok(vec![255u8; (width * height * 4) as usize]);
    }

    
    let temp_path = "temp_chart.png";

    {
        let root = BitMapBackend::new(temp_path, (width, height)).into_drawing_area();
        root.fill(&WHITE)?; 

        let mut min_price = f64::INFINITY;  
        let mut max_price = f64::NEG_INFINITY; 
        
        for path in paths {
            for &price in path {
                if price < min_price {
                    min_price = price;
                }
                if price > max_price {
                    max_price = price;
                }
            }
        }

        let price_range = max_price - min_price;
        let y_min = min_price - price_range * 0.05;
        let y_max = max_price + price_range * 0.05;
        
        let mut chart = ChartBuilder::on(&root)
            .caption("Monte Carlo Price Paths", ("Arial", 20))  
            .margin(15)  
            .x_label_area_size(35)  // Kích thước vùng label trục X
            .y_label_area_size(45)  // Kích thước vùng label trục Y
            .build_cartesian_2d(
                0f64..(paths[0].len() - 1) as f64,  // Trục X: từ 0 đến số ngày
                y_min..y_max,  // Trục Y: từ y_min đến y_max
            )?;

        chart.configure_mesh()
            .x_desc("Days")  // Label trục X
            .y_desc("Price")  // Label trục Y
            .draw()?;

        // Tính số paths cần vẽ (tối đa 20)
        let sample_count = if paths.len() < 20 {
            paths.len()
        } else {
            20
        };

        // Tính bước nhảy (để lấy 20 paths đều nhau)
        let step = if paths.len() > sample_count {
            paths.len() / sample_count
        } else {
            1
        };

        // Vẽ từng path
        let mut path_index = 0;  
        let mut drawn_count = 0;  

        while drawn_count < sample_count && path_index < paths.len() {
            let path = &paths[path_index];

            // Tính màu cho path (từ đỏ → vàng → xanh → tím)
            let hue_ratio = drawn_count as f64 / sample_count as f64;
            let hue_degrees = hue_ratio * 360.0;
            let color = HSLColor(hue_degrees / 360.0, 0.8, 0.5);

            // Chuyển path thành danh sách điểm (day, price)
            let mut points = Vec::new();
            for day in 0..path.len() {
                let price = path[day];
                points.push((day as f64, price));
            }

            // Vẽ đường line nối các điểm
            chart.draw_series(LineSeries::new(points, &color))?;

            // Cập nhật counters
            drawn_count += 1;
            path_index += step;
        }

        root.present()?;
    }
    
    let img = image::open(temp_path)?;
    let rgba_img = img.to_rgba8();
    let buffer = rgba_img.into_raw();

    // Xóa file tạm
    let _ = std::fs::remove_file(temp_path);

    Ok(buffer)
}


pub fn create_histogram(
    final_prices: &[f64],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if final_prices.is_empty() {
        return Ok(vec![255u8; (width * height * 4) as usize]);
    }

    let temp_path = "temp_histogram.png";

    {
        let root = BitMapBackend::new(temp_path, (width, height)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut min_price = f64::INFINITY;
        let mut max_price = f64::NEG_INFINITY;

        for &price in final_prices {
            if price < min_price {
                min_price = price;
            }
            if price > max_price {
                max_price = price;
            }
        }

        let bin_count = 25;  // Số cột trong histogram
        let bin_width = (max_price - min_price) / bin_count as f64;

        if bin_width <= 0.0 {
            return Ok(vec![255u8; (width * height * 4) as usize]);
        }

        // Tạo mảng đếm cho mỗi bin (khởi tạo = 0)
        let mut bins = vec![0; bin_count];

        // Đếm số lượng prices rơi vào mỗi bin
        for &price in final_prices {
            // Tính index của bin cho price này
            let bin_index = ((price - min_price) / bin_width).floor() as usize;

            let bin_index = if bin_index >= bin_count {
                bin_count - 1
            } else {
                bin_index
            };

            // Tăng counter của bin
            bins[bin_index] += 1;
        }

        // Tìm bin có count lớn nhất (để xác định trục Y)
        let mut max_count = 1;
        for &count in &bins {
            if count > max_count {
                max_count = count;
            }
        }

        let mut chart = ChartBuilder::on(&root)
            .caption("Terminal Price Distribution", ("Arial", 20))
            .margin(15)
            .x_label_area_size(35)
            .y_label_area_size(45)
            .build_cartesian_2d(
                min_price..max_price,  // Trục X: giá
                0..max_count,  // Trục Y: số lượng
            )?;

        chart.configure_mesh()
            .x_desc("Final Price")
            .y_desc("Frequency")
            .draw()?;

        for i in 0..bin_count {
            let count = bins[i];

            // Tính tọa độ của cột
            let x0 = min_price + (i as f64) * bin_width;  // Bên trái
            let x1 = x0 + bin_width;  // Bên phải

            // Tạo hình chữ nhật cho cột
            let rectangle = Rectangle::new(
                [(x0, 0), (x1, count)],  // 2 góc của hình chữ nhật
                BLUE.mix(0.7).filled(),  
            );

            // Vẽ cột lên chart
            chart.draw_series(std::iter::once(rectangle))?;
        }

        root.present()?;
    }

    let img = image::open(temp_path)?;
    let rgba_img = img.to_rgba8();
    let buffer = rgba_img.into_raw();

    let _ = std::fs::remove_file(temp_path);

    Ok(buffer)
}
