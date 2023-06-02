mod app;     pub use app::*;
mod ray;     pub use ray::*;
mod camera;  pub use camera::*;
mod color;   pub use color::*;
mod voxel;   pub use voxel::*;
mod vector;  pub use vector::*;
mod utils;   pub use utils::*;

fn main() {
    let mut app = app::App::new(400, 300);
    app.camera.position = [0., 0., 40.].into();
    
    let mut v0 = Voxel::new(0.0.into(), [20;3], (0, 255, 0).into());
    v0.fill_rect([0, 0, 0], [20, 10, 20], true);
    app.voxels.push(v0);

    app.start();
}
