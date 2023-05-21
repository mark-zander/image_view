use wgpu;

use crate::{texture, cli};

pub fn make(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    args: &cli::Cli,
    chan: i32,
    // image_text: &texture::Texture,
    // mesh_uniform: &uniform_buffer::UniformBinding,
    // camera_uniform: &camera::CameraUniform,
    bind_group_layouts: &[&wgpu::BindGroupLayout]
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layouts,
            // &[
            //     &image_text.bind_group_layout,
            //     &mesh_uniform.bind_group_layout,
            //     &camera_uniform.bind
            // ],
            // bind_group_layouts: bind_group_layouts, // NEW!
            push_constant_ranges: &[],
        });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main", // 1.
            buffers: &[], // 2.
        },
        fragment: Some(wgpu::FragmentState { // 3.
            module: &shader,
            entry_point: &args.frag_entry(),
            targets: &[Some(wgpu::ColorTargetState { // 4.
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                // blend: Some(wgpu::BlendState {
                //     color: wgpu::BlendComponent {
                //         src_factor: wgpu::BlendFactor::One,
                //         dst_factor: wgpu::BlendFactor::One,
                //         operation: wgpu::BlendOperation::Add,
                //     },
                //     alpha: wgpu::BlendComponent {
                //         src_factor: wgpu::BlendFactor::One,
                //         dst_factor: wgpu::BlendFactor::Zero,
                //         operation: wgpu::BlendOperation::Add,
                //     }
                // }),
                write_mask: cli::Channel::color_writes(chan),
                // write_mask: wgpu::ColorWrites::ALL,
                // write_mask: args.color_writes(),
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, // 1.
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, // 2.
            // cull_mode: Some(wgpu::Face::Back),
            cull_mode: None,
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: args.polygon_mode(),
            // polygon_mode: wgpu::PolygonMode::Fill,
            // polygon_mode: wgpu::PolygonMode::Line,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        // depth_stencil: None, // 1.
        depth_stencil: Some(wgpu::DepthStencilState {
            format: texture::Depth::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less, // 1.
            stencil: wgpu::StencilState::default(), // 2.
            bias: wgpu::DepthBiasState::default(),
        }),
            multisample: wgpu::MultisampleState {
            count: 1, // 2.
            mask: !0, // 3.
            alpha_to_coverage_enabled: false, // 4.
        },
        multiview: None, // 5.
    });
    render_pipeline
}
