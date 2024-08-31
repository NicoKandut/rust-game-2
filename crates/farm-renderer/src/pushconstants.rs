#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PushConstants {
    time: f32,
    camera_position: [f32; 2],
    camera_zoom: f32,
}

impl Default for PushConstants {
    fn default() -> Self {
        Self {
            time: 0.0,
            camera_position: [0.0, 0.0],
            camera_zoom: 1.0,
        }
    }
}

impl PushConstants {
    pub fn set_time(&mut self, time: f32) {
        self.time = time;
    }

    pub fn set_camera_position(&mut self, position: [f32; 2]) {
        self.camera_position = position;
    }

    pub fn set_camera_zoom(&mut self, zoom: f32) {
        self.camera_zoom = zoom;
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts((self as *const Self) as *const u8, size_of::<Self>())
        }
    }
}