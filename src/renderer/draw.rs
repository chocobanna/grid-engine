use super::{gpu::Gpu, physics::Physics};

pub fn draw_frame(gpu: &mut Gpu, physics: &Physics) {
    let frame = match gpu.surface.get_current_texture() {
        Ok(f) => f,
        Err(_) => {
            gpu.surface.configure(&gpu.device, &gpu.config);
            return;
        }
    };

    let view = frame.texture.create_view(&Default::default());

    let uniform = [
        physics.position[0],
        physics.position[1],
        physics.rotation,
        0.0,
    ];

    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(uniform.as_ptr() as *const u8, 16)
    };

    gpu.queue.write_buffer(&gpu.uniform_buffer, 0, bytes);

    let mut encoder =
        gpu.device
            .create_command_encoder(&Default::default());

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        pass.set_pipeline(&gpu.pipeline);
        pass.set_bind_group(0, &gpu.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }

    gpu.queue.submit(Some(encoder.finish()));
    frame.present();
}