use crate::world::Block;
use crate::consts::*;
use crate::world::vertices::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    width: i32,
    depth: i32,
    height: i32,
    center: [f32;3],
    start: [f32;3],
    initial: [f32;3],
    collection: Vec<Block>,
}

impl Chunk {
    pub fn new(center: [f32;3]) -> Chunk {
        let middle = [center[0]+8.0, center[1], center[2]+8.0];
        Chunk {
            width: 16,
            depth: 16,
            height: 256,
            center: middle,
            start: center,
            initial: center,
            collection: vec![],
        }
    }
    pub fn get_distance(&self, position: [f32; 3]) -> f32 {
        ((position[0]-self.center[0] as f32).powf(2.0) + (position[2]-self.center[2] as f32).powf(2.0)).sqrt()
    }
    pub fn construct(&mut self) {
        for x in 0..(self.depth) {
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
        self.collection.push(Block::new(self.start));
    }
}
impl_vertices!(Chunk);