// src/blit.rs
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RectUniform {
    // center_x, center_y, width_ndc, height_ndc
    rect: [f32; 4],
}

#[derive(Clone)]
pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub sampler: wgpu::Sampler,
    // optional pre-created fullscreen vertex buffer (not strictly required)
    pub vertex_buffer: wgpu::Buffer,
    // uniform buffer reused every frame (we update contents each frame)
    pub uniform_buffer: wgpu::Buffer,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        // shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("blit_shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!("./shader/blit_layer.wgsl")
            ))
        });

        // bind group layout: 0 - texture, 1 - sampler, 2 - uniform
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("blit_bind_group_layout"),
                entries: &[
                    // texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // uniform
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<RectUniform>() as _),
                        },
                        count: None,
                    },
                ],
            });

        // pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("blit_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // create pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // using @builtin(vertex_index)
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("blit_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // small vertex buffer - optional, we can use vertex_index in shader and avoid buffer.
        let quad_verts: &[f32] = &[
            // not used by shader, kept for compatibility (pos.xy, uv.xy)
            -1.0, -1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 0.0,
            -1.0,  1.0, 0.0, 1.0,
            1.0,  1.0, 1.0, 1.0,
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("blit_vertex_buffer"),
            contents: bytemuck::cast_slice(quad_verts),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("blit_uniform_buffer"),
            size: std::mem::size_of::<RectUniform>() as _,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            bind_group_layout,
            sampler,
            vertex_buffer,
            uniform_buffer,
        }
    }

    /// Render blit: draws `src_view` sampled into `render_pass` at `dst_rect_px` (x,y,w,h in physical pixels, origin top-left).
    /// `viewport_size_px` is (width, height) of the render target in physical pixels.
    /// We update the uniform buffer via `queue.write_buffer` and create a per-frame bind_group (texture view changes).
    pub fn render_blit(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'_>,
        src_view: &wgpu::TextureView,
        dst_rect_px: (u32, u32, u32, u32),
        viewport_size_px: (u32, u32),
    ) {
        // compute NDC center & size
        let (px, py, pw, ph) = (
            dst_rect_px.0 as f32,
            dst_rect_px.1 as f32,
            dst_rect_px.2 as f32,
            dst_rect_px.3 as f32,
        );
        let (vw, vh) = (viewport_size_px.0 as f32, viewport_size_px.1 as f32);

        // center in pixels (top-left origin -> center_y)
        let center_x = px + pw * 0.5;
        let center_y = py + ph * 0.5;

        // convert to NDC:
        // ndc_x = (center_x / vw) * 2 - 1
        // ndc_y = 1 - (center_y / vh) * 2   (top-left -> NDC center)
        let ndc_cx = (center_x / vw) * 2.0 - 1.0;
        let ndc_cy = 1.0 - (center_y / vh) * 2.0;
        let ndc_w = (pw / vw) * 2.0;
        let ndc_h = (ph / vh) * 2.0;

        let rect = RectUniform {
            rect: [ndc_cx, ndc_cy, ndc_w, ndc_h],
        };

        // update uniform buffer
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&rect));

        // create bind group per frame (texture view changes across frames/layers)
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
            ],
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        // we use vertex_index in shader; still set vertex buffer optionally
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // draw 4 vertices (triangle strip)
        render_pass.draw(0..4, 0..1);
    }
}

// State can be empty for now, reserved for renderer
#[derive(Default)]
pub struct State {}