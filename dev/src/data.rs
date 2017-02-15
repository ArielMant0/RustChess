// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
}

impl_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

impl_vertex!(Normal, normal);

pub const FIELD_V: [Vertex; 8] = [
    Vertex { position: (0.5, 0.0, -0.5) },
    Vertex { position: (0.5, 0.0, 0.5) },
    Vertex { position: (-0.5, 0.0, 0.5) },
    Vertex { position: (-0.5, 0.0, -0.5) },
    Vertex { position: (0.5, 0.1, -0.5) },
    Vertex { position: (0.5, 0.1, 0.5) },
    Vertex { position: (-0.5, 0.1, 0.5) },
    Vertex { position: (-0.5, 0.1, -0.5) }
];

pub const FIELD_N: [Normal; 6] = [
    Normal { normal: (0.0, -1.0, 0.0) },
    Normal { normal: (0.0, 1.0, 0.0) },
    Normal { normal: (1.0, 0.0, 0.0) },
    Normal { normal: (0.0, 0.0, 1.0) },
    Normal { normal: (-1.0, 0.0, 0.0) },
    Normal { normal: (0.0, 0.0, -1.0) }
];

pub const FIELD_I: [u16; 36] = [
    0, 1, 2,
    2, 3, 0,
    0, 1, 5,
    5, 4, 0,
    4, 5, 6,
    6, 7, 4,
    1, 2, 6,
    6, 5, 1,
    2, 6, 7,
    7, 3, 2,
    3, 7, 4,
    4, 0, 3
];
