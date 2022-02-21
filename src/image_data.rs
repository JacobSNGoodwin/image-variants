use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::Path;

type BaseTitle = String;
type ImageWidth = u32;
type FileName = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    #[serde(flatten)]
    images: HashMap<BaseTitle, SingleImageData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LQIPData {
    pub image: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SingleImageData {
    lqip: Option<LQIPData>,
    #[serde(flatten)]
    image_widths: HashMap<ImageWidth, TypeVariantData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TypeVariantData {
    #[serde(flatten)]
    variant_names: HashMap<String, FileName>,
}

// need to be able to compare key equality
#[derive(
    ArgEnum, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum ImageFormat {
    JPG,
    PNG,
    GIF,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageVariant {
    pub base_name: String,
    pub width: u32,
    pub format: ImageFormat,
}

impl ImageData {
    pub fn new() -> ImageData {
        ImageData {
            images: HashMap::new(),
        }
    }

    pub fn add_record(&mut self, base_name: String, lqip: Option<LQIPData>) {
        self.images.entry(base_name).or_insert(SingleImageData {
            lqip,
            image_widths: HashMap::new(),
        });
    }

    pub fn add_variant(&mut self, image_record: &ImageVariant) {
        let file_name = format!(
            "{}-{}w.{}",
            image_record.base_name,
            image_record.width,
            image_record.format.to_string(),
        );

        let singe_image_data = self
            .images
            .entry(image_record.base_name.to_owned())
            .or_insert(SingleImageData {
                image_widths: HashMap::new(),
                lqip: None,
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
            ImageFormat::PNG => write!(f, "png"),
            ImageFormat::GIF => write!(f, "gif"),
        }
    }
}
