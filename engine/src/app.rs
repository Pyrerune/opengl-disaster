use std::fs::File;
use std::path::Path;
use std::io::Read;
use crate::*;
use crate::consts::*;
use image::RgbaImage;
use glm::TMat4;

fn mat4(val: f32) -> glm::TMat4<f32> {
    glm::mat4(val, 0.0, 0.0, 0.0,
                0.0, val, 0.0, 0.0,
              0.0, 0.0, val, 0.0,
                    0.0, 0.0, 0.0, val)
}

fn load_shader<P: AsRef<Path>>(filename: P) -> String {
    let mut contents = String::new();
    let mut file = File::open(filename).expect("Shader doesn't Exist");
    file.read_to_string(&mut contents).unwrap();
    contents
}
fn load_image<P: AsRef<Path>>(filename: P) -> RgbaImage {
    image::io::Reader::open(filename).unwrap().decode()
        .unwrap().to_rgba8()
}
pub struct App {
    pub camera: Camera,
    pub mouse_pos: (f64, f64),
    pub texture: RgbaImage,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub model: TMat4<f32>,
    pub world: World,
    
}
impl App {
    pub fn new(size: (u32, u32)) -> App {
        let camera = Camera::new(size);
        App {
            camera,
            mouse_pos: (0.0, 0.0),
            texture: load_image("./icon.jpg"),
            vertex_shader: load_shader("engine/shaders/vertex.shader"),
            fragment_shader: load_shader("engine/shaders/fragment.shader"),
            model: mat4(1.0),
            world: World::new(1, 1, [0.0; 3])
        }
    }
}