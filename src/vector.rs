use std::ops::{Add, Mul, Div, Sub, AddAssign};

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vector {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let l = ((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt();
        if l == 0. {
            Self { x: 0., y: 0., z: 0. }
        }
        else {
            Self {
                x: self.x / l,
                y: self.y / l,
                z: self.z / l
            }
        }
    }
    #[inline(always)]
    pub fn rotate_x(self, rad: f32) -> Self {
        let c = rad.cos();
        let s = rad.sin();
        Self {
            x: self.x,
            y: self.y *  c + self.z * s,
            z: self.y * -s + self.z * c
        }
    }
    #[inline(always)]
    pub fn rotate_y(self, rad: f32) -> Self {
        let c = rad.cos();
        let s = rad.sin();
        Self {
            x: self.x * c + self.z * -s,
            y: self.y,
            z: self.x * s + self.z * c
        }
    }
}

impl From<f32> for Vector {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self { x: value, y: value, z: value }
    }
}
impl From<[f32;3]> for Vector {
    #[inline(always)]
    fn from(value: [f32;3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    #[inline(always)]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl AddAssign<Vector> for Vector {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Sub<Vector> for Vector {
    type Output = Vector;
    #[inline(always)]
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
impl Mul<Vector> for Vector {
    type Output = Vector;
    #[inline(always)]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}
impl Mul<f32> for Vector {
    type Output = Vector;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}
impl Mul<Vector> for f32 {
    type Output = Vector;
    #[inline(always)]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}
impl Div<f32> for Vector {
    type Output = Vector;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}