use std::fs::File;
use std::io::Read;
use std::path::Path;
macro_rules! vertex_shader {
    () => {open_shader("shaders/vertex.shader")};
}
macro_rules! fragment_shader {
    () => {open_shader("shaders/fragment.shader")};
}
pub struct Shader {
    vertex_shader: String,
    fragment_shader: String,
}
impl Shader {
    pub fn get_vertex(&self) -> String {
        self.vertex_shader.clone()
    }
    pub fn get_fragment(&self) -> String {
        self.fragment_shader.clone()
    }
}
impl Default for Shader {
    fn default() -> Self {
        Shader {
            vertex_shader: vertex_shader!(),
            fragment_shader:  fragment_shader!(),
        }
    }
}
fn open_shader<T: AsRef<Path>>(filename: T) -> String {
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    content
}


