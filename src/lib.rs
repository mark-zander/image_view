// lib.rs
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::window::Window;
use image::io::Reader as ImageReader;
use wgpu::util::DeviceExt;

// use image::GenericImageView;
// use std::path::PathBuf;
// use anyhow::*;

pub mod cli;
mod mesh;
mod pipeline;
mod texture;
mod camera;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // window: &Window,
    render_pipeline: wgpu::RenderPipeline,
    render_pipeline_red: wgpu::RenderPipeline,
    render_pipeline_green: wgpu::RenderPipeline,
    render_pipeline_blue: wgpu::RenderPipeline,

    image_text: texture::Texture, // image texture
    // mesh_group: mesh::Group,
    mesh_data: mesh::Data,
    mesh_data_red: mesh::Data,
    mesh_data_green: mesh::Data,
    mesh_data_blue: mesh::Data,
    // mesh_desc: mesh::Descriptor,
    // mesh_group: uniform_buffer::Group,
    // mesh_bind_group: wgpu::BindGroup,
    // multi_mesh_desc: [mesh::Descriptor; 3],
    // multi_mesh_uniform: [uniform_buffer::UniformBinding; 3],
    depth: texture::Depth,
    // All this for the camera? Needs it's own struct?
    camera: camera::Camera,
    projection: camera::Projection,
    model_view: camera::ModelView,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    mouse_pressed: bool,
    channel: i32,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(
        window: &Window,
        args: &cli::Cli,
    ) -> Self {
        let size = window.inner_size();

        // This is the size of the mesh. 6 is the smallest possible mesh.
        // Should this be rowsize & nrows? Based on subsampling image.
        // Needs a uniform for rowsize. Compute nindexes.
        // let num_indices = 6;

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                // features: wgpu::Features::empty(),
                features: wgpu::Features::POLYGON_MODE_LINE,
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let image_file = ImageReader::open(&args.image_name()).expect(
            "Error: Failed to open file");
        let image = image_file.decode().expect(
            "Error: Failed to read image");

        let image_text = texture::Texture::from_image(
            &device, &queue, &image, "image data").unwrap();

        // let mesh_desc = mesh::Descriptor::default(args.xres(), args.yres(),
        //     if cli::Channel::is_rgb(args.channel()) { cli::Channel::red() }
        //     else { args.channel() });
        // let mesh_uniform = uniform_buffer::Bind::new(
        //     0, wgpu::ShaderStages::VERTEX_FRAGMENT);
        // let mesh_group = uniform_buffer::Group::new(
        //     1, vec![mesh_uniform], &device);
        // let mesh_bind_group = mesh_group.group(
        //     &device,
        //     &[
        //         mesh_uniform.entry(&device, &mesh_desc.buffer()),
        //     ]
        // );
        // let mesh_group = mesh::Group::new(args, &device);

        let mesh = mesh::Descriptor::default(
            args.xres(), args.yres(), args.channel(), 0.0);
        let mesh_data = mesh::Data::new(mesh, &device);

        let mesh_red = mesh.another(cli::Channel::red(), 0.0);
        let mesh_data_red = mesh::Data::new(mesh_red, &device);
        let mesh_green = mesh.another(cli::Channel::green(), 0.0);
        let mesh_data_green = mesh::Data::new(mesh_green, &device);
        let mesh_blue = mesh.another(cli::Channel::blue(), 0.0);
        let mesh_data_blue = mesh::Data::new(mesh_blue, &device);

        let depth = texture::Depth::create(&device, &config, "depth_texture");

        // Camera initialization code

        let model_view = camera::ModelView::new(
            cgmath::Deg(0.0), cgmath::Deg(0.0)); // model transformations
        let camera = camera::Camera::new(
            (0.0, 0.0, 3.0), cgmath::Deg(-90.0), cgmath::Deg(0.0));
        let projection = camera::Projection::new(
            config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera_controller = camera::CameraController::new(4.0, 0.4);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection, &model_view);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let render_pipeline = pipeline::make(&device, &config, &args,
            mesh_data.channel(),
            &[
                &image_text.bind_group_layout,
                &mesh_data.layout,
                &camera_bind_group_layout,
            ]);

        let render_pipeline_red = pipeline::make(&device, &config, &args,
            mesh_data_red.channel(),
            &[
                &image_text.bind_group_layout,
                &mesh_data_red.layout,
                &camera_bind_group_layout,
            ]);
    
        let render_pipeline_green = pipeline::make(&device, &config, &args,
            mesh_data_green.channel(),
            &[
                &image_text.bind_group_layout,
                &mesh_data_green.layout,
                &camera_bind_group_layout,
            ]);
    
        let render_pipeline_blue = pipeline::make(&device, &config, &args,
            mesh_data_blue.channel(),
            &[
                &image_text.bind_group_layout,
                &mesh_data_blue.layout,
                &camera_bind_group_layout,
            ]);
    
    

        Self {
            // window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            render_pipeline_red,
            render_pipeline_green,
            render_pipeline_blue,
            image_text,
            mesh_data,
            mesh_data_red,
            mesh_data_green,
            mesh_data_blue,
            // mesh_desc,
            // mesh_group,
            // mesh_bind_group,
            // multi_mesh_desc,
            // multi_mesh_uniform,
            depth,
            camera,
            projection,
            model_view,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            mouse_pressed: false,
            channel: args.channel(),
        }

    }

    // pub fn window(&self) -> &Window {
    //     &self.window
    // }

    // impl State
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth = texture::Depth::create(
                &self.device, &self.config, "depth_texture");
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                // self.camera_controller.process_scroll(delta);
                // self.camera_controller.process_mouse(delta, delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }

    }

    fn update(&mut self, dt: std::time::Duration) {
        self.camera_controller.update_model_view(&mut self.model_view, dt);
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection,
            &self.model_view);
        // println!("{:?}", self.camera_uniform);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn render_pass(&mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        chn: i32,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            // There could be more than 1 render target.
            color_attachments: &[
                // This is what @location(0) in the fragment shader targets
                Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // must be zero to overwrite one image on another
                        // blending properly may fix this?
                        load: wgpu::LoadOp::Load,
                        // load: wgpu::LoadOp::Clear(
                        //     wgpu::Color {
                        //         r: 0.0,
                        //         g: 0.0,
                        //         b: 0.0,
                        //         a: 1.0,
                        //     }
                        // ),
                        store: true,
                    }
                })
            ],
            // depth_stencil_attachment: None,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });


        
        match chn {
            1 => {
                render_pass.set_pipeline(&self.render_pipeline_red);
                render_pass.set_bind_group(
                    1, &self.mesh_data_red.bind, &[]);
            },
            2 => {
                render_pass.set_pipeline(&self.render_pipeline_green);
                render_pass.set_bind_group(
                    1, &self.mesh_data_green.bind, &[]);
            },
            3 => {
                render_pass.set_pipeline(&self.render_pipeline_blue);
                render_pass.set_bind_group(
                    1, &self.mesh_data_blue.bind, &[]);
            },
            _ => {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(
                    1, &self.mesh_data.bind, &[]);
            },
        }

        render_pass.set_bind_group(
            0, &self.image_text.bind_group, &[]); // NEW!
        render_pass.set_bind_group(
            2, &self.camera_bind_group, &[]);

        // render_pass.set_bind_group(
        //     1, &self.mesh_data.bind, &[]);


        render_pass.draw(
            0..self.mesh_data.nverts(), 0..1); // 3.
        // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        // render_pass.draw(0..726, 0..1); // 3.
    }


    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {label: Some("Render Encoder"),});
        if !cli::Channel::is_rgb(self.channel) {
            self.render_pass(&mut encoder, &view, 0);
        } else {
            self.render_pass(&mut encoder, &view, 1);
            self.render_pass(&mut encoder, &view, 2);
            self.render_pass(&mut encoder, &view, 3);
        }
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
    
}

pub async fn run(args: &cli::Cli) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    // let mut state = State::new(window, cli, args).await;
    let mut state = State::new(&window, args).await;
    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                state.update(dt);
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

        // let mesh_uniform = uniform_buffer::UniformBinding::new(
        //     mesh_desc.buffer(&device), &device
        // );

        // let multi_mesh_desc: [mesh::Descriptor; 3] = [
        //     mesh::Descriptor::default(
        //         args.xres(), args.yres(), cli::Channel::red()),
        //     mesh::Descriptor::default(
        //         args.xres(), args.yres(), cli::Channel::green()),
        //     mesh::Descriptor::default(
        //         args.xres(), args.yres(), cli::Channel::blue()),
        // ];

        // let multi_mesh_uniform: [uniform_buffer::UniformBinding; 3] =
        //     multi_mesh_desc.map(|x|
        //     uniform_buffer::UniformBinding::new(
        //         x.buffer(&device), &device
        //     ));


        // let multi_mesh_uniform: [uniform_buffer::UniformBinding; 3] = [
        //     uniform_buffer::UniformBinding::new(
        //         multi_mesh_desc[0].mesh_buffer(&device), &device
        //     ),
        //     uniform_buffer::UniformBinding::new(
        //         multi_mesh_desc[1].mesh_buffer(&device), &device
        //     ),
        //     uniform_buffer::UniformBinding::new(
        //         multi_mesh_desc[2].mesh_buffer(&device), &device
        //     ),
        // ];

            // if !cli::Channel::is_rgb(self.channel) {
            // } else {
                // for chn in 1..4 {
                //     render_pass.set_pipeline(&self.render_pipeline); // 2.
                //     render_pass.set_bind_group(
                //         0, &self.image_text.bind_group, &[]); // NEW!
                //     render_pass.set_bind_group(
                //         1, &self.multi_mesh_uniform[chn - 1].bind_group, &[]);
                //     render_pass.set_bind_group(
                //         2, &self.camera_bind_group, &[]);
                //     // render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        
                //     render_pass.draw(0..self.mesh_desc.nverts(), 0..1); // 3.
                // }
            // }
