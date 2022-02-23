use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
    path::Path,
};

use crate::image_data::ImageFormat;
use image::{codecs::jpeg::JpegEncoder, EncodableLayout, GenericImageView};

#[derive(Debug)]
pub struct LQIPData {
    pub image: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum ImageProcError {
    Conversion,
    IO(io::Error),
}

impl Display for ImageProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageProcError::Conversion => write!(f, "There was an error converting the image."),
            ImageProcError::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<image::ImageError> for ImageProcError {
    fn from(e: image::ImageError) -> Self {
        match e {
            image::ImageError::Decoding(_) => ImageProcError::Conversion,
            image::ImageError::Encoding(_) => ImageProcError::Conversion,
            image::ImageError::Parameter(_) => ImageProcError::Conversion,
            image::ImageError::Limits(_) => ImageProcError::Conversion,
            image::ImageError::Unsupported(_) => ImageProcError::Conversion,
            image::ImageError::IoError(e) => ImageProcError::IO(e),
        }
    }
}

impl From<std::io::Error> for ImageProcError {
    fn from(e: std::io::Error) -> Self {
        ImageProcError::IO(e)
    }
}

impl From<&str> for ImageProcError {
    fn from(s: &str) -> Self {
        ImageProcError::IO(std::io::Error::new(std::io::ErrorKind::Other, s))
    }
}

pub type ImageProcResult<T> = Result<T, ImageProcError>;

pub fn create_lqip(input: &String) -> ImageProcResult<LQIPData> {
    let img = image::open(&input)?;
    let path = Path::new(input.as_str());

    let ext = path.extension().ok_or(ImageProcError::IO(io::Error::new(
        io::ErrorKind::Other,
        "Could not extract file extension",
    )))?;

    let ext_str = ext.to_str().ok_or(ImageProcError::IO(io::Error::new(
        io::ErrorKind::Other,
        "Could not extract file extension",
    )))?;

    let (width, height) = img.dimensions();
    let lqip_img = img.resize(30, 30, image::imageops::Nearest).blur(5.0);
    let img_buf = lqip_img.as_bytes();
    let as_base64 = base64::encode(&img_buf);

    Ok(LQIPData {
        image: format!("data:image/{};base64,{}", ext_str, as_base64),
        width,
        height,
    })
}

pub fn create_variant<PIn, POut>(
    in_path: PIn,
    out_path: POut,
    name: String,
    width: u32,
    format: &ImageFormat,
    quality: u8,
) -> ImageProcResult<()>
where
    PIn: AsRef<Path>,
    POut: AsRef<Path>,
{
    let img = image::open(in_path.as_ref())?;

    let out_img = img.resize(width, width * 2, image::imageops::FilterType::Lanczos3);

    let out_file_path = out_path
        .as_ref()
        .join(format!("{}-{}w.{}", name, width, format));

    let mut out_file = File::create(&out_file_path)?;

    match format {
        ImageFormat::JPG => {
            let mut encoder = JpegEncoder::new_with_quality(out_file, quality);
            Ok(encoder.encode_image(&out_img)?)
        }
        ImageFormat::WEBP => {
            let encoder = webp::Encoder::from_image(&out_img)?;
            let webp_img = encoder.encode(quality as f32);
            Ok(out_file.write_all(webp_img.as_bytes())?)
        }
        _ => Ok(out_img.save(out_file_path)?),
    }
}
