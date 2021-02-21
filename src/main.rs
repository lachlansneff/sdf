use camera::{ArcballCamera, Camera};
use tree::CSG;
use ultraviolet::Vec3;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod camera;
mod sdf;
mod tree;

async fn run(event_loop: EventLoop<()>, window: Window) -> ! {
    let mut size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::BackendBit::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_format = adapter.get_swap_chain_preferred_format(&surface);

    let sdf_renderer = sdf::SDFRender::new(&device, swapchain_format);

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let mut camera = ArcballCamera::new(10.0, 1.0);
    let light = Vec3::new(10.0, 30.0, 30.0);
    let fov = 45.0;

    camera.resize(size, fov, 0.1);
    sdf_renderer.write_uniforms(&queue, &camera, light, size, fov);

    let csg = CSG::new_example();
    print!("{}", csg);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        // let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Poll;

        camera.update(&event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                size = new_size;
                // Recreate the swap chain with the new size
                sc_desc.width = new_size.width;
                sc_desc.height = new_size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);

                camera.resize(new_size, fov, 0.1);
                sdf_renderer.write_uniforms(&queue, &camera, light, new_size, fov);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                sdf_renderer.write_uniforms(&queue, &camera, light, size, fov);

                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    sdf_renderer.render(&mut rpass);
                }

                queue.submit(Some(encoder.finish()));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    })
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    wgpu_subscriber::initialize_default_subscriber(None);
    pollster::block_on(run(event_loop, window))
}
