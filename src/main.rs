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
    formats: Option<Vec<OutputTypes>>,

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
enum OutputTypes {
    JPG,
    WEBP,
    AVIF,
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

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let formats = args
        .formats
        .unwrap_or(vec![OutputTypes::JPG, OutputTypes::WEBP]);

    let widths = args.widths.unwrap_or(vec![800, 1200, 1800, 2400]);

    println!("The output types are \"{:?}\"", formats);
    println!("The widths are \"{:?}\"", widths);
}
