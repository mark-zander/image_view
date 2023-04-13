use wgpu::*;
use wgpu::util::DeviceExt;


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshDescriptor {
    rowsize: u32,   // number of vertexes in a row
    nrows: u32,     // number of rows
    xoffset: f32,   // location of first x value
    yoffset: f32,   // location of first y value
    xscale: f32,    // x scale factor
    yscale: f32,    // y scale factor
    channel: u32,   // red, green or blue color channel
    nverts: u32,
}

impl MeshDescriptor {
    pub fn new(
        rowsize: u32,   // number of vertexes in a row
        nrows: u32,     // number of rows
        xoffset: f32,   // location of first x value
        yoffset: f32,   // location of first y value
        xscale: f32,    // x scale factor
        yscale: f32,    // y scale factor
        channel: u32,   // red, green or blue color channel
        nverts: u32,
    ) -> Self {
        Self {
            rowsize,
            nrows,
            xoffset,
            yoffset,
            xscale,
            yscale,
            channel,
            nverts,
        }
    }
    pub fn default() -> Self {
        let rowsize = 3;
        let nrows = 3;
        let nverts = (rowsize - 1) * (nrows - 1) * 6;
        Self {
            rowsize,
            nrows,
            xoffset: -1.0,
            yoffset: -1.0,
            xscale: 1.0,
            yscale: 1.0,
            channel: 0,
            nverts,
        }
    }
    pub fn nverts(self: &Self) -> u32 { self.nverts }
    pub fn mesh_buffer(self: Self, device: &wgpu::Device) -> wgpu::Buffer {
        // let mesh_buffer = 
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh"),
                contents: bytemuck::cast_slice(&[self]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        )
    }
}

pub struct UniformBinding {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl UniformBinding {
    pub fn new(buffer: wgpu::Buffer, device: &wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("bind group layout"),
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("bind_group"),
        });
        Self {
            bind_group_layout,
            bind_group,
        }
    }
}
