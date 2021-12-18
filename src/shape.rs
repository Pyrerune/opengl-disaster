use glium::{VertexBuffer, IndexBuffer};
use glium::index::PrimitiveType;

#[derive(Debug)]
pub struct Shape {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u16>,
}


impl Shape {
    pub fn construct(display: &glium::Display, data: &[Vertex], prim_type: PrimitiveType, idata: Vec<u16>) -> Self {
        let vbuffer = glium::VertexBuffer::new(display, data).unwrap();
        let ibuffer = glium::IndexBuffer::new(display,
                                              prim_type, &idata).unwrap();
        Shape {
            vertex_buffer: vbuffer,
            index_buffer: ibuffer,
        }
    }
}