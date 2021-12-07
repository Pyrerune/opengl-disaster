use crate::vertex::*;
use glium::{VertexBuffer, IndexBuffer};
use glium::index::PrimitiveType;
use glm::TVec3;
use std::collections::HashMap;
use crate::shape::consts::*;

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
pub mod consts {
    pub const SIZE: i32 = 1;
    pub const TEX: [[i32; 2]; 4] = [
        [1, 1],  // top right
        [1, 0],  // bottom right
        [0, 1],  // top left
        [0, 0],  // bottom left
    ];
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
    vertices: [Vertex; 6],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    width: i32,
    depth: i32,
    height: i32,
    center: [i32;3],
    initial: [i32;3],
    blocks: Vec<Block>,
    normals: HashMap<Faces, [i32;3]>
}
#[derive(Debug)]
pub struct World {
    chunks: Vec<Shape>
}
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    center: [i32;3],
    faces: HashMap<Faces, Face>,
}
impl World {
    pub fn new(display: &glium::Display, num_chunks_x: i32, num_chunks_z: i32, start: [i32;3]) -> World {
        let mut chunks = vec![];
        let mut center = start;
        for _i in 0..num_chunks_x {
            for _j in 0..num_chunks_z {
                let mut chunk = Chunk::new(center);
                chunk.construct();
                println!("new chunck");
                chunks.push(chunk.to_shape(display.clone()));
                center[2]+=17;
            }
            center[0] -= 17;
            center[2] = start[2];
        }
        World {
            chunks
        }
    }
}
impl Chunk {
    pub fn new(center: [i32;3]) -> Chunk {
        let normals = Shape::construct_normals(center);
        Chunk {
            width: 16,
            depth: 16,
            height: 256,
            center,
            initial: center,
            blocks: vec![],
            normals
        }
    }
    pub fn to_shape(&self, display: glium::Display) -> Shape {
        let vertices = self.vertices();
        Shape::construct(display, &vertices, PrimitiveType::TrianglesList, (0..vertices.len() as u16).collect())
    }
    pub fn construct(&mut self) {
        for _x in 0..(self.depth) {
            for _j in 0..(self.height / 2) {
                for _i in 0..self.width {

                    self.add_block();
                    self.center[0] += SIZE;
                }
                self.center[0] = self.initial[0];
                self.center[1] += SIZE;
            }

            println!("{:?}", self.center);
            self.center[1] = self.initial[1];
            self.center[2] += SIZE;
        }
    }
    fn add_block(&mut self) {
        let mut faces_to_draw = vec![];
        let [mut min_x, mut min_y, mut min_z] = self.initial;
        if self.center < [self.width, self.height, self.depth] {

            if self.center[0] == self.initial[0] {
                faces_to_draw.push(Faces::Left);
            }
            if self.center[0] == self.initial[0]+(self.width)-1 {
                faces_to_draw.push(Faces::Right);
            }
            if self.center[1] == self.initial[1] {
                faces_to_draw.push(Faces::Bottom);
            }
            if self.center[1] == self.initial[1]+((self.height/2)-1) {
                faces_to_draw.push(Faces::Top);
            }
            if self.center[2] == self.initial[2] {
                faces_to_draw.push(Faces::Back);
            }
            if self.center[2] == self.initial[2]+(self.depth)-1 {
                faces_to_draw.push(Faces::Front);
            }
            self.blocks.push(Shape::cube(self.center, &self.normals, faces_to_draw));
        }
    }
    pub fn vertices(&self) -> Vec<Vertex> {
        let mut vertices = vec![];
        for block in &self.blocks {
            vertices.append(&mut block.vertices());
        }
        vertices
    }

}

impl Block {
    pub fn vertices(&self) -> Vec<Vertex> {
        let mut vertices = vec![];
        for a in self.faces.values() {
            vertices.append(&mut a.vertices.to_vec());
        }
        vertices
    }
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
    pub fn experimental_plane(display: &glium::Display, _width: u32, _height: u32, _depth: u32, start_pos: [i32;3]) -> Vec<Shape> {
        let mut center = start_pos;
        let mut plane = World::new(display, 16,  16, center);
        plane.chunks

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
    fn construct_normals(start: [i32; 3]) -> HashMap<Faces, [i32; 3]> {
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

