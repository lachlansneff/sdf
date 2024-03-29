use core::slice;
use std::mem;
use ultraviolet::{Mat4, UVec2, Vec3};
use wgpu::util::{BufferInitDescriptor, DeviceExt as _};
use winit::dpi::PhysicalSize;

use crate::camera::Camera;
use shared::inst::{Inst, RectangularPrism, Ret, SmoothUnion, Sphere};

const STORAGE_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

macro_rules! include_spirv_shader {
    ($($token:tt)*) => {
        {
            //log::info!("including '{}'", $($token)*);
            wgpu::ShaderModuleDescriptor {
                label: Some($($token)*),
                source: wgpu::util::make_spirv(include_bytes!($($token)*)),
                flags: wgpu::ShaderFlags::empty(),
            }
        }
    };
}

struct Pad<const N: usize>([u8; N]);

impl<const N: usize> Default for Pad<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

#[derive(Default)]
#[repr(C)]
struct RenderParams {
    view_mat: Mat4,
    eye: Vec3,
    _pad0: Pad<4>,
    light: Vec3,
    _pad1: Pad<4>,
    resolution: UVec2,
    grid_size: UVec2,
    neg_z_depth: f32,
    _pad2: Pad<12>,
}

static_assertions::assert_eq_size!(RenderParams, [u8; 128]);

// #[derive(Default)]
// #[repr(C)]
// struct ConeTracingParams {
//     view_mat: Mat4,
//     eye: Vec3,
//     _pad0: Pad<4>,
//     resolution: UVec2,
//     grid_size: UVec2,
//     neg_z_depth: f32,
//     cone_multiplier: f32,
//     _pad1: Pad<8>,
// }

pub struct SDFRender {
    linear_sampler: wgpu::Sampler,
    texture: wgpu::TextureView,
    resolution: PhysicalSize<u32>,
    // starting_depth_buffer: wgpu::Buffer,
    testing_tape: wgpu::Buffer,
    matrices: wgpu::Buffer,

    // cone_trace_bgl: wgpu::BindGroupLayout,
    // cone_trace_bg: wgpu::BindGroup,
    // cone_trace_pipeline: wgpu::ComputePipeline,
    sdf_final_bgl: wgpu::BindGroupLayout,
    sdf_final_bg: wgpu::BindGroup,
    sdf_final_pipeline: wgpu::ComputePipeline,

    blit_bgl: wgpu::BindGroupLayout,
    blit_bg: wgpu::BindGroup,
    blit_pipeline: wgpu::RenderPipeline,
}

impl SDFRender {
    pub fn new(
        device: &wgpu::Device,
        initial_size: PhysicalSize<u32>,
        swapchain_format: wgpu::TextureFormat,
    ) -> Self {
        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        let texture = create_texture(device, initial_size);

        let matrices = &[
            Mat4::identity(),
            Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)),
        ];

        let matrices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsage::STORAGE,
            contents: unsafe {
                slice::from_raw_parts(
                    matrices.as_ptr() as *const u8,
                    matrices.len() * mem::size_of::<Mat4>(),
                )
            },
        });

        let tape = &[
            Inst::make(
                0,
                RectangularPrism {
                    matrix_idx: 0,
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            ),
            // Inst::make(0, Sphere {
            //     matrix_idx: 1,
            //     radius: 0.7,
            // }),
            // Inst::make(0, SmoothUnion { k: 0.5 }),
            Inst::make(0, Ret),
        ];

        let testing_tape = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsage::STORAGE,
            contents: unsafe {
                slice::from_raw_parts(
                    tape.as_ptr() as *const u8,
                    tape.len() * mem::size_of::<Inst>(),
                )
            },
        });

        // let starting_depth_buffer = create_starting_depth_buffer(device, initial_size);

        // let (cone_trace_bgl, cone_trace_pipeline) = create_cone_trace_components(device);
        // let cone_trace_bg = create_cone_trace_bind_group(device, &cone_trace_bgl, &starting_depth_buffer);

        let (sdf_final_bgl, sdf_final_pipeline) = create_sdf_final_components(device);
        let sdf_final_bg =
            create_sdf_final_bind_group(device, &sdf_final_bgl, &texture, &testing_tape, &matrices);

        let (blit_bgl, blit_pipeline) = create_blit_components(device, swapchain_format);
        let blit_bg = create_blit_bind_group(device, &texture, &blit_bgl, &linear_sampler);

        Self {
            linear_sampler,
            texture,
            resolution: initial_size,
            // starting_depth_buffer,
            testing_tape,
            matrices,

            // cone_trace_bgl,
            // cone_trace_bg,
            // cone_trace_pipeline,
            sdf_final_bgl,
            sdf_final_bg,
            sdf_final_pipeline,

            blit_bgl,
            blit_bg,
            blit_pipeline,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: PhysicalSize<u32>) {
        self.texture = create_texture(device, new_size);
        self.resolution = new_size;

        // self.starting_depth_buffer = create_starting_depth_buffer(device, self.resolution);

        // self.cone_trace_bg = create_cone_trace_bind_group(device, &self.cone_trace_bgl, &self.starting_depth_buffer);

        self.sdf_final_bg = create_sdf_final_bind_group(
            device,
            &self.sdf_final_bgl,
            &self.texture,
            &self.testing_tape,
            &self.matrices,
        );

        self.blit_bg =
            create_blit_bind_group(device, &self.texture, &self.blit_bgl, &self.linear_sampler);
    }

    pub fn render(
        &self,
        camera: &dyn Camera,
        light: Vec3,
        field_of_view: f32,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let field_of_view = field_of_view.to_radians();
        let view_mat = camera.matrix().transposed();
        let eye = camera.eye();
        let resolution = UVec2::new(self.resolution.width, self.resolution.height);
        let neg_z_depth = -(self.resolution.width as f32 / (field_of_view / 2.0).tan());
        let grid_size = resolution.map(|t| t + 64 - 1) / 64;

        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

            // // Prerender Cone Tracing
            // let cone_trace_params = ConeTracingParams {
            //     view_mat,
            //     eye,
            //     resolution,
            //     grid_size,
            //     neg_z_depth,
            //     cone_multiplier: {
            //         let vertical_fov = field_of_view * (self.resolution.height as f32 / self.resolution.width as f32);

            //         let s = Vec2::new(
            //             (field_of_view / 2.0).tan() * 1.0 / self.resolution.width as f32,
            //             (vertical_fov / 2.0).tan() * 1.0 / self.resolution.height as f32,
            //         ).mag();
            //         1.0 / (1.0 + s)
            //     },
            //     ..Default::default()
            // };

            // cpass.set_pipeline(&self.cone_trace_pipeline);
            // cpass.set_push_constants(0, unsafe {
            //     slice::from_raw_parts(
            //         &cone_trace_params as *const _ as *const u8,
            //         mem::size_of::<ConeTracingParams>(),
            //     )
            // });
            // cpass.set_bind_group(0, &self.cone_trace_bg, &[]);
            // cpass.dispatch(
            //     (self.resolution.width + 64 * 8 - 1) / (64 * 8),
            //     (self.resolution.height + 64 * 8 - 1) / (64 * 8),
            //     1,
            // );

            // SDF Rendering
            let render_params = RenderParams {
                view_mat,
                eye,
                resolution,
                light,
                neg_z_depth,
                grid_size,
                ..Default::default()
            };

            cpass.set_pipeline(&self.sdf_final_pipeline);
            cpass.set_push_constants(0, unsafe {
                slice::from_raw_parts(
                    &render_params as *const _ as *const u8,
                    mem::size_of::<RenderParams>(),
                )
            });
            cpass.set_bind_group(0, &self.sdf_final_bg, &[]);
            cpass.dispatch(
                (self.resolution.width + 8 - 1) / 8,
                (self.resolution.height + 8 - 1) / 8,
                1,
            );
        }
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.blit_pipeline);
            rpass.set_push_constants(
                wgpu::ShaderStage::FRAGMENT,
                0,
                UVec2::new(self.resolution.width, self.resolution.height).as_byte_slice(),
            );
            rpass.set_bind_group(0, &self.blit_bg, &[]);
            rpass.draw(0..3, 0..1);
        }
    }
}

fn create_blit_bind_group(
    device: &wgpu::Device,
    texture: &wgpu::TextureView,
    layout: &wgpu::BindGroupLayout,
    linear_sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(linear_sampler),
            },
        ],
    })
}

fn create_blit_components(
    device: &wgpu::Device,
    texture_format: wgpu::TextureFormat,
) -> (wgpu::BindGroupLayout, wgpu::RenderPipeline) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    filtering: false,
                    comparison: false,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStage::FRAGMENT,
            range: 0..mem::size_of::<UVec2>() as u32,
        }],
    });

    let vertex_shader =
        device.create_shader_module(&include_spirv_shader!(env!("spirv://blit::vertex")));
    let fragment_shader =
        device.create_shader_module(&include_spirv_shader!(env!("spirv://blit::fragment")));

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "blit::vertex",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "blit::fragment",
            targets: &[texture_format.into()],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
    });

    (bind_group_layout, render_pipeline)
}

fn create_texture(device: &wgpu::Device, size: PhysicalSize<u32>) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: STORAGE_TEXTURE_FORMAT,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::STORAGE,
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}

// fn create_starting_depth_buffer(device: &wgpu::Device, resolution: PhysicalSize<u32>) -> wgpu::Buffer {
//     let length = (resolution.width as usize * resolution.height as usize + 64 * 64 - 1) / (64 * 64);

//     device.create_buffer(&wgpu::BufferDescriptor {
//         label: None,
//         size: (length * mem::size_of::<f32>()) as u64,
//         usage: wgpu::BufferUsage::STORAGE,
//         mapped_at_creation: false,
//     })
// }

// fn create_cone_trace_components(
//     device: &wgpu::Device,
// ) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
//     let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//         label: None,
//         entries: &[
//             wgpu::BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: wgpu::ShaderStage::COMPUTE,
//                 ty: wgpu::BindingType::Buffer {
//                     ty: wgpu::BufferBindingType::Storage { read_only: false },
//                     has_dynamic_offset: false,
//                     min_binding_size: None,
//                 },
//                 count: None,
//             },
//         ],
//     });

//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: None,
//         bind_group_layouts: &[&bind_group_layout],
//         push_constant_ranges: &[wgpu::PushConstantRange {
//             stages: wgpu::ShaderStage::COMPUTE,
//             range: 0..mem::size_of::<ConeTracingParams>() as u32,
//         }],
//     });

//     let shader = device.create_shader_module(&include_spirv!(env!("spirv://compute_renderer::prerender_cone_trace")));

//     let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//         label: None,
//         layout: Some(&pipeline_layout),
//         module: &shader,
//         entry_point: "compute_renderer::prerender_cone_trace",
//     });

//     (bind_group_layout, pipeline)
// }

// fn create_cone_trace_bind_group(
//     device: &wgpu::Device,
//     layout: &wgpu::BindGroupLayout,
//     starting_depth_buffer: &wgpu::Buffer,
// ) -> wgpu::BindGroup {
//     device.create_bind_group(&wgpu::BindGroupDescriptor {
//         label: None,
//         layout,
//         entries: &[
//             wgpu::BindGroupEntry {
//                 binding: 0,
//                 resource: wgpu::BindingResource::Buffer {
//                     buffer: &starting_depth_buffer,
//                     offset: 0,
//                     size: None,
//                 },
//             },
//         ],
//     })
// }

fn create_sdf_final_components(
    device: &wgpu::Device,
) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: STORAGE_TEXTURE_FORMAT,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStage::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStage::COMPUTE,
            range: 0..mem::size_of::<RenderParams>() as u32,
        }],
    });

    let shader = device.create_shader_module(&include_spirv_shader!(env!(
        "spirv://compute_renderer::render_sdf_final"
    )));

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "compute_renderer::render_sdf_final",
    });

    (bind_group_layout, pipeline)
}

fn create_sdf_final_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::TextureView,
    tape: &wgpu::Buffer,
    matrices: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer {
                    buffer: tape,
                    offset: 0,
                    size: None,
                },
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Buffer {
                    buffer: matrices,
                    offset: 0,
                    size: None,
                },
            },
        ],
    })
}
