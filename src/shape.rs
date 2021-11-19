use crate::vertex::Vertex;
use glium::{VertexBuffer, IndexBuffer, Display};
use glium::index::PrimitiveType;
use glm::TVec3;
macro_rules! vertex {
    ($position:expr, $normal:expr, $color:expr) => {
        Vertex::new($position, $color, $normal)
    }
}
fn construct_vertex(vertices: [TVec3<f32>; 3], color: [f32; 3]) -> [Vertex; 3] {
    let mut vertex_data: [Vertex; 3] = [vertex!([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]); 3];
    let b = vertices[0];
    let r = vertices[1];
    let s = vertices[2];
    let qr = r - b;
    let qs = s - b;
    let n = glm::cross(&qr, &qs);
    let vb = vertex!(*b.as_ref(), *n.as_ref(), color);
    let vr = vertex!(*r.as_ref(), *n.as_ref(), color);
    let vs = vertex!(*s.as_ref(), *n.as_ref(), color);
    vertex_data[0] = vb;
    vertex_data[1] = vr;
    vertex_data[2] = vs;
    vertex_data
}
fn get_coords(dimensions: [f32; 3], center: [f32; 3]) -> [(f32, f32); 3] {
    let min_x = center[0] - (dimensions[0] / 2.0);
    let max_x = min_x + dimensions[0];
    let min_y = center[1] - (dimensions[1] / 2.0);
    let max_y = min_y + dimensions[1];
    let min_z = center[2] - (dimensions[2] / 2.0);
    let max_z = min_z + dimensions[2];
    [(min_x, max_x), (min_y, max_y), (min_z, max_z)]
}
pub struct Shape {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    transform: glm::TMat4<f32>,
    color: [f32; 3],
}

impl Shape {
    pub fn get_vbuffer(&self) -> &'_ VertexBuffer<Vertex> {
        &self.vertex_buffer
    }
    pub fn get_ibuffer(&self) -> &'_ IndexBuffer<u16> {
        &self.index_buffer
    }
    pub fn model_matrix(&self) -> glm::TMat4<f32> {
        self.transform
    }
    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
    pub fn construct(display: glium::Display, data: &[Vertex], prim_type: PrimitiveType, idata: &[u16], transform: glm::TMat4<f32>) -> Self {
        let vbuffer = glium::VertexBuffer::new(&display, data).unwrap();
        let ibuffer = glium::IndexBuffer::new(&display,
                            prim_type, idata).unwrap();
        Shape {
            vertex_buffer: vbuffer,
            index_buffer: ibuffer,
            transform,
            color: data[0].color,
        }

    }
    pub fn square(display: &glium::Display, dimensions: [f32; 2], center: [f32; 3], color: [f32; 3], transform: glm::TMat4<f32>) -> Shape {

        let [(min_x, max_x), (min_y, max_y), _] = get_coords([dimensions[0], dimensions[1], 0.0], center);

        let v1 = glm::vec3(min_x, min_y, center[2]);
        let v2= glm::vec3(min_x, max_y, center[2]);
        let v3 = glm::vec3(max_x, max_y, center[2]);
        let v4 = glm::vec3(max_x, min_y, center[2]);
        let l1 = v2 - v1;
        let l2 = v3 - v1;
        let n = glm::cross(&l1, &l2);
        let vertex_data  = &[
            vertex!(*v1.as_ref(), *n.as_ref(), color),
            vertex!(*v2.as_ref(), *n.as_ref(), color),
            vertex!(*v3.as_ref(), *n.as_ref(), color),
            vertex!(*v4.as_ref(), *n.as_ref(), color),
        ];
        Shape::construct(display.clone(), vertex_data, PrimitiveType::TrianglesList, &[0, 1, 2, 0, 2, 3], transform)
    }/*
    pub fn cube(display: &glium::Display, dimensions: [f32; 3], center: [f32; 3], color: [f32; 3], transform: glm::TMat4<f32>) -> Shape {
        let [(min_x, max_x), (min_y, max_y), (min_z, max_z)] = get_coords(dimensions, center);
        let vertex_data = &[
            //back face
            //0 reuse
            vertex!([min_x, min_y, min_z], color),
            //1 reuse
            vertex!([min_x, max_y, min_z], color),
            //2 reuse
            vertex!([max_x, min_y, min_z], color),
            //3 reuse
            vertex!([max_x, max_y, min_z], color),
            //side left
            //0
            //1

            //4 reuse
            vertex!([min_x, min_y, max_z], color),
            //5 reuse
            vertex!([min_x, max_y, max_z], color),
            //side right
            //2
            //3

            //6 reuse
            vertex!([max_x, min_y, max_z], color),
            //7 reuse
            vertex!([max_x, max_y, max_z], color),
            //top
            //1
            //3
            //5
            //7
            //bottom
            //0
            //2
            //4
            //6
            //front
            //4
            // 5
            // 6
            // 7
        ];
        let index_data = &[
            0, 1, 2, 3, 0, 1, 4, 5, 2, 3, 6, 7, 1, 3, 5, 7,
            0, 2, 4, 6, 4, 5, 6, 7
        ];
        Shape::construct(display.clone(), vertex_data, PrimitiveType::TriangleStrip, index_data, transform)
    }
*/
}

