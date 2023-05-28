pub type ColorType = u8;
const RED_BITS: usize = 3;
const GREEN_BITS: usize = 2;
const BLUE_BITS: usize = 3;
const RED_MAX: ColorType = max_value::<RED_BITS>();
const GREEN_MAX: ColorType = max_value::<GREEN_BITS>();
const BLUE_MAX: ColorType = max_value::<BLUE_BITS>();

lazy_static::lazy_static! {
    pub static ref WHITE:   Color = (1., 1., 1.).into();
    pub static ref BLACK:   Color = (0., 0., 0.).into();
    pub static ref RED:     Color = (1., 0., 0.).into();
    pub static ref GREEN:   Color = (0., 1., 0.).into();
    pub static ref BLUE:    Color = (0., 0., 1.).into();
    pub static ref YELLOW:  Color = (1., 1., 0.).into();
    pub static ref CYAN:    Color = (0., 1., 1.).into();
    pub static ref MAGENTA: Color = (1., 0., 1.).into();
    pub static ref GRAY:    Color = (0.5, 0.5, 0.5).into();
}

#[derive(Copy, Clone)]
pub struct Color(pub ColorType);
impl Color {
    #[inline(always)]
    pub fn red(&self) -> f32 {
        (self.0 >> (GREEN_BITS + BLUE_BITS)) as f32 / RED_MAX as f32
    }
    #[inline(always)]
    pub fn green(&self) -> f32 {
        ((self.0 << RED_BITS) >> (GREEN_BITS + BLUE_BITS)) as f32 / GREEN_MAX as f32
    }
    #[inline(always)]
    pub fn blue(&self) -> f32 {
        ((self.0 << (RED_BITS + GREEN_BITS)) >> (GREEN_BITS + BLUE_BITS)) as f32 / BLUE_MAX as f32
    }
    #[inline(always)]
    pub fn draw(&self, frame: &mut [u8], i: usize) {
        frame[i]   = (self.red()   * 255.) as u8;
        frame[i+1] = (self.green() * 255.) as u8;
        frame[i+2] = (self.blue()  * 255.) as u8;
        frame[i+3] = 255;
    }
}

impl From<(u8, u8, u8)> for Color {
    #[inline(always)]
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        (r as f32 / 255., g as f32 / 255., b as f32 / 255.).into()
    }
}
impl From<(f32, f32, f32)> for Color {
    #[inline(always)]
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        let r = if r > 1. { RED_MAX } else if r < 0. { 0 } else {
            ((r * RED_MAX as f32) as ColorType) << (GREEN_BITS + BLUE_BITS)
        };
        let g = if g > 1. { GREEN_MAX } else if g < 0. { 0 } else {
            ((g * GREEN_MAX as f32) as ColorType) << BLUE_BITS
        };
        let b = if b > 1. { BLUE_MAX } else if b < 0. { 0 } else {
            (b * BLUE_MAX as f32) as ColorType
        };
        Color(r + g + b)
    }
}

const fn max_value<const BITS: usize>() -> ColorType {
    let mut res: ColorType = 0;
    let mut i = 0;
    while i < BITS {
        res = (res << 1) + 1;
        i += 1
    }
    res
}