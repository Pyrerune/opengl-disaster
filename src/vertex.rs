use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32;3],
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
    }
}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
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
