use std::str::from_boxed_utf8_unchecked;
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
    }
    pub fn cube(display: &glium::Display, dimensions: [f32; 3], center: [f32; 3], color: [f32; 3], transform: glm::TMat4<f32>) -> Shape {
        let [(min_x, max_x), (min_y, max_y), (min_z, max_z)] = get_coords(dimensions, center);
        let v0 = glm::vec3(min_x, min_y, min_z);
        let v1 = glm::vec3(min_x, max_y, min_z);
        let v2 = glm::vec3(max_x, min_y, min_z);
        let v3 = glm::vec3(max_x, max_y, min_z);
        let v4 = glm::vec3(min_x, min_y, max_z);
        let v5 = glm::vec3(min_x, max_y, max_z);
        let v6 = glm::vec3(max_x, min_y, max_z);
        let v7 = glm::vec3(max_x, max_y, max_z);
        let back_left = v1 - v0;
        let back_bottom = v2 - v0;
        let bottom_left = v4 - v0;
        let top1 = v3 - v1;
        let top2 = v5 - v1;
        let back_normal = glm::cross(&back_left, &back_bottom);
        let right_normal = glm::cross(&back_left, &bottom_left);
        let left_normal = right_normal * -1.0;
        let bottom_normal = glm::cross(&top1, &top2);
        let top_normal = bottom_normal * -1.0;
        let front_normal = back_normal * -1.0;
        let back_index = [0, 1, 2, 1, 2, 3];
        let left_index = [4, 5, 6, 5, 6, 7];
        let right_index = [8, 9, 10, 9, 10, 11];
        let top_index = [12, 13, 14, 13, 14, 15];
        let bottom_index = [16, 17, 18, 17, 18, 19];
        let front_index = [20, 21, 22, 21, 22, 23];
        let index_data = &[back_index, left_index, right_index, top_index, bottom_index, front_index].concat();
        let vertex_data = &[
            //back face
            //0
            vertex!(*v0.as_ref(), *back_normal.as_ref(), color),
            //1
            vertex!(*v1.as_ref(), *back_normal.as_ref(), color),
            //2
            vertex!(*v2.as_ref(), *back_normal.as_ref(), color),
            //3
            vertex!(*v3.as_ref(), *back_normal.as_ref(), color),
            //side left
            //4
            vertex!(*v0.as_ref(), *left_normal.as_ref(), color),
            //5
            vertex!(*v1.as_ref(), *left_normal.as_ref(), color),
            //6
            vertex!(*v4.as_ref(), *left_normal.as_ref(), color),
            //7
            vertex!(*v5.as_ref(), *left_normal.as_ref(), color),
            //side right
            //8
            vertex!(*v2.as_ref(), *right_normal.as_ref(), color),
            //9
            vertex!(*v3.as_ref(), *right_normal.as_ref(), color),
            //10
            vertex!(*v6.as_ref(), *right_normal.as_ref(), color),
            //11
            vertex!(*v7.as_ref(), *right_normal.as_ref(), color),

            //top
            //12
            vertex!(*v1.as_ref(), *top_normal.as_ref(), color),
            //13
            vertex!(*v3.as_ref(), *top_normal.as_ref(), color),
            //14
            vertex!(*v5.as_ref(), *top_normal.as_ref(), color),
            //15
            vertex!(*v7.as_ref(), *top_normal.as_ref(), color),

            //bottom
            //16
            vertex!(*v0.as_ref(), *bottom_normal.as_ref(), color),
            //17
            vertex!(*v2.as_ref(), *bottom_normal.as_ref(), color),
            //18
            vertex!(*v4.as_ref(), *bottom_normal.as_ref(), color),
            //19
            vertex!(*v6.as_ref(), *bottom_normal.as_ref(), color),
            //front
            //20
            vertex!(*v4.as_ref(), *front_normal.as_ref(), color),
            //21
            vertex!(*v5.as_ref(), *front_normal.as_ref(), color),
            //22
            vertex!(*v6.as_ref(), *front_normal.as_ref(), color),
            //23
            vertex!(*v7.as_ref(), *front_normal.as_ref(), color),

        ];
        Shape::construct(display.clone(), vertex_data, PrimitiveType::TrianglesList, index_data, transform)
    }
}

