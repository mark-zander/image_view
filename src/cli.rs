use clap::Parser;
use clap::ValueEnum;
use std::path::PathBuf;
// use image::io::Reader as ImageReader;

#[derive(Parser,Default,Debug)]
#[clap(author="Author Name", version, about)]
/// View image files
struct Cli {
    /// File name of image for viewing
    image_name: PathBuf,
    #[arg(value_enum, short, long, default_value_t=DisplayMode::Color)]
    /// Controls the way each polygon is rasterized
    display_mode: DisplayMode,

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum DisplayMode {
    Wire,
    #[default]
    Color,
}

impl DisplayMode {
    pub fn frag_entry(&self) -> &str {
        match &self {
            DisplayMode::Wire => "fs_wire",
            DisplayMode::Color => "fs_color",
        }
    }
    pub fn polygon_mode(&self) -> wgpu::PolygonMode {
        match &self {
            DisplayMode::Wire => wgpu::PolygonMode::Line,
            _ => wgpu::PolygonMode::Fill,
        }
    }
}



pub struct Args {
    pub image_name: PathBuf,
    // pub image: 
    pub polygon_mode: wgpu::PolygonMode,
    pub frag_entry: String,
}

impl Args {
    pub fn new() -> Self {
        let cli = Cli::parse();
        println!("{:?}", cli);
        // let image = image::open(image_name).expect("Error: File not found!");
        Self {
            image_name: cli.image_name,
            // image,
            polygon_mode: cli.display_mode.polygon_mode(),
            frag_entry: String::from(cli.display_mode.frag_entry()),
        }
    }
}
