// MIT License
//
// Copyright (c) 2017 Franziska Becker, René Warking
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
use vulkano::pipeline::{GraphicsPipeline};
use vulkano::framebuffer::{Framebuffer};
use vulkano::command_buffer::{PrimaryCommandBuffer};

mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/bin/teapot_vs.glsl")} }
mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/bin/teapot_fs.glsl")} }

mod data;
mod model;
mod chess;
mod system;

use model::Model;
use system::System;
use chess::logic::{Color, Position};

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

#[allow(dead_code)]
mod pipeline_layout {
    pipeline_layout!{
        set0: {
            uniforms: UniformBuffer<::vs::ty::Data>
        },
        m_color: {
            col: UniformBuffer<::vs::ty::FigureColor>
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
    command_buffers: Arc<Vec<Vec<Arc<PrimaryCommandBuffer>>>>,
    field_positions: Arc<Vec<Vec<cgmath::Point3<f32>>>>,
    white_figures: Arc<Vec<(Model, cgmath::Point3<f32>)>>,
    black_figures: Arc<Vec<(Model, cgmath::Point3<f32>)>>,
    uniform: Matrices,
    screenwidth: u32,
    screenheight: u32,
    camera: cgmath::Point3<f32>
}

impl GraphicsEngine {
    
    /*
    fn add_command_buffer(&mut self, cmd_buf: Vec<Arc<PrimaryCommandBuffer>>) {
        Arc::get_mut(&mut self.command_buffers).unwrap().push(cmd_buf);
    }
    */

    fn add_field_centers(&mut self, centers: Vec<cgmath::Point3<f32>>) {
        Arc::get_mut(&mut self.field_positions).unwrap().push(centers);
    }
    
    fn set_camera_position(&mut self, pos: cgmath::Point3<f32>) {
        self.camera = pos;
        
        match (pos.x, pos.y, pos.z) {
            (0.0, 5.0, 0.0) => self.uniform.view = cgmath::Matrix4::look_at(pos,
                                                   cgmath::Point3::new(0.0, 0.0, 0.0),
                                                   cgmath::Vector3::new(0.0, 0.0, 1.0)),
            (4.0, 0.6, 0.0) => self.uniform.view = cgmath::Matrix4::look_at(pos,
                                                   cgmath::Point3::new(0.0, 0.0, 0.0),
                                                   cgmath::Vector3::new(0.0, -1.0, 0.0)),
            _ => unreachable!()
        }
    }

    fn move_figure(&mut self, color: Color, from: Position, to: Position) {
        if color == Color::White {
            for mut f in Arc::get_mut(&mut self.white_figures).unwrap() {
                if f.1 == System::from_position(&from) {
                    f.1 = System::from_position(&to);
                    let xdir = f.1.x - System::from_position(&from).x;
                    let zdir = f.1.z - System::from_position(&from).z;
                    // println!("translating white figure by {}, 0.0, {}", xdir, zdir);
                    (f.0).translate((xdir, 0.0, zdir));
                }
            }
        } else {
            for mut f in Arc::get_mut(&mut self.black_figures).unwrap() {
                if f.1 == System::from_position(&from) {
                    f.1 = System::from_position(&to);
                    let xdir = f.1.x - System::from_position(&from).x;
                    let zdir = f.1.z - System::from_position(&from).z;
                    // println!("translating black figure by {}, 0.0, {}", xdir, zdir);
                    (f.0).translate((xdir, 0.0, zdir));
                }
            }
        }
    }

    fn delete_figure(&mut self, color: Color, pos: Position) {
        let at = System::from_position(&pos);
        
        if color == Color::White {
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .retain(|f| f.1 != at);
        } else {
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .retain(|f| f.1 != at);
        }
    }

    fn init_figures(&mut self) {
        for i in 0..8 {
            // Black Pawns
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .push((Model::from_data(&data::pawn::VERTICES, &data::pawn::NORMALS, &data::pawn::INDICES),
                       cgmath::Point3::new(i as f32 - 3.5, 0.1, -2.5)));
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((i as f32 - 3.5, 0.1, -2.5));
            // White Pawns
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .push((Model::from_data(&data::pawn::VERTICES, &data::pawn::NORMALS, &data::pawn::INDICES),
                       cgmath::Point3::new(i as f32 - 3.5, 0.1, 2.5)));
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((i as f32 - 3.5, 0.1, 2.5));
        }
        // Black King
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .push((Model::from_data(&data::king::VERTICES, &data::king::NORMALS, &data::king::INDICES),
                   cgmath::Point3::new(-0.5, 0.1, -3.5)));
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((-0.5, 0.1, -3.5));
        // White King
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .push((Model::from_data(&data::king::VERTICES, &data::king::NORMALS, &data::king::INDICES),
                   cgmath::Point3::new(-0.5, 0.1, 3.5)));
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((-0.5, 0.1, 3.5));

        // Black Queen
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .push((Model::from_data(&data::queen::VERTICES, &data::queen::NORMALS, &data::queen::INDICES),
                   cgmath::Point3::new(0.5, 0.1, -3.5)));
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((0.5, 0.1, -3.5));
        // White Queen
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .push((Model::from_data(&data::queen::VERTICES, &data::queen::NORMALS, &data::queen::INDICES),
                   cgmath::Point3::new(0.5, 0.1, 3.5)));
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((0.5, 0.1, 3.5));

        for i in 0..2 {
            // Black Knight
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .push((Model::from_data(&data::knight::VERTICES, &data::knight::NORMALS, &data::knight::INDICES),
                       cgmath::Point3::new(-2.5 + (i as f32 * 5.0), 0.1, -3.5)));
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0
                .rotate_around_y(90.0);
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0
                .translate((-2.5 + (i as f32 * 5.0), 0.1, -3.5));
            // White Knight
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .push((Model::from_data(&data::knight::VERTICES, &data::knight::NORMALS, &data::knight::INDICES),
                       cgmath::Point3::new(2.5 - (i as f32 * 5.0), 0.1, 3.5)));
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0
                .rotate_around_y(-90.0);
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0
                .translate((2.5 - (i as f32 * 5.0), 0.1, 3.5));
            // Black Rook
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .push((Model::from_data(&data::rook::VERTICES, &data::rook::NORMALS, &data::rook::INDICES),
                       cgmath::Point3::new(-3.5 + (i as f32 * 7.0), 0.1, -3.5)));
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((-3.5 + (i as f32 * 7.0), 0.1, -3.5));
            // White Rook
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .push((Model::from_data(&data::rook::VERTICES, &data::rook::NORMALS, &data::rook::INDICES),
                       cgmath::Point3::new(3.5 - (i as f32 * 7.0), 0.1, 3.5)));
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((3.5 - (i as f32 * 7.0), 0.1, 3.5));
            // Black Bishop
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .push((Model::from_data(&data::bishop::VERTICES, &data::bishop::NORMALS, &data::bishop::INDICES),
                       cgmath::Point3::new(-1.5 + (i as f32 * 3.0), 0.1, -3.5)));
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((-1.5 + (i as f32 * 3.0), 0.1, -3.5));
            // White Bishop
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .push((Model::from_data(&data::bishop::VERTICES, &data::bishop::NORMALS, &data::bishop::INDICES),
                       cgmath::Point3::new(1.5 - (i as f32 * 3.0), 0.1, 3.5)));
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((1.5 - (i as f32 * 3.0), 0.1, 3.5));
        }
    }
    
    fn update_command_buffers(&mut self,
        whites: &Vec<Model>,
        blacks: &Vec<Model>,
        pipeline: &Arc<GraphicsPipeline<vulkano::pipeline::vertex::TwoBuffersDefinition<data::Vertex, data::Normal>,
                       pipeline_layout::CustomPipeline,
                       renderpass::CustomRenderPass>>,
        set: &Arc<pipeline_layout::set0::Set>, 
        framebuffers: &Vec<Arc<Framebuffer<renderpass::CustomRenderPass>>>, 
        renderpass: &Arc<renderpass::CustomRenderPass>)
    {
       let buffers = framebuffers.iter().map(|framebuffer| {
        vulkano::command_buffer::PrimaryCommandBufferBuilder::new(&self.device, self.queue.family())
            .draw_inline(renderpass, &framebuffer, renderpass::ClearValues {
                 //color: [0.827, 0.827, 0.827],
                 color: [0.690, 0.769, 0.871],
                 depth: 1.0,
             })
        }).collect::<Vec<_>>();

        let field_black = vs::ty::FigureColor{ col: cgmath::Vector3::new(0.1, 0.1, 0.1).into() };
        let field_white = vs::ty::FigureColor{ col: cgmath::Vector3::new(1.0, 1.0, 1.0).into() };
        let white = vs::ty::FigureColor{ col: cgmath::Vector3::new(0.8, 0.8, 0.8).into() };
        let black = vs::ty::FigureColor{ col: cgmath::Vector3::new(0.25, 0.25, 0.25).into() };
        
        let mut fields = Vec::new();
        for mut buf in buffers {
            for index in 0..whites.len() {
                buf = buf.draw_indexed(pipeline, (&whites[index].vertex_buffer(&self.device, &self.queue),
                                                  &whites[index].normal_buffer(&self.device, &self.queue)),
                                                  &whites[index].index_buffer(&self.device, &self.queue),
                                                  &vulkano::command_buffer::DynamicState::none(), set, &field_white);
            }
            for index in 0..blacks.len() {
                buf = buf.draw_indexed(pipeline, (&blacks[index].vertex_buffer(&self.device, &self.queue),
                                                  &blacks[index].normal_buffer(&self.device, &self.queue)),
                                                  &blacks[index].index_buffer(&self.device, &self.queue),
                                                  &vulkano::command_buffer::DynamicState::none(), set, &field_black);
            }
            for index in 0..self.white_figures.len() {
                buf = buf.draw_indexed(pipeline, (&self.white_figures[index].0.vertex_buffer(&self.device, &self.queue),
                                                  &self.white_figures[index].0.normal_buffer(&self.device, &self.queue)),
                                                  &self.white_figures[index].0.index_buffer(&self.device, &self.queue),
                                                  &vulkano::command_buffer::DynamicState::none(), set, &white);
                
            }
            for index in 0..self.black_figures.len() {
                buf = buf.draw_indexed(pipeline, (&self.black_figures[index].0.vertex_buffer(&self.device, &self.queue),
                                                  &self.black_figures[index].0.normal_buffer(&self.device, &self.queue)),
                                                  &self.black_figures[index].0.index_buffer(&self.device, &self.queue),
                                                  &vulkano::command_buffer::DynamicState::none(), set, &black);
                
            }
            fields.push(buf.draw_end().build());
        }
        let cmd = Arc::get_mut(&mut self.command_buffers).unwrap();
        cmd.clear();
        cmd.push(fields);
    }

    fn get_field(&self, mouse: (i32, i32)) -> Option<(u8, u8)> {
        let mut x = (( 2.0 * mouse.0 as f32) / self.screenwidth as f32) - 1.0;
        let mut y = (((2.0 * mouse.1 as f32) / self.screenheight as f32) - 1.0) * -1.0;

        x = x / self.uniform.proj.x.x;
        y = y / self.uniform.proj.y.y;

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
                    world = world * translation;

                    let inverse_world = world.invert().unwrap();
                    let mut ray_direction = inverse_world.transform_vector(direction.normalize());
                    ray_direction = ray_direction.normalize();
                    let ray_origin = inverse_world.transform_point(self.camera);

                    if GraphicsEngine::ray_intersect(&ray_origin, &ray_direction) {
                        return Some(self.map_field_positions(i, index))
                    }
                }
            }
        } else {
            println!("Could not invert view matrix");
            return None
        }
        None
    }

    fn map_field_positions(&self, i: usize, j: usize) -> (u8, u8) {
        (((self.field_positions[i][j].x + 5.0) as i32) as u8,
         ((5.0 - self.field_positions[i][j].z) as i32) as u8)
    }

    fn ray_intersect(origin: &cgmath::Point3<f32>, direction: &cgmath::Vector3<f32>) -> bool {
        // TODO: Plane intersection test
        let a = (direction.x * direction.x) + (direction.y * direction.y) + (direction.z * direction.z);
        let b = ((direction.x * origin.x) + (direction.y * origin.y) + (direction.z * origin.z)) * 2.0;
        let c = ((origin.x * origin.x) + (origin.y * origin.y) + (origin.z * origin.z)) - (0.5 * 0.5);

        let discriminant = (b*b) - (4.0 * a * c);

        return discriminant > 0.0
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

    let proj = cgmath::perspective(cgmath::Rad(std::f32::consts::FRAC_PI_2),
                                  { let d = images[0].dimensions(); d[0] as f32 / d[1] as f32 }, 0.01, 100.0);
    //let proj = cgmath::ortho(-15.0, 15.0, 15.0, -15.0, 0.01, 100.0);
    let camera = cgmath::Point3::new(0.0, 5.5, 0.0);
    let view = cgmath::Matrix4::look_at(camera, cgmath::Point3::new(0.0, 0.0, 0.0), cgmath::Vector3::new(0.0, 0.0, 1.0));

    let uniform_buffer = vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::Data>
                               ::from_data(&device, &vulkano::buffer::BufferUsage::all(), Some(queue.family()),
                                vs::ty::Data {
                                    world : <cgmath::Matrix4<f32> as cgmath::SquareMatrix>::identity().into(),
                                    view : view.into(),
                                    proj : proj.into()
                                })
                               .expect("failed to create buffer");

    let vs = vs::Shader::load(&device).expect("failed to create shader module");
    let fs = fs::Shader::load(&device).expect("failed to create shader module");

    let renderpass = renderpass::CustomRenderPass::new(&device, &renderpass::Formats {
        color: (images[0].format(), 1),
        depth: (vulkano::format::D16Unorm, 1),
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

    let mut submissions: Vec<Arc<vulkano::command_buffer::Submission>> = Vec::new();

    let mut graphics = GraphicsEngine{ device: device,
                                       queue: queue,
                                       swapchain: swapchain,
                                       command_buffers: Arc::new(Vec::new()),
                                       field_positions: Arc::new(Vec::new()),
                                       uniform: Matrices {
                                           world : <cgmath::Matrix4<f32> as cgmath::SquareMatrix>::identity(),
                                           view : view,
                                           proj : proj,
                                       },
                                       screenwidth: images[0].dimensions()[0],
                                       screenheight: images[0].dimensions()[1],
                                       white_figures: Arc::new(Vec::new()),
                                       black_figures: Arc::new(Vec::new()),
                                       camera: camera };

    graphics.add_field_centers(white_centers);
    graphics.add_field_centers(black_centers);
    graphics.init_figures();
    graphics.update_command_buffers(&white_fields, &black_fields, &pipeline, &set, &framebuffers, &renderpass);

    let mut system = System::new();

    loop {
        submissions.retain(|s| s.destroying_would_block());

        let image_num = graphics.swapchain.acquire_next_image(Duration::from_millis(1)).unwrap();
        
        for index in 0..graphics.command_buffers.len() {
            submissions.push(vulkano::command_buffer::submit(&graphics.command_buffers[index][image_num], &graphics.queue).unwrap());
        }
        graphics.swapchain.present(&graphics.queue, image_num).unwrap();

        for ev in window.window().poll_events() {
            match ev {
                winit::Event::Closed => return,
                winit::Event::KeyboardInput(winit::ElementState::Pressed, _, Some(the_key)) => {
                    let (cam, up) = match the_key {
                        winit::VirtualKeyCode::Key1 => (cgmath::Point3::new(4.0, 0.6, 0.0), cgmath::Vector3::new(0.0, -1.0, 0.0)),
                        _ => (cgmath::Point3::new(0.0, 5.0, 0.0), cgmath::Vector3::new(0.0, 0.0, 1.0))
                    };
                    graphics.set_camera_position(cam);
                    {
                        let mut buffer_content = uniform_buffer.write(Duration::new(1, 0)).unwrap();

                        buffer_content.view = cgmath::Matrix4::look_at(cam, cgmath::Point3::new(0.0, 0.0, 0.0), up).into();
                    }
                },
                // Update mouse coordinates in System
                winit::Event::MouseMoved(x, y) => system.set_mouse_coordinates(x, y),
                // If a figure was selected, set position as selected in System
                winit::Event::MouseInput(winit::ElementState::Pressed, winit::MouseButton::Left) => {
                    if let Some(selection) = graphics.get_field(system.mouse()) {
                        system.set_selected(selection);
                        // Now we update the figures command buffers to match what happened in the game                       
                        if let Some(result) = system.check_ready_and_play() {
                            graphics.move_figure((result.0).0, (result.0).1, (result.0).2);
                            if let Some(f) = result.1 {
                                graphics.delete_figure(f.0, f.1);
                            }
                            graphics.update_command_buffers(&white_fields, &black_fields, &pipeline, &set, &framebuffers, &renderpass);
                        }
                    }
                },
                _ => ()
            }
        }
    }
}
