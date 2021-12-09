mod app;
mod camera;
mod shader;
mod shape;
mod vertex;
mod world;

#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
pub use app::*;
pub use camera::*;
pub use shader::*;
pub use shape::*;
pub use vertex::*;
pub use world::*;
pub use consts::*;
mod consts {
    pub const SIZE: i32 = 1;
    pub const TEX: [[i32; 2]; 4] = [
        [1, 1],  // top right
        [1, 0],  // bottom right
        [0, 1],  // top left
        [0, 0],  // bottom left
    ];

    pub const FAR: f32 = 17.0;
    pub const STEP: f32 = 100.0;
}
