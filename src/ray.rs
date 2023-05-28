use cgmath::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub inv_dir: Vector3<f32>,
    /// 1 = negative
    /// 0 = positive
    pub sign: [usize;3],
    /// 1 = positive
    /// 0 = negative
    pub inv_sign: [usize;3]
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        let inv_dir = Vector3::new(
            if direction.x != 0. { 1. / direction.x } else { f32::MAX },
            if direction.y != 0. { 1. / direction.y } else { f32::MAX },
            if direction.z != 0. { 1. / direction.z } else { f32::MAX }
        );
        Self {
            origin,
            direction,
            inv_dir,
            sign: [(inv_dir.x < 0.)as usize, (inv_dir.y < 0.)as usize, (inv_dir.z < 0.)as usize],
            inv_sign: [(inv_dir.x >= 0.)as usize, (inv_dir.y >= 0.)as usize, (inv_dir.z >= 0.)as usize]
        }
    }
    pub fn at(&self, d: f32) -> Vector3<f32> {
        self.origin + d * self.direction
    }
}