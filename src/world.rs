

/*


pub fn experimental_plane(display: &glium::Display, width: u32, height: u32, depth: u32, start_pos: [i32; 3]) -> Shape {
    let mut vertices = vec![];
    let normals = Shape::construct_normals(start_pos);
    let mut center = start_pos;
    for x in 0..=width {
        for y in 0..=height {
            for z in 0..=depth {

                let cube = Shape::cube(center, &normals, vec![Faces::Top, Faces::Bottom, Faces::Left, Faces::Right, Faces::Back, Faces::Front]);
                for a in cube.faces.values() {
                    vertices.append(&mut a.vertices.to_vec());
                }
                center[2] += SIZE * 2;
            }
            center[2] = start_pos[2];
            center[1] += SIZE * 2;
        }
        center[1] = start_pos[1];
        center[0] += SIZE * 2;
    }

    let indices = (0..vertices.len()).map(|a| a as u16).collect();
    Shape::construct(display.clone(), &vertices, PrimitiveType::TrianglesList, indices)
}




fn top_face(width: u32, depth: u32, start: [i32;3]) -> (Vec<Vertex>, Vec<u16>) {
    let mut current_pos = start;
    let b = TVec3::new(start[0], start[1], start[2]);
    let r = TVec3::new(start[0] + SIZE, start[1], start[2]);
    let s = TVec3::new(start[0], start[1], start[2] + SIZE);
    let qr = r - b;
    let qs = s - b;
    let n = glm::cross(&qr, &qs);
    let normal = *n.as_ref();
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut index: u16 = 0;
    for i in 0..=width {
        for j in 0..=depth {
            if index != ((width + 1) * i + depth) as u16 && (index + 1 + depth as u16 + 1) < ((width + 1) * (depth + 1)) as u16 {
                indices = [indices, vec![index, index + 1, index + 1 + depth as u16, index + 1, index + 1 + depth as u16, index + 1 + depth as u16 + 1]].concat();
            }

            if (i % 2) == 0 {
                if (j % 2) == 0 {
                    //top
                    vertices.push(vertex!(current_pos, normal, TEX[3]));
                } else {
                    vertices.push(vertex!(current_pos, normal, TEX[1]));
                }
            } else {
                if (j % 2) == 0 {
                    //top
                    vertices.push(vertex!(current_pos, normal, TEX[2]));
                } else {
                    vertices.push(vertex!(current_pos, normal, TEX[0]));
                }
            }
            current_pos[2] += SIZE;

            index += 1;
        }

        current_pos[0] += SIZE;
        current_pos[2] = start[2];

    }
    (vertices, indices)

}
fn bottom_face(width: u32, depth: u32, start: [i32;3]) -> (Vec<Vertex>, Vec<u16>) {
    Shape::top_face(width, depth, start)
}
pub fn optimized_chunk(display: &glium::Display, width: u32, height: u32, depth: u32, start_pos: [i32;3]) -> Shape {
    //TODO FINISH THIS
    let mut current_pos = start_pos;
    let mut top_vertices = Shape::top_face(width, depth, current_pos);
    current_pos[2] += (SIZE * height);
    let bottom_vertices = Shape::top_face(width, depth, current_pos);

    let vertices = vec![top_vertices.0, bottom_vertices.0].concat();
    let indices =
        Shape::construct(display.clone(), &vertices, PrimitiveType::TrianglesList,
                         top_vertices.1);

}
*/