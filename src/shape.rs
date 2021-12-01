use std::time::Instant;
use crate::vertex::Vertex;
use glium::{VertexBuffer, IndexBuffer};
use glium::index::PrimitiveType;
use glm::TVec3;
fn optimize_world(world: &Vec<Face>) -> Vec<Face> {

    let mut new_world: Vec<Face> = world.clone();
    for _i in 0..new_world.len() {
        let value = new_world.remove(0);
        if new_world.contains(&value) {
            if let Some(loc) = new_world.iter().position(|&a| {
                a == value
            }) {
                new_world.remove(loc);
            }
        } else {
            new_world.push(value);
        }

    }
    new_world
}
macro_rules! vertex {
    () => {
        Vertex::default()
    };
    ($position:expr, $normal:expr, $tex_coords:expr) => {
        Vertex::new($position, $tex_coords, $normal)
    }
}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Face {
    vertices: [Vertex; 6],
}



impl Face {
    pub fn construct_face(vertices: &[TVec3<f32>; 4], triangle1_normal: bool, tex_coords: [[f32;2];4]) -> Face {
        let triangle2_vertices: [TVec3<f32>; 3] = vertices[0..=2].try_into().expect("Incorrect Size");
        let triangle1_vertices: [TVec3<f32>; 3] = vertices[1..=3].try_into().expect("Incorrect Size");

        let tex_coords2: [[f32;2];3] = tex_coords[0..=2].try_into().expect("Incorrect Size");
        let tex_coords1: [[f32;2];3] = tex_coords[1..=3].try_into().expect("Incorrect Size");
        let mut triangle1 = construct_vertex(&triangle1_vertices, tex_coords1);
        let mut triangle2 = construct_vertex(&triangle2_vertices, tex_coords2);
        if triangle1_normal {
            triangle2 = replace_normal(&triangle2, &triangle1[0].normal);
        } else {
            triangle1 = replace_normal(&triangle1, &triangle2[0].normal);
        }
        Face {
            vertices: [triangle1, triangle2].concat().try_into().unwrap(),
        }
    }
}


fn replace_normal(vertices: &[Vertex; 3], normal: &[f32;3]) -> [Vertex; 3] {
    let mut vertex_data = vec!();
    for vertex in vertices {
        vertex_data.push(Vertex {
            normal: *normal,
            ..*vertex
        });
    }
    vertex_data.try_into().expect("Wrong Size")
}
fn construct_vertex(vertices: &[TVec3<f32>; 3], tex_coords: [[f32;2];3]) -> [Vertex; 3] {
    let b = vertices[0];
    let r = vertices[1];
    let s = vertices[2];
    let qr = r - b;
    let qs = s - b;
    let n = glm::cross(&qr, &qs);
    let vb = vertex!(*b.as_ref(), *n.as_ref(), tex_coords[0]);
    let vr = vertex!(*r.as_ref(), *n.as_ref(), tex_coords[1]);
    let vs = vertex!(*s.as_ref(), *n.as_ref(), tex_coords[2]);
    [vb, vr, vs]
}
fn get_coords(dimensions: &[f32; 3], center: &[f32; 3]) -> [(f32, f32); 3] {
    let min_x = center[0] - (dimensions[0] / 2.0);
    let max_x = min_x + dimensions[0];
    let min_y = center[1] - (dimensions[1] / 2.0);
    let max_y = min_y + dimensions[1];
    let min_z = center[2] - (dimensions[2] / 2.0);
    let max_z = min_z + dimensions[2];
    [(min_x, max_x), (min_y, max_y), (min_z, max_z)]
}

#[derive(Debug)]
pub struct Shape {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
}
impl Shape {
    pub fn get_vbuffer(&self) -> &'_ VertexBuffer<Vertex> {
        &self.vertex_buffer
    }
    pub fn get_ibuffer(&self) -> &'_ IndexBuffer<u16> {
        &self.index_buffer
    }
    pub fn construct(display: glium::Display, data: &[Vertex], prim_type: PrimitiveType, idata: Vec<u16>) -> Self {
        let vbuffer = glium::VertexBuffer::new(&display, data).unwrap();
        let ibuffer = glium::IndexBuffer::new(&display,
                                              prim_type, &idata).unwrap();
        Shape {
            vertex_buffer: vbuffer,
            index_buffer: ibuffer,
        }
    }
    pub fn optimized_cube(dimensions: &[f32;3], center: &[f32; 3]) -> Vec<Face> {
        let [(min_x, max_x), (min_y, max_y), (min_z, max_z)] = get_coords(dimensions, center);
        let v0 = glm::vec3(min_x, min_y, min_z);
        let v1 = glm::vec3(min_x, max_y, min_z);
        let v2 = glm::vec3(max_x, min_y, min_z);
        let v3 = glm::vec3(max_x, max_y, min_z);
        let tex_coords = [
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [0.0, 1.0],
        ];
        let back_face = Face::construct_face(&[v0, v1, v2, v3], false, tex_coords);

        vec![back_face]
        //TODO somehow optimize the math to not even calculate faces that are hidden
    }
    pub fn cube(dimensions: &[f32; 3], center: &[f32; 3]) -> [Face; 6] {
        let [(min_x, max_x), (min_y, max_y), (min_z, max_z)] = get_coords(dimensions, center);
        let v0 = glm::vec3(min_x, min_y, min_z);
        let v1 = glm::vec3(min_x, max_y, min_z);
        let v2 = glm::vec3(max_x, min_y, min_z);
        let v3 = glm::vec3(max_x, max_y, min_z);
        let v4 = glm::vec3(min_x, min_y, max_z);
        let v5 = glm::vec3(min_x, max_y, max_z);
        let v6 = glm::vec3(max_x, min_y, max_z);
        let v7 = glm::vec3(max_x, max_y, max_z);
        let tex_coords = [
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [0.0, 1.0],
        ];
        let back_face = Face::construct_face(&[v0, v1, v2, v3], false, tex_coords);
        let left_face = Face::construct_face(&[v0, v1, v4, v5], true, tex_coords);

        let right_face = Face::construct_face(&[v2, v3, v6, v7], false, tex_coords);
        let top_face = Face::construct_face(&[v1, v3, v5, v7], true, tex_coords);
        let bottom_face = Face::construct_face(&[v0, v2, v4, v6], false, tex_coords);
        let front_face = Face::construct_face(&[v4, v5, v6, v7], true, tex_coords);
        [back_face, left_face, right_face, top_face, bottom_face, front_face]
    }

    pub fn plane(display: &glium::Display, width: u32, height: u32, depth: u32, start_pos: [f32;3]) -> Shape {
        let start = Instant::now();
        let mut plane: Vec<Face> = vec![];
        let mut center = start_pos;
        for _i in 0..width {
            for _j in 0..depth {
                for _x in 0..height {
                    println!("{:?}", start.elapsed());
                    plane = [plane, Shape::optimized_cube(&[0.5;3], &center).to_vec()].concat();
                    center[1] += 0.5;
                }

                center[1] = start_pos[2];
                center[2] += 0.5;
            }
            center[2] = start_pos[2];
            center[0] += 0.5;
        }
        //let world = optimize_world(&plane);
        let shaped_world = plane.iter().map(|a| {
            a.vertices
        }).collect::<Vec<_>>();
        let vertices = shaped_world.concat();
        let index = (0..vertices.len() as u16).collect::<Vec<_>>();
        Shape::construct(display.clone(), &vertices, PrimitiveType::TrianglesList, index)
    }
}

