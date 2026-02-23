use std::sync::Arc;

use wgpu::*;
use winit::window::Window;

use super::sprite::Sprite;

pub struct Renderer {

    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,

    pipeline: RenderPipeline,

    sprites: Vec<Sprite>,
}

impl Renderer {

    pub async fn new(window: Arc<Window>, sprites: &[Sprite]) -> Self {

        let instance = Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::default(),
                trace: Trace::default(),
                experimental_features: Default::default(),
            })
            .await
            .unwrap();

        let caps = surface.get_capabilities(&adapter);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: 800,
            height: 600,
            present_mode: PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(
            ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::Wgsl(
                    include_str!("../shader.wgsl").into(),
                ),
            },
        );

        let pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        let pipeline =
            device.create_render_pipeline(&RenderPipelineDescriptor {

                label: None,
                layout: Some(&pipeline_layout),

                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },

                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: config.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                }),

                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            sprites: sprites.to_vec(),
        }
    }

    pub fn render(&mut self) {

        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => return,
        };

        let view = frame.texture.create_view(&Default::default());

        let mut encoder =
            self.device
                .create_command_encoder(&Default::default());

        {
            let mut pass =
                encoder.begin_render_pass(&RenderPassDescriptor {

                    label: None,

                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::BLACK),
                            store: StoreOp::Store,
                        },
                    })],

                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });

            pass.set_pipeline(&self.pipeline);

            for _sprite in &self.sprites {
                pass.draw(0..3, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}