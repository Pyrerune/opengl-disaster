#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32;3],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2], normal: [f32;3]) -> Vertex {
        Vertex {
            position,
            tex_coords,
            normal,
        }
    }
}
impl Default for Vertex {
    fn default() -> Self {
        Vertex::new([0.0;3], [0.0;2], [0.0;3])
    }
}

implement_vertex!(Vertex, position, tex_coords, normal);
