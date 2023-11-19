use clap::Parser;
use image::io::Reader as ImageReader;
use crate::converter::ToAsciiArt;

mod converter;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub struct Args {
    /// The path to an image file
    #[arg(long)]
    file: Option<String>,
    /// The width of the output
    #[arg(long, default_value = "80")]
    width: u32,
    /// The height of the output
    #[arg(long, default_value = "80")]
    height: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.file {
        Some(file) => {
            if !std::path::Path::new(&file).exists() {
                return Err("File does not exist".into());
            }
    
            let image = ImageReader::open(file)?.decode()?;
            let converter = converter::ImageConverter::new(image);
            let options = converter::AsciiOptions::new(args.width, args.height);

            println!("{}", converter.to_ascii_art(Some(options)));
            println!("Image converted successfully!");

            Ok(())
        },
        None => {
            Err("No file specified")?
        }
    }
}
