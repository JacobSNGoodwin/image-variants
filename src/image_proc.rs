use std::path::Path;

use image::GenericImageView;

#[derive(Debug)]
pub struct LQIPData {
    image: String,
    width: u32,
    height: u32,
}

pub fn create_lqip(input: &Path) -> Option<LQIPData> {
    let img = match image::open(input) {
        Ok(img) => img,
        Err(_) => return None,
    };

    let ext = match input.extension() {
        Some(ext) => ext,
        None => return None,
    };

    let ext_str = match ext.to_str() {
        Some(s) => s,
        None => return None,
    };

    let (width, height) = img.dimensions();
    let lqip_img = img.resize(30, 30, image::imageops::Nearest).blur(5.0);
    let img_buf = lqip_img.as_bytes();
    let as_base64 = base64::encode(&img_buf);

    Some(LQIPData {
        image: format!("data:image/{};base64,{}", ext_str, as_base64),
        width,
        height,
    })
}
