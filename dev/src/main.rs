// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

extern crate cgmath;
extern crate winit;
extern crate time;

#[macro_use]
extern crate vulkano;
extern crate vulkano_win;

use vulkano_win::VkSurfaceBuild;

use cgmath::{SquareMatrix, InnerSpace, Transform};

use std::sync::Arc;
use std::time::Duration;

use vulkano::device::{Device, Queue};
use vulkano::swapchain::{Swapchain};
use vulkano::image::swapchain::{SwapchainImage};
use vulkano::buffer::cpu_access::{CpuAccessibleBuffer};
use vulkano::pipeline::{GraphicsPipeline};
use vulkano::framebuffer::{Framebuffer};
use vulkano::command_buffer::{PrimaryCommandBuffer};
use vulkano::pipeline::vertex::{Vertex};

mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/bin/teapot_vs.glsl")} }
mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/bin/teapot_fs.glsl")} }

mod data;
mod model;
mod chess;
mod system;

use model::Model;
use system::System;

mod renderpass {
   single_pass_renderpass!{
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: ::vulkano::format::Format,
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: ::vulkano::format::D16Unorm,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {depth}
        }
    }
}

mod pipeline_layout {
    pipeline_layout!{
        set0: {
            uniforms: UniformBuffer<::vs::ty::Data>
        }
    }
}

struct Matrices {
    pub world: cgmath::Matrix4<f32>,
    pub view: cgmath::Matrix4<f32>,
    pub proj: cgmath::Matrix4<f32>
}

struct GraphicsEngine {
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    //images: Vec<Arc<SwapchainImage>>,
    command_buffers: Arc<Vec<Vec<Arc<PrimaryCommandBuffer>>>>,
    field_positions: Arc<Vec<Vec<cgmath::Point3<f32>>>>,
    uniform: Matrices,
    screenwidth: u32,
    screenheight: u32
}

impl GraphicsEngine {

    fn add_command_buffer(&mut self, cmd_buf: Vec<Arc<PrimaryCommandBuffer>>) {
        Arc::get_mut(&mut self.command_buffers).unwrap().push(cmd_buf);
    }

    fn add_field_centers(&mut self, centers: Vec<cgmath::Point3<f32>>) {
        Arc::get_mut(&mut self.field_positions).unwrap().push(centers);
    }

    fn get_field(&self, mouse: (i32, i32)) -> Option<(u8, u8)> {
        let mut x = (( 2.0 * mouse.0 as f32) / self.screenwidth as f32) - 1.0;
        let mut y = (((2.0 * mouse.1 as f32) / self.screenheight as f32) - 1.0) * -1.0;

        x = x / self.uniform.proj.x.x;
        y = y / self.uniform.proj.y.y;
        /*
        for i in 0..self.field_positions.len() {
            for j in 0..self.field_positions[i].len() {
                if x <= self.field_positions[i][j].x + 0.5 &&
                   x >= self.field_positions[i][j].x - 0.5 &&
                   y <= self.field_positions[i][j].z + 0.5 &&
                   y >= self.field_positions[i][j].z - 0.5
                {
                    println!("Position: {}, {} -- {}, {}", x, y,
                        self.field_positions[i][j].x,
                        self.field_positions[i][j].z);
                    return Some(self.map_field_positions(i, j))
                }
            }
        }*/
        
        if let Some(inverse) = self.uniform.view.invert() {
            let direction = cgmath::Vector3{ x: (x * inverse.x.x) + (y * inverse.y.x) + inverse.z.x,
                                             y: (x * inverse.x.y) + (y * inverse.y.y) + inverse.z.y,
                                             z: (x * inverse.x.z) + (y * inverse.y.z) + inverse.z.z };

            for i in 0..self.field_positions.len() {
                for index in 0..self.field_positions[i].len() {
                    let mut world = self.uniform.world.clone();
                    let translation = cgmath::Matrix4::from_translation(cgmath::Vector3 {
                                                            x: self.field_positions[i][index].x,
                                                            y: self.field_positions[i][index].y,
                                                            z: self.field_positions[i][index].z
                                                          });
                    //let translation = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0));
                    world = world * translation;
                    
                    let inverse_world = world.invert().unwrap();
                    let mut ray_direction = inverse_world.transform_vector(direction.normalize());
                    ray_direction = ray_direction.normalize();
                    let ray_origin = inverse_world.transform_point(cgmath::Point3{ x: 0.0,
                                                                                   y: 7.0,
                                                                                   z: 0.0 });;

                    if GraphicsEngine::ray_intersect(&ray_origin, &ray_direction, &self.field_positions[i][index]) {
                        return Some(self.map_field_positions(i, index))
                    }
                }
            }
        } else {
            println!("Could not invert view matrix");
            return None
        }
        println!("Did not find an intersection");
        None
    }

    fn map_field_positions(&self, i: usize, j: usize) -> (u8, u8) {
        (((self.field_positions[i][j].x + 5.0) as i32) as u8,
         ((5.0 - self.field_positions[i][j].z) as i32) as u8)
    }

    fn ray_intersect(origin: &cgmath::Point3<f32>, direction: &cgmath::Vector3<f32>, center: &cgmath::Point3<f32>) -> bool {
        
        let a = (direction.x * direction.x) + (direction.y * direction.y) + (direction.z * direction.z);
        let b = ((direction.x * origin.x) + (direction.y * origin.y) + (direction.z * origin.z)) * 2.0;
        let c = ((origin.x * origin.x) + (origin.y * origin.y) + (origin.z * origin.z)) - (0.5 * 0.5);
        
        let discriminant = (b*b) -(4.0*a*c);

        if (discriminant < 0.0)
        {
            return false;
        }
        true
        /*let normal = cgmath::Vector3::new(center.x, 1.0, center.z);
        let denom = normal.dot(*direction);
        
        println!("Position: {}, {}",
                        center.x,
                        center.z);

        if denom.abs() > 0.0001 {
            let t = (center - origin).dot(normal) / denom;

            return t >= 0.0
        }
        false*/
    }
}

fn main() {
    let extensions = vulkano_win::required_extensions();
    let instance = vulkano::instance::Instance::new(None, &extensions, None).expect("failed to create instance");

    let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
                            .next().expect("no device available");
    println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

    let window = winit::WindowBuilder::new().build_vk_surface(&instance).unwrap();

    let queue = physical.queue_families().find(|q| q.supports_graphics() &&
                                                   window.surface().is_supported(q).unwrap_or(false))
                                                .expect("couldn't find a graphical queue family");

    let device_ext = vulkano::device::DeviceExtensions {
        khr_swapchain: true,
        .. vulkano::device::DeviceExtensions::none()
    };

    let (device, mut queues) = vulkano::device::Device::new(&physical, physical.supported_features(),
                                                            &device_ext, [(queue, 0.5)].iter().cloned())
                               .expect("failed to create device");
    let queue = queues.next().unwrap();

    let (swapchain, images) = {
        let caps = window.surface().get_capabilities(&physical).expect("failed to get surface capabilities");

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let present = caps.present_modes.iter().next().unwrap();
        let usage = caps.supported_usage_flags;
        let format = caps.supported_formats[0].0;

        vulkano::swapchain::Swapchain::new(&device, &window.surface(), caps.min_image_count, format, dimensions, 1,
                                           &usage, &queue, vulkano::swapchain::SurfaceTransform::Identity,
                                           vulkano::swapchain::CompositeAlpha::Opaque,
                                           present, true, None).expect("failed to create swapchain")
    };


    let depth_buffer = vulkano::image::attachment::AttachmentImage::transient(&device, images[0].dimensions(), vulkano::format::D16Unorm).unwrap();

    let proj = cgmath::perspective(cgmath::Rad(std::f32::consts::FRAC_PI_2), { let d = images[0].dimensions(); d[0] as f32 / d[1] as f32 }, 0.01, 100.0);
    //let proj = cgmath::ortho(-15.0, 15.0, 15.0, -15.0, 0.01, 100.0);
    let view = cgmath::Matrix4::look_at(cgmath::Point3::new(0.0, 7.0, 0.0), cgmath::Point3::new(0.0, 0.0, 0.0), cgmath::Vector3::new(0.0, 0.0, 1.0));
    let scale = cgmath::Matrix4::from_scale(1.0);

    let uniform_buffer = vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::Data>
                               ::from_data(&device, &vulkano::buffer::BufferUsage::all(), Some(queue.family()),
                                vs::ty::Data {
                                    world : <cgmath::Matrix4<f32> as cgmath::SquareMatrix>::identity().into(),
                                    view : (view * scale).into(),
                                    proj : proj.into(),
                                })
                               .expect("failed to create buffer");

    let vs = vs::Shader::load(&device).expect("failed to create shader module");
    let fs = fs::Shader::load(&device).expect("failed to create shader module");

    let renderpass = renderpass::CustomRenderPass::new(&device, &renderpass::Formats {
        color: (images[0].format(), 1),
        depth: (vulkano::format::D16Unorm, 1)
    }).unwrap();

    let descriptor_pool = vulkano::descriptor::descriptor_set::DescriptorPool::new(&device);

    let pipeline_layout = pipeline_layout::CustomPipeline::new(&device).unwrap();
    let set = pipeline_layout::set0::Set::new(&descriptor_pool, &pipeline_layout, &pipeline_layout::set0::Descriptors {
        uniforms: &uniform_buffer
    });

    let pipeline = vulkano::pipeline::GraphicsPipeline::new(&device, vulkano::pipeline::GraphicsPipelineParams {
        vertex_input: vulkano::pipeline::vertex::TwoBuffersDefinition::new(),
        vertex_shader: vs.main_entry_point(),
        input_assembly: vulkano::pipeline::input_assembly::InputAssembly::triangle_list(),
        tessellation: None,
        geometry_shader: None,
        viewport: vulkano::pipeline::viewport::ViewportsState::Fixed {
            data: vec![(
                vulkano::pipeline::viewport::Viewport {
                    origin: [0.0, 0.0],
                    depth_range: 0.0 .. 1.0,
                    dimensions: [images[0].dimensions()[0] as f32, images[0].dimensions()[1] as f32],
                },
                vulkano::pipeline::viewport::Scissor::irrelevant()
            )],
        },
        raster: Default::default(),
        multisample: vulkano::pipeline::multisample::Multisample::disabled(),
        fragment_shader: fs.main_entry_point(),
        depth_stencil: vulkano::pipeline::depth_stencil::DepthStencil::simple_depth_test(),
        blend: vulkano::pipeline::blend::Blend::pass_through(),
        layout: &pipeline_layout,
        render_pass: vulkano::framebuffer::Subpass::from(&renderpass, 0).unwrap(),
    }).unwrap();

    let framebuffers = images.iter().map(|image| {
        let attachments = renderpass::AList {
            color: &image,
            depth: &depth_buffer,
        };

        vulkano::framebuffer::Framebuffer::new(&renderpass, [image.dimensions()[0], image.dimensions()[1], 1], attachments).unwrap()
    }).collect::<Vec<_>>();

    let mut white_fields = Vec::new();
    let mut black_fields = Vec::new();
    let mut white_centers = Vec::new();
    let mut black_centers = Vec::new();
    for outer in 0..8 {
        for i in 0..8 {
            if (outer % 2 == 0 && i % 2 == 1) || (outer % 2 == 1 && i % 2 == 0) {
                white_fields.push(Model::from_data(&data::FIELD_V, &data::FIELD_N, &data::FIELD_I));
                white_fields.last_mut().unwrap().translate((i as f32 - 3.5, 0.0, outer as f32 - 3.5));
                white_centers.push(cgmath::Point3{ x: i as f32 - 3.5,
                                                   y: 0.0,
                                                   z: outer as f32 - 3.5 });
            } else {
                black_fields.push(Model::from_data(&data::FIELD_V, &data::FIELD_N, &data::FIELD_I));
                black_fields.last_mut().unwrap().translate((i as f32 - 3.5, 0.0, outer as f32 - 3.5));
                black_centers.push(cgmath::Point3{ x: i as f32 - 3.5,
                                                   y: 0.0,
                                                   z: outer as f32 - 3.5 });
            }
        }
    }

    let buffers = framebuffers.iter().map(|framebuffer| {
        vulkano::command_buffer::PrimaryCommandBufferBuilder::new(&device, queue.family())
            .draw_inline(&renderpass, &framebuffer, renderpass::ClearValues {
                 color: [0.690, 0.769, 0.871],
                 depth: 1.0,
             })
        }).collect::<Vec<_>>();

    let mut fields = Vec::new();
    for mut elem in buffers {
        for index in 0..white_fields.len() {
            elem = elem.draw_indexed(&pipeline, (&white_fields[index].vertex_buffer(&device, &queue), &white_fields[index].normal_buffer(&device, &queue)),
                                           &white_fields[index].index_buffer(&device, &queue), &vulkano::command_buffer::DynamicState::none(), &set, &());
            elem = elem.draw_indexed(&pipeline, (&black_fields[index].vertex_buffer(&device, &queue), &black_fields[index].normal_buffer(&device, &queue)),
                                           &black_fields[index].index_buffer(&device, &queue), &vulkano::command_buffer::DynamicState::none(), &set, &());
        }
        fields.push(elem.draw_end().build());
    }

    let mut submissions: Vec<Arc<vulkano::command_buffer::Submission>> = Vec::new();

    let mut graphics = GraphicsEngine{ device: device,
                                       queue: queue,
                                       swapchain: swapchain,
                                       command_buffers: Arc::new(Vec::new()),
                                       field_positions: Arc::new(Vec::new()),
                                       uniform: Matrices {
                                           world : <cgmath::Matrix4<f32> as cgmath::SquareMatrix>::identity(),
                                           view : (view * scale),
                                           proj : proj,
                                       },
                                       screenwidth: images[0].dimensions()[0],
                                       screenheight: images[0].dimensions()[1]};
    graphics.add_command_buffer(fields);
    graphics.add_field_centers(white_centers);
    graphics.add_field_centers(black_centers);
    let mut system = System::new();

    loop {
        submissions.retain(|s| s.destroying_would_block());

        /*{
            // aquiring write lock for the uniform buffer
            let mut buffer_content = uniform_buffer.write(Duration::new(1, 0)).unwrap();

            let rotation = cgmath::Matrix3::from_angle_y(cgmath::Rad(time::precise_time_ns() as f32 * 0.000000001));

            // since write lock implementd Deref and DerefMut traits,
            // we can update content directly
            buffer_content.world = cgmath::Matrix4::from(rotation).into();
        }*/

        let image_num = graphics.swapchain.acquire_next_image(Duration::from_millis(1)).unwrap();
        for index in 0..graphics.command_buffers.len() {
            submissions.push(vulkano::command_buffer::submit(&graphics.command_buffers[index][image_num], &graphics.queue).unwrap());
        }
        graphics.swapchain.present(&graphics.queue, image_num).unwrap();

        for ev in window.window().poll_events() {
            match ev {
                winit::Event::Closed => return,
                winit::Event::MouseMoved(x, y) => system.set_mouse_coordinates(x, y),
                winit::Event::MouseInput(winit::ElementState::Pressed, winit::MouseButton::Left) => {
                    if let Some(selection) = graphics.get_field(system.mouse()) {
                        system.set_selected(selection);
                        //system.check_ready_and_play();
                    }
                },
                _ => ()
            }
        }
    }
}
