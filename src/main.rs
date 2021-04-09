use camera::{ArcballCamera, Camera};
use tree::CsgTree;
use ultraviolet::Vec3;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod camera;
// mod op;
mod sdf;
mod tree;

async fn run(event_loop: EventLoop<()>, window: Window) -> ! {
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
                features: wgpu::Features::PUSH_CONSTANTS,
                limits: wgpu::Limits {
                    max_push_constant_size: 112,
                    ..Default::default()
                },
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();
    let initial_size = window.inner_size();

    let mut sdf_renderer = sdf::SDFRender::new(&device, initial_size, swapchain_format);

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: initial_size.width,
        height: initial_size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let mut camera = ArcballCamera::new(10.0, 0.3);
    let light = Vec3::new(10.0, 30.0, 30.0);
    let fov = 45.0;

    camera.resize(initial_size, fov, 0.1);

    let csg = CsgTree::new_example();
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
                // Recreate the swap chain with the new size
                sc_desc.width = new_size.width;
                sc_desc.height = new_size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);

                camera.resize(new_size, fov, 0.1);
                sdf_renderer.resize(&device, new_size);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                sdf_renderer.render(&camera, light, fov, &frame.view, &mut encoder);

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
