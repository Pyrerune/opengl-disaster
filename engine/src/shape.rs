use glium::{VertexBuffer, IndexBuffer};
use glium::index::PrimitiveType;
use glm::TVec3;
use std::collections::HashMap;
use crate::*;
fn get_vertex_function(face: Faces) -> fn([i32;3], &[i32;3]) -> [Vertex; 6] {
    match face {
        Faces::Top => {
            top
        }
        Faces::Bottom => {
            bottom
        }
        Faces::Left => {
            left
        }
        Faces::Right => {
            right
        }
        Faces::Back => {
            back
        }
        Faces::Front => {
            front
        }
    }
}


#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Faces {
    Top,
    Bottom,
    Left,
    Right,
    Back,
    Front,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Face {
    pub(crate) vertices: [Vertex; 6],
}

#[derive(Debug)]
pub struct Shape {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
}


impl Shape {
    pub fn get_vbuffer(&self) -> &'_ VertexBuffer<Vertex> {
        &self.vertex_buffer
    }
    pub fn get_ibuffer(&self) -> &'_ IndexBuffer<u16> {
        &self.index_buffer
    }
    pub fn construct(display: glium::Display, data: &[Vertex], prim_type: PrimitiveType, idata: Vec<u16>) -> Self {
        let vbuffer = glium::VertexBuffer::new(&display, data).unwrap();
        let ibuffer = glium::IndexBuffer::new(&display,
                                              prim_type, &idata).unwrap();
        Shape {
            vertex_buffer: vbuffer,
            index_buffer: ibuffer,
        }
    }
    pub fn cube(center: [i32;3], normals: &HashMap<Faces, [i32;3]>, faces_to_draw: Vec<Faces>/*, texture: Option<SrgbTexture2d>*/) -> Block {
        let mut faces = HashMap::new();
        for face in faces_to_draw {
            let normal = normals.get(&face).expect("Hashmap must contain a normal for each face to draw");
            let vertex_constructor = get_vertex_function(face);
            let vertices = vertex_constructor(center, normal);
            faces.insert(face, Face {
                vertices
            });
        }
        Block {
            center,
            faces,
        }
    }
    fn calculate_vectors(b: TVec3<i32>, r: TVec3<i32>, s: TVec3<i32>) -> TVec3<i32> {
        let qr = r - b;
        let qs = s - b;
        glm::cross(&qr, &qs)

    }
    pub fn construct_normals(start: [i32; 3]) -> HashMap<Faces, [i32; 3]> {
        let mut normals = HashMap::new();
        //TOP BOTTOM
        let base = TVec3::new(start[0] - SIZE, start[1] - SIZE, start[2] - SIZE);
        let mut horizontal = TVec3::new(start[0] + SIZE, start[1] - SIZE, start[2] - SIZE);
        let mut vertical = TVec3::new(start[0] - SIZE, start[1] - SIZE, start[2] + SIZE);
        let top_normal = Shape::calculate_vectors(base, horizontal, vertical);

        normals.insert(Faces::Top, *top_normal.as_ref());
        normals.insert(Faces::Bottom, *top_normal.as_ref());
        //LEFT RIGHT
        horizontal = TVec3::new(start[0] - SIZE, start[1] - SIZE, start[2] + SIZE);
        vertical = TVec3::new(start[0] - SIZE, start[1] + SIZE, start[2] - SIZE);
        let left_normal = Shape::calculate_vectors(base, horizontal, vertical);
        normals.insert(Faces::Left, *left_normal.as_ref());
        normals.insert(Faces::Right, *left_normal.as_ref());
        //FRONT BACK

        horizontal = TVec3::new(start[0] + SIZE, start[1] - SIZE, start[2] - SIZE);
        let back_normal = Shape::calculate_vectors(base, horizontal, vertical);

        normals.insert(Faces::Front, *back_normal.as_ref());
        normals.insert(Faces::Back, *back_normal.as_ref());
        normals

    }

}

