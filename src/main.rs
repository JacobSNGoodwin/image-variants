mod image_data;
mod image_proc;

use std::{
    collections::HashSet,
    env::current_dir,
    fs::{create_dir_all, read_dir},
    ops::ControlFlow,
    path::PathBuf,
};

use clap::Parser;
use image_data::{ImageData, ImageFormat};

use crate::{
    image_data::ImageVariant,
    image_proc::{create_lqip, LQIPData},
};

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
    widths: Option<Vec<u16>>,

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

fn main() {
    // TODO - get data file name
    let args = Args::parse();

    let valid_extensions = HashSet::from(ALLOWED_FILE_EXTENSIONS);
    let formats = args
        .formats
        .unwrap_or(vec![ImageFormat::JPG, ImageFormat::WEBP]);
    let widths = args.widths.unwrap_or(vec![800, 1200, 1800, 2400]);

    // println!("The output types are \"{:?}\"", formats);
    // println!("The widths are \"{:?}\"", widths);

    let base_path = current_dir().unwrap();
    let images_path = base_path.join(args.dir);
    let out_path = base_path.join(args.out_dir);

    let image_files_dir = match read_dir(&images_path) {
        Ok(files) => files,
        Err(e) => panic!(
            "Unable to read files in images_path: {:?}. Error: {}",
            images_path, e
        ),
    };

    let filtered_image_files: Vec<PathBuf> = image_files_dir
        .filter_map(|entry| {
            let file_entry = entry.unwrap();
            let file_path = file_entry.path();

            let ext = file_path.extension()?.to_str()?;

            if valid_extensions.contains(ext) {
                Some(file_path)
            } else {
                None
            }
        })
        .collect();

    let mut image_data = ImageData::new();

    filtered_image_files.iter().try_for_each(|path| {
        let name = path
            .file_stem()
            .ok_or_else(|| {
                println!("Could not extract file name from path.");
                ControlFlow::<()>::Continue(())
            })
            .and_then(|val| {
                val.to_str().ok_or_else(|| {
                    println!("Could not extract file name from path.");
                    ControlFlow::<()>::Continue(())
                })
            });

        let name = match name {
            Ok(val) => val,
            Err(cf) => return cf,
        };

        let lqip = match create_lqip(path) {
            Ok(data) => data,
            Err(e) => {
                println!("{}", e);
                return ControlFlow::<()>::Continue(());
            }
        };

        println!("Created lqip_data: {:?}", lqip);

        image_data.add_record(name.to_string());

        widths.iter().for_each(|width| {
            formats.iter().for_each(|format| {
                let image_variant = ImageVariant {
                    base_name: name.to_string(),
                    width: width.to_owned(),
                    format: format.to_owned(),
                };
                image_data.add_variant(&image_variant)
            });
        });

        ControlFlow::Continue(())
    });

    let data_file_path = out_path.join("data.json");
    create_dir_all(out_path).unwrap();
    println!("Attempting to write data info to {:?}", data_file_path);

    image_data.write(&data_file_path).unwrap();

    println!("Success!");
}
