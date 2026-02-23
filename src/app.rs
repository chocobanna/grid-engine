use wgpu::{
    CompositeAlphaMode, Device, Instance, PresentMode, Queue, Surface, SurfaceConfiguration,
    TextureFormat,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopWindowTarget,
    window::Window,
};

pub struct App<'a> {
    instance: Instance,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl<'a> App<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        // Create wgpu instance (all available backends)
        let instance = Instance::default();

        // SAFETY: surface must not outlive the window; we tie it to 'a.
        let surface = instance
            .create_surface(window)
            .expect("failed to create surface");

        // Choose an adapter compatible with the surface
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("failed to find a suitable GPU adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);

        // Prefer sRGB if available (better for typical color)
        let format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let present_mode = if surface_caps.present_modes.contains(&PresentMode::Mailbox) {
            PresentMode::Mailbox
        } else {
            PresentMode::Fifo // guaranteed
        };

        let alpha_mode = if surface_caps
            .alpha_modes
            .contains(&CompositeAlphaMode::Auto)
        {
            CompositeAlphaMode::Auto
        } else {
            surface_caps.alpha_modes[0]
        };

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn handle_event(
        &mut self,
        window: &'a Window,
        event: Event<()>,
        elwt: &EventLoopWindowTarget<()>,
    ) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                WindowEvent::Resized(new_size) => {
                    self.resize(new_size);
                }
                WindowEvent::RedrawRequested => {
                    self.render();
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    // winit will also send a Resized; nothing special needed here.
                }
                _ => {}
            },

            Event::AboutToWait => {
                // Request a redraw every loop (simple "game loop" style).
                window.request_redraw();
            }

            _ => {}
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            // Minimized; skip configuring (some platforms hate 0-sized surfaces).
            return;
        }

        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost) => {
                // Surface got nuked; reconfigure.
                self.surface.configure(&self.device, &self.config);
                return;
            }
            Err(wgpu::SurfaceError::Outdated) => {
                // Swapchain out of date (resize, etc). Next frame will fix it.
                return;
            }
            Err(wgpu::SurfaceError::Timeout) => {
                // Driver hiccup. Skip this frame.
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                // You're done. Game over.
                panic!("wgpu: out of memory");
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // Just clear the screen.
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.08,
                            g: 0.10,
                            b: 0.13,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

// Handy to keep the type explicit if you want it elsewhere.
#[allow(dead_code)]
fn preferred_format_or_first(formats: &[TextureFormat]) -> TextureFormat {
    formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(formats[0])
}