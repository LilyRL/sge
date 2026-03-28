use sge_input::mouse_diff;
use sge_window::{grab_cursor, release_cursor};

use crate::get_camera_3d_mut;

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
        Self {
            mouse_sensitivity,
            enable: false,
        }
    }

    pub fn enable(&mut self) {
        self.enable = true;
    }

    pub fn disable(&mut self) {
        self.enable = false;
    }

    pub fn update(&self) {
        if self.enable {
            let _ = grab_cursor();
            let camera = get_camera_3d_mut();

            let delta = mouse_diff() * self.mouse_sensitivity;
            camera.pitch(-delta.y);
            camera.yaw(-delta.x);
        } else {
            let _ = release_cursor();
        }
    }
}
