#[inline(always)]
pub fn radians(degree: f32) -> f32 {
    (degree * std::f32::consts::PI) / 180.
}