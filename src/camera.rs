use cgmath::{Vector3, Rad, Vector2};

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector2<f32>,

    pub mov_speed: Vector3<f32>,
    pub rot_speed: Vector2<f32>,

    pub fov: Rad<f32>
}