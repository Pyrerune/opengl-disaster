use std::cmp::Ordering;
use crate::consts::*;
#[macro_export]
macro_rules! vertex {
    () => {
        Vertex::default()
    };
    ($position:expr, $tex_coords:expr) => {
        Vertex::new($position, $tex_coords)
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [u32; 2],
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
    pub fn new(position: [f32; 3], tex_coords: [u32; 2]) -> Vertex {
        Vertex {
            position,
            tex_coords
        }
    }

}
impl Default for Vertex {
    fn default() -> Self {
        Vertex::new([0.0;3], [0;2])
    }
}
pub fn all_faces(center: [f32; 3]) -> [[Vertex; 6]; 6] {
    [top(center), bottom(center), left(center), right(center), front(center), back(center)]
}
pub fn bottom(center: [f32; 3]) -> [Vertex; 6] {
    let top_left = [center[0], center[1], center[2]];
    let top_right = [center[0] + SIZE, center[1], center[2]];
    let bottom_left = [center[0], center[1], center[2] + SIZE];
    let bottom_right = [center[0] + SIZE, center[1], center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(bottom_right, TEX[0]),
    ]
}

pub fn top(center: [f32; 3]) -> [Vertex; 6] {
    let top_left = [center[0] + SIZE, center[1] + SIZE, center[2]];
    let top_right = [center[0], center[1] + SIZE, center[2]];
    let bottom_left = [center[0] + SIZE, center[1] + SIZE, center[2] + SIZE];
    let bottom_right = [center[0], center[1] + SIZE, center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(bottom_right, TEX[0]),
    ]
}

pub fn left(center: [f32; 3]) -> [Vertex; 6] {
    let top_left = [center[0], center[1], center[2]];
    let top_right = [center[0], center[1], center[2] + SIZE];
    let bottom_left = [center[0], center[1] + SIZE, center[2]];
    let bottom_right = [center[0], center[1] + SIZE, center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(bottom_right, TEX[0]),
    ]
}

pub fn right(center: [f32; 3]) -> [Vertex; 6] {
    let top_left = [center[0] + SIZE, center[1], center[2] + SIZE];
    let top_right = [center[0] + SIZE, center[1], center[2]];
    let bottom_left = [center[0] + SIZE, center[1] + SIZE, center[2] + SIZE];
    let bottom_right = [center[0] + SIZE, center[1] + SIZE, center[2]];
    [
        Vertex::new(top_left, TEX[3]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(bottom_right, TEX[0]),
    ]
}

pub fn front(center: [f32; 3]) -> [Vertex; 6] {
    let top_left = [center[0], center[1], center[2] + SIZE];
    let top_right = [center[0] + SIZE, center[1], center[2] + SIZE];
    let bottom_left = [center[0], center[1] + SIZE, center[2] + SIZE];
    let bottom_right = [center[0] + SIZE, center[1] + SIZE, center[2] + SIZE];
    [
        Vertex::new(top_left, TEX[3]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(bottom_right, TEX[0]),
    ]
}

pub fn back(center: [f32; 3]) -> [Vertex; 6] {
    let top_left = [center[0] + SIZE, center[1], center[2]];
    let top_right = [center[0], center[1], center[2]];
    let bottom_left = [center[0] + SIZE, center[1] + SIZE, center[2]];
    let bottom_right = [center[0], center[1] + SIZE, center[2]];
    [
        Vertex::new(top_left, TEX[3]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(top_right, TEX[1]),
        Vertex::new(bottom_left, TEX[2]),
        Vertex::new(bottom_right, TEX[0]),
    ]
}