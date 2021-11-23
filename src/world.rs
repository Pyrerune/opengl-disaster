use crate::shape::Shape;

pub enum LineOrientation {
    Vertical,
    HorizX,
    HorizZ,

}
pub enum PlaneOrientation {
    HorizontalX,
    HorizontalZ,
    VerticalX,
    VerticalZ,
}

pub fn line(display: &glium::Display, length: u32, orientation: LineOrientation, start_pos: [f32;3]) -> Vec<Shape> {
    let mut line = vec![];
    let size = &[0.5, 0.5, 0.5];
    let mut center = start_pos;
    for i in 0..length {
        line.push(Shape::cube(display, size, &center));
        match orientation {
            LineOrientation::Vertical => center[1] += 0.5,
            LineOrientation::HorizX => center[0] += 0.5,
            LineOrientation::HorizZ => center[2] += 0.5,
        }
    }
    line
}
pub fn plane(display: &glium::Display, width: u32, height: u32, orientation: PlaneOrientation, start_pos: [f32;3]) -> Vec<Shape> {
    let mut plane = vec![];
    let size = &[0.5, 0.5, 0.5];
    let mut center = start_pos;
    for i in 0..width {
        for j in 0..height {
            plane.push(Shape::cube(display, size, &center));
            match orientation {
                PlaneOrientation::HorizontalX => center[2] += 0.5,
                PlaneOrientation::HorizontalZ => center[0] += 0.5,
                _ => center[1] += 0.5,
            }
        }
        match orientation {
            PlaneOrientation::HorizontalX =>
                {
                    center[2] = start_pos[2];
                    center[0] += 0.5;
                },
            PlaneOrientation::HorizontalZ => {
                center[0] = start_pos[0];
                center[2] += 0.5;
            },
            PlaneOrientation::VerticalX =>
                {
                    center[1] = start_pos[1];
                    center[0] += 0.5;
                },
            PlaneOrientation::VerticalZ =>
                {
                    center[1] = start_pos[1];
                    center[2] += 0.5
                },
        }

    }
    plane
}