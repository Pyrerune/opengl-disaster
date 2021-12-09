use std::collections::HashMap;
use glium::index::PrimitiveType;
use crate::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    width: i32,
    depth: i32,
    height: i32,
    center: [i32;3],
    start: [i32;3],
    initial: [i32;3],

    blocks: Vec<Block>,
    normals: HashMap<Faces, [i32;3]>
}
#[derive(Debug)]
pub struct World {
    chunks: Vec<Chunk>,
    drawn: Vec<Chunk>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub(crate) center: [i32;3],
    pub(crate) faces: HashMap<Faces, Face>,
}

impl World {
    pub fn new(num_chunks_x: i32, num_chunks_z: i32, start: [i32;3]) -> World {
        let mut chunks = vec![];
        let mut center = start;
        for _i in 0..num_chunks_x {
            for _j in 0..num_chunks_z {
                let mut chunk = Chunk::new(center);
                chunk.construct();
                chunks.push(chunk);
                center[2]+=17;
            }
            center[0] -= 17;
            center[2] = start[2];
        }
        World {
            chunks,
            drawn: vec![],
        }
    }
    pub fn update(&mut self, camera_pos: [f32;3]) {
        self.drawn = self.chunks.iter().filter(|&a|  {
            a.get_distance(camera_pos) < FAR
        }).map(|a| a.clone()).collect();
    }
    pub fn get_drawn<'a>(&self) -> Vec<Chunk> {
        self.drawn.clone()
    }
}
impl Chunk {
    pub fn new(center: [i32;3]) -> Chunk {
        let normals = Shape::construct_normals(center);
        let middle = [center[0]+8, center[1], center[2]+8];
        Chunk {
            width: 16,
            depth: 16,
            height: 256,
            center: middle,
            start: center,
            initial: center,
            blocks: vec![],
            normals
        }
    }
    pub fn get_distance(&self, position: [f32; 3]) -> f32 {
        ((position[0]-self.center[0] as f32).powf(2.0) + (position[2]-self.center[2] as f32).powf(2.0)).sqrt()
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
                    self.start[0] += SIZE;
                }
                self.start[0] = self.initial[0];
                self.start[1] += SIZE;
            }

            self.start[1] = self.initial[1];
            self.start[2] += SIZE;
        }
    }
    fn add_block(&mut self) {
        let mut faces_to_draw = vec![];
        if self.start < [self.width, self.height, self.depth] {

            if self.start[0] == self.initial[0] {
                faces_to_draw.push(Faces::Left);
            }
            if self.start[0] == self.initial[0]+(self.width)-1 {
                faces_to_draw.push(Faces::Right);
            }
            if self.start[1] == self.initial[1] {
                faces_to_draw.push(Faces::Bottom);
            }
            if self.start[1] == self.initial[1]+((self.height/2)-1) {
                faces_to_draw.push(Faces::Top);
            }
            if self.start[2] == self.initial[2] {
                faces_to_draw.push(Faces::Back);
            }
            if self.start[2] == self.initial[2]+(self.depth)-1 {
                faces_to_draw.push(Faces::Front);
            }
            self.blocks.push(Shape::cube(self.start, &self.normals, faces_to_draw));
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