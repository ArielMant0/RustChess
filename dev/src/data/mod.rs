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

pub mod pawn;
pub mod king;
pub mod queen;
pub mod bishop;
pub mod knight;
pub mod rook;

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
