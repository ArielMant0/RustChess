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

use vulkano::device::{Device, Queue};
use vulkano::swapchain::{Swapchain};
use vulkano::pipeline::{GraphicsPipeline};
use vulkano::framebuffer::{Framebuffer};
use vulkano::command_buffer::{PrimaryCommandBuffer};

use cgmath::{Point3, Vector3, Matrix4, SquareMatrix, InnerSpace, Transform};

use std::sync::Arc;

use pipeline_layout;
use renderpass;
use system::System;

use chess::logic::{Color, Position};
use model::Model;
use data::{Vertex, Normal, pawn, king, queen, bishop, knight, rook};

/// Pipeline matrices
pub struct Matrices {
    pub world: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub proj: Matrix4<f32>
}

/// Handles model drawing
pub struct GraphicsEngine {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub swapchain: Arc<Swapchain>,
    pub command_buffers: Arc<Vec<Vec<Arc<PrimaryCommandBuffer>>>>,
    pub field_positions: Arc<Vec<Vec<Point3<f32>>>>,
    pub white_figures: Arc<Vec<(Model, Point3<f32>)>>,
    pub black_figures: Arc<Vec<(Model, Point3<f32>)>>,
    pub uniform: Matrices,
    pub screenwidth: u32,
    pub screenheight: u32,
    pub camera: Point3<f32>
}

impl GraphicsEngine {

    /* Not sure if this will ever be needed
    pub fn add_command_buffer(&mut self, cmd_buf: Vec<Arc<PrimaryCommandBuffer>>) {
        Arc::get_mut(&mut self.command_buffers).unwrap().push(cmd_buf);
    }*/

    /// Add the field center's positions
    pub fn add_field_centers(&mut self, centers: Vec<Point3<f32>>) {
        Arc::get_mut(&mut self.field_positions).unwrap().push(centers);
    }

    /// Set camera position
    pub fn set_camera_position(&mut self, pos: Point3<f32>) {
        self.camera = pos;

        match (pos.x, pos.y, pos.z) {
            (0.0, 5.5, 0.0) => self.uniform.view = Matrix4::look_at(pos,
                                                   Point3::new(0.0, 0.0, 0.0),
                                                   Vector3::new(0.0, 0.0, 1.0)),
            (4.0, 0.6, 0.0) => self.uniform.view = Matrix4::look_at(pos,
                                                   Point3::new(0.0, 0.0, 0.0),
                                                   Vector3::new(0.0, -1.0, 0.0)),
            _ => unreachable!()
        }
    }

    /// Translate a figure to given position
    pub fn move_figure(&mut self, color: Color, from: Position, to: Position) {
        if color == Color::White {
            for mut f in Arc::get_mut(&mut self.white_figures).unwrap() {
                let before = System::from_position(&from);
                if f.1 == before {
                    f.1 = System::from_position(&to);
                    let xdir = f.1.x - before.x;
                    let zdir = f.1.z - before.z;
                    (f.0).translate((xdir, 0.0, zdir));
                }
            }
        } else {
            for mut f in Arc::get_mut(&mut self.black_figures).unwrap() {
                let before = System::from_position(&from);
                if f.1 == before {
                    f.1 = System::from_position(&to);
                    let xdir = f.1.x - before.x;
                    let zdir = f.1.z - before.z;
                    (f.0).translate((xdir, 0.0, zdir));
                }
            }
        }
    }

    /// Change the model of a pawn that was upgraded
    pub fn upgrade_pawn(&mut self, up: (Color, Position)) {
        if up.0 == Color::White {
            for mut f in Arc::get_mut(&mut self.white_figures).unwrap() {
                let at = System::from_position(&up.1);
                if f.1 == at {
                    f.0 = Model::from_data(&queen::VERTICES, &queen::NORMALS, &queen::INDICES);
                    f.0.translate((at.x, 0.1, -3.5));
                }
            }
        } else {
            for mut f in Arc::get_mut(&mut self.black_figures).unwrap() {
                let at = System::from_position(&up.1);
                if f.1 == at {
                    f.0 = Model::from_data(&queen::VERTICES, &queen::NORMALS, &queen::INDICES);
                    f.0.translate((at.x, 0.1, 3.5));
                }
            }
        }
    }

    /// Delete a figure
    pub fn delete_figure(&mut self, color: Color, pos: Position) {
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

    /// Initialize all figures at start positions
    pub fn init_figures(&mut self) {
        for i in 0..8 {
            // Black Pawns
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .push((Model::from_data(&pawn::VERTICES, &pawn::NORMALS, &pawn::INDICES),
                       Point3::new(i as f32 - 3.5, 0.1, -2.5)));
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((i as f32 - 3.5, 0.1, -2.5));
            // White Pawns
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .push((Model::from_data(&pawn::VERTICES, &pawn::NORMALS, &pawn::INDICES),
                       Point3::new(i as f32 - 3.5, 0.1, 2.5)));
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((i as f32 - 3.5, 0.1, 2.5));
        }
        // Black King
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .push((Model::from_data(&king::VERTICES, &king::NORMALS, &king::INDICES),
                   Point3::new(-0.5, 0.1, -3.5)));
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((-0.5, 0.1, -3.5));
        // White King
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .push((Model::from_data(&king::VERTICES, &king::NORMALS, &king::INDICES),
                   Point3::new(-0.5, 0.1, 3.5)));
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((-0.5, 0.1, 3.5));

        // Black Queen
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .push((Model::from_data(&queen::VERTICES, &queen::NORMALS, &queen::INDICES),
                   Point3::new(0.5, 0.1, -3.5)));
        Arc::get_mut(&mut self.black_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((0.5, 0.1, -3.5));
        // White Queen
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .push((Model::from_data(&queen::VERTICES, &queen::NORMALS, &queen::INDICES),
                   Point3::new(0.5, 0.1, 3.5)));
        Arc::get_mut(&mut self.white_figures)
            .unwrap()
            .last_mut().unwrap().0.translate((0.5, 0.1, 3.5));

        for i in 0..2 {
            // Black Knight
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .push((Model::from_data(&knight::VERTICES, &knight::NORMALS, &knight::INDICES),
                       Point3::new(-2.5 + (i as f32 * 5.0), 0.1, -3.5)));
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
                .push((Model::from_data(&knight::VERTICES, &knight::NORMALS, &knight::INDICES),
                       Point3::new(2.5 - (i as f32 * 5.0), 0.1, 3.5)));
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
                .push((Model::from_data(&rook::VERTICES, &rook::NORMALS, &rook::INDICES),
                       Point3::new(-3.5 + (i as f32 * 7.0), 0.1, -3.5)));
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((-3.5 + (i as f32 * 7.0), 0.1, -3.5));
            // White Rook
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .push((Model::from_data(&rook::VERTICES, &rook::NORMALS, &rook::INDICES),
                       Point3::new(3.5 - (i as f32 * 7.0), 0.1, 3.5)));
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((3.5 - (i as f32 * 7.0), 0.1, 3.5));
            // Black Bishop
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .push((Model::from_data(&bishop::VERTICES, &bishop::NORMALS, &bishop::INDICES),
                       Point3::new(-1.5 + (i as f32 * 3.0), 0.1, -3.5)));
            Arc::get_mut(&mut self.black_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((-1.5 + (i as f32 * 3.0), 0.1, -3.5));
            // White Bishop
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .push((Model::from_data(&bishop::VERTICES, &bishop::NORMALS, &bishop::INDICES),
                       Point3::new(1.5 - (i as f32 * 3.0), 0.1, 3.5)));
            Arc::get_mut(&mut self.white_figures)
                .unwrap()
                .last_mut().unwrap().0.translate((1.5 - (i as f32 * 3.0), 0.1, 3.5));
        }
    }

    /// Update command buffers
    pub fn update_command_buffers(&mut self,
        whites: &Vec<Model>,
        blacks: &Vec<Model>,
        pipeline: &Arc<GraphicsPipeline<::vulkano::pipeline::vertex::TwoBuffersDefinition<Vertex, Normal>,
                       pipeline_layout::CustomPipeline,
                       renderpass::CustomRenderPass>>,
        set: &Arc<pipeline_layout::set0::Set>,
        framebuffers: &Vec<Arc<Framebuffer<renderpass::CustomRenderPass>>>,
        renderpass: &Arc<renderpass::CustomRenderPass>)
    {
       // For every framebuffer create a command buffer and start drawing
       let buffers = framebuffers.iter().map(|framebuffer| {
        ::vulkano::command_buffer::PrimaryCommandBufferBuilder::new(&self.device, self.queue.family())
            .draw_inline(renderpass, &framebuffer, renderpass::ClearValues {
                 //color: [0.827, 0.827, 0.827],
                 color: [0.690, 0.769, 0.871],
                 depth: 1.0,
             })
        }).collect::<Vec<_>>();

        // Field and figure colors
        let field_black = ::vs::ty::FigureColor{ col: Vector3::new(0.0, 0.0, 0.0).into() };
        let field_white = ::vs::ty::FigureColor{ col: Vector3::new(1.0, 1.0, 1.0).into() };
        let white = ::vs::ty::FigureColor{ col: Vector3::new(0.9, 0.9, 0.9).into() };
        let black = ::vs::ty::FigureColor{ col: Vector3::new(0.15, 0.15, 0.15).into() };

        // For all command buffers record drawing commands for all fields and figures
        // TODO: optimize so we don't have to construct every buffer anew
        let mut fields = Vec::new();
        for mut buf in buffers {
            for index in 0..whites.len() {
                buf = buf.draw_indexed(pipeline, (&whites[index].vertex_buffer(&self.device, &self.queue),
                                                  &whites[index].normal_buffer(&self.device, &self.queue)),
                                                  &whites[index].index_buffer(&self.device, &self.queue),
                                                  &::vulkano::command_buffer::DynamicState::none(), set, &field_white);
            }
            for index in 0..blacks.len() {
                buf = buf.draw_indexed(pipeline, (&blacks[index].vertex_buffer(&self.device, &self.queue),
                                                  &blacks[index].normal_buffer(&self.device, &self.queue)),
                                                  &blacks[index].index_buffer(&self.device, &self.queue),
                                                  &::vulkano::command_buffer::DynamicState::none(), set, &field_black);
            }
            for index in 0..self.white_figures.len() {
                buf = buf.draw_indexed(pipeline, (&self.white_figures[index].0.vertex_buffer(&self.device, &self.queue),
                                                  &self.white_figures[index].0.normal_buffer(&self.device, &self.queue)),
                                                  &self.white_figures[index].0.index_buffer(&self.device, &self.queue),
                                                  &::vulkano::command_buffer::DynamicState::none(), set, &white);

            }
            for index in 0..self.black_figures.len() {
                buf = buf.draw_indexed(pipeline, (&self.black_figures[index].0.vertex_buffer(&self.device, &self.queue),
                                                  &self.black_figures[index].0.normal_buffer(&self.device, &self.queue)),
                                                  &self.black_figures[index].0.index_buffer(&self.device, &self.queue),
                                                  &::vulkano::command_buffer::DynamicState::none(), set, &black);

            }
            fields.push(buf.draw_end().build());
        }
        let cmd = Arc::get_mut(&mut self.command_buffers).unwrap();
        cmd.clear();
        cmd.push(fields);
    }

    /// Get the field in board coordinates that the player selected using a ray intersection test
    pub fn get_field(&self, mouse: (i32, i32)) -> Option<(u8, u8)> {
        // Transform mouse coordinates
        let mut x = (( 2.0 * mouse.0 as f32) / self.screenwidth as f32) - 1.0;
        let mut y = (((2.0 * mouse.1 as f32) / self.screenheight as f32) - 1.0) * -1.0;

        x = x / self.uniform.proj.x.x;
        y = y / self.uniform.proj.y.y;

        if let Some(inverse) = self.uniform.view.invert() {
            let direction = Vector3{ x: (x * inverse.x.x) + (y * inverse.y.x) + inverse.z.x,
                                             y: (x * inverse.x.y) + (y * inverse.y.y) + inverse.z.y,
                                             z: (x * inverse.x.z) + (y * inverse.y.z) + inverse.z.z };

            for i in 0..self.field_positions.len() {
                for index in 0..self.field_positions[i].len() {
                    let mut world = self.uniform.world.clone();
                    // Translate to field's position
                    let translation = Matrix4::from_translation(Vector3 {
                                                            x: self.field_positions[i][index].x,
                                                            y: self.field_positions[i][index].y,
                                                            z: self.field_positions[i][index].z
                                                          });
                    world = world * translation;

                    let inverse_world = world.invert().unwrap();
                    // Calculate ray direction
                    let mut ray_direction = inverse_world.transform_vector(direction.normalize());
                    ray_direction = ray_direction.normalize();
                    // Calculate ray origin
                    let ray_origin = inverse_world.transform_point(self.camera);

                    // If we intersect return field coordinates in board coordinates
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

    /// Maps a field position in the 3D world to a board position
    pub fn map_field_positions(&self, i: usize, j: usize) -> (u8, u8) {
        (((self.field_positions[i][j].x + 4.0) as i32) as u8,
        ((4.0 - self.field_positions[i][j].z) as i32) as u8)
    }

    /// Ray intersection test with a sphere
    pub fn ray_intersect(origin: &Point3<f32>, direction: &Vector3<f32>) -> bool {
        // TODO: Plane intersection test?
        let a = (direction.x * direction.x) + (direction.y * direction.y) + (direction.z * direction.z);
        let b = ((direction.x * origin.x) + (direction.y * origin.y) + (direction.z * origin.z)) * 2.0;
        let c = ((origin.x * origin.x) + (origin.y * origin.y) + (origin.z * origin.z)) - (0.5 * 0.5);

        let discriminant = (b*b) - (4.0 * a * c);

        return discriminant > 0.0
    }
}
