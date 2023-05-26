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

// Reformulates 2 two structures, one that can be copied for use in State.

const RES_DEFAULT: u32 = 51;
const XRES_DEFAULT: u32 = 51;
const YRES_DEFAULT: u32 = 51;
const Z_OFFSET_DEFAULT: f32 = 0.25;
const Z_SCALE_DEFAULT: f32 = 1.0;

#[derive(Parser,Default,Debug)]
#[clap(author="Author Name", version, about)]
/// View image files
pub struct Cli {
    /// File name of image for viewing
    image_name: PathBuf,

    #[arg(short, long)]
    /// Wire frame display
    wire: bool,

    #[arg(value_enum, short, long, default_value_t=Channel::All)]
    /// Channel to be displayed
    channel: Channel,

    #[arg(short, long, default_value_t=XRES_DEFAULT)]
    /// Resolution of the display grid in both x and y
    resolution: u32,

    #[arg(short, long, default_value_t=XRES_DEFAULT)]
    /// X resolution of the display grid
    xres: u32,

    #[arg(short, long, default_value_t=YRES_DEFAULT)]
    /// Y resolution of the display grid
    yres: u32,

    #[arg(short, long, default_value_t=Z_OFFSET_DEFAULT)]
    /// Z displacement between rgb color grids
    offset: f32,

    #[arg(short, long, default_value_t=Z_SCALE_DEFAULT)]
    /// Z scale factor
    scale: f32,

}

impl Cli {
    pub fn new() -> Self { Cli::parse() }
    pub fn image_name(self: &Self) -> &PathBuf { &self.image_name }
    pub fn polygon_mode(self: &Self) -> wgpu::PolygonMode {
        if self.wire { wgpu::PolygonMode::Line }
        else { wgpu::PolygonMode::Fill }
    }
    pub fn frag_entry(self: &Self) -> &str {
        if self.wire { "fs_wire" }
        else { "fs_fill" }
    }

    pub fn channel(self: &Self) -> Channel { self.channel }
    pub fn xres(self: &Self) -> u32 {
        if self.xres != XRES_DEFAULT { self.xres }
        else if self.resolution != RES_DEFAULT { self.resolution }
        else { XRES_DEFAULT }
    }
    pub fn yres(self: &Self) -> u32 {
        if self.yres != YRES_DEFAULT { self.yres }
        else if self.resolution != RES_DEFAULT { self.resolution }
        else { YRES_DEFAULT }
    }
    pub fn args(&self) -> Args {
        Args {
            wire: self.wire,
            channel: self.channel,
            xres: self.xres(),
            yres: self.yres(),
            zoffset: self.offset,
            zscale: self.scale,
        }
    }
    pub fn zoffset(&self) -> f32 { self.offset }
    pub fn zscale(&self) -> f32 { self.scale }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum Channel {
    #[default]
    All = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
    Grey = 4,
    Rgb = 5,
}

impl Channel {
    pub fn value(self: &Self) -> i32 { *self as i32 }
    pub fn is_rgb(&self) -> bool { self == &Channel::Rgb }
    pub fn color_writes(&self) -> wgpu::ColorWrites {
        match self {
            Channel::Red => wgpu::ColorWrites::RED,
            Channel::Green => wgpu::ColorWrites::GREEN,
            Channel::Blue => wgpu::ColorWrites::BLUE,
            _ => wgpu::ColorWrites::ALL,
        }
    }
    // pub fn red() -> i32 { Channel::Red as i32 }
    // pub fn green() -> i32 { Channel::Green as i32 }
    // pub fn blue() -> i32 { Channel::Blue as i32 }
}

#[derive(Copy,Clone)]
pub struct Args {
    wire: bool,
    pub channel: Channel,
    pub xres: u32,
    pub yres: u32,
    pub zoffset: f32,
    pub zscale: f32,
}

impl Args {
    pub fn channel(self: &Self) -> Channel { self.channel }
    pub fn polygon_mode(self: &Self) -> wgpu::PolygonMode {
        if self.wire { wgpu::PolygonMode::Line }
        else { wgpu::PolygonMode::Fill }
    }
    pub fn frag_entry(self: &Self) -> &str {
        if self.wire { "fs_wire" }
        else { "fs_fill" }
    }
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

    // pub fn channel(self: &Self) -> i32 {
    //     match self {
    //         Channel::All => 0,
    //         Channel::Red => 1,
    //         Channel::Green => 2,
    //         Channel::Blue => 3,
    //         Channel::Grey => 4,
    //         Channel::Rgb => 5,
    //     }
    // }

    // #[arg(value_enum, short, long, default_value_t=DisplayMode::Color)]
    // Controls the way each polygon is rasterized
    // display_mode: DisplayMode,

    // pub fn frag_entry(self: &Self) -> &str { self.display_mode.frag_entry() }
    // pub fn polygon_mode(self: &Self) -> wgpu::PolygonMode {
    //     self.display_mode.polygon_mode()
    // }

    // } else if self.channel == Channel::Grey {
        //     "fs_grey"
        // } else {
        //     "fs_color"
            // match self.channel {
            //     Channel::All => "fs_color",
            //     Channel::Red => "fs_red",
            //     Channel::Green => "fs_green",
            //     Channel::Blue => "fs_blue",
            //     Channel::Grey => "fs_grey",
            // }
        // }

    // pub fn color_writes(&self) -> wgpu::ColorWrites {
    //     match self.channel {
    //         Channel::All => wgpu::ColorWrites::ALL,
    //         Channel::Red => wgpu::ColorWrites::RED,
    //         Channel::Green => wgpu::ColorWrites::GREEN,
    //         Channel::Blue => wgpu::ColorWrites::BLUE,
    //         Channel::Grey => wgpu::ColorWrites::ALL,
    //     }
    // }
    // Channel::Red => wgpu::ColorWrites::RED | wgpu::ColorWrites::ALPHA,
            // Channel::Green => wgpu::ColorWrites::GREEN | wgpu::ColorWrites::ALPHA,
            // Channel::Blue => wgpu::ColorWrites::BLUE | wgpu::ColorWrites::ALPHA,

            // Channel::Red => wgpu::ColorWrites::RED,
            // Channel::Green => wgpu::ColorWrites::GREEN,
            // Channel::Blue => wgpu::ColorWrites::BLUE,
