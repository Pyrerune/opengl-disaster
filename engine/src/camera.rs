use crate::FAR;

#[derive(Debug, Clone)]
pub struct Camera {
    view: glm::TMat4<f32>,
    projection: glm::TMat4<f32>,
    position: glm::TVec3<f32>,
    front: glm::TVec3<f32>,
    up: glm::TVec3<f32>,
    delta_time: f32,
    direction: glm::TVec3<f32>,
    pub(crate) pitch: f32,
    pub(crate) yaw: f32,
}
impl Camera {
    pub fn new(display: (u32, u32)) -> Camera {
        let position = glm::vec3(8.0, 129.0, 8.0);
        let target = glm::vec3(0.0, 0.0, 0.0);

        let direction = glm::normalize(&(position-target));
        let up = glm::vec3(0.0, 1.0, 0.0);
        let front = glm::vec3(0.0, 0.0, -1.0);
        let projection = glm::perspective((display.0/display.1) as f32, glm::radians(&glm::vec1(45.0)).x, 0.1, FAR);
        let view = glm::look_at(&position, &(position + front), &up);
        Camera {
            view,
            projection,
            position,
            front,
            up,
            direction,
            delta_time: 0.0,
            pitch: 0.0,
            yaw: -90.0,
        }
    }
    fn update(&mut self) {
        let view = glm::look_at(&self.position, &(self.position + self.front), &self.up);
        self.set_view(view);

    }
    pub fn get_position(&self) -> [f32;3] {
        *self.position.as_ref()
    }
    pub fn set_time(&mut self, delta: f32) {
        self.delta_time = delta;
    }
    pub fn up(&mut self, step: f32) {
        self.position += step * self.delta_time * self.up;
    }
    pub fn down(&mut self, step: f32) {
        self.position -= step * self.delta_time * self.up;
    }
    pub fn forward(&mut self, step: f32) {
        self.position += step * self.delta_time * self.front;
        self.update();
    }
    pub fn backward(&mut self, step: f32) {
        self.position -= step * self.delta_time * self.front;
        self.update();
    }
    pub fn left(&mut self, step: f32) {
        self.position -= glm::normalize(&glm::cross(&self.front, &self.up)) * step * self.delta_time;
        self.update();

    }
    pub fn pitch(&mut self, pitch: f32) {
        self.pitch += pitch;
    }
    pub fn yaw(&mut self, yaw: f32) {
        self.yaw += yaw;
    }
    pub fn transform(&mut self) {
        let r_yaw = glm::radians(&glm::vec1(self.yaw)).x;
        let r_pitch = glm::radians(&glm::vec1(self.pitch)).x;
        self.direction.x = r_yaw.cos() * r_pitch.cos();
        self.direction.z = r_yaw.sin() * r_pitch.cos();
        self.direction.y = r_pitch.sin();
        self.front = glm::normalize(&self.direction);
        self.update();
    }
    pub fn right(&mut self, step: f32) {
        self.position += glm::normalize(&glm::cross(&self.front, &self.up)) * step * self.delta_time;
        self.update();
    }
    pub fn view(&self) -> glm::TMat4<f32> {
        self.view
    }
    pub fn set_view(&mut self, view: glm::TMat4<f32>) {
        self.view = view;
    }
    pub fn projection(&self) -> glm::TMat4<f32> {
        self.projection
    }
}