use clap::Parser;
use clap::ValueEnum;
use std::path::PathBuf;
// use image::io::Reader as ImageReader;

// I had two structs, one Cli that interprested the command line and the
// other Args that translated everything into what was needed by the program.
// This caused all kinds of ownership issues when I got to non-copy values
// such as PathBuf. The new approach simplifies storage. Instead of having
// 2 separate structs I now have just one, Cli, which yields application values
// through functions instead of values in the Args struct.

#[derive(Parser,Default,Debug)]
#[clap(author="Author Name", version, about)]
/// View image files
pub struct Cli {
    /// File name of image for viewing
    image_name: PathBuf,
    #[arg(value_enum, short, long, default_value_t=DisplayMode::Color)]
    /// Controls the way each polygon is rasterized
    display_mode: DisplayMode,

}

impl Cli {
    pub fn new() -> Self { Cli::parse() }
    pub fn image_name(self: &Self) -> &PathBuf { &self.image_name }
    pub fn frag_entry(self: &Self) -> &str { self.display_mode.frag_entry() }
    pub fn polygon_mode(self: &Self) -> wgpu::PolygonMode {
        self.display_mode.polygon_mode()
    }
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
