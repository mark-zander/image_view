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
    // #[arg(value_enum, short, long, default_value_t=DisplayMode::Color)]
    /// Controls the way each polygon is rasterized
    // display_mode: DisplayMode,

    #[arg(short, long)]
    /// Wire frame display
    wire: bool,

    #[arg(value_enum, short, long, default_value_t=Channel::All)]
    /// Channel to be displayed
    channel: Channel,

    #[arg(short, long, default_value_t=11)]
    /// Resolution of the display grid in both x and y
    resolution: u32,

    #[arg(short, long, default_value_t=11)]
    /// X resolution of the display grid
    xres: u32,

    #[arg(short, long, default_value_t=11)]
    /// Y resolution of the display grid
    yres: u32,

}

impl Cli {
    pub fn new() -> Self { Cli::parse() }
    pub fn image_name(self: &Self) -> &PathBuf { &self.image_name }
    // pub fn frag_entry(self: &Self) -> &str { self.display_mode.frag_entry() }
    // pub fn polygon_mode(self: &Self) -> wgpu::PolygonMode {
    //     self.display_mode.polygon_mode()
    // }
    pub fn polygon_mode(self: &Self) -> wgpu::PolygonMode {
        if self.wire { wgpu::PolygonMode::Line }
        else { wgpu::PolygonMode::Fill }
    }
    pub fn frag_entry(self: &Self) -> &str {
        if self.wire {
            "fs_wire"
        } else {
            match self.channel {
                Channel::All => "fs_color",
                Channel::Red => "fs_red",
                Channel::Green => "fs_green",
                Channel::Blue => "fs_blue",
                Channel::Grey => "fs_grey",
            }
        }
    }
    pub fn channel(self: &Self) -> i32 {
        match self.channel {
            Channel::All => 0,
            Channel::Red => 1,
            Channel::Green => 2,
            Channel::Blue => 3,
            Channel::Grey => 4,
        }
    }
    pub fn xres(self: &Self) -> u32 {
        if self.xres > 11 { self.xres }
        else if self.resolution > 11 { self.resolution }
        else { 11 }
    }
    pub fn yres(self: &Self) -> u32 {
        if self.yres > 11 { self.yres }
        else if self.resolution > 11 { self.resolution }
        else { 11 }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum Channel {
    #[default]
    All,
    Red,
    Green,
    Blue,
    Grey,
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
// pub enum DisplayMode {
//     Wire,
//     #[default]
//     Color,
// }

// impl DisplayMode {
//     pub fn frag_entry(&self) -> &str {
//         match &self {
//             DisplayMode::Wire => "fs_wire",
//             _ => "fs_color",
//         }
//     }
//     pub fn polygon_mode(&self) -> wgpu::PolygonMode {
//         match &self {
//             DisplayMode::Wire => wgpu::PolygonMode::Line,
//             _ => wgpu::PolygonMode::Fill,
//         }
//     }
// }

