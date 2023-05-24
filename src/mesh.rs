use wgpu::util::DeviceExt;
use crate::cli;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Descriptor {
    quads_in_row: u32,  // number of quads in a row
    rows_of_quads: u32, // number of rows of quads
    xoffset: f32,       // location of first x value
    yoffset: f32,       // location of first y value
    xscale: f32,        // x scale factor
    yscale: f32,        // y scale factor
    channel: i32,       // red, green or blue color channel
    zdisplace: f32,     // z displacement between rgb color grids
}

impl Descriptor {
    pub fn new(
        rowsize: u32,   // number of vertexes in a row
        nrows: u32,     // number of rows
        xoffset: f32,   // location of first x value
        yoffset: f32,   // location of first y value
        xscale: f32,    // x scale factor
        yscale: f32,    // y scale factor
        channel: i32,   // red, green or blue color channel
        zdisplace: f32,
    ) -> Self {
        Self {
            quads_in_row: rowsize - 1,
            rows_of_quads: nrows - 1,
            xoffset,
            yoffset,
            xscale,
            yscale,
            channel,
            zdisplace,
        }
    }
    // Sets up so that the scale goes from -1 to +1 for both
    // x vertexes and y vertexes
    pub fn default(
        rowsize: u32,   // number of vertexes in a row
        nrows: u32,     // number of rows of vertexes
        channel: i32,
        zdisplace: f32,
    ) -> Self {
        let quads_in_row = rowsize - 1;
        let rows_of_quads = nrows - 1;
        let xscale = 2.0 / quads_in_row as f32;
        let yscale = 2.0 / rows_of_quads as f32;
        Self {
            quads_in_row,
            rows_of_quads,
            xoffset: -1.0,
            yoffset: -1.0,
            xscale,
            yscale,
            channel,
            zdisplace,
        }
    }
    pub fn another(&self, channel: i32, zdisplace: f32) -> Descriptor {
        Descriptor {
            channel,
            zdisplace,
            ..*self
        }
    }
    pub fn nverts(self: &Self) -> u32 {
        self.quads_in_row * self.rows_of_quads * 6
    }
}

pub struct Data {
    pub desc: Descriptor,
    pub layout: wgpu::BindGroupLayout,
    pub bind: wgpu::BindGroup,
}

impl Data {
    pub fn new(
        desc: Descriptor,
        device: &wgpu::Device,
    ) -> Self {
        let layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("mesh::Data bind_group_layout"),
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh::Data buffer"),
            contents: bytemuck::cast_slice(&[desc]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("mesh::Data bind_group"),
        });
        Self {
            desc,
            layout,
            bind,
        }
    }
    pub fn nverts(&self) -> u32 { self.desc.nverts() }
    pub fn channel(&self) -> i32 { self.desc.channel }
}

pub enum Vary {
    Single(Data),
    Triple([Data; 3]),
}

// pub struct Group {
//     pub layout: wgpu::BindGroupLayout,
//     pub data: Vary,
// }

// impl Group {
//     pub fn new(args: &cli::Cli, device: &wgpu::Device) -> Self {

//         let chan0 =  if cli::Channel::is_rgb(args.channel()) {
//             cli::Channel::red()
//         } else { args.channel() };

//         let desc0 = Descriptor::default(args.xres(), args.yres(), chan0, 0.0);
    
//         let descriptors = [
//             desc0,
//             desc0.another(cli::Channel::green(), 0.0),
//             desc0.another(cli::Channel::blue(), 0.0),
//         ];

//         let layout =
//         device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             entries: &[wgpu::BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
//                 ty: wgpu::BindingType::Buffer {
//                     ty: wgpu::BufferBindingType::Uniform,
//                     has_dynamic_offset: false,
//                     min_binding_size: None,
//                 },
//                 count: None,
//             }],
//             label: Some("camera_bind_group_layout"),
//         });

//         let desc0 = Descriptor::default(args.xres(), args.yres(), chan0, 0.0);

//         let data = if (!cli::Channel::is_rgb(args.channel())) {
//             Vary::Single(Data::new(desc0, device, &layout))
//         } else {
//             Vary::Triple([
//                 Data::new(desc0.another(cli::Channel::red(), 0.0), device, &layout),
//                 Data::new(desc0.another(cli::Channel::red(), 0.0), device, &layout),
//                 Data::new(desc0.another(cli::Channel::red(), 0.0), device, &layout),
//             ])
//         };

//         Self {
//             layout,
//             data,
//         }
//     }
//     pub fn nverts(&self) -> u32 {
//         match self.data {
//             Vary::Single(d) => d.desc.nverts(),
//             Vary::Triple(d) => d[0].desc.nverts(),
//         }
//     }

// }

        // let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Camera Buffer"),
        //     contents: bytemuck::cast_slice(&[descriptors[0]]),
        //     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        // });

        // let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: buffer.as_entire_binding(),
        //     }],
        //     label: Some("camera_bind_group"),
        // });

    // pub fn buffer<'a>(&self) -> wgpu::util::BufferInitDescriptor<'a> {
    //     // let mesh_buffer = 
    //     // device.create_buffer_init(
    //         wgpu::util::BufferInitDescriptor {
    //             label: Some("mesh::Descriptor::create_buffer_init"),
    //             contents: bytemuck::cast_slice(&[*self]),
    //             usage: wgpu::BufferUsages::UNIFORM |
    //                 wgpu::BufferUsages::COPY_DST,
    //         }
    //     // )
    // }
    // pub fn entry<'a>(self: Self, device: &wgpu::Device) -> wgpu::BindGroupEntry<'a> {
    //     // let mesh_buffer = 
    //     device.create_buffer_init(
    //         &wgpu::util::BufferInitDescriptor {
    //             label: Some("mesh::Descriptor::create_buffer_init"),
    //             contents: bytemuck::cast_slice(&[self]),
    //             usage: wgpu::BufferUsages::UNIFORM |
    //                 wgpu::BufferUsages::COPY_DST,
    //         }
    //     )
    // }
