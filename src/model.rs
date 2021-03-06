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
use vulkano::buffer::cpu_access::{CpuAccessibleBuffer};

use cgmath::{Deg, Transform, Point3, Matrix3};

use std::sync::Arc;

use data::{Vertex, Normal};

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>
}

impl Model {
    pub fn from_data(v: &[Vertex], n: &[Normal], i: &[u16]) -> Self {
        Model{ vertices: v.iter().cloned().collect(),
               normals: n.iter().cloned().collect(),
               indices: i.iter().cloned().collect() }
    }

    pub fn vertex_buffer(&self, dev: &Arc<Device>, q: &Arc<Queue>) -> Arc<CpuAccessibleBuffer<[Vertex]>> {
        super::vulkano::buffer::cpu_access::CpuAccessibleBuffer
               ::from_iter(dev, &super::vulkano::buffer::BufferUsage::all(), Some(q.family()), self.vertices.iter().cloned())
                     .expect("failed to create model vertex buffer")

    }

    pub fn normal_buffer(&self, dev: &Arc<Device>, q: &Arc<Queue>) -> Arc<CpuAccessibleBuffer<[Normal]>> {
        super::vulkano::buffer::cpu_access::CpuAccessibleBuffer
               ::from_iter(dev, &super::vulkano::buffer::BufferUsage::all(), Some(q.family()), self.normals.iter().cloned())
                     .expect("failed to create model normal buffer")

    }

    pub fn index_buffer(&self, dev: &Arc<Device>, q: &Arc<Queue>) -> Arc<CpuAccessibleBuffer<[u16]>> {
        super::vulkano::buffer::cpu_access::CpuAccessibleBuffer
               ::from_iter(dev, &super::vulkano::buffer::BufferUsage::all(), Some(q.family()), self.indices.iter().cloned())
                     .expect("failed to create model index buffer")

    }

    pub fn translate(&mut self, direction: (f32, f32, f32)) {
        self.vertices = self.vertices.iter()
                                     .map(|x| Vertex{ position:
                                                (x.position.0 + direction.0,
                                                 x.position.1 + direction.1,
                                                 x.position.2 + direction.2)})
                                     .collect();
    }

    pub fn rotate_around_y(&mut self, angle: f32) {
        let rotation = Matrix3::from_angle_y(Deg(angle));

        self.vertices = self.vertices.iter()
                                     .map(|x| {
                let vec = rotation.transform_point(Point3::new(x.position.0, x.position.1, x.position.2));
                Vertex{ position: (vec.x, vec.y, vec.z) }
            })
            .collect();
        self.normals = self.normals.iter()
                                   .map(|x| {
                let vec = rotation.transform_point(Point3::new(x.normal.0, x.normal.1, x.normal.2));
                Normal{ normal: (vec.x, vec.y, vec.z) }
            })
            .collect();
    }

}
