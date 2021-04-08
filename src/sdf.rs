use core::slice;
use std::mem;
use ultraviolet::{Mat4, UVec2, Vec2, Vec3};
use winit::dpi::PhysicalSize;

use crate::camera::Camera;

const STORAGE_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

struct Pad<const N: usize>([u8; N]);

impl<const N: usize> Default for Pad<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

#[derive(Default)]
#[repr(C)]
struct ViewParams {
    matrix: Mat4,
    eye: Vec3,
    _pad0: Pad<4>,
    light: Vec3,
    _pad1: Pad<4>,
    resolution: Vec2,
    z_depth: f32,
    _pad2: Pad<4>,
}

pub struct SDFRender {
    linear_sampler: wgpu::Sampler,
    texture: wgpu::TextureView,
    resolution: PhysicalSize<u32>,

    sdf_final_bind_group_layout: wgpu::BindGroupLayout,
    sdf_final_bind_group: wgpu::BindGroup,
    sdf_final_pipeline: wgpu::ComputePipeline,

    blit_bind_group_layout: wgpu::BindGroupLayout,
    blit_bind_group: wgpu::BindGroup,
    blit_pipeline: wgpu::RenderPipeline,

    view_params: ViewParams,
}

impl SDFRender {
    pub fn new(
        device: &wgpu::Device,
        initial_size: PhysicalSize<u32>,
        swapchain_format: wgpu::TextureFormat,
    ) -> Self {
        println!("loading \"{}\"", env!("sdf_shader.spv"));
        let shader = device.create_shader_module(&wgpu::include_spirv!(env!("sdf_shader.spv")));

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        let texture = create_texture(device, initial_size);

        let (blit_bind_group_layout, blit_pipeline) =
            create_blit_components(device, &shader, swapchain_format);
        let blit_bind_group =
            create_blit_bind_group(device, &texture, &blit_bind_group_layout, &linear_sampler);

        let (sdf_final_bind_group_layout, sdf_final_pipeline) =
            create_sdf_final_components(device, &shader);
        let sdf_final_bind_group =
            create_sdf_final_bind_group(device, &texture, &sdf_final_bind_group_layout);

        Self {
            linear_sampler,
            texture,
            resolution: initial_size,

            sdf_final_bind_group_layout,
            sdf_final_bind_group,
            sdf_final_pipeline,

            blit_bind_group_layout,
            blit_bind_group,
            blit_pipeline,

            view_params: Default::default(),
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: PhysicalSize<u32>) {
        self.texture = create_texture(device, new_size);
        self.resolution = new_size;

        self.sdf_final_bind_group =
            create_sdf_final_bind_group(device, &self.texture, &self.sdf_final_bind_group_layout);

        self.blit_bind_group = create_blit_bind_group(
            device,
            &self.texture,
            &self.blit_bind_group_layout,
            &self.linear_sampler,
        );
    }

    pub fn set_camera(&mut self, camera: &dyn Camera, light: Vec3, fov: f32) {
        let eye = camera.eye();
        let matrix = camera.matrix();
        let resolution: Vec2 =
            Vec2::new(self.resolution.width as f32, self.resolution.height as f32);

        self.view_params = ViewParams {
            matrix: matrix.transposed(),
            eye,
            resolution,
            light,
            z_depth: resolution.y / (fov.to_radians() / 2.0).tan(),
            ..Default::default()
        };
    }

    pub fn render(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

            cpass.set_pipeline(&self.sdf_final_pipeline);
            cpass.set_push_constants(0, unsafe {
                slice::from_raw_parts(
                    &self.view_params as *const _ as *const u8,
                    mem::size_of::<ViewParams>(),
                )
            });
            cpass.set_bind_group(0, &self.sdf_final_bind_group, &[]);
            cpass.dispatch(
                (self.resolution.width + 7) / 8,
                (self.resolution.height + 7) / 8,
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
            rpass.set_bind_group(0, &self.blit_bind_group, &[]);
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
    shader: &wgpu::ShaderModule,
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

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "blit::vertex",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
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

fn create_sdf_final_components(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            // wgpu::BindGroupLayoutEntry {
            //     binding: 0,
            //     visibility: wgpu::ShaderStage::COMPUTE,
            //     ty: wgpu::BindingType::Buffer {
            //         ty: wgpu::BufferBindingType::Storage { read_only: true },
            //         has_dynamic_offset: false,
            //         min_binding_size: None,
            //     },
            //     count: None,
            // },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: STORAGE_TEXTURE_FORMAT,
                    view_dimension: wgpu::TextureViewDimension::D2,
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
            range: 0..mem::size_of::<ViewParams>() as u32,
        }],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: shader,
        entry_point: "compute_renderer::render_sdf_final",
    });

    (bind_group_layout, pipeline)
}

fn create_sdf_final_bind_group(
    device: &wgpu::Device,
    texture: &wgpu::TextureView,
    layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout,
        entries: &[
            // wgpu::BindGroupEntry {
            //     binding: 0,
            //     resource: wgpu::BindingResource::Buffer {
            //         buffer: &tape_buffer,
            //         offset: 0,
            //         size: None,
            //     },
            // },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(texture),
            },
        ],
    })
}
