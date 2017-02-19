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

use std::sync::Arc;
use std::time::Duration;

mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/bin/chess_vs.glsl")} }
mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/bin/chess_fs.glsl")} }

mod data;
mod model;
mod chess;
mod system;
mod graphics;

use model::Model;
use system::System;
use graphics::{GraphicsEngine, Matrices};

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

fn main() {
    // Set up lots of stuff ... see vulkano examples
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
    let camera = cgmath::Point3::new(0.0, 5.5, 0.0);
    let view = cgmath::Matrix4::look_at(camera, cgmath::Point3::new(0.0, 0.0, 0.0), cgmath::Vector3::new(0.0, 0.0, 1.0));

    let uniform_buffer = vulkano::buffer::cpu_access::CpuAccessibleBuffer::<vs::ty::Data>
                               ::from_data(&device, &vulkano::buffer::BufferUsage::all(), Some(queue.family()),
                                vs::ty::Data {
                                    world : <cgmath::Matrix4<f32> as cgmath::SquareMatrix>::identity().into(),
                                    view : view.into(),
                                    proj : proj.into(),
                                    camera: camera.into()
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

    // Initialize chess fields and store their positions
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

    // Construct graphics object
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

    // Construct communicator between game and graphics
    let mut system = System::new();

    // Render loop
    loop {
        submissions.retain(|s| s.destroying_would_block());

        let image_num = graphics.swapchain.acquire_next_image(Duration::from_millis(1)).unwrap();

        for index in 0..graphics.command_buffers.len() {
            submissions.push(vulkano::command_buffer::submit(&graphics.command_buffers[index][image_num], &graphics.queue).unwrap());
        }
        graphics.swapchain.present(&graphics.queue, image_num).unwrap();

        // If there is an AI, let it make a move and update figures
        if system.has_ai() {
            if let Some(result) = system.execute_ai_turn() {
                graphics.move_figure((result.0).0, (result.0).1, (result.0).2);
                if result.1 {
                    graphics.delete_figure(!((result.0).0), (result.0).2);
                }
                if system.upgrade_needed() {
                    graphics.upgrade_pawn(system.upgrade().unwrap());
                }
                graphics.update_command_buffers(&white_fields, &black_fields, &pipeline, &set, &framebuffers, &renderpass);
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }

        // Window events
        for ev in window.window().poll_events() {
            match ev {
                // Window was closed
                winit::Event::Closed => return,
                // Keyboard input
                winit::Event::KeyboardInput(winit::ElementState::Pressed, _, Some(the_key)) => {
                    match the_key {
                        // On escape, reset the selection in system
                        winit::VirtualKeyCode::Escape => system.reset_selection(),
                        // Toggle black player AI
                        winit::VirtualKeyCode::Q => system.toggle_player_ai(false),
                        // Toggle white player AI
                        winit::VirtualKeyCode::W => system.toggle_player_ai(true),
                        // Set camera position and update view matrix
                        _ =>
                        if the_key == winit::VirtualKeyCode::Key1 || the_key == winit::VirtualKeyCode::Key2 {
                            let (cam, up) = {
                                if the_key == winit::VirtualKeyCode::Key1 {
                                    (cgmath::Point3::new(4.0, 0.6, 0.0), cgmath::Vector3::new(0.0, -1.0, 0.0))
                                } else {
                                    (cgmath::Point3::new(0.0, 5.5, 0.0), cgmath::Vector3::new(0.0, 0.0, 1.0))
                                }
                            };
                            graphics.set_camera_position(cam);
                            {
                                let mut buffer_content = uniform_buffer.write(Duration::new(1, 0)).unwrap();

                                buffer_content.view = cgmath::Matrix4::look_at(cam, cgmath::Point3::new(0.0, 0.0, 0.0), up).into();
                                buffer_content.camera = cam.into();
                            }
                        }
                    }
                },
                // Update mouse coordinates in System
                winit::Event::MouseMoved(x, y) => system.set_mouse_coordinates(x, y),
                // If a figure was selected, set position as selected in System
                winit::Event::MouseInput(winit::ElementState::Pressed, winit::MouseButton::Left) => {
                    if let Some(selection) = graphics.get_field(system.mouse()) {
                        system.set_selected(selection);
                        // If two selections were made try to execute a turn and update graphics according to the turn
                        if let Some(result) = system.check_ready_and_play() {
                            graphics.move_figure((result.0).0, (result.0).1, (result.0).2);
                            if result.1 {
                                graphics.delete_figure(!((result.0).0), (result.0).2);
                            }
                            if system.upgrade_needed() {
                                graphics.upgrade_pawn(system.upgrade().unwrap());
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
