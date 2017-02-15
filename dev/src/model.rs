use vulkano::device::{Device, Queue};
use vulkano::buffer::cpu_access::{CpuAccessibleBuffer};

use std::sync::Arc;

use data::{Vertex, Normal};
use chess::logic::Color::{self, Black, White};

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

    pub fn color_buffer(&self, c: Color, dev: &Arc<Device>, q: &Arc<Queue>) -> Arc<CpuAccessibleBuffer<(f32, f32, f32)>> {
        let color = match c {
            Black => (0.0, 0.0, 0.0),
            _ => (1.0, 1.0, 1.0)
        };
        super::vulkano::buffer::cpu_access::CpuAccessibleBuffer
               ::from_data(dev, &super::vulkano::buffer::BufferUsage::all(), Some(q.family()), color)
                     .expect("failed to create model color buffer")
    }

    pub fn translate(&mut self, direction: (f32, f32, f32)) {
        self.vertices = self.vertices.iter()
                                     .map(|x| Vertex{ position:
                                                (x.position.0 + direction.0,
                                                 x.position.1 + direction.1,
                                                 x.position.2 + direction.2)})
                                     .collect();
    }

}

