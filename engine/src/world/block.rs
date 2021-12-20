use crate::{Vertex, all_faces};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Block {
    pub center: [f32;3],
    pub faces: [[Vertex; 6]; 6],
}

impl Block {
    pub fn new(center: [f32; 3]) -> Block {
        let vertices = all_faces(center);
        Block {
            center,
            faces: vertices,
        }
    }
}
impl Into<Vec<Vertex>> for Block {
    fn into(self) -> Vec<Vertex> {
        self.faces.concat()
    }
}