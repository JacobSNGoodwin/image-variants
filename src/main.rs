mod image_data;
mod image_proc;

use clap::Parser;
use rayon::prelude::*;
use std::{
    collections::HashSet,
    env::current_dir,
    fs::{create_dir_all, read_dir},
};

use image_data::{ImageData, ImageFormat};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The relative directory containing the images
    #[clap(short, long, default_value_t = String::from(""))]
    dir: String,

    /// The relative directory containing the images
    #[clap(short, long, default_value_t = String::from("variants"))]
    out_dir: String,

    /// Space-separated list of output file-types
    #[clap(short, long, arg_enum, multiple_values(true))]
    formats: Option<Vec<ImageFormat>>,

    /// Space-separated list of image variant widths in pixels
    #[clap(short, long, multiple_values(true))]
    widths: Option<Vec<u32>>,

    /// The outout quality for JPG and WEBP images.
    /// Should be a value from 1-100.
    #[clap(short, long, default_value_t = 80, validator(quality_range))]
    quality: u8,
}

fn quality_range(v: &str) -> Result<(), String> {
    if let Ok(val) = v.parse::<u8>() {
        if val >= 1 && val <= 100 {
            return Ok(());
        }

        Err(String::from(
            "quality must be an integer value between 0 and 100",
        ))
    } else {
        Err(String::from(
            "quality must be an integer value between 0 and 100",
        ))
    }
}

const ALLOWED_FILE_EXTENSIONS: [&'static str; 7] =
    ["jpg", "jpeg", "png", "gif", "avif", "webp", "svg"];

#[derive(Debug)]
struct InputImage {
    name: String,
    path: String,
}

fn main() {
    let args = Args::parse();

    let valid_extensions = HashSet::from(ALLOWED_FILE_EXTENSIONS);
    let formats = args.formats.unwrap_or(vec![ImageFormat::JPG]);
    let widths = args.widths.unwrap_or(vec![800, 1200, 1800, 2400]);
    let quality = args.quality;

    let base_path = current_dir().expect("There was a problem accessing the current directory.");
    let images_path = base_path.join(args.dir);
    let out_path = base_path.join(args.out_dir);
    create_dir_all(&out_path).expect("Failed to create out_dir");

    let image_files_dir =
        read_dir(&images_path).expect("Could not read directory provided in \"dir\" argument");

    let valid_image_files: Vec<InputImage> = image_files_dir
        .filter_map(|entry| {
            let file_entry = entry.ok()?;
            let path = file_entry.path();
            let ext = path.extension()?.to_str()?;

            if valid_extensions.contains(ext) {
                Some(InputImage {
                    name: String::from(path.file_stem()?.to_str()?),
                    path: String::from(path.to_str()?),
                })
            } else {
                None
            }
        })
        .collect();

    println!("Found the following supported files...");
    println!("{:?}", valid_image_files);

    let mut image_data = ImageData::new();

    valid_image_files.iter().for_each(|image_info| {
        let lqip = match image_proc::create_lqip(&image_info.path) {
            // Todo -> could creat a From or Into
            // Or find better rust way to convert idential structs
            Ok(data) => Some(image_data::LQIPData {
                image: data.image,
                width: data.width,
                height: data.height,
            }),
            Err(_) => {
                println!("Failed to create LQIP for {}", image_info.name);
                None
            }
        };

        image_data.add_record(image_info.name.to_owned(), lqip);

        let width_format_pairs: Vec<(u32, ImageFormat)> = widths
            .iter()
            .flat_map(|w| formats.iter().map(|f| (*w, f.to_owned())))
            .collect();

        let created_images: Vec<image_data::ImageVariant> = width_format_pairs
            .par_iter()
            .filter_map(|(width, format)| {
                println!(
                    "\nConverting image at {} to width: {} and format: {}. Using quality = {}.",
                    image_info.path, width, format, quality,
                );

                match image_proc::create_variant(
                    image_info.path.to_owned(),
                    &out_path,
                    image_info.name.to_owned(),
                    *width,
                    format,
                    quality,
                ) {
                    Ok(_) => {
                        println!("Successfully created image!");
                        Some(image_data::ImageVariant {
                            base_name: image_info.name.to_string(),
                            width: width.to_owned(),
                            format: format.to_owned(),
                        })
                    }
                    Err(e) => {
                        println!("Failed to create image: {:?}", e);
                        None
                    }
                }
            })
            .collect();

        created_images.iter().for_each(|image_record| {
            image_data.add_variant(image_record);
        });
    });

    let data_file_path = out_path.join("data.json");
    println!("Attempting to write data info to {:?}", data_file_path);

    image_data
        .write(&data_file_path)
        .expect("Failed to write image data to file.");

    println!("Conversion completed. See data.json for records created!");
}
