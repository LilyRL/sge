use sge_camera::get_camera_3d_mut;
use sge_input::mouse_diff;
use sge_window::{grab_cursor, release_cursor};

pub struct FirstPersonCameraController {
    mouse_sensitivity: f32,
    enable: bool,
}

impl FirstPersonCameraController {
    pub fn new(mouse_sensitivity: f32) -> Self {
        Self {
            mouse_sensitivity,
            enable: true,
        }
    }
    pub fn disabled(mouse_sensitivity: f32) -> Self {
        let mut s = Self::new(mouse_sensitivity);
        s.enable = false;
        s
    }

    pub fn enable(&mut self) {
        self.enable = true;
    }
    pub fn disable(&mut self) {
        self.enable = false;
    }

    pub fn update(&mut self) {
        let camera = get_camera_3d_mut();

        if self.enable {
            let _ = grab_cursor();
            let delta = mouse_diff() * self.mouse_sensitivity;
            camera.pitch(-delta.y);
            camera.yaw(-delta.x);
        } else {
            let _ = release_cursor();
        }
    }
}
