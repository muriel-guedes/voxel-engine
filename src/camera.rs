use crate::Vector;

pub struct Camera {
    pub position: Vector,
    pub rotation: [f32;2],

    pub mov_speed: Vector,
    pub rot_speed: [f32;2],

    pub fov: f32
}
impl Camera {
    pub fn move_forward(&mut self) {
        self.position += Vector::new(0., 0., -self.mov_speed.z).rotate_y(self.rotation[1])
    }
    pub fn move_backwards(&mut self) {
        self.position += Vector::new(0., 0., self.mov_speed.z).rotate_y(self.rotation[1])
    }
    pub fn move_left(&mut self) {
        self.position += Vector::new(-self.mov_speed.x, 0., 0.).rotate_y(self.rotation[1])
    }
    pub fn move_right(&mut self) {
        self.position += Vector::new(self.mov_speed.x, 0., 0.).rotate_y(self.rotation[1])
    }
    pub fn move_up(&mut self) {
        self.position.y += self.mov_speed.y
    }
    pub fn move_down(&mut self) {
        self.position.y -= self.mov_speed.y
    }
}