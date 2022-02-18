use std::path::Path;

pub fn create_lqip(input: &Path) -> String {
    let img = image::open(input).unwrap();
    let ext = input.extension().unwrap().to_str().unwrap();

    // let (w, h) = img.dimensions();

    let lqip_img = img.resize(30, 30, image::imageops::Nearest).blur(5.0);

    let img_buf = lqip_img.as_bytes();

    let as_base64 = base64::encode(&img_buf);

    lqip_img.save("test.jpg").unwrap();
    format!("data:image/{};base64,{}", ext, as_base64)
}
