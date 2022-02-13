use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The relative directory containing the images
    #[clap(short, long)]
    dir: Option<String>,

    // extention types to convert into
    #[clap(short, long, arg_enum)]
    ext: Vec<OutputTypes>,
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

fn main() {
    let args = Args::parse();

    let dir = args.dir.unwrap_or(String::from(""));

    println!("The dir is \"{}\"", dir);

    println!("The output types are \"{:?}\"", args.ext);
}
