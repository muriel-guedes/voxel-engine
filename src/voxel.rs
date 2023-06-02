use bitvec::BitVec;

use crate::{Ray, Color, Vector};

pub enum Intersection {
    Inside,
    Outside(f32),
    None
}

pub struct Voxel {
    pub size: [i16;3],
    pub bounds: [Vector;2],
    pub data: BitVec<u32>,
    pub color: Color
}
impl Voxel {
    pub fn new(
        center: Vector,
        size: [i16;3],
        color: Color
    ) -> Self {
        assert!(size[0]>0 && size[1]>0 && size[2]>0, "All sides of the Voxel must bigger than 0");
        let mut data = BitVec::<u32>::new();
        data.resize((size[0] * size[1] * size[2])as usize);
        let half_size = Vector::new(size[0] as f32, size[1] as f32, size[2] as f32) / 2.;
        Self {
            size,
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
        self.data.get((x + y * self.size[0] + z * (self.size[0] * self.size[1]))as usize)
    }

    pub fn walk(&self, ray: &Ray, t: f32) -> Option<Color> {
        let p = ray.at(t) - self.bounds[0];
        let dx = if ray.direction.x > 0. { 1 } else { -1 };
        let dy = if ray.direction.y > 0. { 1 } else { -1 };
        let dz = if ray.direction.z > 0. { 1 } else { -1 };
        loop {
            
        }
        None
    }

    pub fn fill_with(&mut self, v: bool) {
        for i in 0..(self.size[0] * self.size[1] * self.size[2])as usize {
            self.data.set(i, v)
        }
    }
    pub fn fill_rect(&mut self, from: [usize;3], to: [usize;3], v: bool) {
        assert!(from[0]<to[0] && from[1]<to[1] && from[2]<to[2], "'from' must be less than 'to'");
        let hw = (self.size[0] * self.size[1])as usize;
        let mut y;
        let mut z;
        for x in from[0]..to[0] {
            y = from[1] * self.size[0] as usize;
            for _ in from[1]..to[1] {
                z = from[2] * hw;
                for _ in from[2]..to[2] {
                    self.data.set(x + y + z, v);
                    z += hw;
                }
                y += self.size[0] as usize;
            }
        }
    }
}