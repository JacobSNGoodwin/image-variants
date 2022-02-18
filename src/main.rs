mod image_data;
use std::{collections::HashSet, env::current_dir, fs::read_dir, path::PathBuf};

use clap::{ArgEnum, Parser};

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
    formats: Option<Vec<FormatType>>,

    /// Space-separated list of image variant widths in pixels
    #[clap(short, long, multiple_values(true))]
    widths: Option<Vec<u16>>,

    /// Do not include a low-quality image placeholder
    #[clap(long)]
    no_lqip: bool,

    /// The outout quality for JPG and WEBP images.
    /// Should be a value from 1-100.
    #[clap(short, long, default_value_t = 80, validator(quality_range))]
    quality: u8,

    /// Only include a low-quality image placeholder
    #[clap(long)]
    lqip_only: bool,
}

#[derive(ArgEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FormatType {
    JPG,
    WEBP,
    PNG,
    GIF,
    SVG,
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
    let args = Args::parse();

    let valid_extensions = HashSet::from(ALLOWED_FILE_EXTENSIONS);
    let formats = args
        .formats
        .unwrap_or(vec![FormatType::JPG, FormatType::WEBP]);
    let widths = args.widths.unwrap_or(vec![800, 1200, 1800, 2400]);

    println!("The output types are \"{:?}\"", formats);
    println!("The widths are \"{:?}\"", widths);

    let base_path = current_dir().unwrap();
    let images_path = base_path.join(args.dir);
    let out_path = base_path.join(args.out_dir);

    println!("Images path: {:?}", images_path);
    println!("Output path: {:?}", out_path);

    println!("{:?}", FormatType::value_variants());

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

    println!("Filtered image files: {:?}", filtered_image_files);
}
