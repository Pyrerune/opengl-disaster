use crate::world::Chunk;
use crate::consts::*;
#[derive(Clone, Debug)]
pub struct World {
    pub chunks: Vec<Chunk>,
}

impl World {
    pub fn new(num_chunks_x: i32, num_chunks_z: i32, start: [f32;3]) -> World {
        let mut chunks = vec![];
        let mut center = start;
        for _i in 0..num_chunks_x {
            for _j in 0..num_chunks_z {
                let mut chunk = Chunk::new(center);
                chunk.construct();
                chunks.push(chunk);
                center[2]+= 16.0 * SIZE;
            }
            center[0] -= 16.0 * SIZE;
            center[2] = start[2];
        }
        World {
            chunks: chunks,
        }
    }
}