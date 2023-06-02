use std::collections::HashSet;
use pixels::{SurfaceTexture, Pixels};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, Window}, dpi::PhysicalSize,
    event::{Event, WindowEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode}};

use crate::{Camera, Ray, Voxel, Intersection, radians, Vector, Color};

pub struct App {
    event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pixels: Pixels,
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub background: Color,
    pub camera: Camera,
    pub voxels: Vec<Voxel>
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&event_loop).unwrap();
        window.focus_window();
    
        let pixels = Pixels::new(width, height, SurfaceTexture::new(width, height, &window)).unwrap();
        
        let camera = Camera {
            position: 0.0.into(),
            rotation: [0.;2],

            mov_speed: 1.0.into(),
            rot_speed: [0.1;2],

            fov: radians(45.)
        };
        
        Self {
            event_loop: Some(event_loop),
            window,
            pixels,
            pressed_keys: Default::default(),
            background: (0, 0, 0).into(),
            camera,
            voxels: Vec::new()
        }
    }
    pub fn start(mut self) {
        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input: KeyboardInput { state: Pressed, virtual_keycode: Some(key), .. }, .. } => {
                        if let VirtualKeyCode::Escape = key {
                            *control_flow = ControlFlow::Exit
                        }
                        self.pressed_keys.insert(key);
                    },
                    WindowEvent::KeyboardInput { input: KeyboardInput { state: Released, virtual_keycode: Some(key), .. }, .. } => {
                        self.pressed_keys.remove(&key);
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => {
                        self.pixels.resize_buffer(new_size.width, new_size.height).unwrap();
                        self.pixels.resize_surface(new_size.width, new_size.height).unwrap();
                    },
                    _ => {}
                }
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    self.render();
                    self.pixels.render().unwrap();
                }
                _ => {}
            }
        })
    }
    pub fn render(&mut self) {
        for key in self.pressed_keys.iter() {
            match key {
                VirtualKeyCode::W => self.camera.move_forward(),
                VirtualKeyCode::S => self.camera.move_backwards(),
                VirtualKeyCode::A => self.camera.move_left(),
                VirtualKeyCode::D => self.camera.move_right(),
                VirtualKeyCode::E => self.camera.move_up(),
                VirtualKeyCode::Q => self.camera.move_down(),
                VirtualKeyCode::I => self.camera.rotation[0] -= self.camera.rot_speed[0],
                VirtualKeyCode::K => self.camera.rotation[0] += self.camera.rot_speed[0],
                VirtualKeyCode::L => self.camera.rotation[1] += self.camera.rot_speed[1],
                VirtualKeyCode::J => self.camera.rotation[1] -= self.camera.rot_speed[1],
                _ => {}
            }
        }

        let window_size = self.window.inner_size();
        let frame = self.pixels.frame_mut();

        let aspect_ratio = window_size.width as f32 / window_size.height as f32;
        let height = (self.camera.fov / 2.).tan();
        let viewport_height = height * 2.;
        let viewport_width = viewport_height * aspect_ratio;

        let mut i = 0;
        for y in 0..window_size.height {
            for x in 0..window_size.width {
                let v = ((y as f32 + 0.5) / (window_size.height - 1) as f32) * 2. - 1.;
                let h = ((x as f32 + 0.5) / (window_size.width - 1) as f32) * 2. - 1.;

                let direction = Vector::new(h * viewport_width, -v * viewport_height, -1.)
                    .rotate_x(self.camera.rotation[0])
                    .rotate_y(self.camera.rotation[1])
                    .normalize();
                let ray = Ray::new(self.camera.position, direction);

                let mut min_dis = f32::MAX;
                let mut intr_voxel = None;

                for voxel in &self.voxels {
                    match voxel.get_intersection(&ray) {
                        Intersection::Outside(t) => {
                            if t < min_dis { min_dis = t } else { continue }
                            intr_voxel = Some(voxel)
                        },
                        Intersection::Inside => {
                            min_dis = 0.;
                            intr_voxel = Some(voxel);
                            break
                        },
                        Intersection::None => {}
                    }
                }

                let mut intr_color = self.background;
                
                if let Some(voxel) = intr_voxel {
                    if let Some(color) = voxel.walk(&ray, min_dis) {
                        intr_color = color
                    }
                }

                intr_color.draw(frame, i);

                i += 4;
            }
        }
    }
}