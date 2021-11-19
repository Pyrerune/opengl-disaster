mod shader;
mod vertex;
mod app;
mod shape;
mod camera;

#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

use glium::glutin;
use crate::app::App;
use glium::glutin::event_loop::EventLoop;

fn event_loop(ev: EventLoop<()>) {

    let mut app = App::new(&ev,"Test World");
    ev.run(move |event, _, control_flow| {
        app.render();
        let frame_time = std::time::Instant::now() + std::time::Duration::from_micros(20);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(frame_time);
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    glutin::event_loop::ControlFlow::Exit
                },
                glutin::event::WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    app.cursor_moved(position)
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    app.keyboard_input(input)
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
        app.key_delay += 1;
        app.mouse_delay += 1;
    });

}
fn main() {
    //TODO implement lighting
    //TODO calculate normal vectors
    let ev = glutin::event_loop::EventLoop::new();

    event_loop(ev);
}
