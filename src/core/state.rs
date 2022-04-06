//====================================================================

use wgpu::util::DeviceExt;

use crate::{
    core::camera,
    render::texture,
    voxels::{chunk, model},
};
//====================================================================

#[derive(Default)]
pub struct InputController {
    pub pressed: Vec<winit::event::VirtualKeyCode>,
}
impl InputController {
    fn add_key(&mut self, key: &winit::event::VirtualKeyCode) {
        if !self.key_pressed(key) {
            self.pressed.push(*key);
        }
    }

    fn remove_key(&mut self, key: &winit::event::VirtualKeyCode) {
        if let Some(index) = self.check_key(key) {
            self.pressed.remove(index);
        }
    }

    fn check_key(&self, key: &winit::event::VirtualKeyCode) -> Option<usize> {
        for x in 0..self.pressed.len() {
            if key == &self.pressed[x] {
                return Some(x);
            }
        }
        return None;
    }

    pub fn key_pressed(&self, key: &winit::event::VirtualKeyCode) -> bool {
        for k in &self.pressed {
            if k == key {
                return true;
            }
        }
        return false;
    }
}

//====================================================================

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    camera: camera::Camera,
    camera_projection: camera::Projection,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: camera::CameraController,

    input_controller: InputController,

    depth_texture: texture::Texture,

    chunks: chunk::ChunkCollection,

    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        //--------------------------------------------------

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        //--------------------------------------------------

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Wgpu Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        //--------------------------------------------------

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        //--------------------------------------------------

        let camera = camera::Camera::new(glam::Vec3::new(0., 1., 2.), 90f32.to_radians(), 0.);
        let camera_projection =
            camera::Projection::new(size.width, size.height, 45f32.to_radians(), 0.1, 100.);

        let camera_controller = camera::CameraController::new(1., 0.02);

        let camera_uniform = camera::CameraUniform::from_camera(&camera, &camera_projection);

        let new_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("My New Buffer"),
            size: 12,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
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
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        //--------------------------------------------------

        let mut chunks = chunk::ChunkCollection::new();

        //chunks.spawn_chunk(&device, glam::IVec3::new(0, 0, 0));
        chunks.spawn_chunks_in_range(&device, glam::IVec3::new(0, 0, 0));

        //--------------------------------------------------

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "Depth Texture");

        //--------------------------------------------------

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        // println!("Available files");
        // let paths = std::fs::read_dir("./").unwrap();

        // for path in paths {
        //     println!("Name: {}", path.unwrap().path().display())
        // }

        // let file_path = "src/render/shader.wgsl";
        // let path = std::path::Path::new(file_path);

        // let contents = std::fs::read_to_string(path).unwrap();

        // let shader = device.create_shader_module(
        //     &wgpu::ShaderModuleDescriptor {
        //         label: Some("Shader Module Dynamically"),
        //         source: wgpu::ShaderSource::Wgsl(
        //             (&contents).into()
        //         )
        //     }
        // );

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../render/shader.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    model::Vertex::desc(),
                    //voxel::VOXEL_DESC,
                    //voxel::VoxelInstance::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        //--------------------------------------------------

        Self {
            surface,
            device,
            queue,
            config,
            size,

            camera,
            camera_projection,
            camera_buffer,
            camera_bind_group,
            camera_controller,

            input_controller: InputController::default(),

            depth_texture,

            chunks,

            render_pipeline,
        }

        //--------------------------------------------------
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.camera_projection
                .resize(new_size.width, new_size.height);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "Depth Texture");
        }
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                if *state == winit::event::ElementState::Pressed {
                    self.input_controller.add_key(&keycode);
                } else {
                    self.input_controller.remove_key(&keycode);
                }
                return true;
            }
            //winit::event::WindowEvent::CursorEntered { device_id } => todo!(),
            _ => return false,
        }
    }

    pub fn update(&mut self) {
        self.camera_controller
            .update(&mut self.camera, &self.input_controller);

        let camera_uniform =
            camera::CameraUniform::from_camera(&self.camera, &self.camera_projection);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.3,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            chunk::DrawChunk::draw_chunks(&mut render_pass, &self.chunks);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        return self.size;
    }
}

//====================================================================
