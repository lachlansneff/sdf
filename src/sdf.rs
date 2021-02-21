use std::mem;

use crevice::std140::{AsStd140, Std140 as _};
use ultraviolet::{Vec2, Vec3};
use winit::dpi::PhysicalSize;

use crate::camera::Camera;

#[macro_export]
macro_rules! include_spirv {
    ($name:literal) => {
        wgpu::include_spirv!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

#[derive(AsStd140)]
struct ShaderUniforms {
    matrix: mint::ColumnMatrix4<f32>,
    eye: mint::Vector3<f32>,
    resolution: mint::Vector2<f32>,
    z_depth: f32,
    light_pos: mint::Vector3<f32>,
}

pub struct SDFRender {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniforms: wgpu::Buffer,
}

impl SDFRender {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        // let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        //     label: None,
        //     source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/sdf.wgsl"))),
        //     flags: wgpu::ShaderFlags::all(),
        // });

        let vert_shader = device.create_shader_module(&include_spirv!("sdf.vert"));
        let frag_shader = device.create_shader_module(&include_spirv!("sdf.frag"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(0),
                },
                count: None,
            }],
        });

        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
            size: mem::size_of::<<ShaderUniforms as AsStd140>::Std140Type>() as u64,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "main",
                targets: &[swapchain_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        Self {
            render_pipeline,
            bind_group,
            uniforms,
        }
    }

    pub fn write_uniforms(
        &self,
        queue: &wgpu::Queue,
        camera: &dyn Camera,
        light: Vec3,
        resolution: PhysicalSize<u32>,
        fov: f32,
    ) {
        let eye = camera.eye();
        let matrix = camera.matrix();
        let resolution: Vec2 = Vec2::new(resolution.width as f32, resolution.height as f32);

        let uniforms = ShaderUniforms {
            matrix: matrix.transposed().into(),
            eye: eye.into(),
            resolution: resolution.into(),
            z_depth: resolution.y / (fov.to_radians() / 2.0).tan(),
            light_pos: light.into(),
        };

        queue.write_buffer(&self.uniforms, 0, uniforms.as_std140().as_bytes())
    }

    pub fn render<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}
