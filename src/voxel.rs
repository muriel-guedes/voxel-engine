use cgmath::Vector3;
use bitvec::BitVec;

use crate::{Ray, Color};

pub enum Intersection {
    Inside,
    Outside(f32),
    None
}

pub struct Voxel {
    pub size: Vector3<i16>,
    pub bounds: [Vector3<f32>;2],
    pub data: BitVec<u32>,
    pub color: Color
}
impl Voxel {
    pub fn new(
        center: Vector3<f32>,
        size: [usize;3],
        color: Color
    ) -> Self {
        let mut data = BitVec::<u32>::new();
        data.resize(size[0] * size[1] * size[2]);
        let half_size = Vector3::new(size[0] as f32, size[1] as f32, size[2] as f32) / 2.;
        Self {
            size: Vector3::new(size[0] as i16, size[1] as i16, size[2] as i16),
            bounds: [center-half_size, center+half_size],
            data,
            color: color.into()
        }
    }

    pub fn get_intersection(&self, ray: &Ray) -> Intersection {
        if
            ray.origin.x < self.bounds[0].x &&
            ray.origin.y < self.bounds[0].y &&
            ray.origin.z < self.bounds[0].z &&
            ray.origin.x > self.bounds[1].x &&
            ray.origin.y > self.bounds[1].y &&
            ray.origin.z > self.bounds[1].z
        {
            return Intersection::Inside
        }
        
        let mut tmin = (self.bounds[ray.sign[0]].x - ray.origin.x) * ray.inv_dir.x;
        let mut tmax = (self.bounds[ray.inv_sign[0]].x - ray.origin.x) * ray.inv_dir.x;

        let tymin = (self.bounds[ray.sign[1]].y - ray.origin.y) * ray.inv_dir.y;
        let tymax = (self.bounds[ray.inv_sign[1]].y - ray.origin.y) * ray.inv_dir.y;

        if (tmin > tymax) || (tymin > tmax) { return Intersection::None }

        if tymin > tmin { tmin = tymin }
        if tymax < tmax { tmax = tymax }

        let tzmin = (self.bounds[ray.sign[2]].z - ray.origin.z) * ray.inv_dir.z;
        let tzmax = (self.bounds[ray.inv_sign[2]].z - ray.origin.z) * ray.inv_dir.z;
        
        if (tmin > tzmax) || (tzmin > tmax) { return Intersection::None }

        if tzmin > tmin { tmin = tzmin }
        // if tzmax < tmax { tmax = tzmax }

        if tmin <= 0. { return Intersection::None }
        
        Intersection::Outside(tmin)
    }

    pub fn get_bit(&self, x: i16, y: i16, z: i16) -> bool {
        self.data.get((x + y * self.size.x + z * (self.size.x * self.size.y))as usize)
    }

    pub fn walk(&self, ray: &Ray, t: f32) -> Option<Color> {
        let p = ray.at(t) - self.bounds[0];
        let mut x = p.x.ceil().min(1.) as i16 - 1;
        let mut y = p.y.ceil().min(1.) as i16 - 1;
        let mut z = p.z.ceil().min(1.) as i16 - 1;
        let mut ex = p.x - p.x.floor();
        let mut ey = p.y - p.y.floor();
        let mut ez = p.z - p.z.floor();
        loop {
            if self.get_bit(x, y, z) { return Some(self.color) }
            if ex < ey && ex < ez {
                ex += ray.direction.x;
                x = p.x.ceil().min(1.) as i16 - 1;
                if x < 0 || x >= self.size.x { return None }
            } else if ey < ez {
                ey += ray.direction.y;
                y = p.y.ceil().min(1.) as i16 - 1;
                if y < 0 || y >= self.size.y { return None }
            } else {
                ez += ray.direction.z;
                z = p.z.ceil().min(1.) as i16 - 1;
                if z < 0 || z >= self.size.z { return None }
            }
        }
    }

    pub fn fill_with(&mut self, v: bool) {
        for i in 0..(self.size.x * self.size.y * self.size.z)as usize {
            self.data.set(i, v)
        }
    }
    pub fn fill_rect(&mut self, from: [usize;3], to: [usize;3], v: bool) {
        assert!(from[0]<to[0] && from[1]<to[1] && from[2]<to[2]);
        let hw = (self.size.x * self.size.y)as usize;
        let mut y;
        let mut z;
        for x in from[0]..to[0] {
            y = from[1] * self.size.x as usize;
            for _ in from[1]..to[1] {
                z = from[2] * hw;
                for _ in from[2]..to[2] {
                    self.data.set(x + y + z, v);
                    z += hw;
                }
                y += self.size.x as usize;
            }
        }
    }
}