use std::alloc::Layout;

use wgpu::*;
use wgpu::util::DeviceExt;


#[derive(Debug, Copy, Clone)]
pub struct Bind {
    pub binding: u32,
    pub layout_entry: wgpu::BindGroupLayoutEntry,
}

impl Bind {
    pub fn new(binding: u32, visibility: wgpu::ShaderStages) -> Self {
        let layout_entry = wgpu::BindGroupLayoutEntry {
            binding: binding,
            // visibility: wgpu::ShaderStages::VERTEX,
            visibility: visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        Self {
            binding,
            layout_entry
        }
    }

    pub fn entry<'a>(
        &self,
        device: &wgpu::Device,
        buffer: &wgpu::util::BufferInitDescriptor
    ) -> wgpu::BindGroupEntry<'a> {
        let a_buffer = device.create_buffer_init(buffer);
        wgpu::BindGroupEntry {
            binding: self.binding,
            resource: a_buffer.as_entire_binding(),
        }
    }
}

pub struct Group {
    pub grouping: u32,
    pub binds: Vec<Bind>,
    pub layout: BindGroupLayout,
}

impl Group {
    pub fn new(
        grouping: u32, binds: Vec<Bind>, device: &wgpu::Device
    ) -> Self {
        let v: Vec<BindGroupLayoutEntry> =
            binds.iter().map(|x| x.layout_entry).collect();
        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &v,
                label: Some("uniform_buffer::BindGroup::layout"),
            }
        );
        Self { grouping, binds, layout }
    }

    pub fn group(
        &self, device: &wgpu::Device, entries: &[BindGroupEntry]
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layout,
            entries: entries,
            label: Some("uniform_buffer::BindGroup::group"),
        })
    }
}

// pub struct UniformBinding {
//     pub bind_group_layout: wgpu::BindGroupLayout,
//     pub bind_group: wgpu::BindGroup,
// }

// impl UniformBinding {
//     pub fn new(buffer: wgpu::Buffer, device: &wgpu::Device) -> Self {
//         let bind_group_layout = device.create_bind_group_layout(
//             &wgpu::BindGroupLayoutDescriptor {
//                 entries: &[
//                     wgpu::BindGroupLayoutEntry {
//                         binding: 0,
//                         // visibility: wgpu::ShaderStages::VERTEX,
//                         visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
//                         ty: wgpu::BindingType::Buffer {
//                             ty: wgpu::BufferBindingType::Uniform,
//                             has_dynamic_offset: false,
//                             min_binding_size: None,
//                         },
//                         count: None,
//                     }
//                 ],
//                 label: Some("bind group layout"),
//             }
//         );
//         let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             layout: &bind_group_layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: buffer.as_entire_binding(),
//                 }
//             ],
//             label: Some("bind_group"),
//         });
//         Self {
//             bind_group_layout,
//             bind_group,
//         }
//     }
// }
