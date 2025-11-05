use macroquad::prelude::*;
use image::{ImageBuffer, RgbaImage};

async fn save_screenshot(path: &str) {
    let screenshot: Image = get_screen_data();
    let (width, height) = (screenshot.width, screenshot.height);
    
    // 更安全的索引计算方法
    let bytes = &screenshot.bytes;
    let mut img: RgbaImage = ImageBuffer::new(width as u32, height as u32);
    
    // 使用行优先顺序处理，避免大尺寸溢出
    for (y, row) in img.rows_mut().enumerate() {
        for (x, pixel) in row.enumerate() {
            // 计算当前像素在字节数组中的起始位置
            let start = (y * width as usize + x) * 4;
            if start + 3 < bytes.len() { // 确保不越界
                *pixel = image::Rgba([
                    bytes[start],
                    bytes[start + 1],
                    bytes[start + 2],
                    bytes[start + 3],
                ]);
            }
        }
    }
    
    img.save(path).expect("Failed to save PNG");
}

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        if is_key_pressed(KeyCode::S) {
            save_screenshot("output.png").await;
            println!("Saved as output.png");
        }

        next_frame().await
    }
}

