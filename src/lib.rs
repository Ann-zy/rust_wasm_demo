use wasm_bindgen::prelude::*;
use image::{self, DynamicImage, imageops::FilterType};
use std::io::Cursor;

#[wasm_bindgen]
pub fn compress_image(data: &[u8], quality: u8) -> Result<Vec<u8>, JsValue> {
    // 解析输入数据为图像
    let image = match image::load_from_memory(data) {
        Ok(img) => img,
        Err(err) => return Err(JsValue::from_str(&format!("Failed to load image: {}", err))),
    };
    // 调整图像大小
    let resized_image = image.resize(80, 80, FilterType::Lanczos3);

    // 编码为 PNG 格式
    let mut cursor = Cursor::new(Vec::new());
    match resized_image.write_to(&mut cursor, image::ImageFormat::Png) {
        Ok(_) => {
            let buffer = cursor.into_inner();
            Ok(buffer)
        },
        Err(err) => Err(JsValue::from_str(&format!("Failed to write image: {}", err))),
    }
}