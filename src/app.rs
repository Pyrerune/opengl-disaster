use glium::glutin::event_loop::EventLoop;
use glium::{glutin, Program, Surface, Frame};
use crate::shape::Shape;
use crate::shader::Shader;
use crate::camera::Camera;
use glium::glutin::event::{KeyboardInput, VirtualKeyCode};
use std::time::Instant;
use glium::glutin::dpi::PhysicalPosition;
use std::path::Path;

const STEP: f32 = 100.0;
fn mat4(val: f32) -> glm::TMat4<f32> {
    glm::mat4(val, 0.0, 0.0, 0.0,
                0.0, val, 0.0, 0.0,
              0.0, 0.0, val, 0.0,
                    0.0, 0.0, 0.0, val)
}
fn load_image<P: AsRef<Path>>(display: &glium::Display, filename: P) -> glium::texture::SrgbTexture2d {
    let image = image::io::Reader::open(filename).unwrap().decode()
        .unwrap().to_rgba8();
    let dimensions = image.dimensions();
    let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
    glium::texture::SrgbTexture2d::new(display, raw).unwrap()
}
#[derive(Debug)]
pub struct App {
    pub display: glium::Display,
    pub program: Program,
    pub camera: Camera,
    start_time: Instant,
    last_frame: f32,
    current_frame: f32,
    pub mouse_pos: PhysicalPosition<f32>,
    x_bound: bool,
    y_bound: bool,
    texture: glium::texture::SrgbTexture2d,
    world: Vec<Shape>,

}
impl App {

    pub fn new<T: Into<String>>(ev: &EventLoop<()>, title: T) -> App {
        let wb = glutin::window::WindowBuilder::new().with_title(title);
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, ev).unwrap();
        let default_shader = Shader::default();
        let program = glium::Program::from_source(&display, &default_shader.get_vertex(), &default_shader.get_fragment(), None).unwrap();
        let viewport = display.get_framebuffer_dimensions();
        let time = std::time::Instant::now();
        //display.gl_window().window().set_cursor_visible(false);
        display.gl_window().window().set_cursor_grab(true).unwrap();
        let texture = load_image(&display, "./icon.png");
        let world = Shape::experimental_plane(&display, 48, 256, 48, [0, 0, 0]);
        display.gl_window().window().set_cursor_position(PhysicalPosition::new(viewport.0 as f32/2 as f32, viewport.1 as f32/2 as f32)).unwrap();
        App {
            display: display.clone(),
            program,
            camera: Camera::new(viewport),
            start_time: time,
            current_frame: 0.0,
            last_frame: 0.0,
            x_bound: false,
            y_bound: false,
            mouse_pos: PhysicalPosition::new(viewport.0 as f32/2 as f32, viewport.1 as f32/2 as f32),
            texture,
            world,
        }
    }

    pub fn draw(&self, shape: &Shape, target: &mut Frame) {
        let light_color: [f32;3] = [1.0, 1.0, 1.0];
        let light_pos: [f32;3] = [-2.0, 2.0, 2.0];
        let uniforms = uniform! {
            model: *mat4(1.0).as_ref(),
            view: *self.camera.view().as_ref(),
            projection: *self.camera.projection().as_ref(),
            lightColor: light_color,
            lightPos: light_pos,
            objectColor: [0.0, 0.0, 0.0],
            tex: &self.texture,

        };
        let params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()

        };
        target.draw(shape.get_vbuffer(), shape.get_ibuffer(), &self.program, &uniforms, &params).unwrap();
    }
    pub fn render(&mut self) {
        self.current_frame = self.start_time.elapsed().as_secs_f32();
        self.camera.set_time(self.current_frame - self.last_frame);
        self.last_frame = self.current_frame;
        let mut target = self.display.draw();
        target.clear_color_and_depth((100.0/255.0, 149.0/255.0, 237.0/255.0, 1.0), 1.0);

        for chunk in &self.world {
            self.draw(chunk, &mut target);
        }
        target.finish().unwrap();
    }
    pub fn keyboard_input(&mut self, input: KeyboardInput) -> glutin::event_loop::ControlFlow {
        let mut cf = glutin::event_loop::ControlFlow::Poll;
        if let Some(key) = input.virtual_keycode {
            match key {
                VirtualKeyCode::Up => {
                    self.camera.forward(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Down => {
                    self.camera.backward(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Left => {
                    self.camera.left(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Right => {
                    self.camera.right(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Escape => {
                    cf = glutin::event_loop::ControlFlow::Exit;
                }
                VirtualKeyCode::E => {
                    self.camera.up(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Q => {
                    self.camera.down(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }

                _ => {}
            }
        }
        cf
    }
    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) -> glutin::event_loop::ControlFlow {
        //println!("Got input");
        let mut xoff = position.x as f32 - self.mouse_pos.x;
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
        self.check_mouse_y();
        glutin::event_loop::ControlFlow::Poll
    }
    fn check_mouse_y(&mut self) {
        let (_, y) = self.display.get_framebuffer_dimensions();
        let lower_y = y / 8;
        let upper_y = (y / 2) + lower_y;
        if (self.mouse_pos.y >= (upper_y) as f32 || self.mouse_pos.y <= (lower_y as f32)) && !self.y_bound {
            self.mouse_pos.y  = upper_y as f32 / 2.0;
            self.y_bound = true;
            self.display.gl_window().window().set_cursor_position(self.mouse_pos).unwrap();
        }
    }
    fn check_mouse_x(&mut self) {
        let (x, _) = self.display.get_framebuffer_dimensions();
        let lower_x = x / 8;
        let upper_x = (x / 2) + lower_x;
        if (self.mouse_pos.x >= (upper_x) as f32 || self.mouse_pos.x <= (lower_x) as f32) && !self.x_bound {
            self.mouse_pos.x = upper_x as f32 / 2.0;
            self.x_bound = true;
            self.display.gl_window().window().set_cursor_position(self.mouse_pos).unwrap();
        }
    }
}