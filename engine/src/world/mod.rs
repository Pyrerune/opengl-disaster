mod world;
mod chunk;
mod block;

pub use world::World;
pub use chunk::Chunk;
pub use block::Block;


pub mod vertices {

    pub use crate::Vertex;
    pub trait CollectVertices {
        fn vertices(&self) -> Vec<Vertex>;
    }
    macro_rules! impl_vertices {
        ($struct_type:ty) => {
            impl CollectVertices for $struct_type {
                fn vertices(&self) -> Vec<Vertex> {
                    let mut vertices = vec![];
                    for single in &self.collection {
                        vertices.append(&mut single.vertices());
                    }
                    vertices
                }
            }
        }
    }
    pub(crate) use impl_vertices;
}