use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::Path;

type BaseTitle = String;
type ImageWidth = u16;
type FileName = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    #[serde(flatten)]
    images: HashMap<BaseTitle, SingleImageData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SingleImageData {
    #[serde(flatten)]
    image_widths: HashMap<ImageWidth, TypeVariantData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TypeVariantData {
    #[serde(flatten)]
    variant_names: HashMap<String, FileName>,
}

// need to be able to compare key equality
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum ImageFormat {
    JPG,
    WEBP,
    PNG,
    GIF,
    SVG,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageRecord {
    base_name: String,
    width: u16,
    format: ImageFormat,
}

impl ImageData {
    pub fn new() -> ImageData {
        ImageData {
            images: HashMap::new(),
        }
    }

    pub fn add_record(&mut self, image_record: &ImageRecord) {
        let file_name = format!(
            "{}-w{}.{}",
            image_record.base_name,
            image_record.width,
            image_record.format.to_string(),
        );

        let singe_image_data = self
            .images
            .entry(image_record.base_name.to_owned())
            .or_insert(SingleImageData {
                image_widths: HashMap::new(),
            });

        let type_variant_data = singe_image_data
            .image_widths
            .entry(image_record.width)
            .or_insert(TypeVariantData {
                variant_names: HashMap::new(),
            });

        type_variant_data
            .variant_names
            .entry(image_record.format.to_string())
            .or_insert(file_name);
    }

    pub fn write(&self, file_path: &Path) -> Result<(), std::io::Error> {
        let data = self.to_string();

        fs::write(file_path, data)
    }
}

impl Display for ImageData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(data) => write!(f, "{}", data),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ImageFormat::JPG => write!(f, "jpg"),
            ImageFormat::WEBP => write!(f, "webp"),
            ImageFormat::PNG => write!(f, "png"),
            ImageFormat::GIF => write!(f, "gif"),
            ImageFormat::SVG => write!(f, "svg"),
        }
    }
}
