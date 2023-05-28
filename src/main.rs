mod app;
mod ray;
mod camera;
mod color;
mod voxel;

pub use ray::*;
pub use camera::*;
pub use color::*;
pub use voxel::*;

fn main() {
    env_logger::init();
    let mut v0 = Voxel::new([0.;3].into(), [20;3], *GREEN);
    v0.fill_rect([0, 0, 0], [20, 10, 20], true);
    app::App::new(400, 300)
        .set_camera_pos([0., 0., 40.])
        .add_voxel(v0)
        .start()
}