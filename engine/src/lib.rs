mod app;
mod camera;
mod vertex;
mod world;
extern crate nalgebra_glm as glm;
pub use app::*;
pub use camera::*;
pub use vertex::*;
pub use world::*;
pub use consts::*;

#[macro_use]
extern crate glium;

implement_vertex!(Vertex, position, tex_coords);
mod consts {
    pub const SIZE: f32 = 0.5;
    pub const TEX: [[u32; 2]; 4] = [
        [1, 1],  // top right
        [1, 0],  // bottom right
        [0, 1],  // top left
        [0, 0],  // bottom left
    ];

    pub const FAR: f32 = 17.0;
    pub const STEP: f32 = 100.0;
}
