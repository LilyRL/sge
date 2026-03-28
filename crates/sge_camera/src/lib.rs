use bevy_math::{Mat4, Vec2};
use d2::{projection, Camera2D};
use d3::Camera3D;
use global::global;

pub mod d2;
pub mod d3;

#[derive(Clone, Debug, Copy)]
pub struct Cameras {
    pub flat: Mat4,
    pub d2: Camera2D,
    pub d3: Camera3D,
    pub flip_y: bool,
}

impl Cameras {
    pub fn set_flip_y(&mut self, value: bool) {
        self.flip_y = value;
        self.d2.set_flip_y(value);
    }
}

global!(Cameras, cameras);

pub fn init(width: u32, height: u32, flip_y: bool) {
    let flat = projection(width, height, flip_y);
    let d2 = Camera2D::new(width, height, flip_y);
    let d3 = Camera3D::new(width, height);

    set_cameras(Cameras {
        flat,
        d2,
        d3,
        flip_y,
    });
    log::info!("Initialized cameras");
}

pub fn update_cameras_on_resize(width: u32, height: u32) {
    let cameras = get_cameras();

    cameras.d2.update_sizes(width, height);
    cameras.d3.update_sizes(width, height);
    cameras.flat = projection(width, height, cameras.flip_y);
}

pub fn get_camera_2d() -> &'static Camera2D {
    &get_cameras().d2
}

pub fn get_flat_projection() -> Mat4 {
    get_cameras().flat
}

pub fn get_camera_3d() -> &'static Camera3D {
    &get_cameras().d3
}

pub fn get_camera_3d_mut() -> &'static mut Camera3D {
    &mut get_cameras().d3
}

pub fn get_camera_2d_mut() -> &'static mut Camera2D {
    &mut get_cameras().d2
}

pub fn camera2d_zoom_at(screen_pos: Vec2, zoom_factor: f32) {
    get_camera_2d_mut().zoom_at(screen_pos, zoom_factor);
}

pub fn cameras_for_resolution(width: u32, height: u32) -> Cameras {
    let current = get_cameras();
    let mut d2 = current.d2;
    d2.update_sizes(width, height);
    let flat = projection(width, height, current.flip_y);
    let mut d3 = current.d3;
    d3.update_sizes(width, height);
    Cameras {
        flat,
        d2,
        d3,
        flip_y: current.flip_y,
    }
}

pub fn screen_to_world(screen_pos: Vec2) -> Vec2 {
    get_camera_2d_mut().screen_to_world(screen_pos)
}

pub fn world_to_screen(world_pos: Vec2) -> Vec2 {
    get_camera_2d_mut().world_to_screen(world_pos)
}
