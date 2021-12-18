
#[macro_use]
extern crate glium;
use glium::glutin;
use glutin::dpi::PhysicalPosition;
use glutin::event::WindowEvent;
use glutin::event::{KeyboardInput, VirtualKeyCode, Event};
use glutin::event_loop::ControlFlow;
use glium::index::PrimitiveType;
use glium::Surface;
use engine::*;
use engine::vertices::CollectVertices;

fn handle_keyboard_input(input: KeyboardInput) -> ControlFlow {
    let mut cf = ControlFlow::Poll;
    match input.virtual_keycode {
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => {
                    cf = ControlFlow::Exit;
                }
                _ => {}
            }
        }
        None => {
            cf = ControlFlow::Poll;
        }
    }
    cf
}
fn handle_mouse_input(position: PhysicalPosition<f64>) -> glutin::event_loop::ControlFlow {
    //println!("Got input");
    /*let mut xoff = position.x as f32 - self.mouse_pos.x;
    let mut yoff = self.mouse_pos.y - position.y as f32;
    if self.x_bound {
        xoff = 0.0;
        self.x_bound = false;
    }
    if self.y_bound {
        yoff = 0.0;
        self.y_bound = false;
    }
    self.mouse_pos = position.cast();
    let sensitivity = 0.05;
    xoff *= sensitivity;
    yoff *= sensitivity;
    //println!("{} {}", xoff, yoff);
    self.camera.pitch(yoff);
    self.camera.yaw(xoff);
    if self.camera.pitch > 89.0 {
        self.camera.pitch = 89.0;
    }
    if self.camera.pitch < -89.0 {
        self.camera.pitch = -89.0;
    }
    self.camera.transform();

    self.check_mouse_x();
    self.check_mouse_y();*/
    glutin::event_loop::ControlFlow::Poll
}
fn handle_window_event(event: WindowEvent)-> ControlFlow {
    let mut cf = ControlFlow::Poll;
    match event {
        WindowEvent::CloseRequested => {
            cf = ControlFlow::Exit;
        }
        WindowEvent::KeyboardInput {input, ..} => {
            cf = handle_keyboard_input(input);
        }
        WindowEvent::CursorMoved {position, ..} => {
            //cf = handle_mouse_input(position);
        }
        _ => {}
    }
    cf
}
fn main() {
    let mut ev = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &ev).unwrap();
    let engine = App::new(display.get_framebuffer_dimensions());
    
    let shape = engine.world.vertices();
    let vbuffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let ibuffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &(0..shape.len() as u16).collect::<Vec<_>>()).unwrap();
    let program = glium::Program::from_source(&display, &engine.vertex_shader, &engine.fragment_shader, None).unwrap();
    let dimensions = engine.texture.dimensions();
    let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&engine.texture.into_raw(), dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, raw).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };
    ev.run(move |ev, _, cf| {
        let mut target = display.draw();
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *cf = ControlFlow::WaitUntil(next_frame_time);
        let uniforms = uniform! {
            model: *engine.model.as_ref(),
            view: *engine.camera.view.as_ref(),
            projection: *engine.camera.projection.as_ref(),
            tex: &texture,

        };
        target.clear_color_and_depth((0.40, 0.58, 0.93, 1.0), 1.0);
        target.draw(&vbuffer, &ibuffer, &program, &uniforms, &params).unwrap();
        target.finish().unwrap();
        match ev {
            Event::WindowEvent {event, ..} => {
                *cf = handle_window_event(event);
            }
            _ => {
                *cf = ControlFlow::Poll;
            }
        }
    });
}
