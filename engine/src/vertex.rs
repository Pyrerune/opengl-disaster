use std::cmp::Ordering;
use crate::consts::*;
#[macro_export]
macro_rules! vertex {
    () => {
        Vertex::default()
    };
    ($position:expr, $normal:expr, $tex_coords:expr) => {
        Vertex::new($position, $tex_coords, $normal)
    }
}
#[derive(Copy, Clone, Debug, Hash)]
pub struct Vertex {
    pub position: [i32; 3],
    pub tex_coords: [i32; 2],
    pub normal: [i32;3],
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
    }
}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}
impl Vertex {
    pub fn new(position: [i32; 3], tex_coords: [i32; 2], normal: [i32;3]) -> Vertex {
        Vertex {
            position,
            tex_coords,
            normal,
        }
    }

}
impl Default for Vertex {
    fn default() -> Self {
        Vertex::new([0;3], [0;2], [0;3])
    }
}
pub fn bottom(center: [i32; 3], normal: &[i32; 3]) -> [Vertex; 6] {
    let top_left = [center[0], center[1], center[2]];
    let top_right = [center[0] + SIZE, center[1], center[2]];
    let bottom_left = [center[0], center[1], center[2] + SIZE];
    let bottom_right = [center[0] + SIZE, center[1], center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(bottom_right, TEX[0], *normal),
    ]
}

pub fn top(center: [i32; 3], normal: &[i32; 3]) -> [Vertex; 6] {
    let top_left = [center[0] + SIZE, center[1] + SIZE, center[2]];
    let top_right = [center[0], center[1] + SIZE, center[2]];
    let bottom_left = [center[0] + SIZE, center[1] + SIZE, center[2] + SIZE];
    let bottom_right = [center[0], center[1] + SIZE, center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(bottom_right, TEX[0], *normal),
    ]
}

pub fn left(center: [i32; 3], normal: &[i32; 3]) -> [Vertex; 6] {
    let top_left = [center[0], center[1], center[2]];
    let top_right = [center[0], center[1], center[2] + SIZE];
    let bottom_left = [center[0], center[1] + SIZE, center[2]];
    let bottom_right = [center[0], center[1] + SIZE, center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(bottom_right, TEX[0], *normal),
    ]
}

pub fn right(center: [i32; 3], normal: &[i32; 3]) -> [Vertex; 6] {
    let top_left = [center[0] + SIZE, center[1], center[2] + SIZE];
    let top_right = [center[0] + SIZE, center[1], center[2]];
    let bottom_left = [center[0] + SIZE, center[1] + SIZE, center[2] + SIZE];
    let bottom_right = [center[0] + SIZE, center[1] + SIZE, center[2]];
    [
        Vertex::new(top_left, TEX[3], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(bottom_right, TEX[0], *normal),
    ]
}

pub fn front(center: [i32; 3], normal: &[i32; 3]) -> [Vertex; 6] {
    let top_left = [center[0], center[1], center[2] + SIZE];
    let top_right = [center[0] + SIZE, center[1], center[2] + SIZE];
    let bottom_left = [center[0], center[1] + SIZE, center[2] + SIZE];
    let bottom_right = [center[0] + SIZE, center[1] + SIZE, center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(bottom_right, TEX[0], *normal),
    ]
}

pub fn back(center: [i32; 3], normal: &[i32; 3]) -> [Vertex; 6] {
    let top_left = [center[0] + SIZE, center[1], center[2]];
    let top_right = [center[0], center[1], center[2]];
    let bottom_left = [center[0] + SIZE, center[1] + SIZE, center[2]];
    let bottom_right = [center[0], center[1] + SIZE, center[2]];
    [
        Vertex::new(top_left, TEX[3], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(top_right, TEX[1], *normal),
        Vertex::new(bottom_left, TEX[2], *normal),
        Vertex::new(bottom_right, TEX[0], *normal),
    ]
}
implement_vertex!(Vertex, position, tex_coords, normal);
